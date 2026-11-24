# What every worker produced, and where it went

Why this file exists: subagent work that is silently dropped is
indistinguishable from work that was never done. Every commit a worker created
is listed here with its fate, and each `aborted` entry says what was missing
from *this* tree rather than "skipped".

Rule the workers run under: a worker reviews the **whole diff** — every file,
every hunk — not the first one that explains the title. A PR is `accept` only
if the reviewer read all of it; a conflict is resolved semantically and both
sides keep their behaviour, never "ours dropped to make it apply".

| Worker | Commit | Subject | Fate |
| --- | --- | --- | --- |
| verify-gpu | `263bb7c0` | Shader tests: require a device readback in every waterfall test | in branch |
| verify-gpu | `9d8a41e7` | GPU: pipeline library loads take a lock for their name | in branch |
| verify-gpu | `263cc4e8` | GPU: keep dropped draws in the full frame reporting interval | in branch |
| verify-gpu | `bf465f04` | GPU: zero the padding of the descriptions the library compares | in branch |
| verify-gpu | `924c3374` | GPU: stop keeping a kind of pipeline the library refuses | in branch |
| verify-next | `9c40c87a` | Shaders: lower image_gather4_c_lz(_o) and offset variants | in branch |
| verify-next | `2ab062db` | Shaders: TFE via CheckAccessFullyMapped | in branch |
| verify-next | `d2c125d9` | HLE: XTaskQueueTerminate scopes to its own tasks | in branch |
| verify-hle | `2b3b64a1` | Shaders, CLI: name a mesh-less host in the census | in branch |
| verify-hle | `50256643` | Shaders: straddling 16-bit loads at offset 3 | in branch |
| verify-hle | `ca8e11ea` | rustfmt of the above's tests | content applied (same edit) |
| verify-hle | `ca82cd6e` | GPU: track shadow-buffer states | in branch |
| verify-hle | `25d8cbde` | rustfmt of the above | in branch |
| verify-hle | `a28134d2` | HLE: title windows through a read-always stand-in | in branch |
| fork-mid | `b57a6a51` | Stencil diagnostics, stability warning | in branch |
| fork-mid | — | bitcode optimiser (`16849898`) | **held**: on by default, invalidates the whole shader cache, rests on a threading assumption; needs an off-by-default port |
| fork-small | `a8086480` | Shader tests: recognise the software adapter | in branch |
| fork-small | `3ba1a124` | Pipelines: configurable wait, prewarm pending | in branch |
| fork-small | — | `90228a7d` parked lane-mask pairs | **aborted**: patches `unspill`/`set_host_max_wave`, which exist only in the fork's squash; this tree handles parked masks in `lower.rs`/`structure.rs` already |
| fork-small | — | `39309893` mesh pipeline test fix | **aborted**: the test and helpers it patches were removed by `9a6d9e67`; mesh coverage now lives in `depth_bounds_test.rs` |
| fork-small | — | `e9765a73` device loss / wave64 | **aborted**: flips `wide_on`, a switch this tree never had |
| verify-shadow | `255a2ce9`..`a991434f` | copy-back dedupe, span write-back, pooled piece, threads | content in branch as one commit `e08c93e0`; taken as tree state because it was resolved against our base, plus two documented edits (panic propagation, a false comment) |
| verify-shader | `d4dc96dd` | Shaders: v_mad_u32_u16, v_mad_i32_i16, v_lerp_u8 | in branch |
| verify-shader | `ec2aa220` | Shaders: v_med3 NaN from the bits | in branch |
| verify-shader | `8516122d` | Shaders: op_sel/omod/clamp on v_interp_p2_f16 | in branch |
| verify-shader | — | `1699` v_add_nc_i16 / v_mad_i16 clamp | **refused**: reads src2 with `half_src` where src0/src1 use `half_int_src`; an inline float there is read as packed f16. Probe on this GPU: 15366 where hardware gives 0. |

## Verification of this table

Every "in branch" row was checked by looking for the worker's upstream commit
hash in the local commit trailer (`git log --format=%B | grep <sha>`), not by
trusting the worker's summary. The two "content applied" rows were checked by
grep for the specific lines the worker produced, because their commits were
squashed or re-applied:

```bash
grep -c 'piece.resize(PIECE, 0)' crates/victory-gpu/src/shadow.rs   # 1
grep -c 'copy_threads'           crates/victory-gpu/src/host.rs     # 2
grep -c 'resume_unwind'          crates/victory-gpu/src/host.rs     # 1
rustfmt --edition 2024 --check   crates/victory-shader/src/emit/valu_tests.rs  # clean
```

## What "complete review" means for a worker here

1. Read the whole diff: `git diff <merge-base> refs/remotes/pr/<n>`, every file.
2. For each changed file, read the surrounding code it patches, not just the hunk.
3. Test the crate, and say plainly when a test **skipped** for lack of a device.
   A skip is not coverage and must not be reported as a pass.
4. Look for the failure mode that matters: a lowering that silently computes a
   wrong value is worse than a refusal, and a validation that silently drops
   user data is worse than an error.
5. Report the exact `test result:` line, the head SHA reviewed, and anything
   not verified.
