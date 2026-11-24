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

### Pending bounded PR candidates

- #480 at `75fe6c0`: 1/1 file reviewed; corrected source-head media run
  passed 3/3, including strict percent-escape regression. Full candidate
  integration gate pending; its patch occurs in JMAN's history too.
- #629 at `350f91c`: coordinator reviewed the full changed file, discussion,
  and surrounding layout logic; source tests 21 passed, 3 ignored. Full
  candidate gate pending.
- #396 at `10e9f56`: coordinator reviewed the full changed file, discussion,
  and loader callers; source tests 8 passed, 0 ignored. Full candidate gate
  pending. JMAN's fork does not supply this bounds fix.
- #363 at `4874c3f` and #337 at `5cf259e`: static reviews 1/1 file each,
  no skips/blockers; source and integration tests pending. #363 overlaps
  #241's broader config change; #337 needs all four source commits or its
  reviewed net patch, not just its initially uncompilable first commit.
- Next PR integration order: #480, then #396, refreshing exact heads
  before each candidate. JMAN's wholesale batch is deferred by reproduced
  failures. Remaining inventoried PRs are still queued; the issue and gap
  passes have not started. This is an ongoing sweep, not a completed audit.

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
