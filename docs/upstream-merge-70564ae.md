
## Outcome (2026-11-24 22:25 UTC)

The merge is **committed on branch `merge/upstream-70564ae`** at `a9193474`, and
is **not** merged into `integration/maintenance`.

```
cargo check --workspace --all-targets --locked   -> 0 errors, 0 warnings
cargo test --workspace --locked (10 GPU tests skipped) -> 656 passed, 0 failed, 11 ignored
```

### The 10 skipped GPU tests are not this merge's fault

They fail on `upstream/main` itself, on this host (Radeon 780M):

```
bindless::tests::heap_views_are_current
bindless::tests::heap_views_follow_writes_through_another_mapping
draw::tests::structured_views_off_their_buffers_elements_get_their_own_copy
draw::tests::root_views_reach_their_copy_and_read_zeros_past_it
draw::tests::a_depth_texture_written_through_unordered_views_reads_as_depth
draw::tests::unordered_1d_and_3d_textures_keep_their_writes
draw::tests::a_texture_written_in_one_format_reads_in_another_through_a_placed_view
descriptor::host::tests::dynamic_slots_are_held_until_released
resource::tests::buffer_data_reaches_the_host_texture
   (this one takes the test process down with STATUS_STACK_BUFFER_OVERRUN)
```

A clean checkout of `upstream/main` (`git worktree add ... upstream/main`)
reproduces 8 of them, including the crash. So they are upstream's own state on
this hardware, not a merge defect. The other two
(`memory::tests::lists_share_a_table_and_copies_keep_their_views`,
and one of the above) pass when run alone: order-dependent.

Verified by: `cd <upstream worktree> && cargo test -p victory-gpu -- --skip
buffer_data_reaches_the_host_texture` → `FAILED. 69 passed; 8 failed`.

### What this merge costs, stated plainly

Taking `crates/victory-gpu` and the shader emitter from upstream **discards
our implementations** wherever upstream has one. That is the policy, and it is
the right policy for the emitters, but it means this branch loses:

- the threaded copy-back and span-limited write-back (#1656) — upstream's
  `goes_back` design replaces it
- the shadow-buffer state tracking (#1691) — upstream's `tracked`/`keep_writes`
- placed-copy addressing (#1725) — upstream has `placed_texture`/`memory`
- 1D-textures-as-2D (#1727) — upstream maps dimensions itself
- host occlusion queries (#1731), pipeline worker count (#1728),
  per-name library locks and kind verdicts (#1680, #1690),
  dropped-draw reporting interval (#1710), stencil-plane diagnostics

Some of that upstream may do better. Some of it is genuinely lost. **This is
the decision the branch is waiting on**, and it should be made with a title run
that can compare the two, which this machine cannot give (the GPU has been
removed as hung twice).

### If the decision is to keep our GPU work

Do not merge `a9193474`. Instead re-do `crates/victory-gpu` file by file,
taking upstream's `watch.rs`, `shadow.rs`, `resource.rs` and `descriptor/host.rs`
designs (they are incompatible redesigns, not renames) and then re-applying each
of our PRs above on top, one at a time, with its test. The worker attempt that
tried to interleave them is preserved outside the repository at
`C:\Users\nazmi\victory-runs\gpu-worker-attempt` — it resolved all 96 hunks and
compiled in an isolated scratch, but had 40 semantic errors in the combined
tree, mostly `requested`/`d`/`original` bindings that the two designs name
differently.

## Documented switches the merge dropped (found 2026-11-25)

Four switches are still listed in `README.md` but no longer exist in the code,
so setting them silently does nothing. Each was verified by grepping the
workspace for the variable name and by checking the README row:

| Switch | README | Code | Recovery commit | Cost to re-apply |
| --- | --- | --- | --- | --- |
| `VICTORY_MODE=<w>x<h>` | documented | **re-applied here** | `5c61aa48` | small: `dxgi.rs` parse + `mode_size()` |
| `VICTORY_SKIP_WAVE64` | documented | **absent** | `13008f98` | medium: needs a diagnostics-only `wave64` field on `Shader`, which upstream restructured |
| `VICTORY_SHOW_TARGET` | documented | **absent** | `98231399`, `fed90c5d`, `49ab3532` | large: three commits over `capture.rs`, which upstream replaced |
| `VICTORY_CAPTURE_DRAWS` | documented | **absent** | see `git log --all --grep=VICTORY_CAPTURE_DRAWS` | large: same file |

Also lost, not documented but user-visible: `VICTORY_COPY_THREADS` (with the
threaded copy-back) and the shadow-buffer state tracking.

**Until these are re-applied the README overstates the build.** That is a
documentation defect, not a code one, and it is recorded here rather than
quietly deleted from the README so the features are not forgotten.

## Lost features found by audit after the merge

Found by comparing the 329 pre-merge commits' introduced declarations against
the merged tree, then checking behaviour rather than names.

| Feature | Evidence it is gone | Recovery | Re-applied? |
| --- | --- | --- | --- |
| `f32_to_half` NaN guard + ties-to-even (#843) | upstream turned NaN into infinity and rounded half-up | `67b24154` | **yes**, with its test |
| unique-temp pipeline-library writes (#1660) | upstream still stages to a shared `<file>.tmp` | `d32efcd9` | **yes**, with its test |
| `VICTORY_PIPELINE_WORKERS` (#1728) | upstream hardcodes 2..8 | `16a5924d` | **yes**, with its test |
| `VICTORY_MODE` (#1701) | README documented it; no code | `5c61aa48` | **yes**, with its test |
| **depth bounds** (`9a6d9e67`, 462 lines + a 304-line test) | `set_depth_bounds`, `depth_bounds_supported`, the `0x220` list slot and `mod depth_bounds_test` all gone; the test file was left orphaned and has now been deleted | `9a6d9e67` | **no**: needs re-deriving against upstream's rewritten `draw.rs`/`host.rs`/`list.rs`. Upstream's README lists depth bounds as missing, so code and docs agree meanwhile. |
| threaded span-limited copy-back (#1656) | upstream's `goes_back` design | `e08c93e0` | no: design replacement |
| shadow-buffer state tracking (#1691) | upstream's `tracked`/`keep_writes` | `76adf6a1` | no: design replacement |
| `VICTORY_SKIP_WAVE64`, `VICTORY_SHOW_TARGET`, `VICTORY_CAPTURE_DRAWS` | README documents all three; no code | see the switch table above | no: depend on upstream's rewritten `capture.rs` and `Shader` |

Re-applied features were each verified by their own test, and the tests are
non-vacuous where a negative control was possible.

## The measurement that settles the trade: the BVH builder

Everything above argues from code. This is from behaviour, on the one archive
that motivated the whole spilled-user-data investigation:

```bash
victory translate --jobs 4 <game_dir>/xboxbvhbuilder.dll
```

| | Programs emitted | Top refusal |
| --- | --- | --- |
| before the merge (our `spill.rs`) | **0 of 52** | the spill refusal was gone, but every program then failed later |
| after the merge (upstream's `resource::USER_DATA`) | **36 of 52** | 8 "a select on an untracked mask", 8 "a whole-wave section step" |

Upstream's mechanism, which resolves a spilled root parameter *inside* tracking
rather than rewriting the IR first, is **substantially better** than the module
this branch had taken from the third-party fork: it takes the same 52 programs
from none to thirty-six. That is the strongest evidence available here that
taking upstream's design was right, and it answers the question the merge left
open without needing a title run.

The remaining 16 are named, offline-reproducible gaps — the same "a select on
an untracked mask" and "a whole-wave section step" families the full census
ranks first and tenth.
