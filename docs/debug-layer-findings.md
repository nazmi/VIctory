# What the D3D12 debug layer reported

Captured 2026-11-25 with the `Graphics Tools` optional Windows feature
installed, which D3D12 needs for both the debug layer and DRED:

```
DISM /Online /Add-Capability /CapabilityName:Tools.Graphics.DirectX~~~~0.0.1.0
cargo build --profile diag -p victory --features diagnostics
VICTORY_D3D12_DEBUG=1 VICTORY_D3D12_DRED=1 .\target\diag\victory.exe run game_scarlett_master.exe --execute
```

Both switches announce themselves at startup (`victory_gpu::host`), so a run
that does not print those two lines is not being validated.

## Finding 1: an illegal view format (fixed)

```
ID3D12Device::CreateShaderResourceView: The Format (0x43, R9G9B9E5_SHAREDEXP)
  is invalid when creating a View; it is not a fully qualified format within
  the same family as the Format of the Resource (0x2a, R32_UINT). id=28
ID3D12Device::RemoveDevice: Device removal has been triggered for the following
  reason (DXGI_ERROR_INVALID_CALL ...) id=232
```

`castable()` in `crates/victory-gpu/src/resource.rs` chose view formats by
element size, so a four-byte format in another typeless family was offered as
a view. D3D12 permits a view only inside the resource's family; the illegal
call removes the device. Fixed in `7852d3e6`: same family, still no depth
formats and no typeless heads, with a property test over all 115 formats.

This matches the `DXGI_ERROR_INVALID_CALL` removal in the crash log at
12:51 UTC. After the fix, a re-run reached frame 1851 in 84 s with **no**
invalid-view message and no invalid call.

## Finding 2: a GPU timeout (open)

With the invalid call gone, the remaining removal is:

```
ID3D12Device::RemoveDevice: ... (DXGI_ERROR_DEVICE_RESET: The hardware took an
  unreasonable amount of time to execute a command ...) id=234
D3D12: Kernel memory failure. There might be a memory leak. id=834
```

That is a TDR, not an invalid command: a command ran past the 2 s GPU timeout.
The README already documents this and the `TdrDelay` registry workaround. It is
the next thing to chase, and the tool for it is the layer's own timing plus
`VICTORY_SHADER_LOOP_LIMIT`, which caps translated shader loops and tells you
whether loops are what make a frame slow.

## Finding 3: a descriptor heap larger than the recommended size (unguarded)

```
CreateDescriptorHeap: NumDescriptors 2097152, device supports TIER_3 which
  supports shader visible descriptor heap size larger than 1000000, but this
  may fail to create on some TIER_3 hardware. id=1013 severity=2
```

A warning, not an error, on this adapter. Worth capping at 1,000,000 views
(the `PORTABLE_VIEWS` figure this tree already carries) so a smaller host does
not lose the heap.

## Finding 4: command allocator reset race (unguarded)

```
ID3D12GraphicsCommandList::{Create,Reset}CommandList: The command allocator is
  currently in-use by another command list or is being reset in another thread.
  id=540 severity=1
```

Severity 1 — a real threading defect: an allocator shared across lists being
reset while another list uses it. It appeared in the same instant as the
removal, so it may be a consequence rather than a cause, but it should not
happen at all.

## What is still unverified

The run that produced these ended by TDR, so nothing here says the world is
stable. The illegal call is gone and provably so; the timeout is not.
