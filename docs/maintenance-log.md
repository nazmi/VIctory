# Maintenance log

## Binding constraint: off-peak only

The user set this as a standing hard rule on 2026-11-25:

> Work only in off-peak hours. Off-peak rates are half the peak rates. Peak
> hours are 01:00-04:00 and 06:00-10:00 UTC, Monday through Friday, excluding
> Chinese public holidays. All other hours are off-peak, including weekends and
> Chinese public holidays in full.

How it is enforced here: before starting a maintenance pass, compute the
current UTC time and check it against those windows; do not begin or continue
a pass inside a peak window, and do not schedule unattended work that would
run into one. Record the check with the pass. A pass that is already running
when a peak window begins must stop rather than run into it.

Current state at the time of writing: 2026-11-24 21:17 UTC, Tuesday -
off-peak. The next peak window opens 2026-11-25 01:00 UTC, 3.7 hours later.

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

### Wave 5 integrations and the world-load diagnosis: 2026-11-25

Gate at `6db9bb08`: both Clippy commands exit 0; workspace 660 passed,
14 ignored, 0 failed; policy 194 files; `cargo fmt --check` 0 differing files.

**A real run of the title was captured** (`latest dump.txt`, local and
gitignored) and analysed in `docs/world-load-diagnosis.md`. It reached frame
3,910 with 596 shaders translated and 251 host pipelines, then faulted writing
to address 0 from title code on a worker thread. 74 distinct shaders were
refused; the full ranked table is in the diagnosis.

Acted on, because the dump proves they matter:
- **#705, 64-bit image atomics** (7 refusals, pixel stage). Adds SFI0 bits 22
  and 24, which DXC calls AtomicInt64OnTypedResource and
  AtomicInt64OnDescriptorHeapResource.
- **The top refusal (31 refreshes, all compute, all at offsets 0x2c–0x40)** is
  a prologue failure in 31 *distinct* shaders, so one systematic cause, not
  thirty-one bugs. The refusal message said "no root parameter" even when the
  parameter existed at a later dword; it now names the actual case (descriptor
  high half / root constant past the first dword / heap index / later dword /
  no parameter) so the next run decides the fix. Diagnostics only — the
  lowering is unchanged, because guessing at address semantics would risk
  mis-binding every buffer in the world.
- **#390** puts a Rust backtrace in the panic log, and **#1071**, **#1110** are
  small correctness fixes (datagram connect-to-zero, save quota arithmetic).

Deliberately not merged, with reasons:
- **#694** would make `image_gather4_l` (7 refusals) translate by dropping the
  LOD operand: wrong texels instead of a refusal. DXIL's `textureGather` has no
  LOD, so a correct lowering needs four `sampleLevel` taps or a mip-specific
  view.
- **#1067** and **#1084** each restructure descriptor inference in the most
  contested function for two refusals apiece; both conflict there.

Also fixed this wave: a 3,358-line run log was committed by accident and
removed again (the branch now gitignores `latest dump.txt` and `*.crash.log`);
two rustfmt drifts were caught after the fact and corrected.

### Wave 4 integrations: 2026-11-24 (28 PRs + 2 lint fixes)

Gate at `2b7cbae1`: both Clippy commands exit 0; workspace 638 passed,
14 ignored, 0 failed; policy 193 files; `cargo fmt --check` 0 differing files.

Held or rejected in this wave, with the reason:
- #959 names the GPU from adapter 0 but reads capabilities from the default
  device, so a report can describe two different adapters; it also needs a
  `Win32_Foundation` feature it does not declare.
- #997 closes the caller's WinHTTP handle when `WinHttpSetOption` fails, where
  the real API just returns 12018 and keeps it.
- #986's copy path reads title memory at submit, so an earlier same-list UAV
  write is not seen.
- #575 does not compile (`Mutex<Vec<ID3D12Resource>>` is not `Send`);
  #608 does not compile and deletes two existing tests; #550 ships an unproven
  fix for #71; #481 deletes `ci.yml` and the `--all-features` clippy run;
  #540/#545 add a second, divergent lint gate on `stable` rather than the
  pinned toolchain.

Corrections made while integrating (each would have been a silent defect):
- #169 expected 16_16_UNORM `0xFFFF` to load as f16 `1.0`; the hardware here
  returns `0x3BFF` (0.99951). The case now uses `0x8000` (0.5), which is exact.
- #319's early return handed out a missed tick without recording it, letting
  two waits share one tick. It now takes the tick with `fetch_max`.
- #92's test hunks collided structurally with five merged test modules; only
  its production hunks are taken, with the coverage re-expressed as two
  GPU-run tests.
- #963's nested `if` collapsed (`size == 0` folded into the pattern).
- #1010's doc comment described only the kernel path; the file now has two.

### ISA gaps closed: 2026-11-24

- `v_add_nc_i32`, `v_sub_nc_i32`, `v_mul_lo_i32` (VOP3 0x37f/0x376/0x16b) and
  the signed 16-bit add/sub now lower. PR #92's production hunks; verified on
  the host GPU with signed-overflow cases (`0x7FFF_FFFF + 1`, the all-ones
  add) rather than left to the author's word.
- The full ISA sweep (all 1,232 opcode names against every mnemonic in
  `emit/`) found that most remaining gaps already have queued PRs upstream, so
  the cheaper path is reviewing those heads rather than writing new patches.
  Recorded as GAP-003.

### Wave 3 integrations: 2026-11-24 (overnight session)

Inventory refreshed mid-session: 501 open PRs (61 new since the first snapshot,
4 closed/merged upstream). Integrating a moving target, so each candidate's
head is re-read before use.

Notable dispositions this wave:
- **Duplicates resolved by choosing one.** #939 and #972 add the same sandbox-id
  minimum to the same function; #939 landed (earlier, and its test also proves
  the buffer and `used` survive a refusal). #957 is a strict subset of #975's
  swizzle assertions; #975 landed.
- **#17 (fmt CI gate) needed #692 (workspace fmt) first.** The gate is red on
  an unformatted tree, so the two landed together, in that order.
- **#190 removes the Direct3D/Fxc binding** because its clamp shader is now
  emitted directly, but the depth-bounds test helper (added after that branch
  forked) compiles synthetic HLSL with D3DCompile. The binding stays for tests
  only, with a comment saying why.
- **#415 fixed a real bug and required a cache migration**: the shader cache's
  root-type codes had drifted from `RootType::from_raw`, so codes above 6
  decoded into the wrong variant. MAGIC went VSH1 -> VSH2 so stale entries are
  not reused, and a test pins that a VSH1 entry is rejected.
- **#644 (hard merge, 8 conflict hunks)** — see its own section below.
- **Rejected/held with evidence**: #481 deletes ci.yml and drops the
  `--all-features` clippy run; #474 admits negative LOD bounds D3D12 forbids;
  #493 drops draws when no view heap is bound, with no test; #504 reports a
  D24S8 depth plane as R32_TYPELESS; #515's new tests reuse probe addresses
  against a process-global journal; #486 emits an export when its copy failed;
  #489 assumes spill dword 0 is user SGPR 16 without checking; #232 replaces CI
  with 1,149 unverifiable lines; #454 patches code with no bounds check; #505
  is an unowned Wine harness CI never runs.

Fixes made to the tree during this wave (each caught by the gate or by a test):
- two missing `SAFETY` comments and a collapsible `if` from integrated PRs;
- a `#[cfg(test)]` lost in a hand merge, which also left the scan tests racing
  the one test that sets `VICTORY_EMIT_DUMP` (now serialised by a module lock);
- a duplicated `[dev-dependencies]` table from the same merge;
- the `dstorage` lane-count flake, serialised on the shared scheduler.

Gate at `312dd157`: both Clippy commands exit 0; workspace 596 passed,
12 ignored, 0 failed; policy 184 files; `cargo fmt --check` 0 differing files.

| Local commit | Subject |
| --- | --- |
| `65c02b0e` | HLE: report the host's commit headroom before the title starts |
| `b8e51047` | XAsync: a canceled pending call's work touches neither freed state nor the next call |
| `49d9f393` | CLI: add AVX2 pre-flight CPU check before executing titles |
| `ee4a9ae6` | GPU: copy-back puts back only what shaders wrote, in two more cases |
| `0739961f` | GPU: every object answers for IUnknown |
| `99eb9327` | feat(gpu): answer format-info and virtual-address feature queries |
| `4d566336` | docs: say rumble is unverified in a live title |
| `6d897c25` | Policy: resolve cache paths inside Windows AppContainer |
| `db44521f` | Link victory.exe with a 16 MiB default thread stack |
| `c051b5da` | GPU: seek directly to private data on final release |
| `9a6d9e67` | GPU: the depth bounds test reaches the host |
| `65e35978` | GPU: keep the frame wait's tick for the diagnostics log |
| `ae526377` | GPU: skip unmapped regions after a failed bulk read into host memory |
| `d819400b` | Run indirect mesh dispatches through indirect execution |
| `e420fd36` | GPU: skip texture-to-buffer readback when host texel size differs |
| `b0d2e84f` | Census: report directory traversal failures |
| `4502afa1` | Query allocation type directly when guarding mapped memory |
| `a4023b67` | GPU: build the indirect-argument clamp with our own writer |
| `4148054b` | style: run cargo fmt across the workspace; fix clippy field-reassign-with-default |
| `7ea49a89` | CI: check the formatting with cargo fmt |
| `d6d02744` | CI: check the formatting with cargo fmt |
| `e28ee842` | Isolate the write-watch fixture from reused virtual addresses |
| `4a8b49ec` | HLE: XSystemGetXboxLiveSandboxId refuses a buffer under 16 bytes |
| `22e61383` | HLE: exercise XGSuggestSwizzleMode TEXTURE_1D swizzle branch |
| `5541a92b` | Shaders: keep the v_cvt_f16_u16/i16 test to integers f16 holds exactly |
| `e9aecbd8` | GPU: use native root type codes in the shader cache |
| `56252f48` | policy: fail when staged content cannot be read |
| `9a68bad9` | CLI: show runtime hashes in shader listing |
| `69c467b7` | GPU: name the user-data kind `root`, not `kind`, in op_text |
| `66a3c2c1` | Metal: add native Apple Silicon GPU conformance harness |
| `abfa4d3f` | Census: deduplicate PE imports without hint |
| `42c80756` | App: reject wrapping RVAs in VICTORY_WATCH |
| `9b703856` | diagnostics: preserve page fault baseline on query failure |
| `a5ccbffd` | GPU diagnostics: pick capture frames by draw count |
| `2f0a1740` | GPU diagnostics: show one of the title's render targets in the window |
| `56961c3b` | App: enforce ensure_outside_repo on configured paths while exempting CLI paths |
| `b1bafdaa` | lint: satisfy the gate after the wave's integrations |
| `b9dd41c8` | fix(cli): move scan test modules after production items |
| `b33b0fed` | GPU: spill oversized host root signatures' user data to root CBVs |
| `312dd157` | Shaders: v_swap_b32 with a non-VGPR source is a move into the destination |

### PR #644 hard merge: 2026-11-24

- PR: "GPU: spill oversized host root signatures' user data to root CBVs"
  (nathanbzrr), head `6fa53a51`, 989 insertions across two files.
- It introduces a `UserDataBind` enum (32-bit constants vs a spilled root CBV),
  `fit_root_signature`/`root_signature_cost`, per-draw scratch memory, and four
  tests including a host dispatch that reads two distinct spilled values.
- HEAD had since gained: the `table_failed` draw-drop guard with its
  power-of-two warning, root CBV/SRV view lists, and the diagnostics `op_text`
  naming user data as table/const. All of these survive; each symbol was
  compared before/after by occurrence count, none decreased.
- Eight conflict hunks were resolved semantically. The PR's test modules were
  folded into the existing ones (a second `mod tests` in a file does not
  compile — the same defect already fixed upstream in #129 and seen again in
  #616), `test_shader` moved to file scope under `cfg(test)`, and three
  merge-cut braces/semicolons were restored.
- Verified: the PR's four new tests pass on this host's GPU; workspace 596
  passed, 12 ignored, 0 failed; both Clippy commands, fmt, policy all clean.

### Shader ISA work: 2026-11-24

- Read AMD's RDNA 2 ISA (local PDF, `pdftotext -layout`) and enumerated every
  opcode name the tree's opcode tables carry against the names the lowerer
  mentions: 942 of 1,232 are never named, concentrated in VOPC (190), DS (146),
  MIMG (86), SMEM (81) and MUBUF (73). That count overstates the gap — many are
  handled by family prefixes (`ds_read*`, `buffer_load*`, `image_*`) — so the
  real gap list is the lowerer's own refusal reasons.
- `v_swap_b32` was implemented for VGPR/VGPR but refused every other source
  form, and had no test. Per the ISA (101 V_SWAP_B32: `tmp = D.u; D.u = S0.u;
  S0.u = tmp`), only a VGPR can be written back, so an SGPR, inline constant or
  literal source makes it a move into the destination. Implemented, with a
  host-GPU test covering both registers after a swap and the constant form.
- Scalar M0-relative moves (`s_movrels_b32`, `s_movreld_b32`, `s_movrelsd_2_b32`)
  stay refused deliberately: DXIL has no dynamically indexed scalars, so the
  register number must be a compile-time constant. Lowering them would need a
  select chain over every candidate SGPR; that is a design decision, not an
  oversight, and is recorded as GAP-001 below.

### Confirmed gaps

| ID | Evidence/location | Impact/priority | Dependencies | Acceptance criteria | Status |
| --- | --- | --- | --- | --- | --- |
| GAP-001 | `ir.rs:725` refuses `s_movrel*`; ISA 46/47/48/49/73 | Low: uniform code with a dynamic SGPR index | None | Either a select chain over the candidate SGPRs, or a documented decision to keep refusing | open |
| GAP-003 | ISA sweep: 942 of 1,232 opcode names are never mentioned in `emit/`, and the analyst found queued PRs covering most simple ALU gaps | Medium: translation coverage | Those PR heads | Each is either integrated or refused with a stated ISA reason | open |
| GAP-002 | PR #645's tests (`root_srv_two_term_v_add_co_and_v_lshl_add`, `root_srv_loop_carried_pointer_induction_phi`) do not exist at HEAD | Medium: two-term and loop-carried-phi descriptor addresses refused | Its branch conflicts with the SSA-sum cache from #139 across 18 hunks | Both shaders lower and produce correct descriptor handles | open |

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

### Wave 6: review-before-integrate: 2026-11-25

Inventory refreshed twice: 1,016 open PRs at 20:39 UTC, **1,024** at 21:04 UTC
(9 new: #1699-#1710; #1672 closed upstream; 8 heads moved). Every integration
below re-read its PR at its exact head SHA. Heads that moved were re-fetched
with a forced ref update, because a plain fetch silently kept a stale local ref
and made #1656 look unfixed when upstream had already fixed it.

**Process change for this wave.** PRs are reviewed and tested in an isolated
worktree *before* being cherry-picked, and worker subagents get write-capable
roles with their own `CARGO_TARGET_DIR`. The earlier read-only `explore` role is
capped at 24 model steps and two of them exhausted that budget mid-task.

Acting on the world-load diagnosis, in order:

- **The refusal text now names which case it is**, and that immediately paid
  off: all 31 world-load refusals read "an address in an SGPR before the first
  root parameter". The previous text conflated three different bugs.
- **But that message was partly an artifact.** `why_no_base` compared the
  register against the declared spans; with no parameters declared the spans are
  empty, `first` stayed `u8::MAX`, and every register compared below it. So a
  program with no layout reported "before the first parameter". Fixed in tree
  (`4b5fe162`) with a test that pins the new wording. The runtime and offline
  paths still disagree about the layout, and that disagreement is the next thing
  to settle — not the address rule.
- **The second title run died differently**: the host GPU was removed as hung
  during a 16-list submission on the Radeon 780M. That is the README's documented
  hazard, not a new defect, and it is a third candidate cause alongside the null
  write and the memory report. No further runs on this iGPU.

Integrated this wave (each reviewed, then tested, then committed with the author
and the source SHA):

| Local commit | PR | Subject |
| --- | --- | --- |
| `bc9c0f60` | #1668 | Shaders: read inline floats as f32 bits in 16-bit integer ops |
| `138e43c5` | #1685 | Shaders: lower v_cmp_{f,t}_{i64,u64} constant 64-bit compares |
| `621c3c17` | #1686 | Shaders: lower pixel shader SV_Coverage input via dx.op.coverage |
| `13ab09bb` | #1667 | flush a kept save blob that had to be copied |
| `4cf31e8e` | #1664 | hle: return S_OK with null handle when PowerSetRequest fails unsupported |
| `d32efcd9` | #1660 | GPU: serialize pipeline library atomic writes with unique temp files |
| `15316a22` | #1657 | XSystem: name the analytics family Windows.Xbox |
| `336bee70` | #1703 | README: the GPU timing switch needs DRED for the per-command notes |
| `aec7eb38` | #1700 | GPU: say when a draw's render target has no host copy or permission |
| `5c61aa48` | #1701 | GPU: VICTORY_MODE overrides the reported console display mode |
| `a9040302` | #1702 | HLE: add VICTORY_NO_HOST_PAD to hide host gamepads |

Corrections made while integrating (each would have been a silent defect):

- **#1656 was rejected, correctly, then fixed upstream.** I found that
  `pooled()` returns a length-0 `Vec` while the new copy-back indexed it
  (`&mut piece[..n]`), so it panicked on the first changed piece; its only new
  test was `#[ignore]`d. The author's new head (`dd2c3eda`) adds
  `piece.resize(PIECE, 0)` and a real `shadow_parallel` test. Re-verification is
  the gate for integrating it.
- **#1675 is a duplicate**, not new work: `lower.rs:3567` already lowers
  `s_absdiff_i32`, `s_sext_i32_i8/i16` and the three `s_pack_*_b16` forms.
  Applying it would add duplicate match arms, which `-D warnings` rejects.
  Its device-value test is stronger than our translation-only test and is worth
  salvaging on its own.
- **#1668 needed HEAD's structure**, not the PR's: HEAD already carried
  `half_int_src`, so only three call sites and their tests were taken.
- **Two test bodies were interleaved by a mechanical merge** in `xgamesave.rs`
  (#1667 against a test added after its base); they were split back into two
  functions rather than accepted as one, which would have mixed two unrelated
  assertions into one test.
- **A reviewer subagent claimed #1685 was already present.** It was not:
  `git show 321580dd:...lower.rs` has no `f`/`t` arm. The integrated commit is
  the only source of that code. Recorded because agent summaries are inputs to
  review, never evidence.
- **`cargo fmt --check` caught two formatting drifts** introduced by the appends
  above; both were folded back into their originating commits with
  `--fixup`/`--autosquash` rather than left as a trailing style commit.

Held with reasons this wave: #1694 (would revert the single-rounding
`v_dot2_f32_f16` fix and turn a refusal into a wrong value), #1654 and #1662
(one inline constant expanded from f64 among f32 siblings), #1663 (drops wave32
forcing; out-of-bounds ADDTID returns 0 instead of refusing). Rejected: #694,
#1067/#1084 deferred as before.

**New evidence: the whole-game shader census.** `victory translate` over every
container in `shaders.rpf` (offline, no GPU) gives 22,875 of 25,322 programs
emitted and a ranked refusal table — see `docs/isa-gaps.md`. The dominant
finding is that descriptor-bits-as-data is at least 1,424 of 2,807 failures,
more than every instruction-encoding gap combined.

**The RDNA 2 ISA PDF used in an earlier session is no longer on this machine.**
A search of the user's disks finds no such file, so no claim in this wave is
cited to it; the census and the repo's own opcode tables replaced it.

Gate at `4b5fe162`: fmt 0 differing, both Clippy commands exit 0, workspace 670
passed / 14 ignored / 0 failed, policy ok. Four commits landed after that gate,
so it is explicitly stale and is re-run on the final tip.

### Wave 6 continued: the fork, and a disposition for every PR: 2026-11-25

Two things the user asked for that the earlier ledger did not provide.

**A disposition for every open PR, not just the integrated ones.**
`docs/pr-disposition.md` now lists all 1,024 open PRs at the 21:04 UTC
snapshot with a state, a local commit where one exists, and a reason where
one was reviewed. The number that matters is in it: **818 have no reviewed
disposition**. They are listed as unreviewed rather than omitted, because a
table that only shows successes reads as if the rest were handled.

**A third-party fork: `github.com/fxceless/victory-fork`.** Ten commits on
top of the same upstream base (`ffe310e`). GitHub does not report it as a fork
of Marvisak/VIctory, and it is not a later upstream: it is a *sibling* tree
built on a different translator architecture. A three-way merge of its first
commit reports **33 conflicted files**, including every shader-core file.

Taken from it:

- **`spill.rs` (spilled user-data tables)** — `c56c7b86`. A compute program
  whose root signature needs more user-data dwords than `COMPUTE_PGM_RSRC2`
  grants keeps none of its parameters in registers: dwords 0 and 1 hold a
  pointer to a table, and parameter N (16 and above) is at byte 4*(N-16) of
  it, read with `s_load_dword*`. This tree had no notion of that. It matters
  because it is exactly the shape that yields "an address whose base is no
  declared parameter", which is the top refusal in the world-load run (31 of
  74, all compute). Only the new module and its single call site were taken,
  not the other 60 files of that commit.
- **`004bf69a`, stencil-plane and shader-view diagnostics** — `7180c20e`.
- **`16849898`, the bitcode optimiser — HELD, not merged.** It is on by
  default (`emit/shader.rs`), so every emitted shader changes and the title's
  whole shader cache is invalidated. Its LOADS pass reuses a memory read on an
  explicit bet that the title cannot observe an unsynchronised store between
  two reads, and its `FIRST` pass changes lowering on a wave64-as-two-waves
  architecture this tree does not have. Its tests are structural plus an
  in-process interpreter; no device values. The fork's own README at
  `004bf69a` records device loss in 4 of 4 runs on its newest commits. That is
  not a safe default; it needs an off-by-default port and a title A/B.

Not taken, with reasons: the wave64-on-32-lane exchange stack (thousands of
lines replacing this tree's wave model — and it exists for a 32-lane host,
which the diagnostic machine is not), the ray-tracing stack (off by default
and unlit on the fork's own machine), and the fork's `Cargo.toml` (it lacks
our thin LTO; it does *not* carry the lint disables an early report claimed —
checked, `git diff HEAD 578d459c -- Cargo.toml` is 15 lines about LTO).

Also in this wave:

| Local commit | From | Subject |
| --- | --- | --- |
| `e08c93e0` | #1656 | GPU: copy back only the span shaders changed, on several threads |
| `c56c7b86` | fork `578d459c` | Shaders: a compute program's spilled user-data table is read as parameters |
| `c21cee43` | #1695 | Shader tests: require a device readback in every waterfall test |
| `9e91c1d2` | #1680 | GPU: pipeline library loads take a lock for their name |
| `f9fe0e37` | #1710 | GPU: keep dropped draws in the full frame reporting interval |
| `a18c2683` | #1690 | GPU: zero the padding of the descriptions the library compares |
| `67be115d` | #1690 | GPU: stop keeping a kind of pipeline the library refuses to load |
| `7180c20e` | fork `004bf69a` | Test fix, stencil diagnostics, stability warning |

Corrections made during this wave:

- **#1656's threaded copy-back absorbed panics.** `join().unwrap_or(0)` turned
  a panicking copy-back thread into a *partial* write-back that the frame then
  continued past. It now resumes the panic. The PR's own comment claiming the
  ranges are "disjoint ranges of one buffer" was also false — nothing enforces
  disjointness — and now says what actually happens.
- **The offline census did not build the program the runtime builds.** It
  called `ir::build` directly while the emitter goes through `spill::build`,
  so a spilled-table shader was rewritten by one path and not the other. That
  is a concrete explanation for why the world-load refusals never reproduced
  offline, and it is fixed (`2e55c8ef`). Evidence that it changes nothing:
  the census over 26,084 programs is byte-identical before and after, which is
  consistent with the fork's claim that only the BVH builder's programs are
  spilled — inference, not measurement.
- **A worker claimed the fork's `Cargo.toml` disables lints.** It does not;
  the only difference is our thin-LTO setting. Checked, not assumed.

### Off-peak rule: recorded

The user made off-peak-only work a standing hard rule. Peak is 01:00-04:00
and 06:00-10:00 UTC, Monday-Friday, excluding Chinese public holidays;
everything else is off-peak. Each pass records the UTC time it started
against that window. The wave-6 continuation ran 21:17-22:00 UTC on a
Tuesday, off-peak; the next peak opens 2026-11-25 01:00 UTC.

### Wave 7: the top world-load refusal is fixed, and the count is honest

The goal this session was to determine which of the three cases the 31 compute
prologue refusals were, and to fix the verified cause. That is done.

**The refusals were located.** They are not in `shaders.rpf` and not in any of
the six other archives: they are in `xboxbvhbuilder.dll`. `victory translate`
on that DLL reports 52 compute programs and **52 identical refusals at offset
0x30** — the runtime's top refusal, reproduced offline on no GPU.

**The cause is spilled user data.** These programs' root signatures need more
user-data dwords than `COMPUTE_PGM_RSRC2` grants, so the hardware keeps no
parameters in registers: dwords 0/1 hold a table pointer and parameter N lives
at byte 4*(N-16) of that table. Our `resource.rs` mapped every parameter to
user SGPRs, so the register held a pointer — an address whose base is no
declared parameter. `spill.rs` (taken from the fork, integrated as `c56c7b86`)
rewrites those loads before tracking.

**Measured, not asserted.** Same machine, same inputs, binaries differing only
by that module:

| | `xboxbvhbuilder.dll` | `shaders.rpf` |
| --- | --- | --- |
| before | 52 × "an address in an SGPR before the first root parameter" | 26,084 programs, refusal table R |
| after | **0** × that refusal; 22 "an address not from a root parameter", 18 "a root UAV or typed root view", 7 "a select on an untracked mask", 3 "a descriptor read only as data", 1 "a global memory write", 1 "a whole-wave section step" | 26,084 programs, refusal table **R, identical** |

The top refusal is gone and the programs now fail later on real remaining
gaps. `shaders.rpf` being identical is the other half of the evidence: only the
BVH builder's programs are spilled, exactly as the fork predicted.

**A process error worth recording.** Every census run earlier in this session
used `./target/release/victory.exe` while building into `CARGO_TARGET_DIR`, so
it ran a stale binary — and one conclusion ("the change fires on none of the
26,084 programs") was drawn from that and had to be withdrawn. The rule now:
build and run the same path, and check the binary's timestamp when a result
looks like a no-op.

**A second measurement error, mine.** I wrote that the 31 refused shaders were
unreachable offline. They were reachable; I had simply never pointed the census
at the DLL. The doc is corrected rather than quietly amended.

Also integrated this wave, each reviewed and tested in an isolated worktree:
`9c40c87a` #1687 (image_gather4_c_lz and offset variants), `2ab062db` #1693
(TFE via CheckAccessFullyMapped), `d2c125d9` #1673 (XTaskQueueTerminate scopes
to its own tasks), `2b3b64a1` #1676 (census names a mesh-less host),
`50256643` #1669 (straddling 16-bit loads at offset 3), `ca82cd6e` #1691
(shadow-buffer states), `a28134d2` #1677 (window stand-in), `d4dc96dd` #1688,
`ec2aa220` #1689, `8516122d` #1696, `dc2e9231` and `67ff16d4` from the fork,
`7180c20e` stencil diagnostics.

**Refused with evidence:** #1699. Its `v_mad_*16` reads its third operand with
`half_src` while src0/src1 use `half_int_src`, so an inline float constant
there is read as a packed f16 — a probe on this GPU measured 15366 where the
hardware's low half is 0. That is the same defect class #1668 fixed one
operand-width over, so taking it would have re-introduced it.

Gate at `67ff16d4`: fmt 0 differing, both Clippy commands exit 0. Census runs
recorded above; the workspace test suite is re-run before any handoff claim.

### Wave 7 batch two: the 1,043-PR snapshot: 2026-11-25

Inventory refreshed to **1,043** open PRs (21 new since the 1,024 snapshot;
#1672 and #1634 closed, #1689 merged upstream — and we already had #1689, so
its closure cost nothing). Three workers ran in isolated worktrees against
disjoint crates, each with its own target directory, each told to review the
whole diff and to report a skipped test as skipped.

**HLE: four in, one refused.**

| Local commit | PR | Subject |
| --- | --- | --- |
| `a067e991` | #1717 | HLE: the retail console ID has the documented dotted form |
| `da106558` | #1719 | DirectStorage: clear the factory out pointer on failure |
| `df16f7d2` | #1720 | report APU heap allocations that fail |
| `df359c78` | #1726 | DirectStorage: read a page list naming page 0 again before failing |

Refused: **#1729**. It rejects `v0` as a version folder, which breaks the
integrated `a_folder_named_like_a_version_does_not_remove_the_save`, and it
returns the newest *readable* version where HEAD deliberately returns
`InvalidData` for a malformed newest one — that is a silent rollback to an
older save replacing a clean refusal. A policy change, so it needs the two
HEAD tests rewritten deliberately, not a cherry-pick. Its "stray folder"
behaviour is already present at HEAD (`v01`, `v+1` rejected).

Checked by hand rather than taken on trust: `#1720` keeps the commit-limit
message byte-identical (`"VIctory: Windows commit limit reached while
committing {len} bytes of {what}: {error}.\n"`), because a worker could
plausibly have dropped the caller's `what`.

**Shader: two in as a pair, two duplicates, one held.**

| Local commit | PR | Subject |
| --- | --- | --- |
| `08c99ea0` | #1712 | Shaders: support line-topology mesh shaders |
| `574c7310` | #1718 | Shaders: mask mesh primitive vertex indices to 9 bits |

`#1712` must travel with `#1718`: alone, an edge-flagged corner yields a host
vertex index of 512 or more while `max_vertices` is at most 256. Both are in,
in that order.

Duplicates, each verified rather than assumed: `#1732` (f32_to_half NaN and
nearest-even) is `67b24154` locally, and the PR's test bodies run clean against
the tree's function; `#1733` (helper lane reads its own value) is `694c4944`,
and the select is at `lower.rs:5387-5396`. A previous worker claimed a
duplicate wrongly (#1685), so both were re-checked in the source.

Held: **#1716**, the bitcode optimiser. It is called unconditionally from
`Shader::finish`, and on a probe branch it fails two of 233 tests — dxil.dll
rejects one module with "Flags must match usage", and another already-working
shader's emitted IR changes. That is a shader that validated becoming one the
validator refuses; it stays out. The probe branch `scratch/1716-probe` is
evidence only.

Gate at `98e720e0`: fmt 0 differing, both Clippy commands exit 0, workspace
**709 passed / 16 ignored / 0 failed**, policy 200 files.

### Wave 7 batch three and an integration audit: 2026-11-25

**GPU batch: five PRs, seven commits, all in.**

| Local commit | PR | Subject |
| --- | --- | --- |
| `42ba51eb`, `ed7413fa` | #1725 | GPU: placed copies take the bytes a GPU write leaves in their memory |
| `6f2255bd` | #1727 | GPU: 1D textures are 2D on the host, one texel high |
| `3a26c01b` | #1731 | GPU: occlusion queries measure on the host |
| `e88c5752` | #1728 | Pipelines: VICTORY_PIPELINE_WORKERS sets the worker count |
| `ea8b7f31` | #1734 | Shaders: wait for a test's fence on an event, not in a spin |

Two carry risk that unit tests cannot see, recorded rather than smoothed over:

- **#1731** changes what a culling title hides. Draws the host drops contribute
  no samples, so an occlusion count can now read low where the old stub answered
  "everything visible" — geometry that used to be drawn can disappear. There is
  no switch back to the stub. That is a visual-regression risk on a real run,
  not something a test can settle here.
- **#1725** takes one snapshot per placed copy per list, so a UAV write landing
  between two reads of the same placed copy inside one list leaves the second
  read stale.

**The audit that mattered.** A question exposed a real gap: were the workers'
branches actually in the integrated branch? They were not, all at once — the
GPU worker's seven commits were verified but not yet cherry-picked when the
question arrived.

The reason the question is hard to answer by inspection is worth writing down:
cherry-picking creates a **new SHA**, so searching this branch's log for a
worker's commit SHA always fails and reads as "the work was never taken". The
correct test is patch-id equivalence:

```bash
git cherry HEAD <worker-branch>    # '-' applied, '+' not applied
```

Run across every worker branch, that now reports `+` only for commits that were
carried across as resolved tree state or reformatted — each of those verified
separately by grepping for its distinctive lines and by diffing HEAD against the
branch, where every remaining difference is *newer* work in HEAD rather than
missing work. The method and its results are in
[worker-accounting.md](worker-accounting.md), and the rule it leaves behind is:
after every worker, require `+` to be empty or explain each `+`; never infer
integration from a worker's summary.

Gate at `ed526231`: fmt 0 differing, both Clippy commands exit 0, workspace
**717 passed / 16 ignored / 0 failed**, policy 200 files.

### Upstream moved: 28 maintainer commits, and what they supersede: 2026-11-25

`upstream/main` advanced from `ffe310eb` to `70564ae0` — 28 commits, 76 files,
**+25,510 / -1,340** — in exactly the files this sweep has been changing. A
three-way merge from the common base reports **36 conflicted files in 186
hunks**: a hard merge, not a rebase.

**The important finding is not the conflict count.** The maintainer has
independently implemented a large part of what this sweep integrated from
pull requests, and in several cases implemented it better:

| Topic | Ours | Upstream |
| --- | --- | --- |
| `image_gather4_l` | refused; #694 rejected because it dropped the LOD operand | **lowered properly**: `gather_at_level` loads the four texels from the level (`lower.rs:8035`), refusing only the cases it cannot prove |
| Lane masks spilled to VGPR lanes | PR #1074 (`2c1f3461`) | same feature, different shape (`restored` vs our `spilled`) |
| Structured-buffer stride | refused 171 programs | a **per-slot stride table** taken from the heap |
| BVH intersection | not implemented; the fork has it | `crates/victory-shader/src/emit/lower/bvh.rs` |
| Title memory at raw addresses | not implemented; the fork has it | `crates/victory-shader/src/range.rs`, `victory-gpu/src/memory.rs` |
| 64-bit image atomics | PR #705 (`cd1bbc84`) | present |
| f16 rounding, denormals, med3, signed zeros | PRs #1668, #1689 | present |

So the merge is not a matter of resolving text. For those topics it is a
question of **which implementation wins**, and the answer is upstream's: it is
the authority, it has tests built for its own architecture (`semantics_tests.rs`
3,010 lines, `flow_tests.rs`, `memory_tests.rs`, `stride_tests.rs`), and
keeping our parallel versions would leave two implementations of the same idea
in one tree.

Still ours alone, and confirmed absent upstream: the percent-escape decoder
(#480), the PE data-directory bound (#396), the spilled user-data table
(`spill.rs` — upstream has no such module), and the rest of the wave-1..7
integrations. Those must survive the merge.

**What was published.** Before touching the merge, `integration/maintenance`
was fast-forwarded to the verified tip and pushed to `origin`:
`321580dd..f355d4b6`, remote and local identical. Gate on that exact commit:
fmt 0 differing, both Clippy commands exit 0, workspace 717 passed / 16 ignored
/ 0 failed, policy 200 files.
