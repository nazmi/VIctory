# Handoff: VIctory maintenance on `integration/maintenance`

Written 2026-11-25 ~15:10 UTC. `integration/maintenance` is the reference
branch. Resume from Git, not from this prose: confirm HEAD and remote before
acting.

## Current branch

- Branch `integration/maintenance`. The code state described here is
  **`82755568`**; this document is the commit immediately after it. Both are
  pushed, and `origin/integration/maintenance` is level with the local branch.
- `upstream/main` **`940d32f6` is merged** (merge commit `afaacd64`, 291
  commits). 496 commits from `upstream/main` to HEAD.
- Gate on this HEAD: `cargo fmt --check` 0 differing, both
  `clippy --workspace --all-targets` configurations 0 errors, workspace
  **754 passed / 0 failed / 12 existing ignores**, asset policy clean.
- Working trees: `VIctory-integration` (this branch, the reference) and
  `VIctory-up2` (branch `merge/up2`, a staging worktree). Both are inside
  `~/gta6`. Build into the repo's own `target/` — see "Operating
  constraints".

## Landed this session

| What | Commit | Evidence |
| --- | --- | --- |
| upstream/main `940d32f6` (291 commits) merged | `afaacd64` | full gate green; `victory-shader`, `victory-pe`, `victory-gpu` list/queue/capture/pipeline/host taken from upstream, our `castable()` family rule and `VICTORY_LOG_FILE` re-applied |
| Host copy made on first use, not on reserve | `3931cae8` | the world-load OOM cause: 3,894 copies / 9,583 MB / 439 `0x8007000E` / device removed → 1,515 copies / 4,803 MB / 0 failures / 240 s clean |
| Present failures logged; the plane we are asked to show logged; `VICTORY_DISPLAY` | `632ad791` | how the Series S black screen was found |
| Refresh-copy rate logged; frame-cost profile | `48f75168` | `docs/perf-hot-path.md`; two optimizations measured and **rejected** (run coalescing was noise; plain memcpy segfaults) |
| `VICTORY_PRESENT_LARGEST` | `fdb116a1` | fixes the Series S black screen: intro plays, 26.8 fps menu, 0 present failures |
| `VICTORY_RENDER_SCALE` + window/viewport/scissor/copy/mip scaling | `9dadc946` | 4,814 MB → 2,174 MB host textures at 0.5; window and present both 1920x1080 |
| PR #453 reviewed, held | `121f817b` | 3 commits conflict across `fn interp` that upstream rewrote (3,877 lines changed since the PR base); see `pr-disposition.md` |

`docs/small-gpu-handoff.md` and `docs/perf-hot-path.md` hold the measurements
in full. Every number below was measured on this machine, one run at a time.

## What works today

- **Series X profile: the intro plays at 1920x1080, no present failures.**
  The title renders 3840x2160 internally.
- **Series S profile: the intro plays only with `VICTORY_PRESENT_LARGEST=1`**
  (set by `scripts/play.ps1 -SmallGpu`). The title renders 3840x2160 and asks
  us to present a separate, empty 1920x1080 texture. D3D12 has no scaling
  copy (`CopyResource` needs equal dimensions; `CopySubresourceRegion` copies
  a box unfiltered), so the platform must scale. `_LARGEST` presents the
  target that holds the picture and lets the window stretch it.
- **`VICTORY_RENDER_SCALE=0.5`** renders 1920x1080 with the window to match;
  the title card and loading screen composite correctly.

## The blocker: a BC1 copy removes the device

**This is pre-existing and unrelated to the render scale** — it reproduces at
`VICTORY_RENDER_SCALE=1.0`. With the debug layer on:

```
ID3D12CommandList::CopyTextureRegion: The destination region extends past, at
least, one of the edges of the destination subresource. When the destination
format is BC1_TYPELESS, the source format is R32G32_TYPELESS, DstX is 0,
DstY is 4752 ... requires the destination subresource to have 1320 width,
6072 height ... only has 5808 width, 5808 height
```

Consequences: a run ends at frame ~1,300-1,600 (about 40 s in), so **the
world is never reached** and no world-frame GPU measurement exists. The GPU
cost in the menu and intro is 2-8 ms, so the render scale has no room to show
a GPU-millisecond win there — that claim is still unproven.

What did not find it: a diagnostic logging every copy into a block-compressed
destination through `copy_texture_region` never fired, so the call comes from
another path — the twin or copy helpers in `crates/victory-gpu/src/draw.rs`
(`CopyTextureRegion` around lines 2570 and 2650), or `CopyResource`
(`host.rs:2629`, `list.rs:476`). Next step: instrument those four sites.

Copy boxes are now clipped to the destination's real subresource size with a
warning, so an oversized box degrades to a partial copy instead of a removed
device — but this particular call is not reaching that clip, which is itself
a clue.

## Next work, in order

1. **Find the BC1 copy's caller** (above). It gates everything else: without
   it no run reaches the world.
2. **Then measure the world** at scale 1.0 vs 0.5 — GPU ms, copy MB/frame,
   host texture MB. `docs/perf-hot-path.md` says the GPU is ~43% of a 1,366 ms
   4K frame, so a 0.5 scale should show a real win if the frame is reached.
3. **Then PR review queue.** `pr-disposition.md` rows still `not yet
   reviewed`; recheck each PR's live state and head SHA first. #453 is
   `deferred` with its conflict evidence recorded — it needs a re-port onto
   `940d32f6` and a value check, not another blind cherry-pick.
4. Remaining `docs/isa-gaps.md` items. The dominant one in every run is the
   lane-mask family (`a lane mask read as data`, `a register joining a lane
   mask and data`) across ~10 compute shaders per frame, plus three pixel
   shaders (`barycentrics read directly`, `a descriptor read only as data`).

## Environment knobs that exist and work

```
VICTORY_RENDER_SCALE=0.5        render targets + window + views + copies, one ratio
VICTORY_PRESENT_LARGEST=1       present the largest colour target, not the empty plane
VICTORY_DISPLAY=1280x720        emulated display mode (title ignores it for render size)
VICTORY_WINDOW_MAX=1280x720     cap the window (title still renders 4K)
VICTORY_LOG_FILE=<path>         log to a file as well as stderr (default on for `run`)
VICTORY_D3D12_DEBUG=1           D3D12 debug layer (diag build) — names invalid commands
VICTORY_CAPTURE=<dir>           save presented frames (half size: capture::save_as)
VICTORY_DUMP_TARGETS=1          save the title's own render targets (half size too)
VICTORY_COPY_MODE=none          timing only
VICTORY_KEYS=<sec>:<key>,...    scripted input (diag build); Enter = pad A
```

`VICTORY_MODE` does not exist — it was in `play.ps1` and nothing read it.

## Operating constraints

- **Do not create directories outside `~/gta6`.** Build into the repo's own
  `target/` (or a worktree's). Scratch logs and captures belong under
  `~/gta6`. The shared scratch dir `~/victory-runs` is used by earlier
  sessions and contains their files — do not delete those.
- **One game run at a time.** Do not run the title twice concurrently; this
  machine has 32 GB shared with the iGPU and two runs thrash.
- The title **may** be run (the operator authorised an autonomous run loop);
  expect the display driver to reset occasionally. Read
  `docs/asset-policy.md` and keep game assets, dumps and raw game logs out of
  the repository.
- Off-peak restriction applies **only to DeepSeek**: not 01:00-04:00 or
  06:00-10:00 UTC on weekdays, excluding Chinese public holidays. Codex is
  not time-restricted.
- Do not deploy read-only subagents (operator instruction).
- For accepted integrations: inspect the full diff, run the full gate, record
  evidence in `docs/maintenance-log.md`, merge with `--no-ff` so the topology
  shows every step, and preserve source attribution. Push only when the
  operator asks.

## Resume checks

```powershell
cd C:\Users\nazmi\gta6\VIctory-integration
git status --short --branch
git log -6 --oneline --first-parent
git rev-parse --short origin/integration/maintenance
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
Get-Content docs/maintenance-log.md -Tail 80
```

A run, logging to the file the app owns:

```powershell
.\target\release\victory.exe run '<path>\game_scarlett_master.exe' --execute
# log: $env:LOCALAPPDATA\victory\data\victory.log
```
