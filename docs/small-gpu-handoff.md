# Small GPU support handoff

Local branch: `fix/small-gpu-memory`, based on integration commit `a7d7a540`.
This checkout is separate from the concurrent ztrx integration. It has not
been merged into the integration branch or pushed.

## Changes

- `resource.rs`: allocate the host texture on first host access using a
  `OnceLock`, rather than at title resource creation. Keep an owned copy of
  the clear value and the initial state for delayed creation. Alias lookup
  and destruction inspect existing copies without triggering allocations.
  Command lists retain their existing shared ownership of textures.
- `host.rs`: identify the actual D3D12 device's DXGI adapter by LUID, report
  UMA, and log process usage against the dynamic OS memory budget every five
  seconds. UMA reports LOCAL only; discrete adapters report both segments.
  Query failures do not prevent rendering. This is reporting, not eviction.
- `scripts/play.ps1 -SmallGpu`: select `--console series-s` and report a
  1280x720 display mode. Normal launches preserve their existing arguments
  and mode. Existing shader safety settings are retained in both modes.

Build with an isolated `CARGO_TARGET_DIR`, then pass the resulting executable
to the launcher through `-Exe`; its default path is `target/release` in this
checkout. The title still chooses internal render sizes from its profile.

## Verification

- Both workspace Clippy configurations pass with warnings denied.
- Serial workspace tests pass: 745 tests in parent harnesses, 11 existing
  ignores. Isolated child tests also pass; summing every printed summary
  would count them twice (761 rather than 745).
- New WARP regressions explicitly require a device. They check that unused
  resources allocate no host textures, first use shares one host texture,
  list ownership survives title resource release, clear values outlive the
  caller, and initial depth state is preserved. The existing buffer-to-texture
  GPU readback test now also asserts deferred creation.
- The adapter identity test verifies the budget adapter matches the actual
  WARP device and tolerates absent software-driver budgets. It runs in a
  subprocess with the existing timeout.
- A synthetic executable verifies normal/small launch arguments, paths with
  spaces, diagnostic-input clearing, shader safety setting preservation,
  display mode selection, and propagation of exit status 7.
- A read-only hardware query on the user's Radeon 780M reports UMA, Shader
  Model 6.6, mesh shader tier 1, and 32-64-lane waves. Its observed local budget
  was 16,439 MiB; this is a dynamic per-process budget, not a fixed requirement.
  Driver version reported by Windows: `32.0.32029.2010`.

Build output, synthetic probes and full check logs are outside the checkout.
No proprietary title data is included in these tests or this handoff.

## Remaining work

There is no measured game memory reduction or stable world-play result yet.
Descriptors and barriers can materialize textures before a draw; only
resources never used by host operations avoid allocation entirely. The change
does not repair untranslated shaders or independently establish the cause of
the device removal seen after allocation failures.

`PLACED_TEXTURES` in `resource.rs` and `LOOSE` in `shadow.rs` still grow during
streaming. Texture upload staging is also retained beyond the texture bytes
reported in the log. Reclaiming these needs allocation-generation tracking
on unmap/reuse and descriptor/list ownership coordination. Dropping cached
resources or imposing a cap that returns missing textures is not a safe fix.

After ztrx integration, port this branch's resource changes semantically if
that work touches texture ownership. Repeat the GPU/diagnostics regressions
and combined checks on the result before using it as the main build. Then
compare the same scene under Series S on the 780M using texture totals and
the new budget lines. Title-run stability and sustained streaming remain
unverified by this branch.

## Continued 2026-11-25: one cache closed, one made visible

The branch is merged into `integration/maintenance` (`0071544e`, merge parents
`7acc3a16` and `35e7bf11`) and pushed. It merged clean: no file overlap with
the ztrx work that landed in between (that touched `crash.rs` and its own
assessment doc). Combined gate on the merge: fmt 0 differing, both Clippy
configurations 0 errors, policy 228 files, workspace **764 passed, 0 failed**.

**Closed: `LOOSE` in `shadow.rs`** (`d592c4de`). Windows of title memory
outside any buffer were held as strong `Arc<Shadow>`s for the rest of the run —
one 4 MiB copy per window ever touched. They are now held weakly and the map is
pruned of dead entries as it grows, which is the pattern `EXACT` in the same
file already uses for per-view copies. The invariant that makes it safe is the
one that comment states: a copy is found again for as long as anything shows
it. A heap slot, a bindless hull and a list with a write to copy back all hold
the `Arc`, so an entry only dies when no consumer can be looking at it.

Dropping a copy loses nothing: a `Shadow` starts empty, its bytes come from a
`refresh` before the read that needs them, and a pending GPU write keeps the
`Arc` alive through its list.

Tested by holding a window, asking again (the same copy returns), dropping it,
asserting it is no longer listed, and asserting it is made afresh on the next
ask. Negative control: leaking one strong reference fails the test with "the
window is still held after its last user dropped it".

**Made visible: `PLACED_TEXTURES`** (`28bcc31f`). Placed textures are created
by `texture_at`, which bypasses `Texture::new`, so `TEXTURES`/`TEXTURE_BYTES`
never counted them and their growth was absent from the memory line. They are
counted now, reported as `placed textures N (never released)`. The count only,
not the bytes: a byte figure needs a `D3D12_RESOURCE_DESC` and this module has
its own description type, so the number would be a guess.

**Not attempted, deliberately.** Releasing placed textures as a cache eviction
would dangle a descriptor: every entry is a COM resource that heap slots made
from it still point at. Doing it properly needs to know which textures live
descriptors still show — the ownership coordination this handoff already
names. Also untouched: texture upload staging retained beyond the texture
bytes, and the descriptor and barrier paths that materialize textures before a
draw (so only resources no host operation touches avoid allocation).

**Still unverified, unchanged by this work:** stable world play, and any
measured memory reduction. Both need a title run, and this machine's display
driver has been removed as hung three times.

## Measured after the upstream 291-commit merge (2026-11-25 13:33-13:37 UTC)

The deferral above was lost when the merge took upstream's `resource.rs`
wholesale; host copies were again made for every texture the title reserved.
It is re-applied on `merge/up2` as commit `3931cae8`.

What the runs measured, same machine (Radeon 780M, 32 GB shared memory),
`game_scarlett_master.exe --execute`:

| | copies made at reserve (before) | copies made on first use (after) |
| --- | --- | --- |
| host textures at the end | 3,894 | 1,515 |
| host texture memory at the end | 9,583 MB | 4,803 MB |
| `0x8007000E` "not enough memory" | 439 | 0 |
| device removals | 97 | 0 |
| run length | 104 s, ended at frame 3,214 | 240 s, ended at frame 4,096 (my timeout) |
| draws dropped | 0 | 0 |

The memory no longer grows: the copy count is flat at 1,515 and the bytes at
4,803 MB for the whole three-and-a-half minutes, where the earlier run added
4.7 GB in 100 s and then failed the next 4 MiB allocation.

Stable world play is **still unverified**, and this is why: the run held
30.0 fps with 8 ms of GPU work in a 33 ms frame, about 13 draws per frame.
That is the boot/menu scene on a vsync-locked present, not the world. The
frame budget is not the constraint in it, so nothing here says how the world
holds up. Reaching the world needs input to the title, which `--execute` does
not give; `scripts/play.ps1` is the path that does.

**Remaining, from the same run:**

- One pixel shader does not translate: `GFX10 code at 0x38c: barycentrics
  read directly is not supported` (hash `98bd9f9ae5f9c7e3`). Its draws are
  dropped, so it is a visible gap, not a silent one.
- Two `victory_gpu::object method not implemented object="device"` slots,
  `0x218` and `0x1c8`.
- One descriptor warning: `buffer view empty or past its host copy first=0
  count=0`.
