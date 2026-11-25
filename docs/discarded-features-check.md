# Every feature the upstream merge discarded, checked against upstream

## Repair update: 2026-11-25

The original audit below is a historical snapshot. The integration repair
has now restored the useful missing behavior against the current design:

| Behavior | Current disposition and evidence |
| --- | --- |
| Hanging draw tests | Recursive test mutex removed. Five draw regressions execute on WARP in bounded subprocesses; timeout, failed child and incomplete/removed fence cases are tested. No `VICTORY_GPU_TESTS` gate for these tests. |
| Depth bounds | Restored for graphics and mesh streams, capability checked, title state reset/rebound. WARP graphics readback verifies acceptance/rejection; this WARP has no mesh shaders, so mesh rendering remains unverified. Upstream geometry shaders and explicit mesh topology are preserved. |
| Occlusion | Counted and binary host queries restored. A WARP test overwrites nonzero sentinels with measured zero counts and leaves the suffix intact. Other query kinds retain upstream behavior. |
| 1D textures | Height-one 2D resources and SRV/UAV/RTV/DSV mappings restored together with shader coordinates and array dimensions. Synthetic descriptors, DXIL validation and WARP 1D/3D readback pass. |
| Library parallel loads | Per-name locks restored around loads/stores, preserving upstream prebuilt metadata. Saves serialize against stores and publish in order; failed saves retry. |
| Library kind verdicts | Retain upstream explicit mesh topology rather than revive the old workaround for a stream lacking topology. Mesh library behavior still needs a capable adapter; no blanket claim that mesh pipelines cannot load. |
| Shadow states / copy-back | Retain upstream tracked ranges, legal states and copy-back architecture. Add upstream `ed0dbfd1`, whose synthetic test proves CPU writes under pending shader-write ranges reach the copy without clobbering later title writes. Old shadow-state and threaded copy-back implementations are incompatible with this design; `VICTORY_COPY_THREADS` is not restored. |
| Stencil diagnostics | Stencil-plane readback and shader-view descriptions restored. |
| Intermediate target / draw trigger | `VICTORY_SHOW_TARGET=<zero-based index>` or `busiest`, and `VICTORY_CAPTURE_DRAWS`, restored. Separate show and dump lists prevent dump-before-present from losing the pick; a bounded WARP regression covers the ordering. |
| Image TFE | Real `CheckAccessFullyMapped` status restored for the supported image paths, with feature metadata. Explicit-level gathers remain refused: upstream added a multi-load path whose residency statuses need aggregation. |
| f16 interpolation | Prefer upstream `e29391f5` using the existing half-register writer, with output modifiers and destination selection. |
| Mesh lines / indices | Restore lines together with upstream `c0228d93` nine-bit corner masks. Synthetic modules validate without requiring mesh hardware. |

Previously recovered mode/worker switches, half conversion and atomic library
writes remain present. No title was run. WARP validates software behavior;
physical-adapter stability and mesh rendering are not established by these checks.


The merge dropped a number of our integrated PRs in favour of upstream's
designs. "Upstream has a better solution" was the claim; this file is the check
behind it, made 2026-11-25 by grepping both trees for the exact symbols and
then, where a name differed, for the behaviour.

## Method

For each item: is the symbol present in `HEAD`? in `upstream/main`? Where absent
from both, search for a functional equivalent under another name before calling
it lost. A hit in neither, with no equivalent, means the feature is **still
valid** and should be carried by this branch.

## Verdicts

| Feature | Ours | Upstream | Verdict |
| --- | --- | --- | --- |
| **Depth bounds** (`9a6d9e67`, 462 lines + a 304-line test) | absent (merge dropped it) | **absent**: no `SetDepthBounds`, no `DepthBounds` anywhere; upstream's README lists "depth bounds" among what is missing | **still valid, not yet re-applied** — needs re-deriving against upstream's rewritten `draw.rs`/`host.rs`/`list.rs`; the largest remaining item |
| **Host occlusion queries** (#1731) | absent | **stub only**: `list.rs:832` answers `Written::Value(1 << 20)` for every occlusion query — "everything visible" | **still valid and strictly better than upstream**: ours measured on the host. Held for a title run, because it changes what a culling title hides |
| **Shadow-buffer states** (#1691) | absent | absent | **still valid** |
| **1D textures as 2D views** (#1727) | absent | absent: upstream builds real `TEXTURE1D` views, so a title sampling a 1D texture with a 2D image instruction still hits the dimension mismatch (`VICTORY_MODE`-class defect: GBV id 940) | **still valid** |
| **Pipeline-library per-name locks / kind verdicts** (#1680, #1690) | absent | absent | **still valid** |
| **Stencil-plane / shader-view diagnostics** (fork `004bf69a`) | absent | absent | **still valid**, diagnostics only |
| **`VICTORY_SKIP_WAVE64`** | **re-applied here** | absent | was documented in README with no code: setting it did nothing |
| **`VICTORY_SHOW_TARGET`** | absent | absent | **still valid**; three commits over `capture.rs`, which upstream replaced |
| **`VICTORY_CAPTURE_DRAWS`** | absent | absent | **still valid**; same file |
| **`VICTORY_COPY_THREADS`** | absent | absent | **still valid**, rode with the threaded copy-back |
| **TFE via `CheckAccessFullyMapped`** (#1693) | absent | absent | **still valid**, shader lowering |
| **`v_interp_p2_f16` op_sel/omod/clamp** (#1696) | absent | absent | **still valid**, shader lowering |
| **Mesh line topology + 9-bit corner mask** (#1712, #1718) | absent | absent | **still valid**, shader lowering; must travel as a pair |
| **`image_gather4_c_lz`** (#1687) | present | present | **superseded**: upstream lowers it, including the LOD handling that made us reject #694 |
| **Placed copies** (#1725) | present | present | **superseded**: upstream's `placed_texture` + memory design |
| **`VICTORY_PIPELINE_WORKERS`** | re-applied earlier | absent | done |
| **`f32_to_half` NaN / ties-to-even** (#843) | re-applied earlier | absent | done |
| **Unique-temp library writes** (#1660) | re-applied earlier | absent | done |
| **`VICTORY_MODE`** | re-applied earlier | absent | done |

## What "still valid" means, and what it does not

It means upstream does not provide the behaviour, so having dropped ours is a
regression on this branch specifically. It does **not** mean each is cheap to
restore: the ones marked with a file that upstream rewrote need their change
re-derived against the new code, not cherry-picked.

The two README-documented but absent switches (`VICTORY_SHOW_TARGET`,
`VICTORY_CAPTURE_DRAWS`) remain a documentation defect until they are restored
or the README is corrected.

## Included tonight

`VICTORY_SKIP_WAVE64`: functional, not cosmetic — a console program written for
64-lane waves can hang a host with 32-lane waves. Restored by extending
`is_skipped` in `pipeline.rs`, so every draw and dispatch path that already
checks it picks the stage filter up for free, with a test for the stage parser.
