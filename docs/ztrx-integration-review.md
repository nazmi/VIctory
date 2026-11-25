# Zettrox/ztrx integration review

## Source and scope

- Source: [Zettrox/ztrx](https://github.com/Zettrox/ztrx), pinned main
  `90b529f878a470fec9bf07b7be316bc769a12232` (rechecked unchanged before integration).
- Shared VIctory base: `ffe310eb2098e52169f4c44db5ffdabf43a72622`.
- Source: 13 commits after that base, 170 tracked source files. Its current
  tree contains no new binary beyond the existing BCPack-table exception.
- Our starting source comparison covered 204 changed paths. Many differences
  are newer local/upstream fixes and tests missing from the fork. A wholesale
  tree replacement would undo them and relax our unsafe-code lint policy.
- Current integration branch: `integration/maintenance`. Accepted changes use
  dedicated `--no-ff` merges. No upstream changes or push were made.

## Accepted and tested

| Feature | Candidate | Local two-parent merge | Review and evidence |
| --- | --- | --- | --- |
| Shared configurable pipeline waits | `5d3b49771edadcca8ad08c59eb036f50bba33adf` | `58e9182c4804555e1b46cf0c0b240b6eadaccf14` | Restore the documented `VICTORY_PIPELINE_WAIT_MS`; default500 ms, zero skips waiting. Reserve the allowance before waiting, refund unused time, reject stale-frame reservations. Existing queues, worker counts, shader creation and cache tracking retained. Source make tests3/0; candidate diagnostics125/0; reservation and stale-frame negative controls fail as intended. |
| Normal-play launcher | `1684dda175b67a3b139b9c5ce7ca1d80c4e2e231` | `7eedff93fc31cfd9cde521045703fa7bedf3c4f6` | Adapt `scripts/play.ps1` to VIctory's cache/data folders and native exit status. Clear inherited scripted-input/capture selections, retaining safety settings. A synthetic native executable verifies spaces in arguments/folders, environment, missing executable and child exit7. Source script returned0 for exit7; candidate and merged script correctly return7. |

The budget port originates in fxceless
`578d459c344aa44753e9a61ea6ba10a8c68f6ea6` and ZazaJr24
`d279e69ae999aedfd3d463b0c45c8fd125c1ad7b`. The launcher originates in
Zettrox's pinned head. Authors and source SHAs are retained in candidate
commit messages. Candidate `cbc3793d` was superseded by `5d3b4977` to correct
source attribution in its commit message; its code tree was unchanged and the
new SHA's combined integration was revalidated.

OCR delegate review: pipeline candidate1/1 files and launcher candidate1/1,
all candidate hunks reviewed, none skipped. No fork-wide review is claimed.
The remaining source was inventoried by features and dependencies.

## Preserved, duplicate or held work

| Area | Disposition |
| --- | --- |
| BCPack dictionary indexing, texture copy runs | Already present locally; source BCpack file differs only by lacking our new37-line malformed-control validation. Keep the local validation. `texture::copy_runs` and constant-width copies are present. |
| Scripted keys on a real gamepad | Current `gameinput::merge_scripted` already provides it; keep the local implementation and tests. |
| Prewarming requested-but-pending pipelines | Current `pipeline::make::ready` records first use before waiting, already matching the source's intent. Keep current queue/worker/cache design. |
| Stencil diagnostics, mode, per-name library locks | Local recoveries and tests remain; do not overwrite them with an older source tree. |
| Uniform-diamond shader threading | Exact source proof tests2/0 and initial port proof tests2/0. End-to-end descriptor/numeric and mask/numeric witnesses already emit under our current translator. Acceptance assertion correctly fails rather than pretending the port recovered a refusal. Unmerged scratch `integrate/ztrx-threading` is retained as evidence; no unproved fallback was enabled. |
| Parked masks / relative VGPR reads | Source modifies an `unspill` implementation absent from our current IR. Relative-read exemption relies on assumptions about the compiler's indexed range. Needs an independent semantic port and bounds proof, not a cherry-pick. |
| Wave64 local/approximate forms, ray tracing | Large coupled lowering/tracking/synchronization changes; require separate correctness witnesses and appropriate GPU checks. Keep current refusal behavior rather than enable approximate fallbacks without proof. |
| In-place memory, relay, late waits and wide copy-back | Coupled resource lifetime, remapping, fence and copy-back architecture. Not independently validated or merged. Our bounded submission waits and regression fixes remain. |
| Bitcode clean-up passes | Broad optimizer with memory/threading and cache-key implications; separate review and off-by-default experimentation required. Not merged. |
| Raw shader dumps/census switches | New source writes in `apps/victory/src/translate.rs` use environment destinations directly, without the runtime outside-repository guard. Do not import those write paths without preserving asset policy. |
| Branding, blanket lint relaxations and source README playability claims | Not imported. Our project identity, lint gates, toolchain pin, coverage and status remain grounded in this branch's evidence. |

## Combined validation

Both accepted candidates passed formatting, both workspace Clippy modes,
`cargo test --workspace --locked -- --test-threads=1` and asset policy before
their merge commits. Final source tree: **739 parent-harness tests passed,
0 failed, 11 existing ignores**. Candidate diagnostics GPU suite: **125 passed,
0 failed, 1 existing benchmark ignored**. Tests that return early for missing
external inputs or capabilities do not establish that coverage; WARP lacks
mesh shaders on this host.

Evidence outside the repository:

- `C:/Users/nazmi/victory-runs/ztrx-source-threading.log` and
  `ztrx-source-pipeline.log`: exact source focused checks.
- `ztrx-pipeline-tests.log`, `ztrx-pipeline-gpu-all.log`,
  `ztrx-pipeline-negative-reservation.log`, `ztrx-pipeline-negative-stale-frame.log`.
- `ztrx-launcher-test.json`: synthetic executable checks; no real title.
- `ztrx-integrated-wait-*` and `ztrx-integrated-launcher-*`: full combined gates.

No title was executed, and no gameplay, frame-rate, NVIDIA stability or
ray-tracing claim from the source README is established for our integration.
