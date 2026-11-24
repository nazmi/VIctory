//! Query heaps. Occlusion heaps measure on the host GPU (the draws that
//! mark them run there), so the title's own culling and dynamic resolution
//! read real counts; the other kinds are still answered at resolve time.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use windows::Win32::Graphics::Direct3D12::{
    D3D12_QUERY_HEAP_DESC, D3D12_QUERY_HEAP_TYPE_OCCLUSION, D3D12_QUERY_TYPE,
    D3D12_QUERY_TYPE_BINARY_OCCLUSION, D3D12_QUERY_TYPE_OCCLUSION, ID3D12QueryHeap,
};

use crate::object::{self, E_INVALIDARG, HRESULT, Iid, Object, QUERY_HEAP, Table, This, guid};
use crate::resource::PAGEABLE_IIDS;

/// Occlusion: samples that passed, or (binary) whether any did.
const OCCLUSION: u32 = 0;
const BINARY: u32 = 1;

const HEAP_IIDS: [Iid; 4] = [
    guid(
        0x0d9658ae,
        0xed45,
        0x469e,
        [0xa6, 0x1d, 0x97, 0x0e, 0xc5, 0x83, 0xca, 0xb4],
    ),
    PAGEABLE_IIDS[0],
    PAGEABLE_IIDS[1],
    PAGEABLE_IIDS[2],
];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Desc {
    kind: u32,
    count: u32,
    nodes: u32,
}

#[repr(C)]
struct Heap {
    object: Object,
    desc: Desc,
    /// The host's own heap for the kinds that measure there, made with the
    /// same count so every index the title marks exists.
    host: Option<ID3D12QueryHeap>,
}

/// The host heap for a description, for the kinds the host can answer.
fn host_heap(desc: &Desc) -> Option<ID3D12QueryHeap> {
    if !matches!(desc.kind, OCCLUSION | BINARY) || desc.count == 0 {
        return None;
    }
    let host = crate::host::host()?;
    let mut heap: Option<ID3D12QueryHeap> = None;
    // SAFETY: plain object creation.
    unsafe {
        host.device
            .CreateQueryHeap(
                &D3D12_QUERY_HEAP_DESC {
                    // One heap type answers both the count and the
                    // pass/fail forms.
                    Type: D3D12_QUERY_HEAP_TYPE_OCCLUSION,
                    Count: desc.count,
                    NodeMask: 0,
                },
                &mut heap,
            )
            .ok()?;
    }
    heap
}

/// Device method.
pub(crate) extern "system" fn create(
    _: This,
    desc: *const Desc,
    iid: *const Iid,
    out: *mut This,
) -> HRESULT {
    // SAFETY: the title passes a description (null is tolerated).
    let Some(&desc) = (unsafe { desc.as_ref() }) else {
        return E_INVALIDARG;
    };
    let host = host_heap(&desc);
    tracing::debug!(?desc, measuring = host.is_some(), "query heap");
    let heap = Box::new(Heap {
        object: Object::new(&FUNCTIONS),
        desc,
        host,
    });
    object::hand_out(QUERY_HEAP, heap, iid, &HEAP_IIDS, out)
}

/// The host heap behind one of ours, and the query type a mark or a
/// resolve uses with it; `None` for the kinds that do not measure on the
/// host.
pub(crate) fn host(this: This) -> Option<(ID3D12QueryHeap, D3D12_QUERY_TYPE)> {
    if this.is_null() {
        return None;
    }
    // SAFETY: the title passes a heap it created here.
    let heap = unsafe { &*this.cast::<Heap>() };
    let kind = match heap.desc.kind {
        OCCLUSION => D3D12_QUERY_TYPE_OCCLUSION,
        BINARY => D3D12_QUERY_TYPE_BINARY_OCCLUSION,
        _ => return None,
    };
    Some((heap.host.clone()?, kind))
}

extern "system" fn query(this: This, iid: *const Iid, out: *mut This) -> HRESULT {
    object::answer(
        QUERY_HEAP,
        iid,
        object::is(iid, &HEAP_IIDS).then_some(this),
        out,
    )
}
extern "system" fn release(this: This) -> u32 {
    object::release_boxed::<Heap>(this)
}

/// An occlusion heap made the way the device makes one, for the tests:
/// `None` when the host refuses it (no GPU, or no heap of that kind).
#[cfg(test)]
pub(crate) fn occlusion_for_test(kind: u32, count: u32) -> Option<This> {
    let desc = Desc {
        kind,
        count,
        nodes: 0,
    };
    let mut out: This = std::ptr::null_mut();
    let hr = create(std::ptr::null(), &desc, &HEAP_IIDS[0], &mut out);
    (hr == object::S_OK && !out.is_null()).then_some(out)
}

/// IUnknown 3, object 4, device child 1.
static FUNCTIONS: Table<24> = {
    let mut f = object::d3d12_object(object::missing_table::<QUERY_HEAP, 24>());
    f[0] = query as *const ();
    f[2] = release as *const ();
    f[7] = object::parent_device as *const (); // 0x38
    Table(f)
};
