# Maintenance log

## Bootstrap: 2026-11-24

- Scope of this run: create maintenance instructions and tracking templates.
  The three-pass maintenance sweep has not started.
- Upstream: <https://github.com/Marvisak/VIctory>, default branch `main`.
- Local branch inspected: `main`.
- Local HEAD at inspection: `ffe310eb2098e52169f4c44db5ffdabf43a72622`.
- `origin` points to upstream; no separate user fork remote was observed.
- Existing user change: `rust-toolchain.toml` pins
  `1.98.1-x86_64-pc-windows-msvc`. Preserve it separately from future fixes.
- Upstream's repository page currently says only collaborators can create
  PRs. Recheck before any future submission.
- Initial connector searches returned 20 PRs and 20 issues. These were
  partial discovery results, not a complete inventory or code review.
- Useful triage lead: [issue #813](https://github.com/Marvisak/VIctory/issues/813)
  lists potential duplicate/overlapping PRs. Verify each claim independently.
- No PRs have been integrated, no issues have been resolved, and no future
  fix commits have been created by this documentation task.

## Plan for the first authorized sweep

### Sweep started: 2026-11-24

- User authorized the three-pass sweep and economical workers for small work.
- Created isolated worktree `../VIctory-integration` on
  `integration/maintenance`, based on upstream `ffe310e`.
- Setup commits: `781f328` (workflow/ledger) and `bfea43d` (MSVC pin).
  The original working copy's modifications remain untouched.
- Economical workers: `inventory` and `baseline` use `gpt-6.1-sol` at low
  reasoning; `review_708` uses the same model at medium reasoning.
- Complete REST snapshot: 445 open PRs, 5 pages; 189 open issues after
  filtering PRs from 634 issue-endpoint entries across 7 pages. Retrieved
  2026-11-24 11:51:11-11:51:36 UTC. Raw metadata stays outside the repository.
- Baseline at `bfea43d`: both CI Clippy commands passed; workspace tests
  failed at the frame-wait consecutive-tick assertion (ticks 5, 7, 8, 9).
  Observed 86 passed, 1 failed, 5 ignored before Cargo stopped. Later crates
  did not run. Policy passed; formatting has 182 existing differing hunks
  across 35 files. No broad formatting changes applied.
- PR #689 is under review to address that reproduced test failure. A local
  candidate `candidate/pr-689` at `f137b30` is not yet accepted or integrated.
- PR #708 passed independent full-file review (1/1 files, none skipped),
  targeted core tests (15 passed), and targeted Clippy. Full integration
  checks are pending. PRs #449 and #480 passed read-only review (1/1 each),
  but their local tests are pending. No upstream PR integrated yet.
- Before further integration, user requested a GitHub fork. Created and
  verified <https://github.com/nazmi/VIctory>, parent `Marvisak/VIctory`.
  Local `origin` now points to the fork; `upstream` points to Marvisak.
  Default pushes target `origin`. No branches have been pushed yet.

1. Create an isolated local integration worktree, record upstream/local bases,
   verify MSVC, and run baseline checks.
2. Inventory every open PR with complete pagination; map dependencies and
   overlaps. Review/test candidates and integrate accepted changes locally.
3. Refresh all open issues; reproduce and resolve those still applicable.
4. Audit remaining gaps; prepare one unique commit per future PR branch.
5. Validate the combined result and hand off the ledger and submission queue.

Record actual models and worktree ownership in each active plan. Start with
bounded economical workers; keep integration decisions with the coordinator.

## Inventory checkpoints

| Pass | Retrieved at | Query/source | Upstream base | Pagination complete? | Total inventoried | Next item |
| --- | --- | --- | --- | --- | --- | --- |
| PRs | 2026-11-24 11:51 UTC | REST pulls, 5 pages; see maintenance-pr-inventory.md | ffe310e | Yes, snapshot | 445 | Continue candidate review/integration |
| Issues | 2026-11-24 11:51 UTC | REST issues, 7 pages; see maintenance-issue-inventory.md | ffe310e | Yes, snapshot | 189 | Refresh and work after PR pass |
| Gaps | Not started | Local audit | Pending | Not applicable | Unknown | After issue pass |

## PR decisions and integrations

Every item in the complete snapshot is tracked in
[maintenance-pr-inventory.md](maintenance-pr-inventory.md). Dependency and
overlap leads are in [maintenance-dependencies.md](maintenance-dependencies.md).
Rows below record completed decisions; unreviewed items remain queued.

| PR/link | Source head SHA | Dependencies/overlap | Review evidence | Tests/evidence | Disposition/reason | Local integration SHA | Date |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [#689](https://github.com/Marvisak/VIctory/pull/689) | e1c3557bdc67f31e13d04a00d2d9ad1ebc65c7e3 | Standalone test correction | Coordinator and economical worker: 1/1 file reviewed, no skips/findings; clock permits skipped ticks | 5 targeted repeats; both Clippy gates; full workspace 342 passes, 6 ignored; policy, all exit 0 | integrated-local; reproduced baseline failure fixed without changing production code | f137b30 | 2026-11-24 |
| [#699](https://github.com/Marvisak/VIctory/pull/699) | db0257e2c133ccdcb4605f0a90bcf03cb1847cad | Superseded by merged #612 | Compared tip algorithm and synthetic two-join test to upstream; 3dd467c is ancestor of ffe310e, and same regression exists at lower.rs:7301 | Existing regression included in passing 342-test workspace run | already-present; no import of mixed branch or duplicate fix | Upstream 3dd467c | 2026-11-24 |
| [#708](https://github.com/Marvisak/VIctory/pull/708) | a9698e7488ab2a7d1bdbf0e6b736ab689b6b2ca2 | Narrow save/cache default fix; differs from JMAN's broader config rewrite | Coordinator plus medium worker, 1/1 file reviewed, no skips/blockers | Source core 15 passes and Clippy; integrated both Clippy gates, workspace 346 passes/6 ignored, policy all exit 0 | integrated-local; no automatic migration of old XDG saves; VICTORY_DATA_DIR retains old data location with this change | 79381585 | 2026-11-24 |
| [#449](https://github.com/Marvisak/VIctory/pull/449) | f625cc508c0d3a1b3f124cd3714dde681440d8c4 | Standalone null-output guard for AnalyticsInfo | Coordinator and economical worker reviewed 1/1 file, no skips/findings; ABI unchanged | Source xsystem tests 4/4; candidate HLE tests 94/94; both workspace Clippy gates, workspace 346 passes/6 ignored, policy 161 files, diff check all exit 0 on fresh candidate target | integrated-local; null output is refused before writing | c64f8aab | 2026-11-24 |
| [#480](https://github.com/Marvisak/VIctory/pull/480) | 75fe6c02b278f18eab33ad893bda1dd68d176287 | Standalone decoder fix; its patch also occurs in JMAN's history | Coordinator reviewed 1/1 file and the decoder's callers; no ABI change; source-head media run 3/3 | Candidate media tests 3/3; both workspace Clippy gates, workspace 347 passes/6 ignored, policy 161 files, diff check all exit 0 on fresh candidate target | integrated-local; malformed escapes are copied verbatim | 06daf0f7 | 2026-11-24 |
| [#396](https://github.com/Marvisak/VIctory/pull/396) | 10e9f56a50214df67dca98b19daac0d8e9b04075 | Standalone PE bounds fix; not in JMAN's fork | Coordinator reviewed 1/1 file, PR discussion, and every `data_directory_offset` caller in `prepare.rs`/`image.rs`; malformed headers now yield `None` instead of an out-of-bounds offset | Candidate PE tests 8/8 and loader tests 25/25 (22 unit + `load_probe` + `title_dlls`); both workspace Clippy gates, workspace 348 passes/6 ignored, policy 161 files, diff check all exit 0 on fresh candidate target | integrated-local; malformed/truncated optional header is refused before any offset is returned | 27dd4a73 | 2026-11-24 |
| [#629](https://github.com/Marvisak/VIctory/pull/629) | 350f91c23a9402b2a556a84c208a80030910ae3f | Standalone format validation; cites AMD's addrlib | Coordinator reviewed 1/1 file, PR body and the `Layout::new` validation chain; `Layout::new` is the only entry point and no caller depends on linear MSAA | Candidate formats tests 24/24 + 4 ignored; both workspace Clippy gates, workspace 349 passes/6 ignored, policy 161 files, diff check all exit 0 on fresh candidate target | integrated-local; multisampled linear descriptors are refused before a layout is built | 25e797e9 | 2026-11-24 |
| [#363](https://github.com/Marvisak/VIctory/pull/363) | 4874c3fdc18e01d0a1973fc62558669ed9e18ead | Conflicts with tests added after its base; overlaps #241's config rework | Coordinator reviewed 1/1 file and resolved the test-only conflict by keeping both sides; author preserved | Candidate core tests 16/16; both workspace Clippy gates, workspace 349 passes/6 ignored, policy 161 files, diff check all exit 0 on fresh candidate target | integrated-local; empty `VICTORY_CONFIG` counts as unset | 4dbc5a71 | 2026-11-24 |
| [#337](https://github.com/Marvisak/VIctory/pull/337) | 5cf259e5fe2f9361fffbc00e9b5bd46067b6686d | Four commits squashed to their reviewed net patch | Coordinator reviewed the net diff of `Title::from_manifest`; storage quota now saturates instead of wrapping | Candidate HLE tests 97/97; both workspace Clippy gates, workspace 350 passes/6 ignored, policy 161 files, diff check all exit 0 on fresh candidate target | integrated-local; quota parse type is explicit `u64` | feeff994 | 2026-11-24 |

## Issue outcomes

No entries yet. Distinguish `resolved-local` from upstream closure.

| Issue/link | Reproduction and acceptance criteria | Related PR/duplicate | Branch/fix SHA | Tests/evidence | Disposition | Next action |
| --- | --- | --- | --- | --- | --- | --- |

## Confirmed gaps

No entries yet. Give each independently actionable gap a stable identifier.

| ID | Evidence/location | Impact/priority | Dependencies | Acceptance criteria | Owner/model | Status |
| --- | --- | --- | --- | --- | --- | --- |

## Future PR queue

No reserved commits yet. Each ready entry must identify exactly one unique
commit relative to its recorded base; document stacked prerequisites.

| Item | Branch | Base SHA | Commit SHA | Unique commits | Dependencies | Target issue(s) | Validation | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |

For each ready entry, append a handoff with its suggested PR title, body,
closure line if justified, reproduction, before/after behavior, checks,
limitations, and submission order. No upstream PR number exists until one
has actually been created.

## Validation runs and handoffs

Record date, exact code SHA or dirty-tree scope, command, exit code, relevant
counts/skips, adapter/environment when relevant, and sanitized evidence.
Do not put proprietary logs or game data in this file. Tests on an earlier
SHA do not validate later code changes.

### Documentation creation: 2026-11-24

- Files created: `AGENTS.md`, `docs/maintenance-log.md`.
- Delegated read-only repository validation research to `workflow_checks`,
  using `gpt-6.1-sol` at low reasoning. It inspected CI, xtask, architecture,
  asset policy, and testing constraints; no worker edits were made.
- `cargo xtask policy`: exit 0; 156 files accepted.
- `git diff --check`: exit 0. Newly created Markdown files were also checked
  explicitly for trailing whitespace and conflict markers.
- Code test suites were not rerun for this documentation-only change.
- No commits, merges, pushes, issue closures, or PR submissions performed.
- Next action for a maintenance sweep: create an isolated integration
  worktree and complete the baseline plus fully paginated inventories.

### PR #689 integration validation: 2026-11-24

- Candidate and accepted code SHA: `f137b30`; parent `bfea43d`.
- Original author retained; commit includes cherry-pick source attribution.
- Worker `baseline`, `gpt-6.1-sol`, low reasoning: independent frame test
  review and 5 targeted repetitions all passed. Coordinator reviewed the
  full changed file and production clock semantics as well.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: exit 0.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`:
  exit 0.
- `cargo test --workspace --locked`: exit 0; 342 reported passes, 6 ignored.
- `cargo xtask policy`: exit 0; 156 files accepted.
- Coverage limit: four differential shader tests returned early because
  console/PC compilers were not configured. Harness passes include these
  unavailable checks. The validator constant-color test did execute.
- WARP environment caveat: `VICTORY_D3D12_WARP=1` selects WARP only with
  diagnostics. Default synthetic GPU tests may use the physical adapter.
  No actual title/world execution was performed.
- Known pre-existing formatting drift remains; no unrelated reformatting.
- Full local logs are outside the repository under the maintenance-data
  `candidate-689` directory. Source head rechecked before integration.

### PR #708 integration validation: 2026-11-24

- Accepted code SHA: `79381585d7c5ec37409cb76aa6b1ae72eed6a35e`.
- Source reviewed/tested at `a9698e7`; original author and source attribution
  retained. Exact current source head rechecked before fast-forwarding.
- Both required workspace Clippy commands: exit 0.
- `cargo test --workspace --locked`: exit 0; 346 harness passes, 6 ignored.
- `cargo xtask policy`: exit 0; 159 files accepted in candidate scope.
- Same unavailable compiler/reference coverage as the preceding validation.
- External full logs: maintenance-data `candidate-708` directory.

### JMAN730 cleanup-fork assessment: 2026-11-24

- User requested duplicate checking and conditional integration only after
  verification. Pinned source: `JMAN730/victory` main at
  `179d34e2ba0de1357d040a8e996e6787148a1b04`.
- Compared with upstream `ffe310e`: 275 additional commits, 184 changed
  files, 34,578 insertions and 2,414 deletions. This is a broad behavioral
  integration, not just formatting cleanup.
- Full historical patch analysis of all 445 inventoried PRs: 103 fully
  represented in nonmerge patch history, 20 partially represented, 320 with
  no matching individual patch, 2 with changed heads. These are duplication
  leads, not acceptance or resulting-behavior proof. See
  [maintenance-jman-duplicates.md](maintenance-jman-duplicates.md).
- Source #449 and #480 have exact patch-history matches in the fork; #363's
  empty-config handling and #406's incomplete-relocation handling are also
  recognizable by inspection. #708 overlaps but its fallback paths differ.
  #396's optional-header bounds check is absent. Do not mark all of these
  PRs integrated just because they occur in the fork's history.
- Economical worker tested clean exact fork source on MSVC with a fresh
  external target: default Clippy, all-features Clippy, workspace tests,
  policy, formatting, and release build all exit 0. Workspace reported
  794 harness passes and 18 ignored; four compiler-dependent differential
  tests returned early. No live title execution or playability claim.
- Additional conditional build check FAILED: with a zero-byte synthetic
  D3D12Core.dll placeholder in a separate outside target's debug/D3D12,
  `cargo test -p victory-shader --test resource_tracking --no-run --locked`
  exits 101 with LNK2001 for D3D12SDKPath and D3D12SDKVersion, then LNK1120.
  No placeholder DLL or probe executable was run. The build script exports
  symbols for external tests that the library defines only under cfg(test).
- Review also identifies capped sampler-table remapping using view-sized
  (32-byte) offsets instead of sampler-sized (16-byte) offsets, plus a
  changed XDG cache root/legacy-save compatibility concern and title-specific
  watchdog behavior placed in the generic loader. Details and reproduction
  status belong in the assessment report.
- Core/PE/loader review covered 24/24 assigned files, including excluded
  tests. GPU/shader review found blockers but does not establish a completed
  184-file review. Other fork changes remain unaccepted.
- Disposition: defer wholesale fork integration; preserve existing tested
  integrations and continue with bounded, independently verified changes.
  No JMAN source commit has been merged into the user's integration branch.
- Follow-up WARP regression reproduced the capped sampler error through
  real State::tables: 1 executed test fails, exit 101; actual remap 57344,
  expected 28672. Host initialization succeeded; no title inputs or command
  list submissions. Test exists only in a separate scratch worktree.
- Full assessment, both reproduced blockers, normal check results, coverage,
  and acceptance conditions: [maintenance-jman-review.md](maintenance-jman-review.md).
- Review tracking models: inventory/patch mapping and all verification used
  economical gpt-6.1-sol at low reasoning; focused core/loader review used
  that model at medium reasoning. Complex GPU/shader review was escalated
  to gpt-6.2-sol at high reasoning. Coordinator independently checked the
  relevant code paths and directed deterministic reproductions.
- Validation correction: reusing Cargo artifacts between integration
  worktrees left xtask checking an earlier checkout (159 files instead of
  the current 161). A separate PR #480 source test also initially reused a
  binary missing its new regression; that result was invalidated and the
  package was rebuilt, executing all three intended tests. Fork-source and
  Agility checks used fresh distinct targets, so their results stand.
  Workflow now requires one Cargo target per worktree. A final fresh-target
  integration gate will replace any ambiguous shared-target evidence.

### Wave 2 integrations: 2026-11-24 (21 PRs)

Same procedure as wave 1. Two integration lessons this wave, both caught by the
gate rather than by review:

- A PR's branch may be **stacked** on another unmerged PR. #695's "net diff"
  was 36 files because it sits on the workspace-wide `cargo fmt` change; only
  its own commit was integrated. Check `git log --oneline <ref> -3` before
  trusting a net diff.
- Hand-merging test modules produces **duplicate `mod tests`** and stray
  braces. #616 added a second `mod tests` to `draw.rs`; the merged tests were
  folded into the existing module. #651 left an extra brace.

Also fixed in this wave, not from a PR:
- `dstorage`'s lane-counting test was flaky: twelve other tests create queues
  through the same process-wide scheduler. They now hold a module lock (as
  `audio` does). Five consecutive crate runs are clean.
- The emit-dump policy test now passes the argument `translate::run` gained
  from #672.

Gate at `3ad7f8ef`: both Clippy commands exit 0; workspace 546 passed,
11 ignored, 0 failed; policy 175 files; `git diff --check` clean.
Formatting: 35 files carry pre-existing drift from the baseline. All 70 Rust
files touched by this branch are rustfmt-clean.

| PR | Subject | Local integration |
| --- | --- | --- |
| [#615](https://github.com/Marvisak/VIctory/pull/615) | `GPU: VICTORY_* switches set to 0 are off` | `bf57211f` |
| [#616](https://github.com/Marvisak/VIctory/pull/616) | `GPU: rectangle draws check their bounds without 32-bit overf` | `1af23c07` |
| [#617](https://github.com/Marvisak/VIctory/pull/617) | `Shaders: reuse lane guards for unchanged EXEC within a block` | `8042c6b7` |
| [#618](https://github.com/Marvisak/VIctory/pull/618) | `GPU: align appended input-element offsets like D3D12` | `9c9e46f6` |
| [#631](https://github.com/Marvisak/VIctory/pull/631) | `Add tests for Console struct functionality` | `a157f3fe` |
| [#640](https://github.com/Marvisak/VIctory/pull/640) | `Retry failed pipeline cache saves` | `842ed08b` |
| [#641](https://github.com/Marvisak/VIctory/pull/641) | `GPU: upload partially-resident streamed textures` | `15907832` |
| [#642](https://github.com/Marvisak/VIctory/pull/642) | `Test: use exact UNORM clear values in enhanced barrier test` | `f5f97470` |
| [#643](https://github.com/Marvisak/VIctory/pull/643) | `Test: use exact UNORM clear values in aliased texture test` | `d1ba3944` |
| [#649](https://github.com/Marvisak/VIctory/pull/649) | `Show on-disk file size in victory inspect` | `d0952126` |
| [#651](https://github.com/Marvisak/VIctory/pull/651) | `GPU: select the readable format and plane for depth/stencil ` | `ce478cfb` |
| [#652](https://github.com/Marvisak/VIctory/pull/652) | `GPU: use selected mip extents for MSAA resolves` | `6a836d63` |
| [#659](https://github.com/Marvisak/VIctory/pull/659) | `Shaders: refuse per-lane reads of mesh v5 in v5 writes and a` | `2ece2f86` |
| [#664](https://github.com/Marvisak/VIctory/pull/664) | `translate: refuse a VICTORY_EMIT_DUMP directory inside the r` | `d350b132` |
| [#665](https://github.com/Marvisak/VIctory/pull/665) | `xtask: count \x escapes and 0x hex arrays as encoded runs in` | `7fc9f665` |
| [#668](https://github.com/Marvisak/VIctory/pull/668) | `shader: support f16 omod and saturate/apply modifiers in v_c` | `3ad7f8ef` |
| [#669](https://github.com/Marvisak/VIctory/pull/669) | `GPU: match a breadcrumb node's name followed by the debug la` | `f6aa6695` |
| [#672](https://github.com/Marvisak/VIctory/pull/672) | `Merge main after the descriptor-tracking prerequisite landed` | `b746d051` |
| [#673](https://github.com/Marvisak/VIctory/pull/673) | `Make translate failure examples stable across workers` | `9a6bc8e0` |
| [#677](https://github.com/Marvisak/VIctory/pull/677) | `Build: thin LTO for release and diag builds` | `de6e76fc` |
| [#695](https://github.com/Marvisak/VIctory/pull/695) | `gpu: VICTORY_D3D12_WARP=0 opts out of the software adapter` | `958693cf` |

Deferred to the hard pile (contested regions, not attempted):
- #644 changes `stage.user_data` from a tuple to a `UserDataBind` enum, in
  `draw.rs`/`pipeline/host.rs`, the same files several pending PRs rewrite.
- #645 traces multi-term/scaled/phi-joined addresses and collides with the SSA
  sum cache from #139 across three files (18 conflict hunks).
- #667 spans nine files and depends on the descriptor-tracking shape #672
  changed.
- #703 is a 1,000-line BCPack restructure competing with the merged bucket
  lookup; its decode equivalence cannot be checked without real BC7 data.

### Wave 1 integrations: 2026-11-24 (35 PRs)

Complete open-PR inventory re-run before this wave: 444 open PRs (13 drafts)
across 5 REST pages, retrieved 2026-11-24; upstream `main` unchanged at
`ffe310e`, so no PR is stale at its base. Every head fetched with
`git fetch upstream '+refs/pull/*/head:refs/remotes/pr/*'` (612 refs).
Trial merges with `git merge-tree --write-tree`: 328 apply cleanly, 116
conflict.

Each integration applies the PR's **net diff against its merge-base**
(`git apply --3way`), runs `cargo test -p <affected crates> --locked` and
`cargo xtask policy`, and commits with the original author plus a `-x`
source trailer. Multi-commit PRs record every source SHA in the message.
Four PRs needed hand merges; each is noted below.

Wave gate at `aa370a59`: both required Clippy commands exit 0;
`cargo test --workspace --locked` exit 0 with 394 passed, 9 ignored, 0 failed;
`cargo xtask policy` 170 files ok; `git diff --check` exit 0. The ignored
count rose from 6 to 9 because three PRs add `#[ignore]`d benchmarks; those
are not counted as coverage.

Dispositions in this wave: 35 `integrated-local`; 3 held as `deferred`
(#132, #219, #312 — concrete review findings, listed under remainder).

| PR | Source head | Local integration |
| --- | --- | --- |
| [#280](https://github.com/Marvisak/VIctory/pull/280) | `7ecc6209` | `25c43bc2` |
| [#323](https://github.com/Marvisak/VIctory/pull/323) | `e08ac6a9` | `f71bd69d` |
| [#345](https://github.com/Marvisak/VIctory/pull/345) | `177783ad` | `90a5dd2d` |
| [#354](https://github.com/Marvisak/VIctory/pull/354) | `9f28f6b9` | `f2f21ab5` |
| [#424](https://github.com/Marvisak/VIctory/pull/424) | `5b9a9b37` | `a751c8a3` |
| [#494](https://github.com/Marvisak/VIctory/pull/494) | `0a96e1e0` | `d5a3f66e` |
| [#532](https://github.com/Marvisak/VIctory/pull/532) | `aa50c7d6` | `f4d0697a` |
| [#543](https://github.com/Marvisak/VIctory/pull/543) | `c9e5db87` | `95ee7d24` |
| [#572](https://github.com/Marvisak/VIctory/pull/572) | `0a1a702b` | `34370fce` |
| [#580](https://github.com/Marvisak/VIctory/pull/580) | `7ca84cc1` | `2db7897e` |
| [#686](https://github.com/Marvisak/VIctory/pull/686) | `741d675c` | `aa0401c2` |
| [#76](https://github.com/Marvisak/VIctory/pull/76) | `dea90ccc` | `f464bd96` |
| [#96](https://github.com/Marvisak/VIctory/pull/96) | `ba42db64` | `bbf60d7f` |
| [#114](https://github.com/Marvisak/VIctory/pull/114) | `0b729de6` | `5d8955e0` |
| [#122](https://github.com/Marvisak/VIctory/pull/122) | `d01b1707` | `461c174f` |
| [#133](https://github.com/Marvisak/VIctory/pull/133) | `01ae55a3` | `13e4e7fe` |
| [#172](https://github.com/Marvisak/VIctory/pull/172) | `4eb0898b` | `5ba70891` |
| [#182](https://github.com/Marvisak/VIctory/pull/182) | `18275322` | `aa370a59` |
| [#183](https://github.com/Marvisak/VIctory/pull/183) | `d4d18220` | `a6df1a25` |
| [#186](https://github.com/Marvisak/VIctory/pull/186) | `3c7d845b` | `791580b7` |
| [#188](https://github.com/Marvisak/VIctory/pull/188) | `2a6e6a4b` | `85834d42` |
| [#194](https://github.com/Marvisak/VIctory/pull/194) | `985d8d4b` | `7e18d623` |
| [#197](https://github.com/Marvisak/VIctory/pull/197) | `64484b42` | `ade65f82` |
| [#207](https://github.com/Marvisak/VIctory/pull/207) | `11ca3ce4` | `188340ae` |
| [#214](https://github.com/Marvisak/VIctory/pull/214) | `9de712f5` | `ba88b6ef` |
| [#221](https://github.com/Marvisak/VIctory/pull/221) | `acdcc416` | `18113a3c` |
| [#228](https://github.com/Marvisak/VIctory/pull/228) | `09ff84b6` | `f53da6cc` |
| [#247](https://github.com/Marvisak/VIctory/pull/247) | `5cff0007` | `2685c9ed` |
| [#324](https://github.com/Marvisak/VIctory/pull/324) | `8fc3a1bb` | `f9eb3bf0` |
| [#335](https://github.com/Marvisak/VIctory/pull/335) | `e3bef089` | `22309608` |
| [#351](https://github.com/Marvisak/VIctory/pull/351) | `93a24b7c` | `e4b302b1` |
| [#357](https://github.com/Marvisak/VIctory/pull/357) | `c6959acb` | `83bd896d` |
| [#359](https://github.com/Marvisak/VIctory/pull/359) | `d549b639` | `e30d5d3b` |
| [#361](https://github.com/Marvisak/VIctory/pull/361) | `3f5eff57` | `e9ca0d50` |
| [#364](https://github.com/Marvisak/VIctory/pull/364) | `e5037565` | `6de0fa33` |

Hand-merged:
- #572 (README) and #323 (README + notes) conflicted with sections added
  after their base; the PR text was preserved and the overlapping prose
  merged by hand.
- #182 conflicted with #133 in the same test module (`files.rs`); both tests
  were kept, each in its own function, and `cargo test -p victory-hle` passed
  afterwards (116 unit tests).
- #345 was integrated as the net diff of its three commits, because its tip
  commit edits a file an earlier commit in the same branch adds.

### Pending bounded PR candidates

- The five curated candidates (#449, #480, #396, #629, #363, #337) are all
  integrated; nothing from that group remains pending.
- The wider snapshot (445 open PRs) is still largely unreviewed: only the
  curated set above has a disposition. JMAN's wholesale batch stays deferred
  by reproduced failures. The issue and gap passes have not started. This is
  an ongoing sweep, not a completed audit.
- Next: refresh the PR snapshot, then triage the remaining open PRs into
  bounded candidates (correctness and unblockers first).

### PR #363 integration validation: 2026-11-24

- Refreshed upstream PR state: open, head unchanged at
  `4874c3fdc18e01d0a1973fc62558669ed9e18ead`; one commit; GitHub reported
  `mergeable: false` against current `main`.
- Review: `Config::load` in `crates/victory-core/src/config.rs` only. An empty
  `VICTORY_CONFIG` was treated as a path, so it bailed out with "points at ,
  which does not exist"; the patch routes the variable through a testable
  `explicit_config_path` helper that treats empty as unset, matching the
  other path variables.
- Conflict resolution: the only conflict was test placement against tests
  added after the PR's base. Resolved semantically by keeping both existing
  tests (`empty_test_titles_env_is_ignored`, `test_titles_env_splits_paths`)
  and appending `empty_config_env_is_ignored`; no production hunk was
  rewritten, the author is preserved, and the `-x` trailer names the source.
- Candidate `candidate/pr-363` was based on `b3cf5b9d`, rebased onto
  `25e797e9` after review, and integrated with the original author and `-x`
  source reference.
- Targeted `cargo test -p victory-core --locked`: 16 passed, including
  `config::tests::empty_config_env_is_ignored`.
- Full workspace gate on a fresh isolated target with Rust 1.98.1 x86_64
  Windows MSVC: both required Clippy commands passed; workspace tests 349
  passed with 6 ignored; `cargo xtask policy` 161 files ok; `git diff
  --check` passed. The targeted suite was re-run after the rebase.
- Local integration commit: `4dbc5a71`. No upstream PR action was taken.

### PR #337 integration validation: 2026-11-24

- Refreshed upstream PR state: open, head at
  `5cf259e5fe2f9361fffbc00e9b5bd46067b6686d`; four commits, `mergeable`
  true against base `main`.
- Review: `Title::from_manifest` in `crates/victory-hle/src/runtime.rs` only.
  `SizeMB` was shifted into bytes, which wraps for values at or above 2^44
  instead of saturating, and its parse type was implicit. The net patch
  parses `u64` explicitly and uses `saturating_mul`. The regression test
  pins both the exact-at-limit value and the saturated `u64::MAX`.
- Integration choice: the PR's first commit did not compile on its own, so
  the reviewed net diff of all four commits was applied as one commit,
  keeping the author and recording both the PR head SHA and all four source
  commit SHAs in the message. The two `rustfmt` commits are folded into it.
- Candidate `candidate/pr-337` was based on `25e797e9`, rebased onto
  `4dbc5a71`, and integrated with the original author.
- Targeted `cargo test -p victory-hle --locked`: 97 passed, including
  `runtime::tests::storage_quota_saturates_when_megabytes_exceed_u64_bytes`.
- Full workspace gate on a fresh isolated target with Rust 1.98.1 x86_64
  Windows MSVC: both required Clippy commands passed; workspace tests 350
  passed with 6 ignored; `cargo xtask policy` 161 files ok; `git diff
  --check` passed. The targeted suite was re-run after the rebase.
- Local integration commit: `feeff994`. No upstream PR action was taken.

### Batch gate: 2026-11-24

- Combined tip `feeff99438738b3c8ba9cc6412f11e5cc0b0c42b` on a newly created,
  worktree-specific external target (`integration-final2-target`); no
  cross-worktree artifacts reused.
- Both required Clippy commands: exit 0. `cargo test --workspace --locked`:
  exit 0, 351 passed, 6 ignored, no failures. `cargo xtask policy`: 161 files
  ok. `git diff --check`: exit 0. No title execution and no GPU/loader
  behaviour was exercised beyond the existing synthetic fixtures.
- The five newly integrated regressions all executed in this run:
  `percent_decode_requires_two_hex_digits`,
  `directory_entries_must_fit_in_the_optional_header_and_file`,
  `linear_layouts_require_one_sample`, `empty_config_env_is_ignored`,
  `storage_quota_saturates_when_megabytes_exceed_u64_bytes`.
- Accepted local integrations now: #689, #708, #449, #480, #396, #629,
  #363, #337. Logs remain outside the repository.

### PR #629 integration validation: 2026-11-24

- Refreshed upstream PR state: open, head unchanged at
  `350f91c23a9402b2a556a84c208a80030910ae3f`; one commit, `mergeable` true
  against base `main`.
- Review: `Layout::new` in `crates/victory-formats/src/texture.rs` only. The
  multisample allowance accepted a `Swizzle::LINEAR` descriptor, but the
  linear path computes rows and offsets for one sample, so a 2x-MSAA linear
  surface reported the same size as 1x. The patch requires a tiled swizzle
  for multisampling, matching the cited GFX10 addrlib validation, and is a
  refusal: it returns `Error::Invalid` before `Layout::linear` runs.
- Checked for behavioural fallout: `Layout::new` has no callers outside the
  module (textures go through `Texture`), and `addressable()` already
  treated multisampled linear layouts as unaddressable, so no working path
  loses support.
- Candidate `candidate/pr-629` was based on `b3cf5b9d` and integrated with
  the original author and `-x` source reference.
- Targeted `cargo test -p victory-formats --locked`: 24 passed, 4 ignored,
  including `texture::tests::linear_layouts_require_one_sample`; the four
  ignored tests need external data and were not counted as coverage.
- Full workspace gate on a fresh isolated target with Rust 1.98.1 x86_64
  Windows MSVC: both required Clippy commands passed; workspace tests 349
  passed with 6 ignored (348 plus the new regression); `cargo xtask policy`
  161 files ok; `git diff --check` passed.
- Local integration commit: `25e797e9`. No upstream PR action was taken.

### PR #396 integration validation: 2026-11-24

- Refreshed upstream PR state: open, head unchanged at
  `10e9f56a50214df67dca98b19daac0d8e9b04075`; one commit, `mergeable` true
  against base `main`.
- Review: `data_directory_offset` in `crates/victory-pe/src/layout.rs` only.
  It checked `index < NumberOfRvaAndSizes` but never that the 8-byte entry
  fits the declared `SizeOfOptionalHeader` or the file. The patch reads the
  size from the file header, bounds the optional header to it, and requires
  both `OPT_NUMBER_OF_RVA_AND_SIZES` and the requested entry to lie inside
  that slice. Valid images are unaffected: PE32+ writers declare 240 bytes,
  so all 16 entries still resolve.
- Callers were checked one by one: `prepare.rs` treats `None` as "directory
  absent" for IMPORT, BOUND_IMPORT and BASERELOC, and `image.rs` parses the
  header itself first, so a refused offset means a malformed image rather
  than a silent loss of relocations on a valid one.
- Candidate `candidate/pr-396` was based on `06daf0f7` and integrated with
  the original author and `-x` source reference.
- Targeted on the candidate target: `cargo test -p victory-pe --locked`
  8 passed, including
  `layout::tests::directory_entries_must_fit_in_the_optional_header_and_file`;
  `cargo test -p victory-loader --locked` 25 passed (22 unit, 2 `load_probe`,
  1 `title_dlls`).
- Full workspace gate on the same isolated target with Rust 1.98.1 x86_64
  Windows MSVC: both required Clippy commands passed; workspace tests 348
  passed with 6 ignored (347 plus the new layout regression); `cargo xtask
  policy` 161 files ok; `git diff --check` passed.
- Local integration commit: `27dd4a73`. No upstream PR action was taken.

### PR #480 integration validation: 2026-11-24

- Refreshed upstream PR state: open, head unchanged at
  `75fe6c02b278f18eab33ad893bda1dd68d176287`; one commit, `mergeStateStatus`
  CLEAN on base `main`.
- Review: `percent_decode` in `crates/victory-hle/src/modules/media.rs` only.
  `u8::from_str_radix` accepts a leading sign, so `%+1` decoded to a control
  byte; the patch requires both escape characters to be ASCII hex digits
  before parsing, leaving malformed escapes literal. Behaviour for valid
  escapes is unchanged and the decoder is local, not part of a public ABI.
- Candidate `candidate/pr-480` was based on `65fc7c2d` and integrated with
  the original author and `-x` source reference.
- Targeted `cargo test -p victory-hle --locked --lib media::`: 3 passed,
  including `percent_decode_requires_two_hex_digits`.
- On a new isolated external target with Rust 1.98.1 x86_64 Windows MSVC:
  both required Clippy commands passed; `cargo test --workspace --locked`
  passed 347 tests with 6 ignored (baseline 346 plus the new regression),
  with `load_probe` and `title_dlls` executed; `cargo xtask policy` passed
  with 161 files; `git diff --check` passed.
- Local integration commit: `06daf0f7`. No upstream PR action was taken.

### PR #449 integration validation: 2026-11-24

- Refreshed upstream PR state: open, head unchanged at
  `f625cc508c0d3a1b3f124cd3714dde681440d8c4`.
- Candidate `candidate/pr-449` was based on `5c90d9cef85fd584b562517e79cbec81952859c6` and integrated with the original author and `-x` source reference.
- Targeted `cargo test -p victory-hle --locked`: 94 passed; no failures.
- On a new isolated external target with Rust 1.98.1 x86_64 Windows MSVC:
  both required Clippy commands passed; `cargo test --workspace --locked`
  passed 346 tests with 6 ignored; `cargo xtask policy` passed with 161 files;
  `git diff --check` passed.
- Local integration commit: `c64f8aab`. No upstream PR action was taken.

### Authoritative final integration gate: 2026-11-24

- Code checked at `c16779d913ea5dc0e8f042740d0a0e3132952f13`; concurrent
  changes were documentation only (AGENTS, this log, JMAN assessment).
- Used newly created, worktree-specific external `integration-final-target`
  with explicit Rust 1.98.1 x86_64 Windows MSVC. No cross-worktree compiled
  artifacts were reused. This supersedes ambiguous shared-target evidence.
- Both exact required workspace Clippy commands: exit 0.
- Workspace tests: exit 0; 346 harness passes, 6 ignored, with four known
  compiler-dependent differential early returns. No title execution.
- Policy: exit 0; 161 files, matching the actual tracked/unignored inventory.
- `git diff --check`: exit 0.
- Full logs remain outside the repository under `integration-final`.
- Accepted local PR integrations remain #689 and #708. JMAN's wholesale
  fork remains deferred because its additional focused checks fail.
