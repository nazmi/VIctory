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
| wave7-hle | `a067e991` | HLE: the retail console ID has the documented dotted form | in branch |
| wave7-hle | `da106558` | DirectStorage: clear the factory out pointer on failure | in branch |
| wave7-hle | `df16f7d2` | report APU heap allocations that fail | in branch |
| wave7-hle | `df359c78` | DirectStorage: read a page list naming page 0 again | in branch |
| wave7-hle | — | `1729` stray/unreadable save folder | **refused**: it rejects `v0`, breaking `a_folder_named_like_a_version_does_not_remove_the_save`, and returns a readable older version where HEAD deliberately refuses an unreadable newest one — a silent rollback replacing a clean refusal. A policy flip, so it needs the two HEAD tests rewritten deliberately, not a cherry-pick. |
| wave7-shader | `08c99ea0` | Shaders: support line-topology mesh shaders (#1712) | in branch |
| wave7-shader | `574c7310` | Shaders: mask mesh vertex indices to 9 bits (#1718) | in branch — **must travel with #1712**: alone, an edge-flagged corner yields a host index >= 512 while `max_vertices` <= 256 |
| wave7-shader | — | `1732` f32_to_half NaN / nearest-even | **duplicate**: landed locally as `67b24154`; verified in `primitive.rs:126-152` and re-ran the PR's test bodies against the tree's function (`bad = 0`) |
| wave7-shader | — | `1733` helper lane reads its own value | **duplicate**: landed locally as `694c4944`; verified the select at `lower.rs:5387-5396` |
| wave7-shader | — | `1716` bitcode optimiser | **held**: unconditional from `Shader::finish`, and on a probe branch it fails 2 of 233 tests — dxil.dll rejects one module ("Flags must match usage") and another shader's emitted IR changes. Probe branch `scratch/1716-probe` is evidence only and must not be integrated. |
| wave7-gpu | `42ba51eb`, `ed7413fa` | GPU: placed copies take the bytes a GPU write leaves (#1725) | in branch |
| wave7-gpu | `508b6a7a` | style: format the placed-copy code | in branch |
| wave7-gpu | `6f2255bd` | GPU: 1D textures are 2D on the host, one texel high (#1727) | in branch |
| wave7-gpu | `3a26c01b` | GPU: occlusion queries measure on the host (#1731) | in branch |
| wave7-gpu | `e88c5752` | Pipelines: VICTORY_PIPELINE_WORKERS (#1728) | in branch |
| wave7-gpu | `ea8b7f31` | Shaders: wait on an event instead of spinning (#1734) | in branch |

## The integration gate: run this before every new wave

The rule, stated as a check rather than an intention:

> No new wave starts until every worker branch has been audited, and every
> branch reports **0 added lines absent from the integrated branch**, or the
> exceptions are named and justified in this file.

The audit compares the lines a worker *added* against the integrated branch's
copy of the same files. Patch-id comparisons (`git cherry`) are not enough on
their own: a cherry-pick after conflict resolution, or any reformatting, gives
the same content a different patch id, so `git cherry` reports `+` for work that
is in fact present. The test that survives reflow is line containment after
whitespace normalisation:

```bash
# agent-audit.py, kept outside the repo (game/maintenance tooling)
# for each codex/agent-* branch:
#   base = merge-base HEAD branch
#   for each file in `git diff --name-only base branch`:
#     for each line the branch added: is it in `git show HEAD:<file>` after
#     collapsing runs of whitespace?
```

### Result at 2026-11-24 21:54 UTC, over the six branches in question

| Branch | Added lines absent from the integrated branch |
| --- | --- |
| `codex/agent-fork-mid-c461d326` | 0 |
| `codex/agent-fork-small-1a4ffce1` | 0 |
| `codex/agent-verify-gpu-c7d011be` | 0 |
| `codex/agent-verify-hle-457cb189` | 0 |
| `codex/agent-verify-next-892d57be` | 0 |
| `codex/agent-verify-shader-0c8f256a` | 0 |
| `codex/agent-verify-shadow-0971b878` | 1, justified below |
| `codex/agent-wave7-gpu-19c5ba52` | 0 |
| `codex/agent-wave7-hle-1a8d6496` | 0 |
| `codex/agent-wave7-shader-7a64a47a` | 0 |

All ten worker branches present in this repository were audited, not only the
six that were in question; the four others (including all three wave-7
branches) also report 0.

The single flagged line is

```rust
handles.into_iter().map(|h| h.join().unwrap_or(0)).sum()
```

in `crates/victory-gpu/src/host.rs`. It is **absent on purpose**: the worker's
copy absorbed a panicking copy-back thread and carried on, which leaves the
submission with a partial write-back that the frame then continues past. It was
replaced with `std::panic::resume_unwind`, so the panic propagates as it did
when this ran on one thread. The reason is in the commit message of `e08c93e0`.
That is the only difference; the rest of the branch, including the `pooled()`
resize fix, the span write-back and the threading, is present.

### What went wrong before this rule

The wave7-gpu worker finished and reported, and its seven commits were verified
but not yet cherry-picked when the next question arrived. Every other branch
was already in. The cause was ordering, not lost work: the coordinator moved on
to reading the next result instead of integrating the finished one first. The
table above is the first time this was checked exhaustively rather than by
sampling, and the rule exists so it cannot recur silently.

## The audit method, and a real miss it caught

A worker branch never merges into the main branch by itself: workers commit on
their own branch, and the coordinator cherry-picks those commits onto
`candidate/wave6`. That is deliberate — a worker that merged would bypass the
coordinator's diff review.

Because a cherry-pick creates a **new SHA**, searching the main branch's log
for a worker's SHA always fails, and that is the trap: it looks like the work
was never taken. The correct tests are:

```bash
git cherry HEAD <worker-branch>   # '-' already applied by patch id, '+' not
git show <worker-commit> | git apply -R --check   # content present?
```

### The miss

Applying that audit on 2026-11-25 found that the **wave7-gpu worker's seven
commits were not in the branch** at the moment the question was asked — five
PRs (#1725, #1727, #1731, #1728, #1734) had been reviewed and verified but not
yet integrated. They are integrated now, and the gate was re-run after them.
The suspicion was correct, and the previous per-worker table would not have
shown it, because that table was written from worker summaries rather than
from the audit.

### Two branches the patch-id test flags but which are present anyway

`codex/agent-verify-hle-457cb189` and `codex/agent-verify-shadow-0971b878`
report `+` for some commits because those were carried across as **tree state**
after conflict resolution, or were reformatted, so the patch no longer applies
byte-identically. Both were checked by content instead:

```bash
grep -c 'piece.resize(PIECE, 0)' crates/victory-gpu/src/shadow.rs   # 1
grep -c 'copy_threads\|resume_unwind' crates/victory-gpu/src/host.rs  # 3
grep -n 'fn tbuffer_load_format_offen' crates/victory-shader/src/emit/valu_tests.rs  # present
grep -c 'buffer_states\|BufferState' crates/victory-gpu/src/host.rs  # 6
```

and by diffing HEAD against the branch: every remaining difference is newer
work in HEAD, not missing work.

Rule for the next session: after every worker, run `git cherry HEAD <branch>`
and require `+` to be empty, or explain each `+`. Do not infer integration from
a summary.

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
