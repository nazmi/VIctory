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
