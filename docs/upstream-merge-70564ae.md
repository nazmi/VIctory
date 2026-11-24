
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
