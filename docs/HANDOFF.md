# Handoff: VIctory maintenance on `integration/maintenance`

Updated 2026-11-25 after verifying current `Zettrox/ztrx` main. Resume on this
branch; current handoff commit is recorded in Git. DeepSeek may be used only
during the off-peak windows below. Codex can continue at any time.

## Current branch

- Branch: `integration/maintenance`, HEAD **`ce6dddb2`**.
- `origin/integration/maintenance` was at `715f6075` when this was written, so
  the local branch is **45 commits ahead and unpushed**. Confirm remote state
  before any push; the last session was told not to push without instruction.
- `upstream/main` is `70564ae0` and **is an ancestor**. `ztrx` source ref
  `90b529f8` (rechecked unchanged).
- Gate on this HEAD: fmt 0 differing, both Clippy modes 0 errors, asset policy
  226 files, `cargo test --workspace --locked` **754 passed, 0 failed**.

## Landed in the resume pass (2026-11-25)

| Item | Local merge | Notes |
| --- | --- | --- |
| PR #1324 staging-write vs publication race | `e63b9e61` | temp removed on a failed write; held-reader regression strengthened |
| PR #1432 `ERROR_PROC_NOT_FOUND` | `fd775f03` | conflict resolved by keeping this tree's ordinal-export lookup and adding the error to its miss path; negative control verified |
| PR #1032 sampler-heap cap at 2048 | `a16cb124` | a request above the D3D12 limit lost the whole heap; capped, with `sampler_slots()` tested without a GPU |
| PR #1436 | — | **closed upstream**, obsolete |

ztrx ports re-verified as working, not merely present: wait-budget tests 8/8
(incl. stale-frame and refund cases) and `scripts/play.ps1` parses clean. A
declaration sweep finds **575 ztrx-only symbols**; the remaining ztrx features
are the coupled subsystems `ztrx-integration-review.md` already holds, not
cheap ports.

## Next work

Continue the pending pull request review queue, then take up the remaining
implementation gaps. The PR inventory is a dated snapshot; refresh each PR's
open state and head SHA before review. The disposition table is
[`pr-disposition.md`](pr-disposition.md), the machine-readable decision record
is [`pr-review-decisions.json`](pr-review-decisions.json), and the previous
complete inventory snapshot is [`pr-disposition-20261124.md`](pr-disposition-20261124.md).

#1324, #1432 and #1032 are landed (above); #1436 is closed upstream, so the
review branches `review/codex-pr-1436`, `-1432` and `-1032` hold no work. The
next review targets should come from `pr-disposition.md` rows still marked
`not yet reviewed`, checking each PR's live state first. See `docs/maintenance-log.md` and `docs/worker-accounting.md` for
prior decisions and worker outcomes. Do not treat earlier large open-PR counts
as current.

Previously completed fixes remain in this branch: the bounded GPU test and
fence hang repairs, upstream-main regression recoveries, and the local PR
integrations recorded in the maintenance log. Do not repeat them without a
new failing reproduction.

## Operating constraints

- The off-peak restriction applies **only to DeepSeek**. Do not use DeepSeek
  during weekday peak windows 01:00–04:00 UTC or 06:00–10:00 UTC, excluding
  Chinese holidays. Codex work is not time-restricted.
- Do not launch the real title on this machine; its display driver has hung.
  Use offline translation or WARP for applicable GPU checks.
- Keep game assets, derived dumps, proprietary paths, and raw game logs outside
  the repository. Follow `docs/asset-policy.md`.
- For accepted integration changes, preserve author/source attribution, inspect
  the full diff, run required checks, record evidence in the maintenance log,
  and use a `--no-ff` merge. Do not push without the user's instruction.

## Resume checks

```powershell
git status --short --branch
git log -5 --oneline --decorate
git rev-parse origin/integration/maintenance
Get-Content docs/maintenance-log.md -Tail 100
```
