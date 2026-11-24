//! Texture contents the title writes itself (decision 0007, "Texture
//! memory"): streamed pages, layout-library copies, movie frames, glyph
//! atlases. Such a texture lives in console memory in its tiled layout;
//! before a submission samples its host copy, the memory is detiled into a
//! staging buffer when its content changed, and the staging buffer is
//! copied into the host texture.
//!
//! "Changed" means a page of the memory was written since the last detiling
//! ([`crate::watch`]), in write-watched memory; in physical-page views,
//! which cannot be watched, a content hash of the whole memory at every
//! submission that samples it.
//!
//! shortcut: a change detiles the whole texture, not the pages written.
//! Only textures read through descriptor tables are brought up to date
//! (directly indexed views are not known per draw); plane 0 is uploaded
//! (every mip and array slice), depth and stencil planes are not.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use windows::Win32::Graphics::Direct3D12::*;

use crate::host::{HostList, host};
use crate::object::This;
use crate::resource::HostTexture;

macro_rules! once {
    ($($arg:tt)*) => {{
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| tracing::warn!($($arg)*));
    }};
}

/// A texture's upload state: the staging buffer laid out as the host
/// texture's subresources, and what was last detiled.
pub(crate) struct Staging {
    buffer: ID3D12Resource,
    /// Its bytes.
    #[cfg(feature = "diagnostics")]
    size: u64,
    data: *mut u8,
    footprints: Vec<D3D12_PLACED_SUBRESOURCE_FOOTPRINT>,
    rows: Vec<u32>,
    layout: victory_formats::texture::Layout,
    /// Mips per array slice.
    mips: u32,
    memory: *const u8,
    last: Mutex<Last>,
}

/// What a texture's staging buffer holds: the hash of the memory detiled
/// (memory not watched), or the write-watch asking it was detiled after,
/// with room to read the memory into.
#[derive(Default)]
struct Last {
    hash: Option<blake3::Hash>,
    asked: u64,
    read: Vec<u8>,
}

// SAFETY: the staging mapping is written only at submission, under
// `last`'s lock; the title memory is only read.
unsafe impl Send for Staging {}
// SAFETY: as above.
unsafe impl Sync for Staging {}

static STAGING: Mutex<Option<HashMap<usize, Arc<Staging>>>> = Mutex::new(None);

/// Textures the host GPU wrote by a copy or resolve: their contents are on
/// the host, not in title memory, which stays as it was.
static HOST_WRITTEN: Mutex<Option<std::collections::HashSet<usize>>> = Mutex::new(None);

/// A host copy or resolve writes `resource`.
pub(crate) fn host_written(resource: This) {
    let mut set = HOST_WRITTEN.lock().unwrap_or_else(|e| e.into_inner());
    set.get_or_insert_with(Default::default)
        .insert(resource as usize);
}

/// The title released a resource.
pub(crate) fn forget(resource: usize) {
    let mut map = STAGING.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(map) = map.as_mut() {
        map.remove(&resource);
    }
    let mut set = HOST_WRITTEN.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(set) = set.as_mut() {
        set.remove(&resource);
    }
}

/// Whether `resource`'s host copy takes its contents from title memory:
/// a texture the host GPU does not render to or write.
pub(crate) fn title_written(texture: &ID3D12Resource) -> bool {
    const TARGET_DEPTH_UNORDERED: i32 = 0x7;
    // SAFETY: a live resource.
    unsafe { texture.GetDesc() }.Flags.0 & TARGET_DEPTH_UNORDERED == 0
}

/// Records, once per list, the copy of `resource`'s staging buffer into
/// its host copy, and the detiling that refreshes the staging buffer at
/// submission if the memory changed. Leaves the texture in the copy
/// destination state.
pub(crate) fn upload(list: &HostList, resource: This, texture: &ID3D12Resource) {
    let host_written = HOST_WRITTEN
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_ref()
        .is_some_and(|set| set.contains(&(resource as usize)));
    // shortcut: a texture the host copied into keeps the host's contents
    // even if the title later writes its memory too.
    if host_written || !list.first_use(resource as usize) {
        return;
    }
    let Some(staging) = staging(resource, texture) else {
        return;
    };
    const COPY_DEST: u32 = 0x400;
    crate::list::ensure_state(list, resource, COPY_DEST);
    // The staging buffer lives as long as `staging`, which the deferred
    // refresh below holds until the list is reset.
    for (i, footprint) in staging.footprints.iter().enumerate() {
        #[cfg(feature = "diagnostics")]
        {
            let f = footprint.Footprint;
            // SAFETY: live resources.
            let (gpu, d) = unsafe { (staging.buffer.GetGPUVirtualAddress(), texture.GetDesc()) };
            let len = u64::from(f.RowPitch) * u64::from(staging.rows[i]) * u64::from(f.Depth);
            list.note_read(
                || {
                    format!(
                        "texture upload {resource:?} ({}x{}x{} f{} mips {}) sub {i} of {} from staging {gpu:#x}+{:#x}: offset {:#x} {}x{}x{} pitch {}",
                        d.Width,
                        d.Height,
                        d.DepthOrArraySize,
                        d.Format.0,
                        d.MipLevels,
                        staging.footprints.len(),
                        staging.size,
                        footprint.Offset,
                        f.Width,
                        f.Height,
                        f.Depth,
                        f.RowPitch
                    )
                },
                (gpu + footprint.Offset, len),
            );
        }
        let to = D3D12_TEXTURE_COPY_LOCATION {
            // SAFETY: a borrowed pointer the location never releases.
            pResource: unsafe { std::mem::transmute_copy(texture) },
            Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                SubresourceIndex: i as u32,
            },
        };
        let from = D3D12_TEXTURE_COPY_LOCATION {
            // SAFETY: as above.
            pResource: unsafe { std::mem::transmute_copy(&staging.buffer) },
            Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                PlacedFootprint: *footprint,
            },
        };
        // SAFETY: an open list; both locations name live resources.
        unsafe { list.list.CopyTextureRegion(&to, 0, 0, 0, &from, None) };
    }
    // Once per submission however many of its lists sample the texture.
    list.defer_texture(staging);
}

/// Whether placed copies are brought up to date from the resource that
/// writes their memory; on unless `VICTORY_PLACED_SYNC=0` (to measure it).
fn placed_sync_on() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var_os("VICTORY_PLACED_SYNC").as_deref() != Some(std::ffi::OsStr::new("0"))
    })
}

/// One subresource of a placed copy, staged: the writer's texels as a buffer
/// holds them, and the same bytes as the placed copy's format reads them.
struct Placement {
    /// With the writer's format (texture to buffer): where in the buffer the
    /// writer's subresource goes.
    from: D3D12_PLACED_SUBRESOURCE_FOOTPRINT,
    /// With the placed copy's format (buffer to texture): where the same
    /// bytes start for the placed copy's subresource.
    to: D3D12_PLACED_SUBRESOURCE_FOOTPRINT,
    /// The part of the subresource both textures have (the smaller of the
    /// two at this mip): the copy box at both ends, so neither can be asked
    /// for texels past its own mip.
    copy: D3D12_BOX,
}

/// The staged copy of one placed copy's contents from the resource that
/// writes its memory. The two formats are of different families (the writer
/// writes 32-bit words, the reader's is the console's packed one), which the
/// host refuses to copy a texture between, so the texels go through a buffer
/// that names the writer's format at one end and the placed copy's at the
/// other.
struct PlaceSync {
    buffer: ID3D12Resource,
    /// The descriptions the buffer and its footprints were made for. A
    /// released host texture's address comes back for another one, so a
    /// cached cell is only used while both descriptions are the same: the
    /// other one's formats, sizes and mips are what a copy is made of.
    from_desc: D3D12_RESOURCE_DESC,
    to_desc: D3D12_RESOURCE_DESC,
    /// The subresources, as one buffer holds them.
    subs: Vec<Placement>,
}

/// The recorded copies, by the two host textures' addresses.
type PlaceSyncs = HashMap<(usize, usize), Arc<PlaceSync>>;

static PLACE_SYNCS: Mutex<Option<PlaceSyncs>> = Mutex::new(None);

/// Copies of placed copies recorded since the frame line last read it.
static SYNCS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// The placed copies recorded since the last call (frame line, diagnostics).
pub(crate) fn take_syncs() -> u64 {
    SYNCS.swap(0, std::sync::atomic::Ordering::Relaxed)
}

/// The host's copyable layout of a texture: the bytes of the staging buffer
/// that holds it, and one footprint per subresource (`None` for a
/// description the host has no layout for).
struct Layout {
    total: u64,
    subs: Vec<D3D12_PLACED_SUBRESOURCE_FOOTPRINT>,
}

/// [`Layout`] of a description, from the host.
fn layout(desc: &D3D12_RESOURCE_DESC) -> Option<Layout> {
    let device = &host()?.device;
    let mips = u32::from(desc.MipLevels);
    let count = if desc.Dimension == D3D12_RESOURCE_DIMENSION_TEXTURE3D {
        mips
    } else {
        mips.checked_mul(u32::from(desc.DepthOrArraySize))?
    };
    if count == 0 {
        return None;
    }
    let mut subs = vec![D3D12_PLACED_SUBRESOURCE_FOOTPRINT::default(); count as usize];
    let mut total = 0u64;
    // SAFETY: outputs for `count` subresources.
    unsafe {
        device.GetCopyableFootprints(
            desc,
            0,
            count,
            0,
            Some(subs.as_mut_ptr()),
            None,
            None,
            Some(&mut total),
        );
    }
    (total != 0).then_some(Layout { total, subs })
}

/// The bytes one row of `sub` holds, without the pitch's padding, from the
/// description (the host's own row sizes are not the ones a copy reads by
/// for a 3D texture).
fn row_bytes(desc: &D3D12_RESOURCE_DESC, sub: u32) -> Option<u64> {
    let (bytes, block) = crate::resource::element(desc.Format.0 as u32)?;
    let mips = u32::from(desc.MipLevels).max(1);
    let mip = if desc.Dimension == D3D12_RESOURCE_DIMENSION_TEXTURE3D {
        sub
    } else {
        sub % mips
    };
    let width = (desc.Width >> mip).max(1).div_ceil(block);
    Some(width * bytes)
}

/// The buffer layout of the pair: the writer's subresources as a buffer
/// holds them, and the same bytes as the placed copy's format reads them.
/// `None` when the two do not hold the same texels in the same places, in
/// which case no copy is recorded at all.
fn placements(
    writer: &D3D12_RESOURCE_DESC,
    placed: &D3D12_RESOURCE_DESC,
) -> Option<(u64, Vec<Placement>)> {
    let from = layout(writer)?;
    let to = layout(placed)?;
    if from.subs.len() != to.subs.len() {
        return None;
    }
    let total = from.total.max(to.total);
    let mut subs = Vec::with_capacity(from.subs.len());
    for (i, (from_sub, to_sub)) in from.subs.into_iter().zip(to.subs).enumerate() {
        let (a, b) = (from_sub.Footprint, to_sub.Footprint);
        // The same rows both ways: a row-per-row copy through the buffer
        // needs the two ends laid out alike (same bytes per row and texel,
        // same rows and slices, the same places in the buffer).
        let row = row_bytes(writer, i as u32)?;
        if a.Width != b.Width
            || a.Height != b.Height
            || a.Depth != b.Depth
            || a.RowPitch != b.RowPitch
            || from_sub.Offset != to_sub.Offset
            || row != row_bytes(placed, i as u32)?
            || row > u64::from(a.RowPitch)
        {
            return None;
        }
        // The footprint must lie inside the buffer: every row of every
        // slice, the last one as long as its row is (the pitch pads the
        // others).
        let rows = u64::from(a.Height).checked_mul(u64::from(a.Depth))?;
        let end = rows
            .checked_sub(1)?
            .checked_mul(u64::from(a.RowPitch))?
            .checked_add(row)?
            .checked_add(from_sub.Offset)?;
        if a.Width == 0 || a.Height == 0 || a.Depth == 0 || end > total {
            return None;
        }
        subs.push(Placement {
            from: from_sub,
            to: to_sub,
            copy: D3D12_BOX {
                left: 0,
                top: 0,
                front: 0,
                right: a.Width,
                bottom: a.Height,
                back: a.Depth,
            },
        });
    }
    Some((total, subs))
}

/// Whether the host copies a texture of `format` in `dimension`: the copy
/// commands take only formats the adapter can make a texture of.
fn copyable(device: &ID3D12Device, format: u32, dimension: D3D12_RESOURCE_DIMENSION) -> bool {
    use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;
    let wanted = match dimension {
        D3D12_RESOURCE_DIMENSION_TEXTURE1D => D3D12_FORMAT_SUPPORT1_TEXTURE1D,
        D3D12_RESOURCE_DIMENSION_TEXTURE2D => D3D12_FORMAT_SUPPORT1_TEXTURE2D,
        D3D12_RESOURCE_DIMENSION_TEXTURE3D => D3D12_FORMAT_SUPPORT1_TEXTURE3D,
        _ => return false,
    };
    let mut support = D3D12_FEATURE_DATA_FORMAT_SUPPORT {
        Format: DXGI_FORMAT(format as i32),
        ..Default::default()
    };
    // SAFETY: the query writes the structure it is given, whole.
    let asked = unsafe {
        device.CheckFeatureSupport(
            D3D12_FEATURE_FORMAT_SUPPORT,
            std::ptr::from_mut(&mut support).cast(),
            size_of::<D3D12_FEATURE_DATA_FORMAT_SUPPORT>() as u32,
        )
    };
    asked.is_ok() && support.Support1.contains(wanted)
}

/// The staging state of the pair (writer, placed copy), made on first use
/// and remade when the descriptions behind its addresses changed.
fn place_sync(writer: &HostTexture, placed: &HostTexture, address: u64) -> Option<Arc<PlaceSync>> {
    use windows::core::Interface;
    let device = &host()?.device;
    // SAFETY: live host textures, both held by the caller across the copy.
    let (from_desc, to_desc) = unsafe { (writer.texture.GetDesc(), placed.texture.GetDesc()) };
    let refuse = |why: &str| {
        tracing::debug!(
            why,
            address = format_args!("{address:#x}"),
            from = ?from_desc,
            to = ?to_desc,
            "placed copy: not staged"
        );
        None
    };
    if writer.texture.as_raw() == placed.texture.as_raw() {
        return refuse("the same texture");
    }
    let texture = |d: &D3D12_RESOURCE_DESC| {
        d.Dimension != D3D12_RESOURCE_DIMENSION_BUFFER
            && d.SampleDesc.Count == 1
            // The copy commands take only the plain layout.
            && d.Layout == D3D12_TEXTURE_LAYOUT_UNKNOWN
    };
    if !texture(&from_desc) || !texture(&to_desc) || from_desc.Dimension != to_desc.Dimension {
        return refuse("not two single-sample plain textures of one dimension");
    }
    if from_desc.Format == to_desc.Format {
        return refuse("the same format at both ends: no buffer needed");
    }
    // The same texels: the same bytes and block width at both ends, so the
    // rows the writer leaves are the rows the reader reads.
    let (from_bytes, from_block) = crate::resource::element(from_desc.Format.0 as u32)?;
    let (to_bytes, to_block) = crate::resource::element(to_desc.Format.0 as u32)?;
    if (from_bytes, from_block) != (to_bytes, to_block) {
        return refuse("formats of different texel sizes");
    }
    if !copyable(device, from_desc.Format.0 as u32, from_desc.Dimension)
        || !copyable(device, to_desc.Format.0 as u32, to_desc.Dimension)
    {
        return refuse("a format the host cannot copy as a texture");
    }
    let key = (
        writer.texture.as_raw() as usize,
        placed.texture.as_raw() as usize,
    );
    {
        let map = PLACE_SYNCS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(cell) = map.as_ref().and_then(|m| m.get(&key))
            && cell.from_desc == from_desc
            && cell.to_desc == to_desc
        {
            return Some(cell.clone());
        }
    }
    let (total, subs) = placements(&from_desc, &to_desc)?;
    if subs.is_empty() {
        return None;
    }
    let desc = D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
        Width: total,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleDesc: windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
        ..Default::default()
    };
    let heap = D3D12_HEAP_PROPERTIES {
        Type: D3D12_HEAP_TYPE_DEFAULT,
        ..Default::default()
    };
    let mut buffer: Option<ID3D12Resource> = None;
    // SAFETY: plain creation of a staging buffer.
    unsafe {
        device
            .CreateCommittedResource(
                &heap,
                D3D12_HEAP_FLAG_NONE,
                &desc,
                D3D12_RESOURCE_STATE_COPY_DEST,
                None,
                &mut buffer,
            )
            .ok()?;
    }
    let buffer = buffer?;
    crate::host::name(&buffer, || format!("placed copy staging {total:#x}"));
    tracing::info!(
        bytes = total,
        subresources = subs.len(),
        from = from_desc.Format.0,
        to = to_desc.Format.0,
        width = from_desc.Width,
        height = from_desc.Height,
        depth = from_desc.DepthOrArraySize,
        address = format_args!("{address:#x}"),
        "placed copy: staged between the writer's format and the placed one"
    );
    let cell = Arc::new(PlaceSync {
        buffer,
        from_desc,
        to_desc,
        subs,
    });
    let mut map = PLACE_SYNCS.lock().unwrap_or_else(|e| e.into_inner());
    map.get_or_insert_with(HashMap::new)
        .insert(key, cell.clone());
    Some(cell)
}

/// Records a barrier on the staging buffer, whose state every list leaves as
/// it found it: copy destination at the start of each copy, since no
/// resource state is tracked for it.
fn staging_barrier(list: &HostList, buffer: &ID3D12Resource, before: u32, after: u32) {
    use windows::Win32::Graphics::Direct3D12::D3D12_RESOURCE_STATES as State;
    let barrier = crate::host::transition(buffer, State(before as i32), State(after as i32));
    // SAFETY: an open list and the staging buffer, which only these copies
    // use.
    unsafe { list.list.ResourceBarrier(&[barrier]) };
}

/// Brings a placed copy of title memory up to date with the resource that
/// writes that memory (a UAV the title's shaders write), in this list and so
/// before the draw or dispatch that reads it: the placed copy is made from
/// the title bytes, which a GPU write never reaches.
///
/// Returns whether the placed copy's contents come from the writer (a copy
/// recorded in this list, or already recorded in it), so the caller can
/// leave out an upload of title memory that holds none of them.
pub(crate) fn sync_placed(list: &HostList, placed: This, view: &crate::descriptor::View) -> bool {
    if !placed_sync_on() {
        return false;
    }
    let crate::descriptor::View::Placed(false, resource, _) = view else {
        return false;
    };
    const BUFFER: u32 = 1;
    if resource[0] == BUFFER {
        return false;
    }
    let Some(placed_copy) = crate::resource::host_copy(placed) else {
        return false;
    };
    let Some((memory, _)) = crate::resource::texture_memory(placed) else {
        return false;
    };
    let Some(writer) = crate::resource::writer_at(memory as u64, placed, &placed_copy.texture)
    else {
        return false;
    };
    // Both host copies are held here until the copy is recorded below, and
    // the list keeps them (and the staging buffer) until it is reset: a
    // concurrent release cannot free what the recorded copy names.
    let Some(cell) = place_sync(&writer, &placed_copy, memory as u64) else {
        return false;
    };
    if !list.first_sync(Arc::as_ptr(&placed_copy) as usize) {
        return true;
    }
    SYNCS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    list.keep(&cell.buffer);
    list.keep(&writer.texture);
    list.keep(&placed_copy.texture);
    const COPY_DEST: u32 = 0x400;
    const COPY_SOURCE: u32 = 0x800;
    // The writer's shaders may have written it: a barrier orders the copy
    // after them (the prelude puts it in that state when this list first
    // uses it).
    crate::list::ensure_state_host(list, &writer, COPY_SOURCE);
    crate::list::ensure_state_host(list, &placed_copy, COPY_DEST);
    for (i, sub) in cell.subs.iter().enumerate() {
        let source = D3D12_TEXTURE_COPY_LOCATION {
            // SAFETY: borrowed pointers the locations never release.
            pResource: unsafe { std::mem::transmute_copy(&writer.texture) },
            Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                SubresourceIndex: i as u32,
            },
        };
        let staging = D3D12_TEXTURE_COPY_LOCATION {
            // SAFETY: as above.
            pResource: unsafe { std::mem::transmute_copy(&cell.buffer) },
            Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                PlacedFootprint: sub.from,
            },
        };
        #[cfg(feature = "diagnostics")]
        list.note_read(
            || {
                format!(
                    "placed copy {:#x} sub {i} of {} from the writer: {:#x} {w}x{h}x{d} f{from_f} to f{to_f}",
                    placed as usize,
                    cell.subs.len(),
                    memory as usize,
                    w = sub.copy.right,
                    h = sub.copy.bottom,
                    d = sub.copy.back,
                    from_f = sub.from.Footprint.Format.0,
                    to_f = cell.to_desc.Format.0,
                )
            },
            (0, 0),
        );
        // SAFETY: an open list; both locations name live resources, and the
        // box is inside each subresource and the buffer (`placements`).
        unsafe {
            list.list
                .CopyTextureRegion(&staging, 0, 0, 0, &source, Some(&raw const sub.copy))
        };
    }
    staging_barrier(list, &cell.buffer, COPY_DEST, COPY_SOURCE);
    for (i, sub) in cell.subs.iter().enumerate() {
        let staging = D3D12_TEXTURE_COPY_LOCATION {
            // SAFETY: borrowed pointers the locations never release.
            pResource: unsafe { std::mem::transmute_copy(&cell.buffer) },
            Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                PlacedFootprint: sub.to,
            },
        };
        let placed_at = D3D12_TEXTURE_COPY_LOCATION {
            // SAFETY: as above.
            pResource: unsafe { std::mem::transmute_copy(&placed_copy.texture) },
            Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                SubresourceIndex: i as u32,
            },
        };
        #[cfg(feature = "diagnostics")]
        list.note_read(
            || {
                format!(
                    "placed copy {:#x} sub {i} of {} to the placed copy: {:#x} {w}x{h}x{d} f{from_f} to f{to_f}",
                    placed as usize,
                    cell.subs.len(),
                    memory as usize,
                    w = sub.copy.right,
                    h = sub.copy.bottom,
                    d = sub.copy.back,
                    from_f = sub.to.Footprint.Format.0,
                    to_f = cell.to_desc.Format.0,
                )
            },
            (0, 0),
        );
        // SAFETY: an open list; the staging buffer covers the subresource,
        // and the box is inside the placed copy's subresource.
        unsafe {
            list.list
                .CopyTextureRegion(&placed_at, 0, 0, 0, &staging, Some(&raw const sub.copy))
        };
    }
    staging_barrier(list, &cell.buffer, COPY_SOURCE, COPY_DEST);
    tracing::debug!(
        placed = format_args!("{:#x}", placed as usize),
        subresources = cell.subs.len(),
        address = format_args!("{:#x}", memory as usize),
        "placed copy synced from the resource that writes its memory"
    );
    true
}

impl Staging {
    /// Detiles the texture memory into the staging buffer if it changed
    /// since the last time (and is mapped at all).
    pub(crate) fn refresh(&self) {
        use std::sync::atomic::Ordering::Relaxed;
        let started = std::time::Instant::now();
        let read = self.refresh_now();
        let preparing = &crate::host::PREPARING;
        preparing[2].fetch_add(started.elapsed().as_micros() as u64, Relaxed);
        preparing[3].fetch_add(read as u64, Relaxed);
    }

    /// Returns the bytes of title memory read.
    fn refresh_now(&self) -> usize {
        let size = self.layout.size as usize;
        if size == 0 {
            return 0;
        }
        let start = self.memory as usize;
        let mut last = self.last.lock().unwrap_or_else(|e| e.into_inner());
        let last = &mut *last;
        if let Some((now, written)) = crate::watch::writes(start, start + size, false) {
            if last.asked != 0 && written.iter().all(|&(_, w)| w <= last.asked) {
                return 0;
            }
            last.asked = now;
            // Read through the copy, which stands unmapped pages: memory of
            // big pools is too slow to ask page by page.
            last.read.resize(size, 0);
            crate::host::copy_mapped(last.read.as_mut_ptr(), self.memory, size, false);
            self.detile(&last.read);
            #[cfg(feature = "diagnostics")]
            crate::copystat::add(crate::copystat::C::TexWatched, size as u64);
            return size;
        }
        last.read.resize(size, 0);
        match read_resident(self.memory, &mut last.read) {
            Residency::Unmapped => {
                last.hash = None;
                #[cfg(feature = "diagnostics")]
                tracing::debug!(
                    memory = ?self.memory,
                    size,
                    desc = ?self.layout.desc,
                    "a sampled texture's memory is not committed"
                );
                0
            }
            Residency::Full => {
                let hash = blake3::hash(&last.read);
                #[cfg(feature = "diagnostics")]
                crate::copystat::add(crate::copystat::C::TexHashed, size as u64);
                if last.hash == Some(hash) {
                    return size;
                }
                last.hash = Some(hash);
                self.detile(&last.read);
                #[cfg(feature = "diagnostics")]
                crate::copystat::add(crate::copystat::C::TexChanged, size as u64);
                size
            }
            Residency::Partial(intervals) => {
                let (hash, resident_bytes) = hash_intervals(&last.read, &intervals);
                #[cfg(feature = "diagnostics")]
                crate::copystat::add(crate::copystat::C::TexHashed, resident_bytes as u64);
                if last.hash == Some(hash) {
                    return resident_bytes;
                }
                zero_gaps(&mut last.read, &intervals);
                last.hash = Some(hash);
                self.detile(&last.read);
                #[cfg(feature = "diagnostics")]
                crate::copystat::add(crate::copystat::C::TexChanged, resident_bytes as u64);
                resident_bytes
            }
        }
    }

    /// Detiles `tiled` (the texture's memory) into the staging buffer.
    fn detile(&self, tiled: &[u8]) {
        #[cfg(feature = "diagnostics")]
        if tiled.iter().all(|&b| b == 0) {
            tracing::debug!(
                memory = ?self.memory,
                desc = ?self.layout.desc,
                "a sampled texture's memory is all zero (not streamed yet, or a failed BCPack read)"
            );
        }
        for (i, (footprint, &rows)) in self.footprints.iter().zip(&self.rows).enumerate() {
            let (mip, slice) = (i as u32 % self.mips, i as u32 / self.mips);
            let f = footprint.Footprint;
            let pitch = f.RowPitch as usize;
            let len = pitch * rows as usize * f.Depth as usize;
            // SAFETY: the subresource's part of the mapped staging buffer.
            let linear = unsafe {
                std::slice::from_raw_parts_mut(self.data.add(footprint.Offset as usize), len)
            };
            if let Err(e) =
                self.layout
                    .detile(tiled, mip, slice, linear, pitch, pitch * rows as usize)
            {
                tracing::warn!(?e, mip, slice, "texture not detiled");
                return;
            }
        }
    }
}

/// Which byte intervals of a title texture are currently mapped and readable.
#[derive(Debug, PartialEq, Eq)]
enum Residency {
    Unmapped,
    Full,
    Partial(Vec<(usize, usize)>),
}

/// Copies committed, readable regions of `[src, src + dst.len())` into `dst`
/// without `VirtualQuery` overhead on the fully mapped fast path.
fn read_resident(src: *const u8, dst: &mut [u8]) -> Residency {
    if dst.is_empty() || crate::host::read_into(src, dst) {
        return Residency::Full;
    }
    use windows_sys::Win32::System::Memory::{
        MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_GUARD, PAGE_NOACCESS, VirtualQuery,
    };
    const PAGE_SIZE: usize = 4096;

    let base = src as usize;
    let end = base.saturating_add(dst.len());
    let mut p = base;
    let mut intervals: Vec<(usize, usize)> = Vec::new();
    let push_interval = |intervals: &mut Vec<(usize, usize)>, start: usize, stop: usize| {
        if start >= stop {
            return;
        }
        if let Some(last) = intervals.last_mut()
            && last.1 == start
        {
            last.1 = stop;
        } else {
            intervals.push((start, stop));
        }
    };

    while p < end {
        // SAFETY: plain data filled in by the query.
        let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
        // SAFETY: a valid out pointer of the right size.
        let n = unsafe { VirtualQuery(p as *const _, &mut info, size_of_val(&info)) };
        if n == 0 {
            break;
        }
        let region_end = (info.BaseAddress as usize)
            .saturating_add(info.RegionSize)
            .min(end);
        let step = region_end.saturating_sub(p);
        if step == 0 {
            break;
        }
        let next_p = p.saturating_add(step);
        if info.State != MEM_COMMIT || (info.Protect & (PAGE_NOACCESS | PAGE_GUARD)) != 0 {
            p = next_p;
            continue;
        }
        let offset = p - base;
        let region_len = region_end - p;
        if crate::host::read_into(p as *const u8, &mut dst[offset..offset + region_len]) {
            push_interval(&mut intervals, offset, offset + region_len);
        } else {
            // Concurrent unmap inside a committed region: fall back to 4 KiB pages.
            let mut q = p;
            while q < region_end {
                let next_page = (q & !(PAGE_SIZE - 1)).saturating_add(PAGE_SIZE);
                let page_end = next_page.min(region_end);
                let page_len = page_end.saturating_sub(q);
                if page_len == 0 {
                    break;
                }
                let page_offset = q - base;
                if crate::host::read_into(
                    q as *const u8,
                    &mut dst[page_offset..page_offset + page_len],
                ) {
                    push_interval(&mut intervals, page_offset, page_offset + page_len);
                }
                q = q.saturating_add(page_len);
            }
        }
        p = next_p;
    }

    if intervals.is_empty() {
        Residency::Unmapped
    } else if intervals.as_slice() == [(0, dst.len())] {
        Residency::Full
    } else {
        Residency::Partial(intervals)
    }
}

/// Hashes only the resident byte intervals (and their bounds) and returns `(hash, resident_bytes)`.
fn hash_intervals(data: &[u8], intervals: &[(usize, usize)]) -> (blake3::Hash, usize) {
    let mut hasher = blake3::Hasher::new();
    let mut resident_bytes = 0usize;
    for &(start, end) in intervals {
        hasher.update(&(start as u64).to_le_bytes());
        hasher.update(&(end as u64).to_le_bytes());
        hasher.update(&data[start..end]);
        resident_bytes += end - start;
    }
    (hasher.finalize(), resident_bytes)
}

/// Zeroes unmapped gaps between sorted `intervals` in `data` so evicted tiles leave no stale bytes.
fn zero_gaps(data: &mut [u8], intervals: &[(usize, usize)]) {
    let mut cursor = 0usize;
    for &(start, end) in intervals {
        if cursor < start {
            data[cursor..start].fill(0);
        }
        cursor = end;
    }
    if cursor < data.len() {
        data[cursor..].fill(0);
    }
}

/// Whether `[at, at + len)` is committed memory (streamed textures may be
/// unmapped until their pages arrive).
pub(crate) fn committed(at: *const u8, len: usize) -> bool {
    use windows_sys::Win32::System::Memory::{MEM_COMMIT, MEMORY_BASIC_INFORMATION, VirtualQuery};
    let mut p = at as usize;
    let end = p + len;
    while p < end {
        // SAFETY: plain data filled in by the query.
        let mut info: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
        // SAFETY: a valid out pointer of the right size.
        let n = unsafe { VirtualQuery(p as *const _, &mut info, size_of_val(&info)) };
        if n == 0 || info.State != MEM_COMMIT {
            return false;
        }
        p = info.BaseAddress as usize + info.RegionSize;
    }
    true
}

/// The staging state of a title texture, made on first use.
fn staging(resource: This, texture: &ID3D12Resource) -> Option<Arc<Staging>> {
    let mut map = STAGING.lock().unwrap_or_else(|e| e.into_inner());
    let map = map.get_or_insert_with(HashMap::new);
    if let Some(s) = map.get(&(resource as usize)) {
        return Some(s.clone());
    }
    let (memory, desc) = crate::resource::texture_memory(resource)?;
    let Some(layout) = crate::resource::texture_layout(&desc).map(|t| t.planes[0].layout.clone())
    else {
        once!(
            ?desc,
            "texture contents without a console layout not uploaded"
        );
        return None;
    };
    if layout.desc.swizzle.xor() {
        once!(?desc, "texture contents in an XOR mode not uploaded");
        return None;
    }
    let host = host()?;
    // SAFETY: a live resource.
    let d = unsafe { texture.GetDesc() };
    let count = if d.Dimension == D3D12_RESOURCE_DIMENSION_TEXTURE3D {
        u32::from(d.MipLevels)
    } else {
        u32::from(d.MipLevels) * u32::from(d.DepthOrArraySize)
    };
    let mut footprints = vec![D3D12_PLACED_SUBRESOURCE_FOOTPRINT::default(); count as usize];
    let mut rows = vec![0u32; count as usize];
    let mut total = 0;
    // SAFETY: outputs for `count` subresources.
    unsafe {
        host.device.GetCopyableFootprints(
            &d,
            0,
            count,
            0,
            Some(footprints.as_mut_ptr()),
            Some(rows.as_mut_ptr()),
            None,
            Some(&mut total),
        );
    }
    let (buffer, data) = host.upload_buffer(total)?;
    crate::host::name(&buffer, || {
        format!(
            "staging of {resource:?} {}x{}x{} f{}",
            d.Width, d.Height, d.DepthOrArraySize, d.Format.0
        )
    });
    tracing::debug!(?desc, total, "texture contents uploaded from title memory");
    let s = Arc::new(Staging {
        buffer,
        #[cfg(feature = "diagnostics")]
        size: total,
        data,
        footprints,
        rows,
        layout,
        mips: u32::from(d.MipLevels),
        memory,
        last: Mutex::default(),
    });
    map.insert(resource as usize, s.clone());
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC};
    use windows_sys::Win32::System::Memory::{
        MEM_COMMIT, MEM_DECOMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_NOACCESS, PAGE_READWRITE,
        VirtualAlloc, VirtualFree,
    };

    /// RAII guard for a `VirtualAlloc(..., MEM_RESERVE, ...)` address range.
    struct VirtualRegion {
        ptr: *mut u8,
        len: usize,
    }

    impl VirtualRegion {
        fn reserve(len: usize) -> Self {
            // SAFETY: reserves uncommitted virtual address space of `len` bytes, released on drop.
            let ptr = unsafe { VirtualAlloc(std::ptr::null(), len, MEM_RESERVE, PAGE_NOACCESS) }
                .cast::<u8>();
            assert!(!ptr.is_null(), "VirtualAlloc MEM_RESERVE failed");
            Self { ptr, len }
        }

        fn commit(&self, offset: usize, len: usize) {
            assert!(offset.checked_add(len).is_some_and(|end| end <= self.len));
            // SAFETY: commits pages within our reserved virtual address range.
            let committed = unsafe {
                VirtualAlloc(self.ptr.add(offset).cast(), len, MEM_COMMIT, PAGE_READWRITE)
            };
            assert!(!committed.is_null(), "VirtualAlloc MEM_COMMIT failed");
        }

        fn decommit(&self, offset: usize, len: usize) {
            assert!(offset.checked_add(len).is_some_and(|end| end <= self.len));
            // SAFETY: decommits pages within our reserved virtual address range.
            let ok = unsafe { VirtualFree(self.ptr.add(offset).cast(), len, MEM_DECOMMIT) };
            assert_ne!(ok, 0, "VirtualFree MEM_DECOMMIT failed");
        }

        /// Writes `bytes` at `offset`; caller must ensure `[offset..offset + bytes.len())` is committed.
        unsafe fn write_slice(&self, offset: usize, bytes: &[u8]) {
            assert!(
                offset
                    .checked_add(bytes.len())
                    .is_some_and(|end| end <= self.len)
            );
            // SAFETY: caller guarantees the destination range is committed and writable.
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.add(offset), bytes.len());
            }
        }
    }

    impl Drop for VirtualRegion {
        fn drop(&mut self) {
            // SAFETY: `self.ptr` was allocated with `VirtualAlloc(..., MEM_RESERVE, ...)` and not yet freed.
            unsafe { VirtualFree(self.ptr.cast(), 0, MEM_RELEASE) };
        }
    }

    /// RAII guard releasing a console COM resource so `STAGING` entries are cleaned up on drop.
    struct ResourceGuard(This);

    impl Drop for ResourceGuard {
        fn drop(&mut self) {
            type Release = extern "system" fn(This) -> u32;
            crate::object::slot::<Release>(self.0, 0x10)(self.0);
        }
    }

    #[test]
    fn partial_residency_reads_only_mapped_regions_and_detects_new_mips() {
        const TILE: usize = 64 * 1024;
        const TOTAL: usize = 256 * 1024;
        let region = VirtualRegion::reserve(TOTAL);
        let mut dst = vec![0xAAu8; TOTAL];

        assert_eq!(read_resident(region.ptr, &mut dst), Residency::Unmapped);

        region.commit(0, TILE);
        let tile0: Vec<u8> = (0..TILE)
            .map(|i| ((i * 37 + 11) & 0xff) as u8 | 1)
            .collect();
        // SAFETY: `[0..TILE)` was committed above.
        unsafe { region.write_slice(0, &tile0) };

        assert_eq!(
            read_resident(region.ptr, &mut dst),
            Residency::Partial(vec![(0, TILE)])
        );
        assert_eq!(dst[..TILE], tile0[..]);
        let (hash0, bytes0) = hash_intervals(&dst, &[(0, TILE)]);
        assert_eq!(bytes0, TILE);

        // Mutating unmapped tail bytes in `dst` must not affect the resident hash.
        dst[TILE..].fill(0x55);
        assert_eq!(
            read_resident(region.ptr, &mut dst),
            Residency::Partial(vec![(0, TILE)])
        );
        let (hash0_repeat, bytes0_repeat) = hash_intervals(&dst, &[(0, TILE)]);
        assert_eq!((hash0_repeat, bytes0_repeat), (hash0, TILE));

        // Commit a disjoint second 64 KiB tile at `[128 KiB..192 KiB)`.
        region.commit(2 * TILE, TILE);
        let tile2: Vec<u8> = (0..TILE)
            .map(|i| ((i * 53 + 29) & 0xff) as u8 | 1)
            .collect();
        // SAFETY: `[2 * TILE..3 * TILE)` was committed above.
        unsafe { region.write_slice(2 * TILE, &tile2) };

        let res = read_resident(region.ptr, &mut dst);
        assert_eq!(
            res,
            Residency::Partial(vec![(0, TILE), (2 * TILE, 3 * TILE)])
        );
        assert_eq!(dst[..TILE], tile0[..]);
        assert_eq!(dst[2 * TILE..3 * TILE], tile2[..]);
        let (hash_two, bytes_two) = hash_intervals(&dst, &[(0, TILE), (2 * TILE, 3 * TILE)]);
        assert_eq!(bytes_two, 2 * TILE);
        assert_ne!(hash_two, hash0);

        // Decommit the second tile while leaving junk across unmapped gaps; `zero_gaps` must clear it.
        region.decommit(2 * TILE, TILE);
        dst[TILE..].fill(0xCC);
        let res_after_evict = read_resident(region.ptr, &mut dst);
        assert_eq!(res_after_evict, Residency::Partial(vec![(0, TILE)]));
        let Residency::Partial(intervals) = res_after_evict else {
            unreachable!();
        };
        let (hash_after_evict, bytes_after_evict) = hash_intervals(&dst, &intervals);
        assert_eq!((hash_after_evict, bytes_after_evict), (hash0, TILE));
        zero_gaps(&mut dst, &intervals);
        assert_eq!(dst[..TILE], tile0[..]);
        assert!(dst[TILE..].iter().all(|&b| b == 0));

        // Committing the remaining gaps yields `Residency::Full`.
        region.commit(TILE, TILE);
        region.commit(2 * TILE, 2 * TILE);
        assert_eq!(read_resident(region.ptr, &mut dst), Residency::Full);
    }

    fn readback_mip(
        host: &crate::host::Host,
        resource: This,
        host_tex: &ID3D12Resource,
        subresource: u32,
        width: u32,
        height: u32,
    ) -> Vec<u8> {
        let row_pitch = width * 4;
        assert_eq!(row_pitch % D3D12_TEXTURE_DATA_PITCH_ALIGNMENT, 0);
        let byte_len = row_pitch as usize * height as usize;

        let readback_heap = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_READBACK,
            ..Default::default()
        };
        let readback_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Width: byte_len as u64,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            ..Default::default()
        };
        let mut readback: Option<ID3D12Resource> = None;
        // SAFETY: plain creation of a D3D12 readback buffer.
        unsafe {
            host.device
                .CreateCommittedResource(
                    &readback_heap,
                    D3D12_HEAP_FLAG_NONE,
                    &readback_desc,
                    D3D12_RESOURCE_STATE_COPY_DEST,
                    None,
                    &mut readback,
                )
                .unwrap();
        }
        let readback = readback.unwrap();

        let back = host.new_list().unwrap();
        crate::list::ensure_state(&back, resource, 0x800);
        let to = D3D12_TEXTURE_COPY_LOCATION {
            // SAFETY: a borrowed pointer the location never releases.
            pResource: unsafe { std::mem::transmute_copy(&readback) },
            Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                PlacedFootprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
                    Offset: 0,
                    Footprint: D3D12_SUBRESOURCE_FOOTPRINT {
                        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                        Width: width,
                        Height: height,
                        Depth: 1,
                        RowPitch: row_pitch,
                    },
                },
            },
        };
        let from = D3D12_TEXTURE_COPY_LOCATION {
            // SAFETY: a borrowed pointer the location never releases.
            pResource: unsafe { std::mem::transmute_copy(host_tex) },
            Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                SubresourceIndex: subresource,
            },
        };
        // SAFETY: `back.list` is open and both resources are live.
        unsafe { back.list.CopyTextureRegion(&to, 0, 0, 0, &from, None) };
        back.close();
        host.execute(&[&back]);

        let mut mapped = std::ptr::null_mut();
        let range = D3D12_RANGE {
            Begin: 0,
            End: byte_len,
        };
        // SAFETY: `host.execute` waited for GPU completion and the buffer spans `byte_len` bytes.
        unsafe {
            readback.Map(0, Some(&range), Some(&mut mapped)).unwrap();
            let out = std::slice::from_raw_parts(mapped.cast::<u8>(), byte_len).to_vec();
            readback.Unmap(0, None);
            out
        }
    }

    #[test]
    fn partially_resident_mip_tail_uploads_to_host_texture() {
        let Some(host) = crate::host::host() else {
            return;
        };
        use victory_formats::texture::{Dimension, Swizzle, Texture, TextureDesc};

        let tex = Texture::new(&TextureDesc {
            dimension: Dimension::Tex2d,
            width: 256,
            height: 256,
            depth_or_array: 1,
            mips: 3,
            format: DXGI_FORMAT_R8G8B8A8_UNORM.0 as u32,
            samples: 1,
            swizzle: Swizzle::SW_64KB_S,
        })
        .unwrap();
        let layout = &tex.planes[0].layout;
        assert_eq!(layout.first_tail_mip, 2);
        assert_eq!(layout.mips[2].offset, 0);
        assert!(layout.mips[2].tail.is_some());
        assert_eq!(layout.mips[1].offset, 65536);
        assert_eq!(layout.mips[0].offset, 131072);
        assert_eq!(layout.size, 393216);

        let region = VirtualRegion::reserve(layout.size as usize);
        region.commit(0, 65536);

        let expected_mip2: Vec<u8> = (0..64 * 64 * 4)
            .map(|i| (((i * 13 + 7) % 251) as u8) | 1)
            .collect();
        let mut tiled = vec![0u8; layout.size as usize];
        layout
            .tile(&mut tiled, 2, 0, &expected_mip2, 256, 64 * 256)
            .unwrap();
        assert!(tiled[65536..].iter().all(|&b| b == 0));
        // SAFETY: tile 0 `[0..65536)` was committed above.
        unsafe { region.write_slice(0, &tiled[..65536]) };
        assert!(!committed(region.ptr, layout.size as usize));

        const _: () =
            assert!(size_of::<D3D12_RESOURCE_DESC>() == size_of::<crate::resource::ResourceDesc>());
        let desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Alignment: 0,
            Width: 256,
            Height: 256,
            DepthOrArraySize: 1,
            MipLevels: 3,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT(0x109),
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        let mut resource = std::ptr::null();
        let hr = crate::resource::create_at_address(
            std::ptr::null(),
            region.ptr as u64,
            (&raw const desc).cast(),
            0,
            std::ptr::null(),
            &crate::resource::PAGEABLE_IIDS[0],
            &mut resource,
        );
        assert_eq!(hr, crate::object::S_OK);
        let resource_guard = ResourceGuard(resource);
        let host_tex = crate::resource::host_resource(resource_guard.0).unwrap();

        let list = host.new_list().unwrap();
        upload(&list, resource_guard.0, host_tex);
        list.close();
        host.execute(&[&list]);

        assert_eq!(
            readback_mip(host, resource_guard.0, host_tex, 2, 64, 64),
            expected_mip2
        );
        assert!(
            readback_mip(host, resource_guard.0, host_tex, 1, 128, 128)
                .iter()
                .all(|&b| b == 0)
        );

        // Stream in mip 1 while leaving mip 0 `[131072..393216)` unmapped.
        region.commit(65536, 65536);
        let expected_mip1: Vec<u8> = (0..128 * 128 * 4)
            .map(|i| (((i * 29 + 19) % 241) as u8) | 1)
            .collect();
        tiled.fill(0);
        layout
            .tile(&mut tiled, 1, 0, &expected_mip1, 512, 128 * 512)
            .unwrap();
        // SAFETY: tile 1 `[65536..131072)` was committed above.
        unsafe { region.write_slice(65536, &tiled[65536..131072]) };
        assert!(!committed(region.ptr, layout.size as usize));

        list.reset();
        upload(&list, resource_guard.0, host_tex);
        list.close();
        host.execute(&[&list]);

        assert_eq!(
            readback_mip(host, resource_guard.0, host_tex, 1, 128, 128),
            expected_mip1
        );
        assert_eq!(
            readback_mip(host, resource_guard.0, host_tex, 2, 64, 64),
            expected_mip2
        );

        // Evict mip 1: `zero_gaps` must clear stale cached bytes so host mip 1 becomes zeroed.
        region.decommit(65536, 65536);
        list.reset();
        upload(&list, resource_guard.0, host_tex);
        list.close();
        host.execute(&[&list]);

        assert!(
            readback_mip(host, resource_guard.0, host_tex, 1, 128, 128)
                .iter()
                .all(|&b| b == 0)
        );
        assert_eq!(
            readback_mip(host, resource_guard.0, host_tex, 2, 64, 64),
            expected_mip2
        );
    }

    /// A description the pair's stages are made from (as a host texture's
    /// would be).
    fn desc(format: u32, width: u64, height: u32, depth: u16, mips: u16) -> D3D12_RESOURCE_DESC {
        D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE3D,
            Width: width,
            Height: height,
            DepthOrArraySize: depth,
            MipLevels: mips,
            Format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT(format as i32),
            SampleDesc: windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
            ..Default::default()
        }
    }

    #[test]
    fn a_placed_copy_is_staged_only_between_the_same_texels() {
        let Some(host) = crate::host::host() else {
            return; // no host GPU
        };
        let _ = host;
        // The world's fog volumes: R32 written, R9G9B9E5 read.
        let writer = desc(42, 8, 4, 2, 1);
        let placed = desc(67, 8, 4, 2, 1);
        let (total, subs) = placements(&writer, &placed).expect("staged");
        assert_eq!(subs.len(), 1, "one subresource");
        assert_eq!(subs[0].copy.right, 8);
        assert_eq!(subs[0].copy.bottom, 4);
        assert_eq!(subs[0].copy.back, 2);
        // The last row's bytes are inside the buffer, past the padded rows.
        let last = subs[0].from.Offset + 3 * u64::from(subs[0].from.Footprint.RowPitch) + 8 * 4;
        assert!(last <= total, "{last} of {total}");
        // 8 bytes a texel (R16G16B16A16) does not hold the same texels.
        assert!(placements(&writer, &desc(10, 8, 4, 2, 1)).is_none());
        // Nor do another mip count or size lay out alike.
        assert!(placements(&writer, &desc(67, 8, 4, 2, 2)).is_none());
        assert!(placements(&writer, &desc(67, 16, 4, 2, 1)).is_none());
    }
}
