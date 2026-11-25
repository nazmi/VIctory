# The rest of ztrx: what each piece is, and why it is not in

Written 2026-11-25 against ztrx `90b529f8` (rechecked; unchanged) and our
`integration/maintenance`. This replaces guesswork with a per-piece answer:
`portable`, `already present`, `coupled`, or `blocked`.

## The size of it

ztrx differs from the shared base `ffe310e` by **93 files and ~36,000 added
lines**, 21,078 of them in `crates/victory-gpu` and 15,401 in
`crates/victory-shader`. It is not a set of patches to lift; it is a
parallel implementation of the GPU and shader layers. A declaration sweep
finds **575 functions, constants and statics that exist only there**.

## Console-instruction emulation: resolved, and smaller than it looked

The permission question is settled by the project owner (2026-11-25), so the
licence is no longer a blocker. But the port itself turned out to be almost
unnecessary:

**This tree already had its own implementation.** `emulate_amd_only` in
`crates/victory-loader/src/crash.rs` covers `clzero` (including the
`clzero`/`add`/`cmp`/`jb` loop, folded into one exception), `monitorx`,
`mwaitx`, `rdpru`, `clwb` (including a whole write-back loop) and SSE4a
`extrq`/`insertq` — written in-house, with no XCL provenance. A side-by-side
comparison of both decoders found **exactly one encoding ztrx handled and we
did not: `rdpid`**.

That one is now in (`90bd357d`): `F3 [rex] 0F C7 /7` reads the destination from
the ModRM, writes the host's processor number, and steps over the instruction.

| Instruction | Before | Now |
| --- | --- | --- |
| `clzero`, `clzero` loop | emulated | unchanged |
| `monitorx`, `mwaitx` | emulated | unchanged |
| `rdpru` | emulated | unchanged |
| `clwb`, `clwb` loop | emulated | unchanged |
| SSE4a `extrq`/`insertq` | emulated | unchanged |
| **`rdpid`** | **faulted** | **emulated** |
| `ud2` | crashes (correct) | unchanged: ztrx's `Ud2` case also returns continue-search |

So the largest "blocked" item in the earlier list cost one instruction and no
third-party code at all. The lesson is recorded rather than glossed: the 491-line
file was assumed to be a wholesale import need, and checking the two decoders
against each other took minutes.

## Already present — verified, not assumed

| Piece | Check |
| --- | --- |
| Texture copy runs with a compile-time length (`copy_runs::<8/16/32>`) | `crates/victory-formats/src/texture.rs:752-755` |
| BCPack dictionary entry with 0 index bits | `crates/victory-formats/src/bcpack.rs:234`, test at `:1635` |
| Host pipeline wait budgets (`VICTORY_PIPELINE_WAIT_MS`) | 8 tests in `pipeline::make` |
| `scripts/play.ps1` launcher | verified end to end |
| Stencil-plane diagnostics | `7180c20e` |
| Software-adapter recognition | `a8086480` |
| Spilled user-data table | upstream's `resource::USER_DATA` supersedes our `spill.rs`; measured 0→36 of 52 BVH-builder programs |

## Branding only — deliberately not imported

`The game is no longer named in the tree` (`4e17478b`) rewrites "Grand Theft
Auto VI" to "the title" across most of `victory-hle` — `dstorage.rs` (20 lines),
`gameinput.rs` (20), `xpackage.rs` (4), `lifecycle.rs` (4), `audio.rs` (6), and
a dozen one-line files. There is no behaviour in any of it. The one real change
in that set is a 6-line log line for audio allocations of 32 MiB or more, which
is diagnostics for an accounting problem rather than a fix.

## Coupled — cannot be lifted, each needs its own port

| Piece | Lines | Depends on |
| --- | --- | --- |
| Wave64-on-32-lane threading: `emit/threading.rs` (218), `emit/own_bits.rs` (914), the `wave.rs`/`lower.rs`/`structure.rs`/`ir.rs` rewrites | thousands | a different wave model. The earlier review already tried the port: its exact-source proof *failed* — the end-to-end witnesses already emit under our translator, so the port recovers no refusal |
| In-place title memory: `gpu/inplace.rs` (1170), `gpu/memory.rs` (319), `resource::MEMORY`, and the `xmem.rs` hooks (`title_memory_section`/`_view`/`_remapping`) | ~1,700 | each other. `xmem.rs` calls into them by name, so it moves only with them |
| Ray tracing: `shader/bvh.rs` (696, the CPU reference), `emit/intersect.rs` (557), `emit/intersect_tests.rs` (690), `resource::MEMORY` | ~2,000 | the in-place memory layer plus the emitter's image-op path, and it needs a GPU to verify |
| Memory instrumentation: `gpu/copystat.rs` additions (1159), `emit/bitcode/census.rs` (174) | ~1,300 | **`Tag::Guard/Clamp/Counter`**, which our `bitcode.rs` does not have. `census.rs` reads `self.tags`, and our `Op` enum has no tags at all — so the diagnostic moves only with the tagging that the emitter rework introduced |
| Bitcode optimiser: `emit/bitcode/opt.rs` (2136) | 2,136 | on by default from `Shader::finish`; on a probe branch it failed 2 of 233 tests (dxil.dll rejected one module, another shader's emitted IR changed). Held pending an off-by-default port and a title A/B |
| `shadow.rs` (2789), `host.rs` (4594), `draw.rs` (1573), `resource.rs` (1600), `descriptor/host.rs` (1546) | ~12,000 | these are the files upstream redesigned. Taking ztrx's versions would revert upstream's change journal, views-off-heap-slots, and page-table root views |

## Portable and taken

| Piece | Commit |
| --- | --- |
| `VICTORY_TRANSLATE_ONLY=<hash>,...` — narrows a sweep to named containers, using the `hash=` a run log prints | `8493fcbb` |

Verified end to end on `xboxbvhbuilder.dll`: 52 containers unfiltered, 1 with a
prefix, 1 with the same prefix upper-cased and padded, 0 with a non-matching
prefix.

## Portable and deliberately not taken

`VICTORY_DUMP_MATCH=<dir>;<text>|...` dumps the containers whose DXIL contains
given text. Useful, but it writes shader containers to a directory named by an
environment variable with **no outside-repository guard**, and the asset policy
requires one for game-derived output. Porting it means adding that guard first;
taking it as written would breach the policy.

## What this means for "work on the rest of ztrx"

There is no remainder that is both independent and valuable. Of 36,000 lines:

- **~490** console-instruction lines: **resolved** — this tree already had its
  own emulation, and the only real gap was `rdpid`, now added (`90bd357d`),
- **~200** are already present,
- **~200** are branding,
- **~2,000** are proven not to help (the threading port recovers no refusal),
- **~19,000** are coupled subsystems that each need their own design, port and
  correctness witness,
- **~2,100** (the optimiser) are held on evidence of breakage.

The honest next steps, in value order, are ports rather than imports: in-place
title memory (which unblocks ray tracing), then the instrumentation set, then
the optimiser behind an off-by-default switch with a title A/B. Each is a
project with its own tests, not a merge.
