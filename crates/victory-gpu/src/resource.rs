//! Heaps, committed and placed resources, and resource sizes.
//!
//! Heaps and resources are host memory and a resource's GPU address is its
//! CPU address, as on the console; textures also get a host copy, and
//! buffers a host copy when a draw reads them (`shadow`).
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::alloc::{Layout, alloc_zeroed, dealloc};
use std::ffi::c_void;
use std::ptr::NonNull;

use victory_formats::texture;

use crate::object::{
    self, E_INVALIDARG, E_OUTOFMEMORY, HEAP, HRESULT, Iid, Object, RESOURCE, S_OK, Table, This,
    guid,
};

/// The interfaces every device child also answers for.
pub(crate) const PAGEABLE_IIDS: [Iid; 3] = [
    guid(
        0x63ee58fb,
        0x1268,
        0x4835,
        [0x86, 0xda, 0xf0, 0x08, 0xce, 0x62, 0xf0, 0xd6],
    ),
    guid(
        0x905db94b,
        0xa00c,
        0x4140,
        [0x9d, 0xf5, 0x2b, 0x64, 0xca, 0x9e, 0xa3, 0x57],
    ),
    guid(
        0xc4fec28f,
        0x7966,
        0x4e95,
        [0x9f, 0x94, 0xf4, 0x31, 0xcb, 0x56, 0xc3, 0xb8],
    ),
];
const HEAP_IIDS: [Iid; 4] = [
    guid(
        0x6b3b2502,
        0x6e51,
        0x45b3,
        [0x90, 0xee, 0x98, 0x84, 0x26, 0x5e, 0x8d, 0xf3],
    ),
    PAGEABLE_IIDS[0],
    PAGEABLE_IIDS[1],
    PAGEABLE_IIDS[2],
];
const RESOURCE_IIDS: [Iid; 4] = [
    guid(
        0x696442be,
        0xa72e,
        0x4059,
        [0xbc, 0x79, 0x5b, 0x5c, 0x98, 0x04, 0x0f, 0xad],
    ),
    PAGEABLE_IIDS[0],
    PAGEABLE_IIDS[1],
    PAGEABLE_IIDS[2],
];

const KIB_64: u64 = 64 << 10;
const MIB_4: u64 = 4 << 20;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct HeapProperties {
    kind: u32,
    cpu_pages: u32,
    pool: u32,
    creation_nodes: u32,
    visible_nodes: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct HeapDesc {
    size: u64,
    properties: HeapProperties,
    alignment: u64,
    flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ResourceDesc {
    dimension: u32,
    alignment: u64,
    width: u64,
    height: u32,
    depth_or_array_size: u16,
    mip_levels: u16,
    format: u32,
    sample_count: u32,
    sample_quality: u32,
    layout: u32,
    flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AllocationInfo {
    size: u64,
    alignment: u64,
}

const _: () = assert!(size_of::<HeapProperties>() == 20);
const _: () = assert!(size_of::<HeapDesc>() == 48);
const _: () = {
    use std::mem::offset_of;
    assert!(offset_of!(ResourceDesc, dimension) == 0);
    assert!(offset_of!(ResourceDesc, alignment) == 8);
    assert!(offset_of!(ResourceDesc, flags) == 48);
    assert!(size_of::<ResourceDesc>() == 56);
};

/// Zeroes the struct padding words (offsets 4..8 and 52..56) of a raw
/// `ResourceDesc` word array so equal descriptions compare identically.
pub(crate) fn canonical_resource_desc(mut d: [u32; 14]) -> [u32; 14] {
    d[1] = 0;
    d[13] = 0;
    d
}

const BUFFER: u32 = 1;
const TEXTURE_3D: u32 = 4;

/// Bytes per element and the element's width in texels (4 for block
/// compression), by DXGI format number. `None` for 1-bit and video
/// formats. The console adds 116/117 (10-bit float color), 118-120 (16-bit
/// depth with 8-bit stencil, two planes: 3 bytes per texel), 189 (signed
/// 10-bit color), 190 (two 4-bit channels) and 191 (24/8 color).
pub(crate) fn element(format: u32) -> Option<(u64, u64)> {
    Some(match format {
        1..=4 => (16, 1),
        5..=8 => (12, 1),
        9..=22 => (8, 1),
        23..=47 | 67 | 87..=93 | 116 | 117 | 189 | 191 => (4, 1),
        118..=120 => (3, 1),
        48..=59 | 68 | 69 | 85 | 86 | 115 => (2, 1),
        60..=65 | 190 => (1, 1),
        70..=72 | 79..=81 => (8, 4),
        73..=78 | 82..=84 | 94..=99 => (16, 4),
        _ => return None,
    })
}

/// A texture's layout in console memory, as the console's layout library
/// gives it (`victory-hle`'s `xg` serves the same): the description's
/// layout field is `0x100 | swizzle mode`, 1 linear, 0 the library's pick.
pub(crate) fn texture_layout(desc: &ResourceDesc) -> Option<texture::Texture> {
    use texture::{Dimension, Swizzle, TextureDesc};
    const TEXTURE_1D: u32 = 2;
    const TEXTURE_2D: u32 = 3;
    const RENDER_TARGET: u32 = 1;
    const DEPTH: u32 = 2;
    let dimension = match desc.dimension {
        TEXTURE_1D => Dimension::Tex1d,
        TEXTURE_2D => Dimension::Tex2d,
        TEXTURE_3D => Dimension::Tex3d,
        _ => return None,
    };
    let swizzle = match desc.layout {
        0 if dimension == Dimension::Tex1d => Swizzle::LINEAR,
        0 if desc.flags & DEPTH != 0 => Swizzle::SW_64KB_Z_X,
        0 if desc.flags & RENDER_TARGET != 0 => Swizzle::SW_64KB_R_X,
        0 => Swizzle::SW_4KB_S,
        1 => Swizzle::LINEAR,
        l if l & !0xff == 0x100 => Swizzle(l & 0xff),
        _ => return None,
    };
    texture::Texture::new(&TextureDesc {
        dimension,
        width: u32::try_from(desc.width).ok()?,
        height: desc.height,
        depth_or_array: u32::from(desc.depth_or_array_size),
        mips: u32::from(desc.mip_levels),
        format: desc.format,
        samples: desc.sample_count.max(1),
        swizzle,
    })
    .ok()
}

/// Bytes and alignment a resource takes: buffers their width (64 KiB
/// aligned), textures their console layout (formats it does not know:
/// every mip of every array slice and sample, packed, 64 KiB aligned),
/// rounded up to the alignment.
fn footprint(desc: &ResourceDesc) -> Option<AllocationInfo> {
    if desc.dimension != BUFFER
        && let Some(t) = texture_layout(desc)
    {
        // As the console: the layout's own alignment unless more is asked
        // (4 KiB for 4 KiB modes, where PC says 64 KiB). Grand Theft Auto
        // VI keeps these sizes in its texture tables, and its streaming
        // asserts when they disagree with the streamed layout.
        let alignment = desc.alignment.max(t.alignment);
        if !alignment.is_power_of_two() {
            return None;
        }
        return Some(AllocationInfo {
            size: t.size.checked_next_multiple_of(alignment)?,
            alignment,
        });
    }
    let bytes = if desc.dimension == BUFFER {
        desc.width
    } else {
        let (element, block) = element(desc.format)?;
        let (depth, slices) = if desc.dimension == TEXTURE_3D {
            (u64::from(desc.depth_or_array_size), 1)
        } else {
            (1, u64::from(desc.depth_or_array_size))
        };
        let height = u64::from(desc.height);
        let mips = mip_count(desc);
        let mut chain = 0u64;
        for mip in 0..mips.min(64) {
            let w = (desc.width >> mip).max(1).div_ceil(block);
            let h = (height >> mip).max(1).div_ceil(block);
            let d = (depth >> mip).max(1);
            chain = chain.checked_add(w.checked_mul(h)?.checked_mul(d)?.checked_mul(element)?)?;
        }
        chain
            .checked_mul(slices)?
            .checked_mul(u64::from(desc.sample_count.max(1)))?
    };
    if bytes == 0 {
        return None;
    }
    let minimum = if desc.sample_count > 1 { MIB_4 } else { KIB_64 };
    let alignment = desc.alignment.max(minimum);
    if !alignment.is_power_of_two() {
        return None;
    }
    Some(AllocationInfo {
        size: bytes.checked_next_multiple_of(KIB_64)?,
        alignment,
    })
}

/// A format as the host has it. The console's 16-bit depth with stencil
/// (and its typeless views) has no PC format: the 24-bit ones stand in.
pub(crate) fn pc_format(format: u32) -> u32 {
    match format {
        118 => 45,
        119 => 46,
        120 => 47,
        other => other,
    }
}

/// The typeless format of a format's family (itself if it has none).
pub(crate) fn family(format: u32) -> u32 {
    const FAMILIES: [(u32, u32); 22] = [
        (1, 4),
        (5, 8),
        (9, 14),
        (15, 18),
        (19, 22),
        (23, 25),
        (27, 32),
        (33, 38),
        (39, 43),
        (44, 47),
        (48, 52),
        (53, 59),
        (60, 65),
        (70, 72),
        (73, 75),
        (76, 78),
        (79, 81),
        (82, 84),
        (90, 91),
        (92, 93),
        (94, 96),
        (97, 99),
    ];
    match format {
        87 => 90,
        88 => 92,
        f => FAMILIES
            .iter()
            .find(|&&(first, last)| (first..=last).contains(&f))
            .map_or(f, |&(first, _)| first),
    }
}

/// sRGB formats and their linear twins.
pub(crate) const SRGB: [(u32, u32); 3] = [(29, 28), (91, 87), (93, 88)];

/// The format a view of a title texture must name when its host copy is
/// typeless (see [`DEPTH_FORMATS`], [`SRGB`]): the title's own; `None` if
/// the host copy is typed.
pub(crate) fn view_format(this: This) -> Option<u32> {
    let own = pc_format(resource(this).desc.format);
    let host = host_resource(this)?;
    // SAFETY: a live resource.
    let format = unsafe { host.GetDesc() }.Format.0 as u32;
    (format != own).then_some(own)
}

/// Depth formats with their typeless family: depth textures that shaders
/// read are made typeless on the host, which only allows views of
/// typeless depth resources.
pub(crate) const DEPTH_FORMATS: [(u32, u32); 4] = [(20, 19), (40, 39), (45, 44), (55, 53)];

/// Mip levels, the full chain for 0.
fn mip_count(desc: &ResourceDesc) -> u32 {
    let depth = if desc.dimension == TEXTURE_3D {
        u64::from(desc.depth_or_array_size)
    } else {
        1
    };
    match desc.mip_levels {
        0 => {
            64 - desc
                .width
                .max(u64::from(desc.height))
                .max(depth)
                .max(1)
                .leading_zeros()
        }
        n => u32::from(n),
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlacedFootprint {
    pub(crate) offset: u64,
    pub(crate) format: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) depth: u32,
    pub(crate) row_pitch: u32,
}

impl PlacedFootprint {
    /// `(lines, row_bytes, total_bytes)` spanned in a buffer: every row
    /// pitched but the last.
    pub(crate) fn layout(&self) -> Option<(u64, u64, u64)> {
        let (element, block) = element(self.format)?;
        let lines = u64::from(self.height)
            .div_ceil(block)
            .checked_mul(u64::from(self.depth))?;
        let row_bytes = u64::from(self.width).div_ceil(block).checked_mul(element)?;
        if lines == 0 {
            return Some((0, row_bytes, 0));
        }
        let total = u64::from(self.row_pitch)
            .checked_mul(lines - 1)?
            .checked_add(row_bytes)?;
        Some((lines, row_bytes, total))
    }

    /// The bytes it spans in a buffer: every row pitched but the last.
    pub(crate) fn bytes(&self) -> Option<u64> {
        self.layout().map(|(_, _, total)| total)
    }
}
const _: () = assert!(size_of::<PlacedFootprint>() == 32);

const NO_FOOTPRINT: PlacedFootprint = PlacedFootprint {
    offset: u64::MAX,
    format: 0,
    width: 0,
    height: 0,
    depth: 0,
    row_pitch: 0,
};

/// One subresource's size in an upload buffer: footprint (offset 0), rows
/// and bytes per row. Planes of depth-stencil formats are not told apart.
fn subresource(desc: &ResourceDesc, index: u32) -> Option<(PlacedFootprint, u32, u64)> {
    const PITCH: u64 = 256;
    if desc.dimension == BUFFER {
        let footprint = PlacedFootprint {
            offset: 0,
            format: 0,
            width: u32::try_from(desc.width).ok()?,
            height: 1,
            depth: 1,
            row_pitch: u32::try_from(desc.width.next_multiple_of(PITCH)).ok()?,
        };
        return (index == 0).then_some((footprint, 1, desc.width));
    }
    let mips = mip_count(desc);
    let slices = if desc.dimension == TEXTURE_3D {
        1
    } else {
        u32::from(desc.depth_or_array_size).max(1)
    };
    let (mip, slice) = (index % mips, index / mips);
    if slice >= slices {
        return None;
    }
    let (element, block) = element(desc.format)?;
    let width = (desc.width >> mip).max(1).next_multiple_of(block);
    let height = (u64::from(desc.height) >> mip)
        .max(1)
        .next_multiple_of(block);
    let depth = if desc.dimension == TEXTURE_3D {
        (u32::from(desc.depth_or_array_size) >> mip).max(1)
    } else {
        1
    };
    let row_size = width / block * element;
    let footprint = PlacedFootprint {
        offset: 0,
        format: desc.format,
        width: u32::try_from(width).ok()?,
        height: u32::try_from(height).ok()?,
        depth,
        row_pitch: u32::try_from(row_size.next_multiple_of(PITCH)).ok()?,
    };
    Some((footprint, u32::try_from(height / block).ok()?, row_size))
}

/// Device method: where each subresource goes in an upload buffer, rows
/// 256-byte and subresources 512-byte aligned, as on PC. Unknown sizes give
/// the documented all-ones values.
pub(crate) extern "system" fn copyable_footprints(
    _: This,
    desc: *const ResourceDesc,
    first: u32,
    count: u32,
    base: u64,
    layouts: *mut PlacedFootprint,
    rows: *mut u32,
    row_sizes: *mut u64,
    total: *mut u64,
) {
    const PLACEMENT: u64 = 512;
    // SAFETY: the title passes a description (null is tolerated).
    let desc = unsafe { desc.as_ref() };
    let mut end = base;
    let mut known = desc.is_some();
    for i in 0..count {
        let one = desc.and_then(|d| subresource(d, first + i));
        let (layout, row_count, row_size) = match one {
            Some((mut footprint, row_count, row_size)) => {
                footprint.offset = end.next_multiple_of(PLACEMENT);
                let pitch = u64::from(footprint.row_pitch);
                let lines = u64::from(row_count) * u64::from(footprint.depth);
                end = footprint.offset + pitch * lines.saturating_sub(1) + row_size;
                (footprint, row_count, row_size)
            }
            None => {
                known = false;
                (NO_FOOTPRINT, u32::MAX, u64::MAX)
            }
        };
        let i = i as usize;
        // SAFETY: each output is null or holds `count` entries.
        unsafe {
            if !layouts.is_null() {
                layouts.add(i).write(layout);
            }
            if !rows.is_null() {
                rows.add(i).write(row_count);
            }
            if !row_sizes.is_null() {
                row_sizes.add(i).write(row_size);
            }
        }
    }
    if !known {
        tracing::warn!(?desc, "no footprint for this resource");
    }
    if !total.is_null() {
        // SAFETY: checked non-null.
        unsafe { total.write(if known { end - base } else { u64::MAX }) };
    }
}

/// Device method: the footprint of resources placed one after another.
/// Returns a 16-byte value, through a hidden first argument. Invalid input
/// gives the documented size `u64::MAX`.
pub(crate) extern "system" fn allocation_info(
    out: *mut AllocationInfo,
    _: This,
    _visible: u32,
    count: u32,
    descs: *const ResourceDesc,
) -> *mut AllocationInfo {
    let descs = if descs.is_null() {
        &[][..]
    } else {
        // SAFETY: the title passes `count` descriptions.
        unsafe { std::slice::from_raw_parts(descs, count as usize) }
    };
    // SAFETY: the caller passes room for the value.
    unsafe { out.write(place(descs.iter(), |_, _| ())) };
    out
}

/// The newer description: the plain one, then a sampler feedback mip
/// region (width, height, depth).
#[repr(C)]
pub(crate) struct ResourceDesc1 {
    desc: ResourceDesc,
    mip_region: [u32; 3],
}
const _: () = assert!(size_of::<ResourceDesc1>() == 72);

/// Where one resource goes in [`allocation_info2`]'s placement.
#[repr(C)]
pub(crate) struct AllocationInfo1 {
    offset: u64,
    alignment: u64,
    size: u64,
}

/// Device method 0x360 (`ID3D12Device8`): [`allocation_info`] for newer
/// descriptions, also telling where each resource goes.
pub(crate) extern "system" fn allocation_info2(
    out: *mut AllocationInfo,
    _: This,
    _visible: u32,
    count: u32,
    descs: *const ResourceDesc1,
    each: *mut AllocationInfo1,
) -> *mut AllocationInfo {
    let descs = if descs.is_null() {
        &[][..]
    } else {
        // SAFETY: the title passes `count` descriptions.
        unsafe { std::slice::from_raw_parts(descs, count as usize) }
    };
    let total = place(descs.iter().map(|d| &d.desc), |i, info| {
        if !each.is_null() {
            // SAFETY: the title passes room for `count` entries.
            unsafe { each.add(i).write(info) };
        }
    });
    // SAFETY: the caller passes room for the value.
    unsafe { out.write(total) };
    out
}

/// Resources placed one after another: the whole footprint, telling `each`
/// where every resource goes. Unknown sizes give the documented `u64::MAX`.
fn place<'a>(
    descs: impl Iterator<Item = &'a ResourceDesc>,
    mut each: impl FnMut(usize, AllocationInfo1),
) -> AllocationInfo {
    let mut total = AllocationInfo {
        size: 0,
        alignment: KIB_64,
    };
    for (i, desc) in descs.enumerate() {
        let Some(one) = footprint(desc) else {
            tracing::warn!(?desc, "no size for this resource");
            each(
                i,
                AllocationInfo1 {
                    offset: u64::MAX,
                    alignment: 0,
                    size: u64::MAX,
                },
            );
            return AllocationInfo {
                size: u64::MAX,
                alignment: 0,
            };
        };
        tracing::debug!(?desc, ?one, "allocation info");
        let offset = total.size.next_multiple_of(one.alignment);
        each(
            i,
            AllocationInfo1 {
                offset,
                alignment: one.alignment,
                size: one.size,
            },
        );
        total.size = offset + one.size;
        total.alignment = total.alignment.max(one.alignment);
    }
    total
}

/// Zeroed host memory standing in for GPU memory.
pub(crate) struct Memory {
    ptr: NonNull<u8>,
    layout: Layout,
    /// Set when the memory came from the title's allocator, with these
    /// allocation attributes (it goes back there).
    attributes: Option<u64>,
}

pub type AllocRoutine = extern "system" fn(usize, u64) -> *mut c_void;
pub type FreeRoutine = extern "system" fn(*mut c_void, u64);
static ALLOCATOR: std::sync::OnceLock<(AllocRoutine, FreeRoutine)> = std::sync::OnceLock::new();

/// Committed resources get their memory from the title's general
/// allocator, as on the console, where titles hook it to place graphics
/// memory themselves (Grand Theft Auto VI places its back buffers in a
/// block of its own and checks they fill it). The runtime names it before
/// the device exists.
pub fn use_allocator(alloc: AllocRoutine, free: FreeRoutine) {
    let _ = ALLOCATOR.set((alloc, free));
}

/// Maps the physical pages behind one range of title memory at another,
/// `(to, from, len)` (`from` 0 unmaps); false on failure.
pub type PageMapper = fn(u64, u64, u64) -> bool;
static PAGE_MAPPER: std::sync::OnceLock<PageMapper> = std::sync::OnceLock::new();

/// The runtime's page mapper, for the queues' page-mapping copies (memory
/// is the runtime's, as on the console).
pub fn use_page_mapper(mapper: PageMapper) {
    let _ = PAGE_MAPPER.set(mapper);
}

pub(crate) fn page_mapper() -> Option<PageMapper> {
    PAGE_MAPPER.get().copied()
}

/// Allocation attributes for resource memory: allocator id, page size
/// (bits 14-15: 2 MiB pages from 2 MiB up, else 64 KiB), alignment as log2
/// (bits 24-28) and memory type (bits 29-32: write-combined for GPU-only
/// heaps, cacheable for the CPU-visible ones).
/// shortcut: the console runtime's own choice of page size and memory type
/// is not known; this is the plausible one.
fn resource_attributes(info: AllocationInfo, heap_kind: u32) -> u64 {
    const GRAPHICS_ALLOCATOR: u64 = 128 << 16;
    const DEFAULT_HEAP: u32 = 1;
    let pages: u64 = if info.size >= 2 << 20 { 2 } else { 1 };
    let memory: u64 = if heap_kind == DEFAULT_HEAP { 1 } else { 3 };
    let alignment = u64::from(info.alignment.max(1).trailing_zeros());
    GRAPHICS_ALLOCATOR | pages << 14 | alignment << 24 | memory << 29
}

impl Memory {
    /// A committed resource's memory: from the title's allocator if the
    /// runtime named one, else our own; zeroed either way.
    fn for_resource(info: AllocationInfo, heap_kind: u32) -> Result<Memory, HRESULT> {
        let Some(&(alloc, _)) = ALLOCATOR.get() else {
            return Memory::new(info);
        };
        let layout = Layout::from_size_align(info.size as usize, info.alignment as usize)
            .map_err(|_| E_INVALIDARG)?;
        let attributes = resource_attributes(info, heap_kind);
        let ptr =
            NonNull::new(alloc(layout.size(), attributes).cast::<u8>()).ok_or(E_OUTOFMEMORY)?;
        // SAFETY: the allocator handed out `size` bytes.
        unsafe { ptr.as_ptr().write_bytes(0, layout.size()) };
        Ok(Memory {
            ptr,
            layout,
            attributes: Some(attributes),
        })
    }

    /// `size` zeroed bytes aligned to `alignment` (a power of two).
    pub(crate) fn zeroed(size: u64, alignment: u64) -> Result<Memory, HRESULT> {
        Memory::new(AllocationInfo { size, alignment })
    }

    pub(crate) fn address(&self) -> u64 {
        self.ptr.as_ptr() as u64
    }

    fn new(info: AllocationInfo) -> Result<Memory, HRESULT> {
        let layout = usize::try_from(info.size)
            .ok()
            .zip(usize::try_from(info.alignment).ok())
            .filter(|&(size, _)| size > 0)
            .and_then(|(size, align)| Layout::from_size_align(size, align).ok())
            .ok_or(E_INVALIDARG)?;
        // SAFETY: the size is non-zero.
        let ptr = NonNull::new(unsafe { alloc_zeroed(layout) }).ok_or(E_OUTOFMEMORY)?;
        Ok(Memory {
            ptr,
            layout,
            attributes: None,
        })
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        match (self.attributes, ALLOCATOR.get()) {
            (Some(attributes), Some(&(_, free))) => free(self.ptr.as_ptr().cast(), attributes),
            // SAFETY: allocated in `new` with this layout.
            _ => unsafe { dealloc(self.ptr.as_ptr(), self.layout) },
        }
    }
}

#[repr(C)]
struct Heap {
    object: Object,
    desc: HeapDesc,
    memory: Memory,
}

pub(crate) extern "system" fn create_heap(
    _: This,
    desc: *const HeapDesc,
    iid: *const Iid,
    out: *mut This,
) -> HRESULT {
    // SAFETY: the title passes a description (null is tolerated).
    let Some(&desc) = (unsafe { desc.as_ref() }) else {
        return E_INVALIDARG;
    };
    tracing::debug!(
        size = desc.size,
        kind = desc.properties.kind,
        flags = format_args!("{:#x}", desc.flags),
        "heap"
    );
    if desc.alignment != 0 && !desc.alignment.is_power_of_two() {
        return E_INVALIDARG;
    }
    let info = AllocationInfo {
        size: desc.size,
        alignment: desc.alignment.max(KIB_64),
    };
    let memory = match Memory::new(info) {
        Ok(memory) => memory,
        Err(hr) => return hr,
    };
    let heap = Box::new(Heap {
        object: Object::new(&HEAP_FUNCTIONS),
        desc,
        memory,
    });
    object::hand_out(HEAP, heap, iid, &HEAP_IIDS, out)
}

extern "system" fn heap_query(this: This, iid: *const Iid, out: *mut This) -> HRESULT {
    object::answer(HEAP, iid, object::is(iid, &HEAP_IIDS).then_some(this), out)
}
extern "system" fn heap_release(this: This) -> u32 {
    object::release_boxed::<Heap>(this)
}
extern "system" fn heap_desc(this: This, out: *mut HeapDesc) {
    if !out.is_null() {
        // SAFETY: `this` is a heap; the title passes room for a description.
        unsafe { out.write((*this.cast::<Heap>()).desc) };
    }
}

/// A resource's memory: its own, or a range of a heap it holds a reference
/// to.
enum Backing {
    Committed(Memory),
    Placed {
        heap: This,
        offset: u64,
    },
    /// Memory the title mapped itself, at this address.
    Address(*mut u8),
}

impl Drop for Backing {
    fn drop(&mut self) {
        if let Backing::Placed { heap, .. } = *self {
            heap_release(heap);
        }
    }
}

#[repr(C)]
struct Resource {
    object: Object,
    desc: ResourceDesc,
    heap: HeapProperties,
    heap_flags: u32,
    backing: Backing,
    /// Textures also live on the host GPU (decision 0007); buffers do not
    /// yet.
    #[cfg(windows)]
    host: Option<std::sync::Arc<HostTexture>>,
}

/// A title texture's host copy and the state it is in. The title's barriers
/// name console states (looser than the host's), so we transition from this
/// one. Lists that use the texture hold it until they are reset: the title
/// may release a resource whose commands it recorded but not yet submitted
/// (console memory outlives the object; a host texture must outlive the
/// lists that name it).
#[cfg(windows)]
pub(crate) struct HostTexture {
    pub(crate) texture: windows::Win32::Graphics::Direct3D12::ID3D12Resource,
    pub(crate) state: std::sync::atomic::AtomicU32,
}

/// Moves the host texture to `wanted` (a host texture state); the host
/// resource and the state it leaves, if that is a change.
#[cfg(windows)]
pub(crate) fn host_transition(this: This, wanted: u32) -> Option<(*mut c_void, u32)> {
    use windows::core::Interface;
    let host = resource(this).host.as_ref()?;
    let before = host
        .state
        .swap(wanted, std::sync::atomic::Ordering::Relaxed);
    (before != wanted).then(|| (host.texture.as_raw(), before))
}

/// The host GPU's copy of a title resource, if it has one.
#[cfg(windows)]
pub(crate) fn host_resource<'a>(
    this: This,
) -> Option<&'a windows::Win32::Graphics::Direct3D12::ID3D12Resource> {
    if this.is_null() {
        return None;
    }
    resource(this).host.as_deref().map(|h| &h.texture)
}

/// A title texture's host copy, shared, for a list to hold.
#[cfg(windows)]
pub(crate) fn host_copy(this: This) -> Option<std::sync::Arc<HostTexture>> {
    if this.is_null() {
        return None;
    }
    resource(this).host.clone()
}

#[cfg(windows)]
const PC_FLAGS: u32 = 0xf; // render target, depth, UAV, no shader access
#[cfg(windows)]
const WRITABLE_FLAGS: u32 = 0x7; // render target, depth, UAV
#[cfg(windows)]
const DEPTH_FLAG: u32 = 0x2;
#[cfg(windows)]
const UNORDERED_FLAG: u32 = 0x4;
#[cfg(windows)]
const DENY_SHADER_RESOURCE_FLAG: u32 = 0x8;

#[cfg(windows)]
fn host_flags(flags: u32) -> u32 {
    let mut f = flags & PC_FLAGS;
    // PC refuses depth targets that are also unordered-access (the console
    // allows both; Grand Theft Auto VI's shadow maps ask for it).
    // shortcut: the host copy loses UAV use; give it a second view-only
    // copy if draws write it as a UAV.
    if f & DEPTH_FLAG != 0 {
        f &= !UNORDERED_FLAG;
    }
    f
}

#[cfg(windows)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct TextureKey {
    dimension: u32,
    width: u64,
    height: u32,
    depth_or_array: u16,
    mips: u32,
    samples: u32,
    family: u32,
}

#[cfg(windows)]
struct LiveTexture {
    resource: usize,
    key: TextureKey,
    host_flags: u32,
    host_format: u32,
    host: std::sync::Weak<HostTexture>,
}

#[cfg(windows)]
static LIVE_TEXTURES: std::sync::Mutex<std::collections::BTreeMap<u64, Vec<LiveTexture>>> =
    std::sync::Mutex::new(std::collections::BTreeMap::new());

#[cfg(windows)]
fn backing_address(backing: &Backing) -> u64 {
    match *backing {
        Backing::Committed(ref memory) => memory.address(),
        // SAFETY: placement checked the range lies inside the heap.
        Backing::Placed { heap, offset } => unsafe {
            (*heap.cast::<Heap>())
                .memory
                .ptr
                .as_ptr()
                .add(offset as usize) as u64
        },
        Backing::Address(at) => at as u64,
    }
}

#[cfg(windows)]
fn texture_key(desc: &ResourceDesc) -> TextureKey {
    TextureKey {
        dimension: desc.dimension,
        width: desc.width,
        height: desc.height,
        depth_or_array: desc.depth_or_array_size,
        mips: mip_count(desc),
        samples: desc.sample_count.max(1),
        family: family(pc_format(desc.format)),
    }
}

#[cfg(windows)]
fn is_typeless(format: u32) -> bool {
    family(format) == format && family(format.wrapping_add(1)) == format
}

#[cfg(windows)]
fn find_live_texture(addr: u64, desc: &ResourceDesc) -> Option<std::sync::Arc<HostTexture>> {
    if addr == 0 || desc.dimension == BUFFER {
        return None;
    }
    let want_key = texture_key(desc);
    let want_flags = host_flags(desc.flags);
    let want_format = pc_format(desc.format);

    let mut map = LIVE_TEXTURES.lock().unwrap_or_else(|e| e.into_inner());
    let entries = map.get_mut(&addr)?;
    entries.retain(|e| e.host.strong_count() > 0);
    if entries.is_empty() {
        map.remove(&addr);
        return None;
    }
    let compatible = |entry: &LiveTexture| -> bool {
        if entry.key != want_key {
            return false;
        }
        if (entry.host_flags & (want_flags & WRITABLE_FLAGS)) != (want_flags & WRITABLE_FLAGS) {
            return false;
        }
        if want_flags & DENY_SHADER_RESOURCE_FLAG == 0
            && entry.host_flags & DENY_SHADER_RESOURCE_FLAG != 0
        {
            return false;
        }
        if want_flags & DENY_SHADER_RESOURCE_FLAG == 0
            && DEPTH_FORMATS
                .iter()
                .any(|&(typed, _)| typed == entry.host_format)
        {
            return false;
        }
        if !is_typeless(entry.host_format)
            && want_flags & WRITABLE_FLAGS != 0
            && entry.host_format != want_format
        {
            return false;
        }
        true
    };
    let mut fallback = None;
    for entry in entries.iter() {
        if !compatible(entry) {
            continue;
        }
        let Some(host) = entry.host.upgrade() else {
            continue;
        };
        if entry.host_flags & WRITABLE_FLAGS != 0 {
            return Some(host);
        }
        if fallback.is_none() {
            fallback = Some(host);
        }
    }
    fallback
}

#[cfg(windows)]
fn register_live_texture(addr: u64, entry: LiveTexture) {
    LIVE_TEXTURES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .entry(addr)
        .or_default()
        .push(entry);
}

#[cfg(windows)]
fn unregister_live_texture(addr: u64, resource: usize) {
    if addr == 0 {
        return;
    }
    let mut map = LIVE_TEXTURES.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(entries) = map.get_mut(&addr) {
        entries.retain(|e| e.resource != resource);
        if entries.is_empty() {
            map.remove(&addr);
        }
    }
}

/// A host texture like the title's: same description, in GPU memory,
/// without the console's layout and flags.
#[cfg(windows)]
fn host_texture(
    desc: &ResourceDesc,
    state: u32,
    clear: *const c_void,
) -> Option<windows::Win32::Graphics::Direct3D12::ID3D12Resource> {
    use windows::Win32::Graphics::Direct3D12::*;
    if desc.dimension == BUFFER {
        return None;
    }
    let host = crate::host::host()?;
    let mut d = *desc;
    d.layout = 0;
    d.alignment = 0;
    d.flags = host_flags(d.flags);
    d.format = pc_format(d.format);
    if d.flags & DENY_SHADER_RESOURCE_FLAG == 0
        && let Some(&(_, typeless)) = DEPTH_FORMATS.iter().find(|&&(f, _)| f == d.format)
    {
        d.format = typeless;
    }
    // The host has no sRGB unordered access: such textures are typeless,
    // their unordered access views plain UNORM.
    if d.flags & UNORDERED_FLAG != 0 && SRGB.iter().any(|&(srgb, _)| srgb == d.format) {
        d.format = family(d.format);
    }
    // The clear value names the format too: format, then color or depth
    // and stencil (5 words).
    let clear_value = (!clear.is_null()).then(|| {
        // SAFETY: the title passes a whole clear value.
        let mut value = unsafe { clear.cast::<[u32; 5]>().read_unaligned() };
        value[0] = pc_format(value[0]);
        value
    });
    let heap = D3D12_HEAP_PROPERTIES {
        Type: D3D12_HEAP_TYPE_DEFAULT,
        ..Default::default()
    };
    let mut texture: Option<ID3D12Resource> = None;
    // SAFETY: the description has the host layout (checked size, same
    // fields); the out pointer is ours.
    let made = unsafe {
        host.device.CreateCommittedResource(
            &heap,
            D3D12_HEAP_FLAG_NONE,
            (&raw const d).cast(),
            D3D12_RESOURCE_STATES(
                crate::host::legal_for(d.flags, crate::host::texture_state(state)) as i32,
            ),
            // The same layout: format, then color or depth and stencil.
            clear_value.as_ref().map(|v| std::ptr::from_ref(v).cast()),
            &mut texture,
        )
    };
    if let Err(e) = made {
        tracing::warn!(error = %e, desc = ?d, "no host texture");
    }
    if let Some(t) = &texture {
        crate::host::name(t, || {
            format!(
                "texture {}x{} f{} flags {:#x}",
                d.width, d.height, d.format, d.flags
            )
        });
    }
    texture
}

/// A texture's console memory and description; `None` for buffers.
pub(crate) fn texture_memory(this: This) -> Option<(*const u8, ResourceDesc)> {
    if this.is_null() {
        return None;
    }
    let r = resource(this);
    (r.desc.dimension != BUFFER).then(|| (r.data().cast_const(), r.desc))
}

/// A buffer's host memory and size; `None` for textures.
pub(crate) fn buffer_bytes(this: This) -> Option<(*mut u8, u64)> {
    if this.is_null() {
        return None;
    }
    let r = resource(this);
    (r.desc.dimension == BUFFER).then(|| (r.data(), r.desc.width))
}

impl Resource {
    fn data(&self) -> *mut u8 {
        match self.backing {
            Backing::Committed(ref memory) => memory.ptr.as_ptr(),
            // SAFETY: placement checked the range lies inside the heap.
            Backing::Placed { heap, offset } => unsafe {
                (*heap.cast::<Heap>())
                    .memory
                    .ptr
                    .as_ptr()
                    .add(offset as usize)
            },
            Backing::Address(at) => at,
        }
    }
}

fn resource<'a>(this: This) -> &'a Resource {
    // SAFETY: called from resource methods only.
    unsafe { &*this.cast::<Resource>() }
}

// The creation methods' own arguments, passed on.
#[allow(clippy::too_many_arguments)]
fn hand_out_resource(
    desc: ResourceDesc,
    heap: HeapProperties,
    heap_flags: u32,
    backing: Backing,
    state: u32,
    clear: *const c_void,
    iid: *const Iid,
    out: *mut This,
) -> HRESULT {
    #[cfg(not(windows))]
    let _ = (state, clear);
    #[cfg(windows)]
    let addr = if desc.dimension != BUFFER {
        backing_address(&backing)
    } else {
        0
    };
    #[cfg(windows)]
    let host = if desc.dimension != BUFFER {
        find_live_texture(addr, &desc).or_else(|| {
            host_texture(&desc, state, clear).map(|texture| {
                let state = crate::host::legal_state(&texture, crate::host::texture_state(state));
                std::sync::Arc::new(HostTexture {
                    texture,
                    state: state.into(),
                })
            })
        })
    } else {
        None
    };
    let resource = Box::new(Resource {
        object: Object::new(&RESOURCE_FUNCTIONS),
        desc,
        heap,
        heap_flags,
        backing,
        #[cfg(windows)]
        host,
    });
    #[cfg(windows)]
    if let Some(host) = &resource.host {
        add_addressed(
            std::ptr::from_ref::<Resource>(&resource) as usize,
            resource.data() as u64,
            desc,
            host,
        );
    }
    #[cfg(windows)]
    if desc.dimension == BUFFER {
        let at = std::ptr::from_ref::<Resource>(&resource) as usize;
        crate::shadow::register(at, resource.data() as u64, desc.width);
    } else if addr != 0
        && let Some(ref h) = resource.host
    {
        let ptr = std::ptr::from_ref::<Resource>(&resource) as usize;
        // SAFETY: a live host texture.
        let hd = unsafe { h.texture.GetDesc() };
        register_live_texture(
            addr,
            LiveTexture {
                resource: ptr,
                key: texture_key(&desc),
                host_flags: hd.Flags.0 as u32,
                host_format: hd.Format.0 as u32,
                host: std::sync::Arc::downgrade(h),
            },
        );
    }
    object::hand_out(RESOURCE, resource, iid, &RESOURCE_IIDS, out)
}

#[cfg(windows)]
impl Drop for Resource {
    fn drop(&mut self) {
        #[cfg(feature = "diagnostics")]
        if self
            .host
            .as_ref()
            .is_some_and(|h| std::sync::Arc::strong_count(h) > 1)
        {
            // Before lists held their textures, this freed a texture that
            // lists not yet run still named (a GPU page fault).
            static OUTLIVED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let n = OUTLIVED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if n.is_power_of_two() {
                tracing::info!(
                    n,
                    desc = ?self.desc,
                    "a title texture was released while a list not yet reset names its host copy"
                );
            }
        }
        crate::texture::forget(std::ptr::from_ref::<Resource>(self) as usize);
        crate::descriptor::forget(std::ptr::from_ref::<Resource>(self) as usize);
        forget_addressed(std::ptr::from_ref::<Resource>(self) as usize);
        if self.desc.dimension == BUFFER {
            crate::shadow::unregister(
                std::ptr::from_ref::<Resource>(self) as usize,
                self.data() as u64,
            );
        } else {
            unregister_live_texture(
                self.data() as u64,
                std::ptr::from_ref::<Resource>(self) as usize,
            );
        }
    }
}

pub(crate) extern "system" fn create_committed(
    _: This,
    heap: *const HeapProperties,
    heap_flags: u32,
    desc: *const ResourceDesc,
    state: u32,
    clear: *const c_void,
    iid: *const Iid,
    out: *mut This,
) -> HRESULT {
    // SAFETY: the title passes both descriptions (null is tolerated).
    let (Some(&heap), Some(&desc)) = (unsafe { (heap.as_ref(), desc.as_ref()) }) else {
        return E_INVALIDARG;
    };
    let Some(info) = footprint(&desc) else {
        tracing::warn!(?desc, "no size for this resource");
        return E_INVALIDARG;
    };
    tracing::debug!(
        dimension = desc.dimension,
        size = info.size,
        heap = heap.kind,
        "committed resource"
    );
    let memory = match Memory::for_resource(info, heap.kind) {
        Ok(memory) => memory,
        Err(hr) => return hr,
    };
    hand_out_resource(
        desc,
        heap,
        heap_flags,
        Backing::Committed(memory),
        state,
        clear,
        iid,
        out,
    )
}

/// Device method 0x3f8, inferred from Grand Theft Auto VI's calls
/// (title+0x193e1b4: its back buffers, heap flag "allow display"): the
/// committed resource creation of the enhanced-barrier interfaces,
/// `(heap, heap flags, desc1, initial layout, clear value, protected
/// session, castable format count, castable formats, iid, out)`. The host
/// copy starts in the common state, as layouts are not tracked.
#[allow(clippy::too_many_arguments)] // the method's own arguments
pub(crate) extern "system" fn create_committed3(
    this: This,
    heap: *const HeapProperties,
    heap_flags: u32,
    desc: *const ResourceDesc,
    _layout: u32,
    clear: *const c_void,
    _protected: This,
    _castable_count: u32,
    _castable: *const u32,
    iid: *const Iid,
    out: *mut This,
) -> HRESULT {
    const COMMON: u32 = 0;
    create_committed(this, heap, heap_flags, desc, COMMON, clear, iid, out)
}

/// Registered page pools (64 KiB pages the title maps into tiled
/// resources by page index): first address and page count; a handle is an
/// index plus one, a removed pool leaves `None`.
static PAGE_POOLS: std::sync::Mutex<Vec<Option<(u64, u32)>>> = std::sync::Mutex::new(Vec::new());

fn page_pools() -> std::sync::MutexGuard<'static, Vec<Option<(u64, u32)>>> {
    PAGE_POOLS.lock().unwrap_or_else(|e| e.into_inner())
}

/// Device method 0x250: `(address, page count, handle out)`.
pub(crate) extern "system" fn register_page_pool(
    _: This,
    address: u64,
    pages: u32,
    out: *mut usize,
) -> HRESULT {
    if out.is_null() || pages == 0 || !address.is_multiple_of(KIB_64) {
        return E_INVALIDARG;
    }
    tracing::debug!(address = format_args!("{address:#x}"), pages, "page pool");
    let mut pools = page_pools();
    pools.push(Some((address, pages)));
    // SAFETY: checked non-null; the title passes room for a handle.
    unsafe { out.write(pools.len()) };
    S_OK
}

/// Device method 0x258.
pub(crate) extern "system" fn unregister_page_pool(_: This, handle: usize) -> HRESULT {
    match page_pools().get_mut(handle.wrapping_sub(1)) {
        Some(pool @ Some(_)) => {
            *pool = None;
            S_OK
        }
        _ => E_INVALIDARG,
    }
}

/// Device method 0x428, one of the `ID3D12Device9` to `ID3D12Device11`
/// additions. From Grand Theft Auto VI's call: a resource at a virtual
/// address the title mapped itself, `(address, desc1, initial barrier
/// layout, clear value, iid, out)`, as `CreatePlacedResourceX` with
/// enhanced barriers. GPU addresses are CPU addresses, so the resource is
/// a view of that memory. `D3D12_RESOURCE_DESC1` starts like the plain
/// description; its sampler feedback region is not needed.
pub(crate) extern "system" fn create_at_address(
    _: This,
    address: u64,
    desc: *const ResourceDesc,
    layout: u32,
    clear: *const c_void,
    iid: *const Iid,
    out: *mut This,
) -> HRESULT {
    // SAFETY: the title passes a description (null is tolerated).
    let Some(&desc) = (unsafe { desc.as_ref() }) else {
        return E_INVALIDARG;
    };
    if address == 0 {
        return E_INVALIDARG;
    }
    tracing::debug!(
        address = format_args!("{address:#x}"),
        dimension = desc.dimension,
        width = desc.width,
        layout,
        "resource at an address"
    );
    const COMMON: u32 = 0;
    const DEFAULT: HeapProperties = HeapProperties {
        kind: 1,
        cpu_pages: 0,
        pool: 0,
        creation_nodes: 1,
        visible_nodes: 1,
    };
    hand_out_resource(
        desc,
        DEFAULT,
        0,
        Backing::Address(address as *mut u8),
        COMMON,
        clear,
        iid,
        out,
    )
}

/// Textures the title placed in its own memory and views without a
/// resource object (device 0x230, Grand Theft Auto VI's streamed and
/// interface textures), by address and description: a resource at that
/// address, made the first time a draw reads one.
/// shortcut: never released (one host texture per distinct address and
/// description viewed); release them when the title unmaps or reuses the
/// memory once streaming grows VRAM use.
#[cfg(windows)]
static PLACED_TEXTURES: std::sync::Mutex<std::collections::BTreeMap<(u64, [u32; 14]), usize>> =
    std::sync::Mutex::new(std::collections::BTreeMap::new());

/// The resource for a texture placed at `address` with the resource
/// description `desc` (as words), made on first use.
#[cfg(windows)]
pub(crate) fn texture_at(address: u64, desc: &[u32; 14]) -> Option<This> {
    let desc = canonical_resource_desc(*desc);
    // SAFETY: 56 bytes in the description's layout.
    let d = unsafe { desc.as_ptr().cast::<ResourceDesc>().read_unaligned() };
    if d.dimension == BUFFER {
        return None;
    }
    let mut map = PLACED_TEXTURES.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(&cached_r) = map.get(&(address, desc)) {
        let cached_host = resource(cached_r as This).host.clone();
        // SAFETY: a live host texture.
        let cached_writable = cached_host
            .as_ref()
            .is_some_and(|h| unsafe { h.texture.GetDesc() }.Flags.0 as u32 & WRITABLE_FLAGS != 0);
        if !cached_writable
            && let Some(live) = find_live_texture(address, &d)
            // SAFETY: a live host texture.
            && unsafe { live.texture.GetDesc() }.Flags.0 as u32 & WRITABLE_FLAGS != 0
            && !cached_host
                .as_ref()
                .is_some_and(|c| std::sync::Arc::ptr_eq(c, &live))
        {
            let mut out = std::ptr::null();
            let made = create_at_address(
                std::ptr::null(),
                address,
                &d,
                0,
                std::ptr::null(),
                &RESOURCE_IIDS[0],
                &mut out,
            );
            if made == S_OK {
                if resource(out).host.is_some() {
                    // Do not free `cached_r`: earlier callers may still hold it.
                    map.insert((address, desc), out as usize);
                    return Some(out);
                }
                resource_release(out);
            }
        }
        return cached_host.is_some().then_some(cached_r as This);
    }
    let mut out = std::ptr::null();
    let made = create_at_address(
        std::ptr::null(),
        address,
        &d,
        0,
        std::ptr::null(),
        &RESOURCE_IIDS[0],
        &mut out,
    );
    if made != S_OK {
        return None;
    }
    map.insert((address, desc), out as usize);
    resource(out).host.is_some().then_some(out)
}

/// The host copies of title textures, by the title memory they are in: where
/// a placed copy of that memory takes its contents from, when another
/// resource writes it (see `crate::texture::sync_placed`).
#[cfg(windows)]
struct Addressed {
    /// The resource object, so its release drops the entry.
    object: usize,
    desc: ResourceDesc,
    /// The host copy, held: a lookup never names a freed texture.
    host: std::sync::Arc<HostTexture>,
}

#[cfg(windows)]
static ADDRESSED: std::sync::Mutex<std::collections::BTreeMap<u64, Vec<Addressed>>> =
    std::sync::Mutex::new(std::collections::BTreeMap::new());

/// Notes that `host` is the host copy of the texture `object` at `address`.
#[cfg(windows)]
fn add_addressed(
    object: usize,
    address: u64,
    desc: ResourceDesc,
    host: &std::sync::Arc<HostTexture>,
) {
    if desc.dimension == BUFFER {
        return;
    }
    ADDRESSED
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .entry(address)
        .or_default()
        .push(Addressed {
            object,
            desc,
            host: host.clone(),
        });
}

/// The title released the resource `object`: its host copy is no writer.
#[cfg(windows)]
fn forget_addressed(object: usize) {
    let mut map = ADDRESSED.lock().unwrap_or_else(|e| e.into_inner());
    map.retain(|_, entries| {
        entries.retain(|e| e.object != object);
        !entries.is_empty()
    });
}

/// The resource whose host copy writes the title memory that `placed` is a
/// copy of: the latest one at `address` of the same size, texel width and
/// format family, with unordered access (its shaders write it). The two
/// formats differ (the reader's is the console's packed one), so the copy
/// has to go through a buffer, one format at each end: it needs the same
/// bytes per texel, which a format of the same family differs from.
#[cfg(windows)]
pub(crate) fn writer_at(
    address: u64,
    placed: This,
    placed_copy: &windows::Win32::Graphics::Direct3D12::ID3D12Resource,
) -> Option<std::sync::Arc<HostTexture>> {
    use windows::Win32::Graphics::Direct3D12::D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
    use windows::core::Interface;
    // SAFETY: a live resource.
    let copy = unsafe { placed_copy.GetDesc() };
    let (bytes, block) = element(copy.Format.0 as u32)?;
    // A placed copy of a buffer, or of a multisampled or non-plain-layout
    // texture, is not something a copy between textures reaches.
    if copy.Dimension.0 == BUFFER as i32
        || copy.SampleDesc.Count != 1
        || copy.Layout != windows::Win32::Graphics::Direct3D12::D3D12_TEXTURE_LAYOUT_UNKNOWN
    {
        return None;
    }
    let map = ADDRESSED.lock().unwrap_or_else(|e| e.into_inner());
    for entry in map.get(&address)?.iter().rev() {
        if entry.object == placed as usize || entry.desc.dimension == BUFFER {
            continue;
        }
        // SAFETY: live host textures (each entry holds one).
        if entry.host.texture.as_raw() == placed_copy.as_raw() {
            continue;
        }
        // SAFETY: a live host texture (the entry holds one).
        let d = unsafe { entry.host.texture.GetDesc() };
        if d.Dimension != copy.Dimension
            || d.Width != copy.Width
            || d.Height != copy.Height
            || d.DepthOrArraySize != copy.DepthOrArraySize
            || d.MipLevels != copy.MipLevels
            || d.SampleDesc.Count != copy.SampleDesc.Count
            || d.Layout != copy.Layout
            || d.Format == copy.Format
            || d.Flags.0 & D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS.0 == 0
        {
            continue;
        }
        if element(d.Format.0 as u32) != Some((bytes, block)) {
            continue;
        }
        tracing::debug!(
            address = format_args!("{address:#x}"),
            writer = ?entry.desc,
            placed = ?copy,
            "a placed copy takes its contents from the resource that writes its memory"
        );
        return Some(entry.host.clone());
    }
    None
}

pub(crate) extern "system" fn create_placed(
    _: This,
    heap: This,
    offset: u64,
    desc: *const ResourceDesc,
    state: u32,
    clear: *const c_void,
    iid: *const Iid,
    out: *mut This,
) -> HRESULT {
    // SAFETY: the title passes a description (null is tolerated).
    let Some(&desc) = (unsafe { desc.as_ref() }) else {
        return E_INVALIDARG;
    };
    if heap.is_null() {
        return E_INVALIDARG;
    }
    // SAFETY: the title passes one of our heaps.
    let h = unsafe { &*heap.cast::<Heap>() };
    let fits = footprint(&desc).is_some_and(|info| {
        offset.is_multiple_of(info.alignment)
            && offset
                .checked_add(info.size)
                .is_some_and(|end| end <= h.desc.size)
    });
    if !fits {
        tracing::warn!(
            ?desc,
            offset,
            heap = h.desc.size,
            "placed resource does not fit"
        );
        return E_INVALIDARG;
    }
    object::add_ref(heap);
    let backing = Backing::Placed { heap, offset };
    hand_out_resource(
        desc,
        h.desc.properties,
        h.desc.flags,
        backing,
        state,
        clear,
        iid,
        out,
    )
}

extern "system" fn resource_query(this: This, iid: *const Iid, out: *mut This) -> HRESULT {
    object::answer(
        RESOURCE,
        iid,
        object::is(iid, &RESOURCE_IIDS).then_some(this),
        out,
    )
}
extern "system" fn resource_release(this: This) -> u32 {
    object::release_boxed::<Resource>(this)
}
/// Memory is always mapped (subresources are not told apart yet); a null
/// `out` only asks for residency.
extern "system" fn map(this: This, _sub: u32, _read: *const c_void, out: *mut *mut u8) -> HRESULT {
    if !out.is_null() {
        // SAFETY: checked non-null; the title passes room for a pointer.
        unsafe { out.write(resource(this).data()) };
    }
    S_OK
}
extern "system" fn unmap(_: This, _sub: u32, _written: *const c_void) {}
extern "system" fn resource_desc(this: This, out: *mut ResourceDesc) {
    if !out.is_null() {
        // SAFETY: checked non-null; the title passes room for a description.
        unsafe { out.write(resource(this).desc) };
    }
}
/// Device method `GetCustomHeapProperties`, returning its struct through a
/// hidden first argument: what a standard heap type is as a custom heap.
/// Memory is unified and cache-coherent (as the architecture query says),
/// so every type is in pool L0, default heaps are not CPU-visible and the
/// others are write-back.
pub(crate) extern "system" fn custom_heap_properties(
    out: *mut HeapProperties,
    _: This,
    nodes: u32,
    kind: u32,
) -> *mut HeapProperties {
    const CUSTOM: u32 = 4;
    const NOT_AVAILABLE: u32 = 1;
    const WRITE_BACK: u32 = 3;
    const L0: u32 = 1;
    let nodes = nodes.max(1);
    let cpu_pages = match kind {
        1 => NOT_AVAILABLE,
        2 | 3 => WRITE_BACK,
        _ => 0,
    };
    let properties = HeapProperties {
        kind: CUSTOM,
        cpu_pages,
        pool: if cpu_pages == 0 { 0 } else { L0 },
        creation_nodes: nodes,
        visible_nodes: nodes,
    };
    // SAFETY: the title passes room for the result (null is tolerated).
    if let Some(out) = unsafe { out.as_mut() } {
        *out = properties;
    }
    out
}

extern "system" fn gpu_address(this: This) -> u64 {
    let r = resource(this);
    let at = r.data() as u64;
    // The console's GPU addresses fit in 40 bits; its ray-tracing builder
    // refuses others (diagnostics: which memory breaks that).
    #[cfg(feature = "diagnostics")]
    if at >> 40 != 0 {
        static ONCE: [std::sync::Once; 2] = [std::sync::Once::new(), std::sync::Once::new()];
        let backing = match r.backing {
            Backing::Committed(ref m) => format!("committed, attributes {:?}", m.attributes),
            Backing::Placed { offset, .. } => format!("placed in a heap at {offset:#x}"),
            Backing::Address(_) => "at a title address".into(),
        };
        ONCE[usize::from(r.desc.dimension == BUFFER)].call_once(|| {
            tracing::warn!(address = format_args!("{at:#x}"), backing, desc = ?r.desc, "a GPU address above 40 bits")
        });
    }
    at
}
extern "system" fn heap_properties(
    this: This,
    properties: *mut HeapProperties,
    flags: *mut u32,
) -> HRESULT {
    let r = resource(this);
    // SAFETY: either pointer may be null; otherwise the title passes room.
    unsafe {
        if let Some(p) = properties.as_mut() {
            *p = r.heap;
        }
        if let Some(f) = flags.as_mut() {
            *f = r.heap_flags;
        }
    }
    S_OK
}

/// IUnknown 3, object 4, device child 1, heap 1.
static HEAP_FUNCTIONS: Table<24> = {
    let mut f = object::d3d12_object(object::missing_table::<HEAP, 24>());
    f[0] = heap_query as *const ();
    f[2] = heap_release as *const ();
    f[7] = object::parent_device as *const (); // 0x38
    f[8] = heap_desc as *const (); // 0x40
    Table(f)
};

/// IUnknown 3, object 4, device child 1, resource 10 (7 as on PC, then 3
/// console ones).
static RESOURCE_FUNCTIONS: Table<32> = {
    let mut f = object::d3d12_object(object::missing_table::<RESOURCE, 32>());
    f[0] = resource_query as *const ();
    f[2] = resource_release as *const ();
    f[7] = object::parent_device as *const (); // 0x38
    f[8] = map as *const (); // 0x40
    f[9] = unmap as *const (); // 0x48
    f[10] = resource_desc as *const (); // 0x50
    f[11] = gpu_address as *const (); // 0x58
    f[14] = heap_properties as *const (); // 0x70
    Table(f)
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::slot;

    type Release = extern "system" fn(This) -> u32;
    type Map = extern "system" fn(This, u32, *const c_void, *mut *mut u8) -> HRESULT;
    type Address = extern "system" fn(This) -> u64;

    const PROPERTIES: HeapProperties = HeapProperties {
        kind: 1,
        cpu_pages: 0,
        pool: 0,
        creation_nodes: 0,
        visible_nodes: 0,
    };

    fn buffer(width: u64) -> ResourceDesc {
        ResourceDesc {
            dimension: BUFFER,
            alignment: 0,
            width,
            height: 1,
            depth_or_array_size: 1,
            mip_levels: 1,
            format: 0,
            sample_count: 1,
            sample_quality: 0,
            layout: 1,
            flags: 0,
        }
    }

    fn texture(width: u64, height: u32, format: u32, mips: u16) -> ResourceDesc {
        ResourceDesc {
            dimension: 3,
            width,
            height,
            format,
            mip_levels: mips,
            layout: 0,
            ..buffer(0)
        }
    }

    fn heap(size: u64) -> This {
        let desc = HeapDesc {
            size,
            properties: PROPERTIES,
            alignment: 0,
            flags: 0,
        };
        let mut heap = std::ptr::null();
        assert_eq!(
            create_heap(std::ptr::null(), &desc, &HEAP_IIDS[0], &mut heap),
            S_OK
        );
        heap
    }

    #[test]
    fn heaps_are_zeroed_aligned_memory() {
        let heap = heap(1 << 20);
        // SAFETY: a heap we just made.
        let memory = unsafe { &(*heap.cast::<Heap>()).memory };
        assert_eq!(memory.ptr.as_ptr() as u64 % KIB_64, 0);
        let mut back = std::mem::MaybeUninit::<HeapDesc>::uninit();
        slot::<extern "system" fn(This, *mut HeapDesc)>(heap, 0x40)(heap, back.as_mut_ptr());
        // SAFETY: written above.
        assert_eq!(unsafe { back.assume_init() }.size, 1 << 20);
        assert_eq!(slot::<Release>(heap, 0x10)(heap), 0);

        let bad = HeapDesc {
            size: 1 << 20,
            properties: PROPERTIES,
            alignment: 3,
            flags: 0,
        };
        let mut out = std::ptr::null();
        assert_eq!(
            create_heap(std::ptr::null(), &bad, &HEAP_IIDS[0], &mut out),
            E_INVALIDARG
        );
    }

    #[test]
    fn device_setup_heap_properties_and_resources_at_addresses() {
        let device = crate::device::device();
        // GetCustomHeapProperties (0xd0): the struct comes back through the
        // hidden first argument, which is also returned.
        type Custom =
            extern "system" fn(*mut HeapProperties, This, u32, u32) -> *mut HeapProperties;
        let mut p = PROPERTIES;
        let back = slot::<Custom>(device, 0xd0)(&mut p, device, 1, 2);
        assert_eq!(back, &raw mut p);
        assert_eq!(
            (
                p.kind,
                p.cpu_pages,
                p.pool,
                p.creation_nodes,
                p.visible_nodes
            ),
            (4, 3, 1, 1, 1),
            "upload: custom, write-back, L0"
        );
        slot::<Custom>(device, 0xd0)(&mut p, device, 1, 1);
        assert_eq!(p.cpu_pages, 1, "default: not CPU-visible");

        // 0x428: a buffer over memory the title owns, as GTA VI asks.
        type At = extern "system" fn(
            This,
            u64,
            *const ResourceDesc,
            u32,
            *const c_void,
            *const Iid,
            *mut This,
        ) -> HRESULT;
        let memory = vec![0u8; 0x1_0000];
        let address = memory.as_ptr() as u64;
        let mut r = std::ptr::null();
        let create = slot::<At>(device, 0x428);
        let desc = buffer(0x1_0000);
        let null = std::ptr::null();
        assert_eq!(
            create(
                device,
                address,
                &desc,
                u32::MAX,
                null,
                &RESOURCE_IIDS[0],
                &mut r
            ),
            0
        );
        assert_eq!(slot::<Address>(r, 0x58)(r), address);
        assert_eq!(slot::<Release>(r, 0x10)(r), 0);
        // The title's memory is still there after the release.
        assert_eq!(memory[0xffff], 0);
        assert_ne!(
            create(device, 0, &desc, u32::MAX, null, &RESOURCE_IIDS[0], &mut r),
            0
        );
    }

    #[test]
    fn sizes_cover_every_mip() {
        let size = |d: &ResourceDesc| footprint(d).map(|i| i.size);
        assert_eq!(size(&buffer(256)), Some(KIB_64));
        // Textures take their console layout and its alignment (4 KiB
        // standard swizzle when the description leaves it to the library):
        // 512x512 BC7 with 10 mips is 0x56000 bytes, 4 KiB aligned, as on
        // the console.
        let layout = |d: &ResourceDesc| texture_layout(d).unwrap().size;
        let rgba = texture(256, 256, 28, 0);
        assert_eq!(size(&rgba), Some(layout(&rgba).next_multiple_of(4096)));
        let bc7 = footprint(&texture(512, 512, 98, 10));
        let console = AllocationInfo {
            size: 0x56000,
            alignment: 0x1000,
        };
        assert_eq!(bc7, Some(console));
        assert!(layout(&rgba) >= 4 * (256 * 256 * 4) / 3, "every mip");
        // BC1 is 8 bytes per 4x4 block: 1024x1024, one mip = 512 KiB.
        assert_eq!(size(&texture(1024, 1024, 71, 1)), Some(512 << 10));
        assert_eq!(size(&texture(16, 16, 66, 1)), None, "1-bit format");
        // Linear layout (1) and an explicit mode (64 KiB display, 0x10a).
        let linear = ResourceDesc {
            layout: 1,
            ..texture(100, 60, 28, 1)
        };
        assert_eq!(
            texture_layout(&linear).unwrap().planes[0]
                .layout
                .desc
                .swizzle
                .0,
            0
        );
        let display = ResourceDesc {
            layout: 0x10a,
            ..texture(100, 60, 28, 1)
        };
        assert_eq!(
            texture_layout(&display).unwrap().planes[0]
                .layout
                .desc
                .swizzle
                .0,
            10
        );

        let mut info = AllocationInfo {
            size: 0,
            alignment: 0,
        };
        let descs = [buffer(100), texture(256, 256, 28, 0)];
        allocation_info(&mut info, std::ptr::null(), 0, 2, descs.as_ptr());
        assert_eq!(
            info,
            AllocationInfo {
                size: KIB_64 + size(&descs[1]).unwrap(),
                alignment: KIB_64
            }
        );
        allocation_info(&mut info, std::ptr::null(), 0, 1, &texture(1, 1, 999, 1));
        assert_eq!(info.size, u64::MAX);
    }

    #[test]
    fn upload_footprints_follow_the_pc_rules() {
        let null = std::ptr::null();
        let mut layouts = [NO_FOOTPRINT; 2];
        let (mut rows, mut sizes, mut total) = ([0; 2], [0; 2], 0);
        let mut footprints = |desc: &ResourceDesc, first, count| {
            let (l, r, s) = (layouts.as_mut_ptr(), rows.as_mut_ptr(), sizes.as_mut_ptr());
            copyable_footprints(null, desc, first, count, 0, l, r, s, &mut total);
            (layouts, rows, sizes, total)
        };
        // 100x60 RGBA8, 2 mips: 400-byte rows pitched to 512.
        let (l, r, s, t) = footprints(&texture(100, 60, 28, 2), 0, 2);
        assert_eq!((l[0].row_pitch, r[0], s[0]), (512, 60, 400));
        // Mip 1 (50x30, 200-byte rows) starts after 59 pitched rows and one
        // packed row, rounded up to 512.
        assert_eq!(l[1].offset, (512 * 59 + 400u64).next_multiple_of(512));
        assert_eq!((l[1].width, l[1].height, r[1]), (50, 30, 30));
        assert_eq!(t, l[1].offset + 256 * 29 + 200);
        // BC1: rows of 4x4 blocks, 8 bytes each.
        let (_, r, s, _) = footprints(&texture(64, 64, 71, 1), 0, 1);
        assert_eq!((r[0], s[0]), (16, 128));
        let (_, _, _, t) = footprints(&texture(64, 64, 71, 1), 1, 1);
        assert_eq!(t, u64::MAX, "no second subresource");
    }

    /// A 4x4 texture filled from a title buffer through our list and queue,
    /// then read back from the host GPU.
    #[cfg(windows)]
    #[test]
    fn buffer_data_reaches_the_host_texture() {
        use windows::Win32::Graphics::Direct3D12::*;
        let Some(host) = crate::host::host() else {
            return; // no host GPU here
        };
        let null = std::ptr::null();
        let iid = &RESOURCE_IIDS[0];
        const COPY_DEST: u32 = 0x400;
        let (mut image, mut source) = (null, null);
        let rgba = texture(4, 4, 28, 1);
        assert_eq!(
            create_committed(
                null,
                &PROPERTIES,
                0,
                &rgba,
                COPY_DEST,
                null.cast(),
                iid,
                &mut image
            ),
            S_OK
        );
        assert_eq!(
            create_committed(
                null,
                &PROPERTIES,
                0,
                &buffer(1024),
                0,
                null.cast(),
                iid,
                &mut source
            ),
            S_OK
        );
        let (data, _) = buffer_bytes(source).unwrap();
        // SAFETY: the buffer holds 1024 bytes.
        let bytes = unsafe { std::slice::from_raw_parts_mut(data, 1024) };
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (i % 251) as u8;
        }

        let queue_iid = guid(
            0x0ec870a6,
            0x5d7e,
            0x4c22,
            [0x8c, 0xfc, 0x5b, 0xaa, 0xe0, 0x76, 0x16, 0xed],
        );
        let allocator_iid = guid(
            0x6102dee4,
            0xaf59,
            0x4b09,
            [0xb9, 0x99, 0xb4, 0x4d, 0x73, 0xf0, 0x9b, 0x24],
        );
        let list_iid = guid(
            0x5b160d0f,
            0xac1b,
            0x4185,
            [0x8b, 0xa8, 0xb3, 0xae, 0x42, 0xa5, 0xa4, 0x55],
        );
        let (mut queue, mut allocator, mut list) = (null, null, null);
        let queue_desc = [0u32; 4];
        assert_eq!(
            crate::queue::create(null, queue_desc.as_ptr().cast(), &queue_iid, &mut queue),
            S_OK
        );
        assert_eq!(
            crate::list::create_allocator(null, 0, &allocator_iid, &mut allocator),
            S_OK
        );
        assert_eq!(
            crate::list::create_list(null, 0, 0, allocator, null, &list_iid, &mut list),
            S_OK
        );

        #[repr(C)]
        struct Location {
            resource: This,
            kind: u32,
            footprint: PlacedFootprint,
        }
        let rows = PlacedFootprint {
            offset: 0,
            format: 28,
            width: 4,
            height: 4,
            depth: 1,
            row_pitch: 256,
        };
        let to = Location {
            resource: image,
            kind: 0,
            footprint: PlacedFootprint { offset: 0, ..rows },
        };
        let from = Location {
            resource: source,
            kind: 1,
            footprint: rows,
        };
        type Copy =
            extern "system" fn(This, *const Location, u32, u32, u32, *const Location, *const u8);
        slot::<Copy>(list, 0x80)(list, &to, 0, 0, 0, &from, null.cast());
        slot::<extern "system" fn(This) -> HRESULT>(list, 0x48)(list);
        slot::<extern "system" fn(This, u32, *const This)>(queue, 0x50)(queue, 1, &list);

        // Read the host texture back into a readback buffer.
        let readback_heap = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_READBACK,
            ..Default::default()
        };
        let readback_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Width: 1024,
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
        let mut readback: Option<ID3D12Resource> = None;
        // SAFETY: plain creation of a readback buffer.
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
        let host_image = host_resource(image).unwrap();
        let back = host.new_list().unwrap();
        let (_, before) = host_transition(image, 0x800).unwrap();
        // The upload list holds the texture until it is reset.
        assert!(std::sync::Arc::strong_count(&host_copy(image).unwrap()) > 2);
        use windows::core::Interface;
        let out_to = Location {
            resource: readback.as_raw().cast_const().cast(),
            kind: 1,
            footprint: rows,
        };
        let out_from = Location {
            resource: host_image.as_raw().cast_const().cast(),
            kind: 0,
            footprint: to.footprint,
        };
        // SAFETY: an open list, live resources, locations in the host layout.
        unsafe {
            back.list.ResourceBarrier(&[crate::host::transition(
                host_image,
                D3D12_RESOURCE_STATES(before as i32),
                D3D12_RESOURCE_STATE_COPY_SOURCE,
            )]);
            back.list.CopyTextureRegion(
                (&raw const out_to).cast(),
                0,
                0,
                0,
                (&raw const out_from).cast(),
                None,
            );
        }
        back.close();
        host.execute(&[&back]);
        let mut mapped = std::ptr::null_mut();
        // SAFETY: the copy finished (execute waits); the buffer is 1024 bytes.
        let read = unsafe {
            readback
                .Map(
                    0,
                    Some(&D3D12_RANGE {
                        Begin: 0,
                        End: 1024,
                    }),
                    Some(&mut mapped),
                )
                .unwrap();
            std::slice::from_raw_parts(mapped.cast::<u8>(), 1024)
        };
        for row in 0..4 {
            let at = row * 256;
            assert_eq!(read[at..at + 16], bytes[at..at + 16], "row {row}");
        }
    }

    #[cfg(windows)]
    #[test]
    fn copy_texture_region_reads_back_into_title_buffer() {
        if crate::host::host().is_none() {
            return;
        }
        let null = std::ptr::null();
        let iid = &RESOURCE_IIDS[0];
        let tex_desc = ResourceDesc {
            dimension: 3, // Texture2D
            alignment: 0,
            width: 4,
            height: 4,
            depth_or_array_size: 1,
            mip_levels: 1,
            format: 28, // R8G8B8A8_UNORM
            sample_count: 1,
            sample_quality: 0,
            layout: 0,
            flags: 0,
        };
        let (mut image, mut upload_buf, mut dst_aligned, mut dst_packed, mut dst_crop) =
            (null, null, null, null, null);
        assert_eq!(
            create_committed(
                null,
                &PROPERTIES,
                0,
                &tex_desc,
                0,
                null.cast(),
                iid,
                &mut image
            ),
            S_OK
        );
        assert_eq!(
            create_committed(
                null,
                &PROPERTIES,
                0,
                &buffer(1024),
                0,
                null.cast(),
                iid,
                &mut upload_buf
            ),
            S_OK
        );
        assert_eq!(
            create_committed(
                null,
                &PROPERTIES,
                0,
                &buffer(2048),
                0,
                null.cast(),
                iid,
                &mut dst_aligned
            ),
            S_OK
        );
        assert_eq!(
            create_committed(
                null,
                &PROPERTIES,
                0,
                &buffer(256),
                0,
                null.cast(),
                iid,
                &mut dst_packed
            ),
            S_OK
        );
        assert_eq!(
            create_committed(
                null,
                &PROPERTIES,
                0,
                &buffer(1024),
                0,
                null.cast(),
                iid,
                &mut dst_crop
            ),
            S_OK
        );

        // Fill a 4x4 RGBA8 pattern (row_pitch 256): pixel (x, y) = [x*40+1, y*50+2, (x+y)*20+3, 0xff].
        let (up_ptr, _) = buffer_bytes(upload_buf).unwrap();
        // SAFETY: upload_buf holds 1024 bytes.
        let up = unsafe { std::slice::from_raw_parts_mut(up_ptr, 1024) };
        for y in 0..4usize {
            for x in 0..4usize {
                let at = y * 256 + x * 4;
                up[at..at + 4].copy_from_slice(&[
                    (x * 40 + 1) as u8,
                    (y * 50 + 2) as u8,
                    ((x + y) * 20 + 3) as u8,
                    0xff,
                ]);
            }
        }

        let queue_iid = guid(
            0x0ec870a6,
            0x5d7e,
            0x4c22,
            [0x8c, 0xfc, 0x5b, 0xaa, 0xe0, 0x76, 0x16, 0xed],
        );
        let allocator_iid = guid(
            0x6102dee4,
            0xaf59,
            0x4b09,
            [0xb9, 0x99, 0xb4, 0x4d, 0x73, 0xf0, 0x9b, 0x24],
        );
        let list_iid = guid(
            0x5b160d0f,
            0xac1b,
            0x4185,
            [0x8b, 0xa8, 0xb3, 0xae, 0x42, 0xa5, 0xa4, 0x55],
        );
        let (mut queue, mut allocator, mut list) = (null, null, null);
        let queue_desc = [0u32; 4];
        assert_eq!(
            crate::queue::create(null, queue_desc.as_ptr().cast(), &queue_iid, &mut queue),
            S_OK
        );
        assert_eq!(
            crate::list::create_allocator(null, 0, &allocator_iid, &mut allocator),
            S_OK
        );
        assert_eq!(
            crate::list::create_list(null, 0, 0, allocator, null, &list_iid, &mut list),
            S_OK
        );

        #[repr(C)]
        struct Location {
            resource: This,
            kind: u32,
            footprint: PlacedFootprint,
        }
        type Copy = extern "system" fn(
            This,
            *const Location,
            u32,
            u32,
            u32,
            *const Location,
            *const [u32; 6],
        );
        let copy = slot::<Copy>(list, 0x80);

        let full_rows = PlacedFootprint {
            offset: 0,
            format: 28,
            width: 4,
            height: 4,
            depth: 1,
            row_pitch: 256,
        };
        let tex_loc = Location {
            resource: image,
            kind: 0,
            footprint: PlacedFootprint {
                offset: 0,
                ..full_rows
            },
        };
        let up_loc = Location {
            resource: upload_buf,
            kind: 1,
            footprint: full_rows,
        };
        // 1) Buffer -> Texture upload.
        copy(list, &tex_loc, 0, 0, 0, &up_loc, std::ptr::null());

        // 2) Texture -> Buffer readback at non-zero offset (256), 256-aligned row pitch.
        let aligned_loc = Location {
            resource: dst_aligned,
            kind: 1,
            footprint: PlacedFootprint {
                offset: 256,
                ..full_rows
            },
        };
        copy(list, &aligned_loc, 0, 0, 0, &tex_loc, std::ptr::null());

        // 3) Texture -> Buffer readback with unaligned row_pitch (16 bytes, packed).
        let packed_loc = Location {
            resource: dst_packed,
            kind: 1,
            footprint: PlacedFootprint {
                offset: 32,
                format: 28,
                width: 4,
                height: 4,
                depth: 1,
                row_pitch: 16,
            },
        };
        copy(list, &packed_loc, 0, 0, 0, &tex_loc, std::ptr::null());

        // 4) Texture -> Buffer sub-box readback: 2x2 crop from (1, 1) to (3, 3).
        let crop_box = [1u32, 1, 0, 3, 3, 1];
        let crop_loc = Location {
            resource: dst_crop,
            kind: 1,
            footprint: PlacedFootprint {
                offset: 0,
                format: 28,
                width: 2,
                height: 2,
                depth: 1,
                row_pitch: 256,
            },
        };
        copy(list, &crop_loc, 0, 0, 0, &tex_loc, &crop_box);

        // 5) Console formats 118..=120 (3 B/texel) map to 4-byte PC formats
        // (45..=47): skipped cleanly without breaking Close() or touching dst.
        let mut dst_d16s8 = null;
        assert_eq!(
            create_committed(
                null,
                &PROPERTIES,
                0,
                &buffer(1024),
                0,
                null.cast(),
                iid,
                &mut dst_d16s8
            ),
            S_OK
        );
        let d16s8_loc = Location {
            resource: dst_d16s8,
            kind: 1,
            footprint: PlacedFootprint {
                offset: 0,
                format: 118,
                width: 4,
                height: 4,
                depth: 1,
                row_pitch: 256,
            },
        };
        copy(list, &d16s8_loc, 0, 0, 0, &tex_loc, std::ptr::null());

        assert_eq!(
            slot::<extern "system" fn(This) -> HRESULT>(list, 0x48)(list),
            S_OK
        );
        slot::<extern "system" fn(This, u32, *const This)>(queue, 0x50)(queue, 1, &list);

        // Verify 256-aligned readback at offset 256 (leading 256 bytes untouched).
        let (al_ptr, _) = buffer_bytes(dst_aligned).unwrap();
        // SAFETY: dst_aligned holds 2048 bytes.
        let al = unsafe { std::slice::from_raw_parts(al_ptr, 2048) };
        assert!(
            al[..256].iter().all(|&b| b == 0),
            "bytes before offset touched"
        );
        for row in 0..4 {
            let at = 256 + row * 256;
            assert_eq!(
                al[at..at + 16],
                up[row * 256..row * 256 + 16],
                "aligned row {row}"
            );
        }

        // Verify unaligned packed readback (row_pitch 16, offset 32).
        let (pk_ptr, _) = buffer_bytes(dst_packed).unwrap();
        // SAFETY: dst_packed holds 256 bytes.
        let pk = unsafe { std::slice::from_raw_parts(pk_ptr, 256) };
        assert!(
            pk[..32].iter().all(|&b| b == 0),
            "bytes before packed offset touched"
        );
        for row in 0..4 {
            let at = 32 + row * 16;
            assert_eq!(
                pk[at..at + 16],
                up[row * 256..row * 256 + 16],
                "packed row {row}"
            );
        }

        // Verify 2x2 sub-box crop from (1, 1)..(3, 3).
        let (cr_ptr, _) = buffer_bytes(dst_crop).unwrap();
        // SAFETY: dst_crop holds 1024 bytes.
        let cr = unsafe { std::slice::from_raw_parts(cr_ptr, 1024) };
        for row in 0..2 {
            let src_at = (row + 1) * 256 + 4; // x=1, y=row+1
            let dst_at = row * 256;
            assert_eq!(
                cr[dst_at..dst_at + 8],
                up[src_at..src_at + 8],
                "crop row {row}"
            );
        }

        // Verify skipped 118..=120 readback left its buffer untouched.
        let (d16_ptr, _) = buffer_bytes(dst_d16s8).unwrap();
        // SAFETY: dst_d16s8 holds 1024 bytes.
        let d16 = unsafe { std::slice::from_raw_parts(d16_ptr, 1024) };
        assert!(
            d16.iter().all(|&b| b == 0),
            "mismatched format 118 buffer touched"
        );
    }

    #[test]
    fn committed_and_placed_resources_map_at_their_gpu_address() {
        let null = std::ptr::null();
        let mut committed = null;
        let hr = create_committed(
            null,
            &PROPERTIES,
            0,
            &buffer(256),
            0,
            null.cast(),
            &RESOURCE_IIDS[0],
            &mut committed,
        );
        assert_eq!(hr, S_OK);
        let mut data = std::ptr::null_mut();
        assert_eq!(
            slot::<Map>(committed, 0x40)(committed, 0, null.cast(), &mut data),
            S_OK
        );
        assert_eq!(slot::<Address>(committed, 0x58)(committed), data as u64);
        // SAFETY: the buffer is at least 256 zeroed bytes.
        assert_eq!(unsafe { *data.add(255) }, 0);
        assert_eq!(slot::<Release>(committed, 0x10)(committed), 0);

        let heap = heap(4 * KIB_64);
        let place = |offset, desc: &ResourceDesc, out: &mut This| {
            create_placed(
                null,
                heap,
                offset,
                desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                out,
            )
        };
        let mut placed = null;
        assert_eq!(place(KIB_64, &buffer(KIB_64), &mut placed), S_OK);
        // SAFETY: a heap we made.
        let base = unsafe { (*heap.cast::<Heap>()).memory.ptr.as_ptr() } as u64;
        assert_eq!(slot::<Address>(placed, 0x58)(placed), base + KIB_64);
        let mut other = null;
        assert_eq!(
            place(4 * KIB_64, &buffer(1), &mut other),
            E_INVALIDARG,
            "past the end"
        );
        assert_eq!(
            place(100, &buffer(1), &mut other),
            E_INVALIDARG,
            "misaligned"
        );
        // The resource keeps the heap alive.
        assert_eq!(slot::<Release>(heap, 0x10)(heap), 1);
        assert_eq!(slot::<Release>(placed, 0x10)(placed), 0);
    }

    #[cfg(windows)]
    fn desc_words(desc: &ResourceDesc, pad1: u32, pad13: u32) -> [u32; 14] {
        // SAFETY: `ResourceDesc` is 56 bytes (`[u32; 14]`).
        let mut words: [u32; 14] = unsafe { std::mem::transmute_copy(desc) };
        words[1] = pad1;
        words[13] = pad13;
        words
    }

    #[cfg(windows)]
    fn fresh_va(hold: &mut Vec<Memory>) -> Memory {
        loop {
            let mem = Memory::zeroed(KIB_64, KIB_64).unwrap();
            let addr = mem.address();
            let in_live = LIVE_TEXTURES.lock().unwrap().contains_key(&addr);
            let in_placed = PLACED_TEXTURES
                .lock()
                .unwrap()
                .keys()
                .any(|&(a, _)| a == addr);
            if !in_live && !in_placed {
                return mem;
            }
            hold.push(mem);
        }
    }

    #[cfg(windows)]
    #[test]
    fn placed_srv_and_aliased_resource_see_rendered_target_pixels() {
        use std::sync::Arc;
        use std::sync::atomic::Ordering;
        use windows::Win32::Graphics::Direct3D12::*;
        use windows::core::Interface;

        let Some(host) = crate::host::host() else {
            return;
        };
        let mut hold = Vec::new();
        let backing = fresh_va(&mut hold);
        let addr = backing.address();
        let null = std::ptr::null();

        let rt_desc = ResourceDesc {
            flags: 1, // ALLOW_RENDER_TARGET
            ..texture(16, 16, 28, 1)
        };
        let mut r1 = null;
        assert_eq!(
            create_at_address(
                null,
                addr,
                &rt_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut r1
            ),
            S_OK
        );

        let rtv_slots = [0u64; 16];
        let rtv_handle = rtv_slots.as_ptr() as u64;
        crate::descriptor::create_rtv(null, r1, null.cast(), rtv_handle);

        let queue_iid = guid(
            0x0ec870a6,
            0x5d7e,
            0x4c22,
            [0x8c, 0xfc, 0x5b, 0xaa, 0xe0, 0x76, 0x16, 0xed],
        );
        let allocator_iid = guid(
            0x6102dee4,
            0xaf59,
            0x4b09,
            [0xb9, 0x99, 0xb4, 0x4d, 0x73, 0xf0, 0x9b, 0x24],
        );
        let list_iid = guid(
            0x5b160d0f,
            0xac1b,
            0x4185,
            [0x8b, 0xa8, 0xb3, 0xae, 0x42, 0xa5, 0xa4, 0x55],
        );
        let (mut queue, mut allocator, mut list) = (null, null, null);
        let queue_desc = [0u32; 4];
        assert_eq!(
            crate::queue::create(null, queue_desc.as_ptr().cast(), &queue_iid, &mut queue),
            S_OK
        );
        assert_eq!(
            crate::list::create_allocator(null, 0, &allocator_iid, &mut allocator),
            S_OK
        );
        assert_eq!(
            crate::list::create_list(null, 0, 0, allocator, null, &list_iid, &mut list),
            S_OK
        );

        type ClearRtv = extern "system" fn(This, u64, *const [f32; 4], u32, *const [i32; 4]);
        let clear = [64.0f32 / 255.0, 128.0 / 255.0, 191.0 / 255.0, 1.0];
        slot::<ClearRtv>(list, 0x1a0)(list, rtv_handle, &clear, 0, null.cast());
        assert_eq!(
            slot::<extern "system" fn(This) -> HRESULT>(list, 0x48)(list),
            S_OK
        );
        slot::<extern "system" fn(This, u32, *const This)>(queue, 0x50)(queue, 1, &list);

        let srv_desc = texture(16, 16, 28, 1);
        let srv_words_a = desc_words(&srv_desc, 0xdead_beef, 0x1234_5678);
        let srv_words_b = desc_words(&srv_desc, 0xcafe_babe, 0x8765_4321);
        let placed = texture_at(addr, &srv_words_a).expect("placed texture");
        let placed_same = texture_at(addr, &srv_words_b).expect("canonical padding match");
        assert_eq!(placed, placed_same);

        let mut r2 = null;
        assert_eq!(
            create_at_address(
                null,
                addr,
                &srv_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut r2
            ),
            S_OK
        );

        let h1 = host_copy(r1).expect("r1 host");
        let hp = host_copy(placed).expect("placed host");
        let h2 = host_copy(r2).expect("r2 host");
        assert!(Arc::ptr_eq(&h1, &hp));
        assert!(Arc::ptr_eq(&h1, &h2));
        assert!(!crate::texture::title_written(
            host_resource(placed).unwrap()
        ));
        // Reusing a live HostTexture must not reset its tracked D3D12 state.
        assert_eq!(h1.state.load(Ordering::Relaxed), 0x4);

        // Read back `placed`'s host texture and verify the cleared pixels survive intact.
        let readback_bytes = 16usize * 256;
        let readback_heap = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_READBACK,
            ..Default::default()
        };
        let readback_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Width: readback_bytes as u64,
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
        let mut readback: Option<ID3D12Resource> = None;
        // SAFETY: plain creation of a readback buffer.
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
        let host_image = host_resource(placed).unwrap();
        let back = host.new_list().unwrap();
        let (_, before) = host_transition(placed, 0x800).unwrap();
        assert_eq!(before, 0x4);

        #[repr(C)]
        struct Location {
            resource: This,
            kind: u32,
            footprint: PlacedFootprint,
        }
        let rows = PlacedFootprint {
            offset: 0,
            format: 28,
            width: 16,
            height: 16,
            depth: 1,
            row_pitch: 256,
        };
        let out_to = Location {
            resource: readback.as_raw().cast_const().cast(),
            kind: 1,
            footprint: rows,
        };
        let out_from = Location {
            resource: host_image.as_raw().cast_const().cast(),
            kind: 0,
            footprint: PlacedFootprint { offset: 0, ..rows },
        };
        // SAFETY: open list, live resources, locations in the host layout.
        unsafe {
            back.list.ResourceBarrier(&[crate::host::transition(
                host_image,
                D3D12_RESOURCE_STATES(before as i32),
                D3D12_RESOURCE_STATE_COPY_SOURCE,
            )]);
            back.list.CopyTextureRegion(
                (&raw const out_to).cast(),
                0,
                0,
                0,
                (&raw const out_from).cast(),
                None,
            );
        }
        back.close();
        host.execute(&[&back]);
        assert_eq!(host.log_messages(), None);

        let mut mapped = std::ptr::null_mut();
        // SAFETY: `execute` waited for the copy; `readback` is `readback_bytes` long.
        unsafe {
            readback
                .Map(
                    0,
                    Some(&D3D12_RANGE {
                        Begin: 0,
                        End: readback_bytes,
                    }),
                    Some(&mut mapped),
                )
                .unwrap();
            let data = std::slice::from_raw_parts(mapped.cast::<u8>(), readback_bytes);
            for y in 0..16 {
                let row = &data[y * 256..y * 256 + 16 * 4];
                for x in 0..16 {
                    assert_eq!(
                        &row[x * 4..x * 4 + 4],
                        &[64, 128, 191, 255],
                        "pixel ({x}, {y})"
                    );
                }
            }
            readback.Unmap(0, None);
        }

        assert_eq!(slot::<Release>(list, 0x10)(list), 0);
        assert_eq!(slot::<Release>(allocator, 0x10)(allocator), 0);
        assert_eq!(slot::<Release>(queue, 0x10)(queue), 0);
        assert_eq!(slot::<Release>(r2, 0x10)(r2), 0);
        assert_eq!(slot::<Release>(r1, 0x10)(r1), 0);
        let _ = rtv_slots[0];
    }

    #[cfg(windows)]
    #[test]
    fn incompatible_alias_flags_or_geometry_do_not_share_host_texture() {
        use std::sync::Arc;

        let Some(_host) = crate::host::host() else {
            return;
        };
        let mut hold = Vec::new();
        let mem_a = fresh_va(&mut hold);
        let va_a = mem_a.address();
        let null = std::ptr::null();

        let srv_desc = texture(16, 16, 28, 1);
        let rt_desc = ResourceDesc {
            flags: 1, // ALLOW_RENDER_TARGET
            ..srv_desc
        };
        let rt_srgb_desc = ResourceDesc {
            format: 29, // R8G8B8A8_UNORM_SRGB
            ..rt_desc
        };
        let wider_desc = texture(32, 16, 28, 1);
        let other_family_desc = texture(16, 16, 10, 1); // R16G16B16A16_FLOAT

        let (mut r_srv, mut r_rt, mut r_rt_srgb, mut r_wider, mut r_fam) =
            (null, null, null, null, null);
        assert_eq!(
            create_at_address(
                null,
                va_a,
                &srv_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut r_srv
            ),
            S_OK
        );
        // SRV-only must NOT be reused for ALLOW_RENDER_TARGET.
        assert_eq!(
            create_at_address(
                null,
                va_a,
                &rt_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut r_rt
            ),
            S_OK
        );
        assert!(!Arc::ptr_eq(
            &host_copy(r_srv).unwrap(),
            &host_copy(r_rt).unwrap()
        ));

        // Typed R8G8B8A8_UNORM RT must NOT be reused for typed R8G8B8A8_UNORM_SRGB RT.
        assert_eq!(
            create_at_address(
                null,
                va_a,
                &rt_srgb_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut r_rt_srgb
            ),
            S_OK
        );
        assert!(!Arc::ptr_eq(
            &host_copy(r_rt).unwrap(),
            &host_copy(r_rt_srgb).unwrap()
        ));

        // Different geometry and different format family must NOT share.
        assert_eq!(
            create_at_address(
                null,
                va_a,
                &wider_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut r_wider
            ),
            S_OK
        );
        assert_eq!(
            create_at_address(
                null,
                va_a,
                &other_family_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut r_fam
            ),
            S_OK
        );
        for other in [r_srv, r_rt, r_rt_srgb] {
            assert!(!Arc::ptr_eq(
                &host_copy(other).unwrap(),
                &host_copy(r_wider).unwrap()
            ));
            assert!(!Arc::ptr_eq(
                &host_copy(other).unwrap(),
                &host_copy(r_fam).unwrap()
            ));
        }

        for r in [r_srv, r_rt, r_rt_srgb, r_wider, r_fam] {
            assert_eq!(slot::<Release>(r, 0x10)(r), 0);
        }
        assert!(
            !LIVE_TEXTURES.lock().unwrap().contains_key(&va_a),
            "releasing all aliases removes LIVE_TEXTURES bucket"
        );

        // DENY_SHADER_RESOURCE depth texture must NOT be reused for shader-readable alias.
        let depth_deny_desc = ResourceDesc {
            flags: DEPTH_FLAG | DENY_SHADER_RESOURCE_FLAG,
            ..texture(16, 16, 40, 1) // D32_FLOAT
        };
        let float_srv_desc = texture(16, 16, 41, 1); // R32_FLOAT
        let (mut r_depth, mut r_float) = (null, null);
        assert_eq!(
            create_at_address(
                null,
                va_a,
                &depth_deny_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut r_depth
            ),
            S_OK
        );
        assert_eq!(
            create_at_address(
                null,
                va_a,
                &float_srv_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut r_float
            ),
            S_OK
        );
        assert!(!Arc::ptr_eq(
            &host_copy(r_depth).unwrap(),
            &host_copy(r_float).unwrap()
        ));
        assert_eq!(slot::<Release>(r_depth, 0x10)(r_depth), 0);
        assert_eq!(slot::<Release>(r_float, 0x10)(r_float), 0);
        assert!(!LIVE_TEXTURES.lock().unwrap().contains_key(&va_a));

        // `texture_at` before then after an RT exists at a fresh VA upgrades to the live RT.
        let mem_b = fresh_va(&mut hold);
        let va_b = mem_b.address();
        let srv_words = desc_words(&srv_desc, 0x1111_2222, 0x3333_4444);
        let before = texture_at(va_b, &srv_words).expect("placed texture before RT");
        assert!(crate::texture::title_written(
            host_resource(before).unwrap()
        ));

        let mut rt_b = null;
        assert_eq!(
            create_at_address(
                null,
                va_b,
                &rt_desc,
                0,
                null.cast(),
                &RESOURCE_IIDS[0],
                &mut rt_b
            ),
            S_OK
        );
        assert!(!Arc::ptr_eq(
            &host_copy(before).unwrap(),
            &host_copy(rt_b).unwrap()
        ));

        let after = texture_at(va_b, &srv_words).expect("placed texture after RT");
        assert_ne!(after, before);
        assert!(Arc::ptr_eq(
            &host_copy(after).unwrap(),
            &host_copy(rt_b).unwrap()
        ));
        assert!(!crate::texture::title_written(
            host_resource(after).unwrap()
        ));

        let after_pad = texture_at(va_b, &desc_words(&srv_desc, 0xaaaa_bbbb, 0xcccc_dddd))
            .expect("canonical padding lookup after upgrade");
        assert_eq!(after_pad, after);

        assert_eq!(slot::<Release>(rt_b, 0x10)(rt_b), 0);
        assert!(
            LIVE_TEXTURES
                .lock()
                .unwrap()
                .get(&va_b)
                .is_some_and(|v| v.iter().all(|e| e.resource != rt_b as usize)),
            "released rt_b is unregistered from LIVE_TEXTURES"
        );
    }

    /// different families, so the copy goes through a staging buffer with
    /// one format at each end.
    #[test]
    fn a_placed_copy_takes_the_bytes_of_its_writer() {
        use windows::Win32::Graphics::Direct3D12::*;
        use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;

        let Some(host) = crate::host::host() else {
            return; // no host GPU
        };
        // 8x4x2 3D textures of 4 bytes a texel: the writer in R32_SINT (42,
        // as the world's fog volumes are written), the placed copy in the
        // console's packed R9G9B9E5 (67, as their gather reads them).
        let writer_desc = ResourceDesc {
            dimension: TEXTURE_3D,
            width: 8,
            height: 4,
            depth_or_array_size: 2,
            mip_levels: 1,
            format: 42,
            sample_count: 1,
            flags: 4, // unordered access
            layout: 0,
            ..buffer(0)
        };
        let placed_desc = ResourceDesc {
            format: 67,
            flags: 0,
            ..writer_desc
        };
        let words = |d: &ResourceDesc| -> [u32; 14] {
            // SAFETY: a description is 56 bytes of plain words, aligned.
            unsafe { std::ptr::from_ref(d).cast::<[u32; 14]>().read() }
        };
        let memory = Memory::zeroed(4 * KIB_64, KIB_64).expect("title memory");
        let address = memory.address();
        let writer = texture_at(address, &words(&writer_desc)).expect("a writer");
        let placed = texture_at(address, &words(&placed_desc)).expect("a placed copy");

        // The writer's host texture, as a shader fill left it.
        let writer_host = host_resource(writer)
            .expect("the writer's host copy")
            .clone();
        // SAFETY: a live resource.
        let d = unsafe { writer_host.GetDesc() };
        let mut from = [D3D12_PLACED_SUBRESOURCE_FOOTPRINT::default(); 1];
        let mut total = 0u64;
        // SAFETY: one output for the one subresource.
        unsafe {
            host.device.GetCopyableFootprints(
                &d,
                0,
                1,
                0,
                Some(from.as_mut_ptr()),
                None,
                None,
                Some(&mut total),
            );
        }
        let (staging, data) = host.upload_buffer(total).expect("staging");
        {
            let f = from[0].Footprint;
            for z in 0..f.Depth {
                for y in 0..f.Height {
                    for x in 0..f.Width {
                        let at = from[0].Offset as usize
                            + (z * f.Height * f.RowPitch + y * f.RowPitch) as usize
                            + (x * 4) as usize;
                        // SAFETY: inside the staging buffer's mapping.
                        unsafe {
                            (data.add(at) as *mut u32)
                                .write(x + y * f.Width + z * f.Width * f.Height)
                        };
                    }
                }
            }
        }
        let list = host.new_list().expect("new_list");
        let texture_at_location = |resource: &ID3D12Resource, sub: u32| {
            D3D12_TEXTURE_COPY_LOCATION {
                // SAFETY: borrowed pointers the locations never release.
                pResource: unsafe { std::mem::transmute_copy(resource) },
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    SubresourceIndex: sub,
                },
            }
        };
        let staging_location = |footprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT| {
            D3D12_TEXTURE_COPY_LOCATION {
                // SAFETY: as above.
                pResource: unsafe { std::mem::transmute_copy(&staging) },
                Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    PlacedFootprint: footprint,
                },
            }
        };
        crate::list::ensure_state(&list, writer, 0x400);
        // SAFETY: an open list and live resources of the same size.
        unsafe {
            list.list.CopyTextureRegion(
                &texture_at_location(&writer_host, 0),
                0,
                0,
                0,
                &staging_location(from[0]),
                None,
            )
        };
        list.close();
        host.execute(&[&list]);

        // The copy from it into the placed copy, as a reader's bind makes.
        list.reset();
        let view = crate::descriptor::View::Placed(false, words(&placed_desc), [0; 16]);
        assert!(
            crate::texture::sync_placed(&list, placed, &view),
            "the placed copy is synced"
        );
        list.close();
        host.execute(&[&list]);

        // Read it back and compare with what the writer held.
        let placed_copy = host_copy(placed).expect("the placed copy's host copy");
        // SAFETY: a live resource.
        let d = unsafe { placed_copy.texture.GetDesc() };
        let mut to = [D3D12_PLACED_SUBRESOURCE_FOOTPRINT::default(); 1];
        let mut back_total = 0u64;
        // SAFETY: one output for the one subresource.
        unsafe {
            host.device.GetCopyableFootprints(
                &d,
                0,
                1,
                0,
                Some(to.as_mut_ptr()),
                None,
                None,
                Some(&mut back_total),
            );
        }
        let back_heap = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_READBACK,
            ..Default::default()
        };
        let back_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Width: back_total,
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
        let mut back: Option<ID3D12Resource> = None;
        // SAFETY: plain creation of a read-back buffer.
        unsafe {
            host.device
                .CreateCommittedResource(
                    &back_heap,
                    D3D12_HEAP_FLAGS(0),
                    &back_desc,
                    D3D12_RESOURCE_STATE_COPY_DEST,
                    None,
                    &mut back,
                )
                .expect("a read-back buffer");
        }
        let back = back.expect("a read-back buffer");
        let back_at =
            |footprint| windows::Win32::Graphics::Direct3D12::D3D12_TEXTURE_COPY_LOCATION {
                // SAFETY: a borrowed pointer the location never releases.
                pResource: unsafe { std::mem::transmute_copy(&back) },
                Type:
                    windows::Win32::Graphics::Direct3D12::D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
                Anonymous: windows::Win32::Graphics::Direct3D12::D3D12_TEXTURE_COPY_LOCATION_0 {
                    PlacedFootprint: footprint,
                },
            };
        list.reset();
        crate::list::ensure_state_host(&list, &placed_copy, 0x800);
        // SAFETY: an open list; both locations name live resources of the
        // same size.
        unsafe {
            list.list.CopyTextureRegion(
                &back_at(to[0]),
                0,
                0,
                0,
                &texture_at_location(&placed_copy.texture, 0),
                None,
            )
        };
        list.close();
        host.execute(&[&list]);
        let mut data = std::ptr::null_mut();
        let range = D3D12_RANGE {
            Begin: 0,
            End: back_total as usize,
        };
        // SAFETY: a read-back buffer this list resolved into.
        unsafe { back.Map(0, Some(&range), Some(&mut data)).expect("map") };
        let f = to[0].Footprint;
        let mut wrong = 0;
        for z in 0..f.Depth {
            for y in 0..f.Height {
                for x in 0..f.Width {
                    let at = to[0].Offset as usize
                        + (z * f.Height * f.RowPitch + y * f.RowPitch) as usize
                        + (x * 4) as usize;
                    // SAFETY: inside the read-back buffer's mapping.
                    let got = unsafe { (data.add(at) as *const u32).read() };
                    if got != x + y * f.Width + z * f.Width * f.Height {
                        wrong += 1;
                    }
                }
            }
        }
        // SAFETY: the same buffer, read above.
        unsafe { back.Unmap(0, None) };
        assert_eq!(wrong, 0, "the placed copy holds the writer's texels");
    }
}
