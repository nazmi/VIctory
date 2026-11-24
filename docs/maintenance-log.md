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
| PRs | Not started | Pending | Pending fetch | No | Unknown | Full inventory |
| Issues | Not started | Pending | Pending fetch | No | Unknown | After PR pass |
| Gaps | Not started | Local audit | Pending | Not applicable | Unknown | After issue pass |

## PR decisions and integrations

No entries yet. Add one row per inventoried PR, including rejected,
duplicate, already-present, deferred, and blocked items.

| PR/link | Source head SHA | Dependencies/overlap | Review evidence | Tests/evidence | Disposition/reason | Local integration SHA | Date |
| --- | --- | --- | --- | --- | --- | --- | --- |

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
