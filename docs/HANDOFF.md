# Handoff: VIctory maintenance on `integration/maintenance`

Updated 2026-11-25 after verifying current `Zettrox/ztrx` main. Resume on this
branch; current handoff commit is recorded in Git. DeepSeek may be used only
during the off-peak windows below. Codex can continue at any time.

## Current branch

- Branch: `integration/maintenance`.
- At handoff preparation, local HEAD was `5d0c65e7`; the origin tracking ref was
  `715f6075`, so the local branch was ahead by 40 commits. Confirm remote state
  before any push.
- The ztrx source ref remains `90b529f878a470fec9bf07b7be316bc769a12232`.
  Its complete history/tree is not merged. Two selected ports are present as
  no-fast-forward merges: shared pipeline wait budgets (`58e9182c`, candidate
  `5d3b4977`) and the normal-play launcher (`7eedff93`, candidate `1684dda1`).
- Those two candidate integrations passed formatting, both workspace Clippy
  modes, workspace tests (739 passed, 0 failed, 11 existing ignores), and asset
  policy. See [ztrx integration review](ztrx-integration-review.md) for exact
  source SHAs, held features, limitations, and external test logs.
- No push was performed. Verify status before continuing; preserve any user
  changes and do not push unless requested.

## Next work

Continue the pending pull request review queue, then take up the remaining
implementation gaps. The PR inventory is a dated snapshot; refresh each PR's
open state and head SHA before review. The disposition table is
[`pr-disposition.md`](pr-disposition.md), the machine-readable decision record
is [`pr-review-decisions.json`](pr-review-decisions.json), and the previous
complete inventory snapshot is [`pr-disposition-20261124.md`](pr-disposition-20261124.md).

PR #1324 has a reviewed candidate on `review/codex-pr-1324` at `7f94c8f1`.
Its targeted tests, Clippy, formatting, and policy checks passed; the combined
workspace gate and no-ff merge remain outstanding. PRs #1436, #1432, and #1032
have clean unstarted review branches. Recheck these refs and statuses before
resuming. See `docs/maintenance-log.md` and `docs/worker-accounting.md` for
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
