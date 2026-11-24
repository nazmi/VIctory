//! Descriptor heaps: zeroed memory, with handles that are plain addresses
//! (CPU and GPU alike, as on the console).
//!
//! Descriptor sizes: views 32 bytes and samplers 16, the size of RDNA 2's
//! image and sampler descriptors, so a title that copies descriptors itself
//! moves whole ones. Render target and depth views are ours alone; 128 bytes
//! leaves room for whatever stride a title assumes.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use crate::object::{self, DESCRIPTOR_HEAP, E_INVALIDARG, HRESULT, Iid, Object, Table, This, guid};
use crate::resource::{Memory, PAGEABLE_IIDS};

#[cfg(windows)]
pub(crate) mod host;
#[cfg(windows)]
pub(crate) mod lease;

const HEAP_IIDS: [Iid; 4] = [
    guid(
        0x8efb471d,
        0x616c,
        0x4f49,
        [0x90, 0xf7, 0x12, 0x7b, 0xb7, 0x63, 0xfa, 0x51],
    ),
    PAGEABLE_IIDS[0],
    PAGEABLE_IIDS[1],
    PAGEABLE_IIDS[2],
];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct HeapDesc {
    kind: u32,
    count: u32,
    flags: u32,
    nodes: u32,
}

/// Newer titles (Grand Theft Auto VI) pass the description followed by
/// memory of their own for the descriptors: an address and its size in
/// bytes, or a null address to have the heap allocate (inferred from the
/// title's call sites; older titles pass the plain description).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Placement {
    address: u64,
    size: u64,
}

#[repr(C)]
pub(crate) struct Heap {
    object: Object,
    desc: HeapDesc,
    /// The first descriptor: in `memory`, or in the title's own memory.
    pub(crate) start: u64,
    memory: Option<Memory>,
    /// Shader-visible view and sampler heaps get a host heap with the same
    /// descriptor order when a draw first uses them (decision 0007, step
    /// 3); slots written before wait in `pending`.
    #[cfg(windows)]
    host: std::sync::OnceLock<Option<host::HostHeap>>,
    #[cfg(windows)]
    pending: std::sync::Mutex<Vec<u32>>,
}

pub(crate) fn heap<'a>(this: This) -> &'a Heap {
    // SAFETY: called from descriptor heap methods only.
    unsafe { &*this.cast::<Heap>() }
}

impl Heap {
    /// Whether this is a sampler heap.
    pub(crate) fn samplers(&self) -> bool {
        self.desc.kind == SAMPLERS
    }

    /// The host heap, made on first use from what the title wrote so far.
    /// Made late because a host view heap the size of Grand Theft Auto VI's
    /// costs over 100 MB of commit, which its boot needs for its page pool.
    #[cfg(windows)]
    pub(crate) fn host(&self) -> Option<&host::HostHeap> {
        if let Some(h) = self.host.get() {
            return h.as_ref();
        }
        if self.desc.flags & SHADER_VISIBLE == 0 || !matches!(self.desc.kind, VIEWS_KIND | SAMPLERS)
        {
            return None;
        }
        let mut pending = self.pending.lock().unwrap_or_else(|e| e.into_inner());
        // A recorder can take the fast path as soon as the OnceLock is
        // initialized. Publish only after replaying the queued writes.
        self.host
            .get_or_init(|| {
                let step = u64::from(increment(self.desc.kind)?);
                let h = host::HostHeap::new(self.samplers(), self.desc.count)?;
                for i in pending.drain(..) {
                    h.mirror(i, self.start + u64::from(i) * step, false);
                }
                Some(h)
            })
            .as_ref()
    }

    /// The descriptor index of `handle` in this heap, if it lies in it.
    pub(crate) fn index(&self, handle: u64) -> Option<u32> {
        let step = u64::from(increment(self.desc.kind)?);
        let at = handle.checked_sub(self.start)?;
        let index = at / step;
        (index < u64::from(self.desc.count)).then_some(index as u32)
    }
}

const VIEWS_KIND: u32 = 0;
const SAMPLERS: u32 = 1;
const SHADER_VISIBLE: u32 = 1;

/// Shader-visible heaps by address range, for mirroring descriptor writes:
/// start, end, heap.
#[cfg(windows)]
static SHADER_HEAPS: std::sync::Mutex<Vec<(u64, u64, usize)>> = std::sync::Mutex::new(Vec::new());

/// The shader-visible heap holding `handle`, and the descriptor's index.
#[cfg(windows)]
fn shader_slot<'a>(handle: u64) -> Option<(&'a Heap, u32)> {
    let heaps = SHADER_HEAPS.lock().unwrap_or_else(|e| e.into_inner());
    let &(_, _, at) = heaps
        .iter()
        .find(|&&(start, end, _)| (start..end).contains(&handle))?;
    // SAFETY: registered heaps are live (removed when dropped).
    let heap = unsafe { &*(at as *const Heap) };
    Some((heap, heap.index(handle)?))
}

/// Makes the host descriptor for what the title wrote at `handle`, if it
/// is in a shader-visible heap.
#[cfg(windows)]
fn mirror(handle: u64) {
    let Some((heap, index)) = shader_slot(handle) else {
        return;
    };
    let mut pending = heap.pending.lock().unwrap_or_else(|e| e.into_inner());
    match heap.host.get() {
        None => pending.push(index),
        Some(host) => {
            drop(pending);
            if let Some(host) = host {
                host.mirror(index, handle, false);
            }
        }
    }
}

/// Gives back dynamic slots of the host heap behind the title heap at
/// `heap`, if that heap still exists.
#[cfg(windows)]
pub(crate) fn release_dynamic(heap: usize, first: u32, n: u32) {
    let heaps = SHADER_HEAPS.lock().unwrap_or_else(|e| e.into_inner());
    if heaps.iter().any(|&(_, _, h)| h == heap)
        // SAFETY: registered heaps are live (removed when dropped).
        && let Some(Some(host)) = unsafe { &*(heap as *const Heap) }.host.get()
    {
        host.release(first, n);
    }
}

#[cfg(windows)]
impl Drop for Heap {
    fn drop(&mut self) {
        let at = std::ptr::from_ref::<Heap>(self) as usize;
        let mut heaps = SHADER_HEAPS.lock().unwrap_or_else(|e| e.into_inner());
        heaps.retain(|&(_, _, h)| h != at);
    }
}

/// Bytes per descriptor, by heap type (views, samplers, render targets,
/// depth).
fn increment(kind: u32) -> Option<u32> {
    [32, 16, 128, 128].get(kind as usize).copied()
}

/// Device method.
pub(crate) extern "system" fn increment_size(_: This, kind: u32) -> u32 {
    increment(kind).unwrap_or(0)
}

/// What our render target views hold: the resource and the view
/// description, turned into a host view when a command uses them.
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct RtvRecord {
    tag: u32,
    described: u32,
    pub(crate) resource: This,
    desc: [u32; 6],
}
const RTV_TAG: u32 = u32::from_le_bytes(*b"vRTV");
const _: () = assert!(size_of::<RtvRecord>() <= 112);

impl RtvRecord {
    /// The view description in the host's type (the same layout).
    #[cfg(windows)]
    pub(crate) fn desc(
        &self,
    ) -> Option<windows::Win32::Graphics::Direct3D12::D3D12_RENDER_TARGET_VIEW_DESC> {
        const TEXTURE_2D: u32 = 4;
        self.host_desc(TEXTURE_2D)
    }

    /// The depth view description in the host's type (the same layout).
    #[cfg(windows)]
    pub(crate) fn depth_desc(
        &self,
    ) -> Option<windows::Win32::Graphics::Direct3D12::D3D12_DEPTH_STENCIL_VIEW_DESC> {
        const TEXTURE_2D: u32 = 3;
        self.host_desc(TEXTURE_2D)
    }

    /// The description words as a host type, the format as the host has it.
    /// Without one, a typeless host copy gets a 2D view (`dimension`) in
    /// the title's format, which the host needs spelled out.
    #[cfg(windows)]
    fn host_desc<T>(&self, dimension: u32) -> Option<T> {
        assert!(size_of::<T>() <= size_of::<[u32; 6]>());
        let mut words = if self.described != 0 {
            self.desc
        } else {
            [
                crate::resource::view_format(self.resource)?,
                dimension,
                0,
                0,
                0,
                0,
            ]
        };
        words[0] = crate::resource::pc_format(words[0]);
        // SAFETY: both host view descriptions are 24 bytes in this layout.
        Some(unsafe { (&raw const words).cast::<T>().read_unaligned() })
    }
}

/// Device method: records the view at `handle` (descriptor memory).
pub(crate) extern "system" fn create_rtv(
    _: This,
    resource: This,
    desc: *const [u32; 6],
    handle: u64,
) {
    if handle == 0 {
        return;
    }
    // SAFETY: the title passes a description or null.
    let described = unsafe { desc.as_ref() };
    let record = RtvRecord {
        tag: RTV_TAG,
        described: u32::from(described.is_some()),
        resource,
        desc: described.copied().unwrap_or_default(),
    };
    // SAFETY: the handle is a descriptor slot, at least 112 bytes.
    unsafe { (handle as *mut RtvRecord).write_unaligned(record) };
}

/// Device method: a depth view, recorded as render target views are.
pub(crate) extern "system" fn create_dsv(
    this: This,
    resource: This,
    desc: *const [u32; 6],
    handle: u64,
) {
    create_rtv(this, resource, desc, handle);
    if handle != 0 {
        // SAFETY: just written by `create_rtv`.
        unsafe { (handle as *mut u32).write_unaligned(DSV_TAG) };
    }
}
const DSV_TAG: u32 = u32::from_le_bytes(*b"vDSV");

/// A shader view or sampler as the title described it (descriptions in the
/// host's layout, kept as words), for the host to build its own from.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum View {
    /// Constant buffer: GPU address and size.
    Constant(u64, u32),
    /// Shader resource: resource, description (none: the resource's own).
    Shader(usize, Option<[u32; 10]>),
    /// Unordered access: resource, counter resource, description.
    Unordered(usize, usize, Option<[u32; 10]>),
    Sampler([u32; 13]),
    /// The console's own sampler description: flags, filters per axis,
    /// addressing, LOD and anisotropy settings, border color, then
    /// console-only tuning words.
    ConsoleSampler([u32; 27]),
    /// A view of memory without a resource object: whether it is unordered
    /// access, the resource description, then the view's (the plain one
    /// followed by the memory address and console fields).
    Placed(bool, [u32; 14], [u32; 16]),
}

impl View {
    /// Whether the view reads buffer memory (constant buffers, buffer views
    /// of resources or of memory) or memory the title placed a texture in:
    /// such views get host slots only for draws.
    #[cfg(windows)]
    fn is_buffer(&self) -> bool {
        match self {
            View::Constant(..) | View::Placed(..) => true,
            View::Shader(r, _) | View::Unordered(r, _, _) => {
                crate::resource::buffer_bytes(*r as This).is_some()
            }
            View::Sampler(_) | View::ConsoleSampler(_) => false,
        }
    }
}

/// Every distinct view in use; a descriptor holds a tag, an index into it
/// and the entry's generation, which survive titles copying descriptors as
/// bytes. A view of a resource lives as long as the resource: when the
/// title releases it, its entries are freed for reuse under a new
/// generation, so descriptors (and host slots made from them) naming the
/// old one go stale instead of naming whatever is made at the same address
/// next.
/// shortcut: views without a resource object (constant buffers, memory
/// views, samplers) are kept for the run; they are bounded by the title's
/// memory and sampler states.
struct Views {
    /// Each entry's view and generation.
    list: Vec<(View, u32)>,
    index: std::collections::BTreeMap<View, u32>,
    /// Entries naming each resource.
    by_resource: std::collections::BTreeMap<usize, Vec<u32>>,
    free: Vec<u32>,
}

static VIEWS: std::sync::Mutex<Views> = std::sync::Mutex::new(Views {
    list: Vec::new(),
    index: std::collections::BTreeMap::new(),
    by_resource: std::collections::BTreeMap::new(),
    free: Vec::new(),
});
const VIEW_TAG: u32 = u32::from_le_bytes(*b"vVIE");

impl View {
    /// The resource objects the view names.
    fn resources(&self) -> [usize; 2] {
        match *self {
            View::Shader(r, _) => [r, 0],
            View::Unordered(r, counter, _) => [r, counter],
            _ => [0, 0],
        }
    }
}

fn record(handle: u64, view: View) {
    if handle == 0 {
        return;
    }
    let mut views = VIEWS.lock().unwrap_or_else(|e| e.into_inner());
    let i = match views.index.get(&view) {
        Some(&i) => i,
        None => {
            let i = match views.free.pop() {
                Some(i) => {
                    views.list[i as usize].0 = view.clone();
                    i
                }
                None => {
                    views.list.push((view.clone(), 1));
                    views.list.len() as u32 - 1
                }
            };
            for r in view.resources().into_iter().filter(|&r| r != 0) {
                views.by_resource.entry(r).or_default().push(i);
            }
            views.index.insert(view, i);
            i
        }
    };
    let generation = views.list[i as usize].1;
    // SAFETY: the handle is a descriptor slot, at least 16 bytes.
    unsafe { (handle as *mut [u32; 3]).write_unaligned([VIEW_TAG, i, generation]) };
    drop(views);
    #[cfg(windows)]
    mirror(handle);
}

/// The title released `resource`: the views naming it go stale.
pub(crate) fn forget(resource: usize) {
    let mut views = VIEWS.lock().unwrap_or_else(|e| e.into_inner());
    let Some(entries) = views.by_resource.remove(&resource) else {
        return;
    };
    for i in entries {
        let (view, generation) = views.list[i as usize].clone();
        // Entries freed through a view's other resource may hold another
        // view by now.
        if views.index.get(&view) != Some(&i) || !view.resources().contains(&resource) {
            continue;
        }
        views.index.remove(&view);
        views.list[i as usize].1 = generation.wrapping_add(1).max(1);
        // The view's other resource (a counter) keeps a stale entry, which
        // `forget` of that resource skips.
        views.free.push(i);
    }
}

/// The view of entry `i` if `generation` is still its own.
#[cfg(windows)]
fn view_at(i: u32, generation: u32) -> Option<View> {
    let views = VIEWS.lock().unwrap_or_else(|e| e.into_inner());
    let (view, current) = views.list.get(i as usize)?;
    (*current == generation).then(|| view.clone())
}

/// The view recorded at `handle`, if one is (and its resource is alive).
pub(crate) fn view(handle: u64) -> Option<View> {
    if handle == 0 {
        return None;
    }
    // SAFETY: the handle is a descriptor slot the title passes back.
    let [tag, i, generation] = unsafe { (handle as *const [u32; 3]).read_unaligned() };
    let views = VIEWS.lock().unwrap_or_else(|e| e.into_inner());
    let (view, current) = views.list.get(i as usize)?;
    (tag == VIEW_TAG && *current == generation).then(|| view.clone())
}

/// A description of `N` words, if the title passes one.
fn words<const N: usize>(desc: *const [u32; N]) -> Option<[u32; N]> {
    // SAFETY: the title passes a description or null.
    (!desc.is_null()).then(|| unsafe { desc.read_unaligned() })
}

#[cfg(windows)]
const _: () = {
    use std::mem::{offset_of, size_of};
    use windows::Win32::Graphics::Direct3D12::*;
    assert!(size_of::<D3D12_SHADER_RESOURCE_VIEW_DESC>() == 40);
    assert!(offset_of!(D3D12_SHADER_RESOURCE_VIEW_DESC, Format) == 0);
    assert!(offset_of!(D3D12_SHADER_RESOURCE_VIEW_DESC, ViewDimension) == 4);
    assert!(offset_of!(D3D12_SHADER_RESOURCE_VIEW_DESC, Shader4ComponentMapping) == 8);
    assert!(offset_of!(D3D12_SHADER_RESOURCE_VIEW_DESC, Anonymous) == 16);
    assert!(size_of::<D3D12_BUFFER_SRV>() == 24);
    assert!(offset_of!(D3D12_BUFFER_SRV, Flags) == 16);
    assert!(size_of::<D3D12_BUFFER_SRV_FLAGS>() == 4);
    assert!(size_of::<D3D12_TEX1D_SRV>() == 12);
    assert!(size_of::<D3D12_TEX1D_ARRAY_SRV>() == 20);
    assert!(size_of::<D3D12_TEX2D_SRV>() == 16);
    assert!(size_of::<D3D12_TEX2D_ARRAY_SRV>() == 24);
    assert!(size_of::<D3D12_TEX2DMS_SRV>() == 4);
    assert!(size_of::<D3D12_TEX2DMS_ARRAY_SRV>() == 8);
    assert!(size_of::<D3D12_TEX3D_SRV>() == 12);
    assert!(size_of::<D3D12_TEXCUBE_SRV>() == 12);
    assert!(size_of::<D3D12_TEXCUBE_ARRAY_SRV>() == 20);
    assert!(size_of::<D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV>() == 8);

    assert!(size_of::<D3D12_UNORDERED_ACCESS_VIEW_DESC>() == 40);
    assert!(offset_of!(D3D12_UNORDERED_ACCESS_VIEW_DESC, Format) == 0);
    assert!(offset_of!(D3D12_UNORDERED_ACCESS_VIEW_DESC, ViewDimension) == 4);
    assert!(offset_of!(D3D12_UNORDERED_ACCESS_VIEW_DESC, Anonymous) == 8);
    assert!(size_of::<D3D12_BUFFER_UAV>() == 32);
    assert!(offset_of!(D3D12_BUFFER_UAV, Flags) == 24);
    assert!(size_of::<D3D12_BUFFER_UAV_FLAGS>() == 4);
    assert!(size_of::<D3D12_TEX1D_UAV>() == 4);
    assert!(size_of::<D3D12_TEX1D_ARRAY_UAV>() == 12);
    assert!(size_of::<D3D12_TEX2D_UAV>() == 8);
    assert!(size_of::<D3D12_TEX2D_ARRAY_UAV>() == 16);
    assert!(size_of::<D3D12_TEX2DMS_UAV>() == 4);
    assert!(size_of::<D3D12_TEX2DMS_ARRAY_UAV>() == 8);
    assert!(size_of::<D3D12_TEX3D_UAV>() == 12);
};

/// Zeroes union alignment padding (`d[3]`) and unused union tail words of a
/// raw `D3D12_SHADER_RESOURCE_VIEW_DESC` word array.
fn canonical_srv_desc(mut d: [u32; 10]) -> [u32; 10] {
    d[3] = 0;
    let keep = match d[1] {
        1 => 9,  // Buffer (d[9] is trailing struct padding)
        2 => 7,  // Texture1D
        3 => 9,  // Texture1DArray
        4 => 8,  // Texture2D
        5 => 10, // Texture2DArray
        6 => 4,  // Texture2DMS (UnusedField_NothingToDefine only)
        7 => 6,  // Texture2DMSArray
        8 => 7,  // Texture3D
        9 => 7,  // TextureCube
        10 => 9, // TextureCubeArray
        11 => 6, // RaytracingAccelerationStructure
        _ => 10,
    };
    d[keep..].fill(0);
    d
}

/// Zeroes unused union tail words (and `Buffer`'s trailing padding word) of a
/// raw `D3D12_UNORDERED_ACCESS_VIEW_DESC` word array. The union begins at
/// offset 8 (`d[2]`) with no leading padding.
fn canonical_uav_desc(mut d: [u32; 10]) -> [u32; 10] {
    let keep = match d[1] {
        1 => 9, // Buffer (d[9] is trailing struct padding)
        2 => 3, // Texture1D
        3 => 5, // Texture1DArray
        4 => 4, // Texture2D
        5 => 6, // Texture2DArray
        6 => 2, // Texture2DMS (UnusedField_NothingToDefine only)
        7 => 4, // Texture2DMSArray
        8 => 5, // Texture3D
        _ => 10,
    };
    d[keep..].fill(0);
    d
}

fn canonical_placed_desc(mut desc: [u32; 16], canonical: fn([u32; 10]) -> [u32; 10]) -> [u32; 16] {
    let head = canonical(std::array::from_fn(|i| desc[i]));
    desc[..10].copy_from_slice(&head);
    desc
}

/// Device method.
pub(crate) extern "system" fn create_cbv(_: This, desc: *const [u32; 4], handle: u64) {
    if let Some([lo, hi, size, _]) = words(desc) {
        let address = u64::from(lo) | u64::from(hi) << 32;
        record(handle, View::Constant(address, size));
    }
}

/// Device method.
pub(crate) extern "system" fn create_srv(
    _: This,
    resource: This,
    desc: *const [u32; 10],
    handle: u64,
) {
    record(
        handle,
        View::Shader(resource as usize, words(desc).map(canonical_srv_desc)),
    );
}

/// Device method.
pub(crate) extern "system" fn create_uav(
    _: This,
    resource: This,
    counter: This,
    desc: *const [u32; 10],
    handle: u64,
) {
    let view = View::Unordered(
        resource as usize,
        counter as usize,
        words(desc).map(canonical_uav_desc),
    );
    record(handle, view);
}

/// Device methods 0x230 and 0x238: shader resource and unordered access
/// views of memory the title placed itself, `(resource desc, view desc,
/// handle)`.
pub(crate) extern "system" fn create_placed_srv(
    _: This,
    resource: *const [u32; 14],
    desc: *const [u32; 16],
    handle: u64,
) {
    if let (Some(resource), Some(desc)) = (words(resource), words(desc)) {
        let resource = crate::resource::canonical_resource_desc(resource);
        let desc = canonical_placed_desc(desc, canonical_srv_desc);
        record(handle, View::Placed(false, resource, desc));
    }
}

pub(crate) extern "system" fn create_placed_uav(
    _: This,
    resource: *const [u32; 14],
    desc: *const [u32; 16],
    handle: u64,
) {
    if let (Some(resource), Some(desc)) = (words(resource), words(desc)) {
        let resource = crate::resource::canonical_resource_desc(resource);
        let desc = canonical_placed_desc(desc, canonical_uav_desc);
        record(handle, View::Placed(true, resource, desc));
    }
}

/// Device method.
pub(crate) extern "system" fn create_sampler(_: This, desc: *const [u32; 13], handle: u64) {
    if let Some(desc) = words(desc) {
        record(handle, View::Sampler(desc));
    }
}

/// Device method 0x1e8.
pub(crate) extern "system" fn create_sampler_x(_: This, desc: *const [u32; 27], handle: u64) {
    if let Some(desc) = words(desc) {
        record(handle, View::ConsoleSampler(desc));
    }
}

/// Device method: `count` descriptors of `kind` from `src` to `dst`.
pub(crate) extern "system" fn copy_simple(_: This, count: u32, dst: u64, src: u64, kind: u32) {
    let step = u64::from(increment(kind).unwrap_or(0));
    let len = u64::from(count) * step;
    if dst != 0 && src != 0 {
        // SAFETY: both ranges are descriptor slots the title owns.
        unsafe { std::ptr::copy(src as *const u8, dst as *mut u8, len as usize) };
        #[cfg(windows)]
        for i in 0..u64::from(count) {
            mirror(dst + i * step);
        }
    }
}

/// Device method: ranges of descriptors (start and count each) gathered
/// from the sources and scattered to the destinations, in order.
#[allow(clippy::too_many_arguments)] // the method's own arguments
pub(crate) extern "system" fn copy(
    _: This,
    dst_ranges: u32,
    dst_starts: *const u64,
    dst_sizes: *const u32,
    src_ranges: u32,
    src_starts: *const u64,
    src_sizes: *const u32,
    kind: u32,
) {
    let Some(step) = increment(kind) else {
        return;
    };
    // Each range as (start, count); a null size array means one descriptor
    // per range.
    let ranges = |n: u32, starts: *const u64, sizes: *const u32| -> Vec<(u64, u32)> {
        (0..n as usize)
            .map(|i| {
                // SAFETY: the title passes `n` starts and sizes (or null sizes).
                unsafe {
                    (
                        *starts.add(i),
                        if sizes.is_null() { 1 } else { *sizes.add(i) },
                    )
                }
            })
            .collect()
    };
    if dst_starts.is_null() || src_starts.is_null() {
        return;
    }
    let src = ranges(src_ranges, src_starts, src_sizes);
    let mut dst = ranges(dst_ranges, dst_starts, dst_sizes).into_iter();
    let (mut to, mut room) = dst.next().unwrap_or((0, 0));
    for (mut from, mut left) in src {
        while left > 0 {
            while room == 0 {
                let Some(next) = dst.next() else { return };
                (to, room) = next;
            }
            let n = left.min(room);
            copy_simple(std::ptr::null(), n, to, from, kind);
            let bytes = u64::from(n) * u64::from(step);
            (to, from, room, left) = (to + bytes, from + bytes, room - n, left - n);
        }
    }
}

/// The view recorded at `handle`, if one is.
pub(crate) fn rtv_record(handle: u64) -> Option<RtvRecord> {
    tagged_record(handle, RTV_TAG)
}

/// The depth view recorded at `handle`, if one is.
pub(crate) fn dsv_record(handle: u64) -> Option<RtvRecord> {
    tagged_record(handle, DSV_TAG)
}

fn tagged_record(handle: u64, tag: u32) -> Option<RtvRecord> {
    if handle == 0 {
        return None;
    }
    // SAFETY: the handle is a descriptor slot the title passes back.
    let record = unsafe { (handle as *const RtvRecord).read_unaligned() };
    (record.tag == tag).then_some(record)
}

/// Device method.
pub(crate) extern "system" fn create_heap(
    _: This,
    desc_ptr: *const HeapDesc,
    iid: *const Iid,
    out: *mut This,
) -> HRESULT {
    // SAFETY: the title passes a description (null is tolerated).
    let Some(&desc) = (unsafe { desc_ptr.as_ref() }) else {
        return E_INVALIDARG;
    };
    let Some(size) = increment(desc.kind).map(|i| u64::from(i) * u64::from(desc.count.max(1)))
    else {
        return E_INVALIDARG;
    };
    // SAFETY: read only as a hint: a title that passes the plain
    // description has its own stack or data after it.
    let placement = unsafe { desc_ptr.add(1).cast::<Placement>().read_unaligned() };
    // shortcut: a placement is recognised by its size matching the count,
    // since the plain and extended descriptions share the slot and the
    // interface version; find the flag that tells them apart if a title
    // trips over this.
    let placed = placement.address != 0 && placement.size == size;
    tracing::debug!(?desc, placed, "descriptor heap");
    let (start, memory) = if placed {
        (placement.address, None)
    } else {
        match Memory::zeroed(size, 64) {
            Ok(memory) => (memory.address(), Some(memory)),
            Err(hr) => return hr,
        }
    };
    let visible = desc.flags & SHADER_VISIBLE != 0 && matches!(desc.kind, VIEWS_KIND | SAMPLERS);
    let heap = Box::new(Heap {
        object: Object::new(&FUNCTIONS),
        desc,
        start,
        memory,
        #[cfg(windows)]
        host: std::sync::OnceLock::new(),
        #[cfg(windows)]
        pending: Default::default(),
    });
    #[cfg(windows)]
    if visible {
        let at = std::ptr::from_ref::<Heap>(&heap) as usize;
        let mut heaps = SHADER_HEAPS.lock().unwrap_or_else(|e| e.into_inner());
        heaps.push((start, start + size, at));
    }
    #[cfg(not(windows))]
    let _ = visible;
    object::hand_out(DESCRIPTOR_HEAP, heap, iid, &HEAP_IIDS, out)
}

extern "system" fn query(this: This, iid: *const Iid, out: *mut This) -> HRESULT {
    let found = object::is(iid, &HEAP_IIDS).then_some(this);
    object::answer(DESCRIPTOR_HEAP, iid, found, out)
}
extern "system" fn release(this: This) -> u32 {
    object::release_boxed::<Heap>(this)
}
extern "system" fn desc(this: This, out: *mut HeapDesc) {
    if !out.is_null() {
        // SAFETY: checked non-null; the title passes room for a description.
        unsafe { out.write(heap(this).desc) };
    }
}
/// The same address for the CPU and the GPU start.
extern "system" fn start(this: This) -> u64 {
    heap(this).start
}

/// IUnknown 3, object 4, device child 1, heap 3.
static FUNCTIONS: Table<24> = {
    let mut f = object::d3d12_object(object::missing_table::<DESCRIPTOR_HEAP, 24>());
    f[0] = query as *const ();
    f[2] = release as *const ();
    f[7] = object::parent_device as *const (); // 0x38
    f[8] = desc as *const (); // 0x40
    f[9] = start as *const (); // 0x48 CPU
    f[10] = start as *const (); // 0x50 GPU
    Table(f)
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::{S_OK, slot};

    /// What newer titles pass; tests use it throughout, so that reading the
    /// placement stays inside the value.
    #[repr(C)]
    struct Extended(HeapDesc, Placement);

    fn create(desc: HeapDesc, placement: Placement, out: &mut This) -> HRESULT {
        let extended = Extended(desc, placement);
        create_heap(std::ptr::null(), &extended.0, &HEAP_IIDS[0], out)
    }

    #[test]
    fn heaps_hold_count_descriptors_from_their_start() {
        let null = std::ptr::null();
        let desc = HeapDesc {
            kind: 0,
            count: 1000,
            flags: 1,
            nodes: 0,
        };
        let none = Placement {
            address: 0,
            size: 0,
        };
        let mut h = null;
        assert_eq!(create(desc, none, &mut h), S_OK);
        let cpu = slot::<extern "system" fn(This) -> u64>(h, 0x48)(h);
        assert_eq!(cpu % 64, 0);
        assert_eq!(slot::<extern "system" fn(This) -> u64>(h, 0x50)(h), cpu);
        assert_eq!(increment_size(null, 0), 32);
        // SAFETY: the heap holds 1000 descriptors of 32 bytes.
        unsafe { (cpu as *mut u8).add(999 * 32 + 31).write(1) };
        assert_eq!(increment_size(null, 9), 0);
        let bad = HeapDesc { kind: 9, ..desc };
        assert_eq!(create(bad, none, &mut h), E_INVALIDARG);
    }

    #[test]
    fn views_survive_copies() {
        let null = std::ptr::null();
        let mut heap = vec![0u64; 4 * 8]; // four 32-byte slots
        let at = |i: usize| heap.as_ptr() as u64 + 32 * i as u64;
        let (s0, s1, s2, s3) = (at(0), at(1), at(2), at(3));
        create_cbv(null, &[0x1000, 1, 256, 0], s0);
        assert_eq!(view(s0), Some(View::Constant(0x1_0000_1000, 256)));
        create_srv(null, null, std::ptr::null(), s1);
        assert_eq!(view(s1), Some(View::Shader(0, None)));
        // The same view twice is one entry.
        create_srv(null, null, std::ptr::null(), s2);
        // SAFETY: both slots are in `heap`.
        assert_eq!(unsafe { *(s1 as *const [u32; 3]) }, unsafe {
            *(s2 as *const [u32; 3])
        });
        // Two ranges of one into one range of two.
        let (dst, dst_n, src, src_n) = ([s2], [2u32], [s0, s1], [1u32, 1]);
        copy(
            null,
            1,
            dst.as_ptr(),
            dst_n.as_ptr(),
            2,
            src.as_ptr(),
            src_n.as_ptr(),
            0,
        );
        assert_eq!(view(s2), view(s0));
        assert_eq!(view(s3), view(s1));
        assert_eq!(view(at(0) + 8), None);
        heap[0] = 0;
    }

    /// A released resource's views go stale; the same view of a new
    /// resource at the same address is a new record.
    #[test]
    fn views_die_with_their_resource() {
        let null = std::ptr::null();
        let slots = [0u64; 2 * 4];
        let (a, b) = (slots.as_ptr() as u64, slots.as_ptr() as u64 + 32);
        let resource = 0x7777_0000usize as This;
        create_srv(null, resource, std::ptr::null(), a);
        assert_eq!(view(a), Some(View::Shader(0x7777_0000, None)));
        forget(0x7777_0000);
        assert_eq!(view(a), None, "stale after release");
        create_srv(null, resource, std::ptr::null(), b);
        assert_eq!(view(b), Some(View::Shader(0x7777_0000, None)));
        // SAFETY: both slots are in `slots`.
        let (old, new) = unsafe { (*(a as *const [u32; 3]), *(b as *const [u32; 3])) };
        assert_ne!(old, new, "a new generation");
    }

    /// Grand Theft Auto VI places its shader-visible heap in its own memory
    /// and checks that the heap starts there.
    #[test]
    fn placed_heaps_start_at_the_title_memory() {
        let mut memory = vec![0u8; 64 * 32];
        let address = memory.as_mut_ptr() as u64;
        let desc = HeapDesc {
            kind: 0,
            count: 64,
            flags: 1,
            nodes: 1,
        };
        let mut h = std::ptr::null();
        assert_eq!(
            create(
                desc,
                Placement {
                    address,
                    size: 64 * 32
                },
                &mut h
            ),
            S_OK
        );
        assert_eq!(slot::<extern "system" fn(This) -> u64>(h, 0x48)(h), address);
        assert_eq!(slot::<extern "system" fn(This) -> u64>(h, 0x50)(h), address);
        // A size that does not match the count is not a placement.
        assert_eq!(create(desc, Placement { address, size: 7 }, &mut h), S_OK);
        assert_ne!(slot::<extern "system" fn(This) -> u64>(h, 0x48)(h), address);
    }

    #[test]
    fn canonicalization_deduplicates_padding_and_unused_union_words() {
        let mut raw_res = [0u32; 14];
        for (i, word) in raw_res.iter_mut().enumerate() {
            *word = 0x100 + i as u32;
        }
        let canon_res = crate::resource::canonical_resource_desc(raw_res);
        assert_eq!(canon_res[1], 0);
        assert_eq!(canon_res[13], 0);
        assert_eq!(canon_res[0], raw_res[0]);
        assert_eq!(canon_res[2..13], raw_res[2..13]);

        let slots = [0u64; 4 * 4]; // four 32-byte slots
        let slot_at = |i: usize| slots.as_ptr() as u64 + 32 * i as u64;
        let raw_record = |h: u64| -> [u32; 3] {
            // SAFETY: `h` points inside `slots`.
            unsafe { *(h as *const [u32; 3]) }
        };

        let null = std::ptr::null();
        let resource = 0x6666_0000usize as This;
        let (s0, s1, s2, s3) = (slot_at(0), slot_at(1), slot_at(2), slot_at(3));

        // SRV dimensions 1..=11: padding word d[3] and tail d[keep..] are zeroed.
        let srv_keeps = [
            (1u32, 9usize),
            (2, 7),
            (3, 9),
            (4, 8),
            (5, 10),
            (6, 4),
            (7, 6),
            (8, 7),
            (9, 7),
            (10, 9),
            (11, 6),
        ];
        for &(dim, keep) in &srv_keeps {
            let mut a = [0u32; 10];
            a[0] = 28;
            a[1] = dim;
            a[2] = 0x1688;
            for (i, w) in a[4..keep].iter_mut().enumerate() {
                *w = 10 + dim * 10 + i as u32;
            }
            let mut b = a;
            a[3] = 0xdead_beef;
            b[3] = 0xcafe_babe;
            for (i, w) in a[keep..].iter_mut().enumerate() {
                *w = 0xaaaa_0000 + i as u32;
            }
            for (i, w) in b[keep..].iter_mut().enumerate() {
                *w = 0x5555_0000 + i as u32;
            }

            create_srv(null, resource, &a, s0);
            create_srv(null, resource, &b, s1);
            assert_eq!(raw_record(s0), raw_record(s1), "SRV dim {dim} deduplicates");
            let Some(View::Shader(_, Some(stored))) = view(s0) else {
                panic!("expected View::Shader for SRV dim {dim}");
            };
            assert_eq!(stored[3], 0, "SRV dim {dim} zeroes d[3]");
            assert!(
                stored[keep..].iter().all(|&w| w == 0),
                "SRV dim {dim} zeroes tail d[{keep}..]"
            );
            assert_eq!(
                stored[4..keep],
                a[4..keep],
                "SRV dim {dim} keeps live words"
            );
        }
        let srv_live_a = [28, 4, 0x1688, 0, 0, 1, 0, 0, 0, 0];
        let srv_live_b = [28, 4, 0x1688, 0, 1, 1, 0, 0, 0, 0];
        create_srv(null, resource, &srv_live_a, s0);
        create_srv(null, resource, &srv_live_b, s1);
        assert_ne!(raw_record(s0), raw_record(s1));

        // UAV dimensions 1..=8: union starts at d[2] with no leading padding.
        let uav_keeps = [
            (1u32, 9usize),
            (2, 3),
            (3, 5),
            (4, 4),
            (5, 6),
            (6, 2),
            (7, 4),
            (8, 5),
        ];
        for &(dim, keep) in &uav_keeps {
            let mut a = [0u32; 10];
            a[0] = 28;
            a[1] = dim;
            for (i, w) in a[2..keep].iter_mut().enumerate() {
                *w = 20 + dim * 10 + i as u32;
            }
            let mut b = a;
            for (i, w) in a[keep..].iter_mut().enumerate() {
                *w = 0xbbbb_0000 + i as u32;
            }
            for (i, w) in b[keep..].iter_mut().enumerate() {
                *w = 0x4444_0000 + i as u32;
            }

            create_uav(null, resource, null, &a, s0);
            create_uav(null, resource, null, &b, s1);
            assert_eq!(raw_record(s0), raw_record(s1), "UAV dim {dim} deduplicates");
            let Some(View::Unordered(_, _, Some(stored))) = view(s0) else {
                panic!("expected View::Unordered for UAV dim {dim}");
            };
            assert!(
                stored[keep..].iter().all(|&w| w == 0),
                "UAV dim {dim} zeroes tail d[{keep}..]"
            );
            assert_eq!(
                stored[2..keep],
                a[2..keep],
                "UAV dim {dim} keeps live words"
            );
        }
        let uav_live_a = [28, 4, 0, 0, 0, 0, 0, 0, 0, 0];
        let uav_live_b = [28, 4, 1, 0, 0, 0, 0, 0, 0, 0];
        create_uav(null, resource, null, &uav_live_a, s0);
        create_uav(null, resource, null, &uav_live_b, s1);
        assert_ne!(raw_record(s0), raw_record(s1));

        // Placed SRV and Placed UAV canonicalize ResourceDesc + first 10 view words
        // while preserving desc[10..16] (GPU address and console fields).
        let res_a = [
            3,
            0x1111_2222,
            0,
            0,
            16,
            0,
            16,
            0x0001_0001,
            28,
            1,
            0,
            0,
            0,
            0x3333_4444,
        ];
        let mut res_b = res_a;
        res_b[1] = 0x5555_6666;
        res_b[13] = 0x7777_8888;

        let psrv_a = [
            28, 4, 0x1688, 0x1234, 0, 1, 0, 0, 0x9abc, 0xdef0, 0x1000, 0x2, 3, 4, 5, 6,
        ];
        let mut psrv_b = psrv_a;
        psrv_b[3] = 0x4321;
        psrv_b[8] = 0x1111;
        psrv_b[9] = 0x2222;
        create_placed_srv(null, &res_a, &psrv_a, s0);
        create_placed_srv(null, &res_b, &psrv_b, s1);
        assert_eq!(raw_record(s0), raw_record(s1));
        let Some(View::Placed(false, stored_res, stored_desc)) = view(s0) else {
            panic!("expected placed SRV");
        };
        assert_eq!((stored_res[1], stored_res[13]), (0, 0));
        assert_eq!((stored_desc[3], stored_desc[8], stored_desc[9]), (0, 0, 0));
        assert_eq!(stored_desc[10..16], psrv_a[10..16]);

        let mut psrv_diff_tail = psrv_a;
        psrv_diff_tail[15] = 99;
        create_placed_srv(null, &res_a, &psrv_diff_tail, s2);
        assert_ne!(raw_record(s0), raw_record(s2));

        let puav_a = [
            28, 4, 1, 0, 0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x6666, 0x2000, 0x3, 7, 8, 9, 10,
        ];
        let mut puav_b = puav_a;
        puav_b[4..10].fill(0xffff_0000);
        create_placed_uav(null, &res_a, &puav_a, s2);
        create_placed_uav(null, &res_b, &puav_b, s3);
        assert_eq!(raw_record(s2), raw_record(s3));
        let Some(View::Placed(true, stored_res, stored_desc)) = view(s2) else {
            panic!("expected placed UAV");
        };
        assert_eq!((stored_res[1], stored_res[13]), (0, 0));
        assert!(stored_desc[4..10].iter().all(|&w| w == 0));
        assert_eq!(stored_desc[10..16], puav_a[10..16]);

        forget(resource as usize);
        let _ = slots[0];
    }
}
