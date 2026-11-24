# VIctory

*Read this in: [English](README.md) | [Bahasa Indonesia](languages/README.id.md)*

Runs Grand Theft Auto VI (Xbox Series X|S) natively on Windows, from files
the user supplies.

The title's x86-64 code runs unchanged inside our process; VIctory supplies
the platform underneath it and converts the title's data, ahead of time where
it can and on first use where it must (shaders). See
[docs/architecture.md](docs/architecture.md).

## Status (2026-11-23)

**Not playable yet.** The title boots, runs its menus at its own 30 fps and
loads the world, where rendering fails.

| Stage | State |
|-------|-------|
| Boot, first-boot screens (accessibility, language, settings, legal) | works, 30 fps |
| Intro video with sound | works |
| Start screen, profile and saves | works |
| Loading into the world (about 50 s) | works |
| The world (first mission's intro) | reached and runs with indirect draws; its title card renders, the scene after it is still dark; 0.5-1.8 s per frame |

Tested on one machine: Windows 11, AMD RX 6700 XT.

> **Warning.** Entering the world on a real GPU can currently hang it, and
> Windows then resets the display driver (every monitor goes black for a
> moment). To look further, use a diagnostics build with
> `VICTORY_D3D12_WARP=1`, which renders on the software adapter.

What each part does today and what it still needs:

| Subsystem | Today | Needs |
|-----------|-------|-------|
| Loader | Runs the title's executable and its DLLs in our process; crash reports name the faulting code and are kept in `crashes.log` | – |
| Runtime services | One signed-in offline user, licences, packages, saves that survive a kill at any step, text entry dialog | Speech is silent; the dialog and relaunch are untested live |
| Network | Offline by design: the title cannot reach the network | – |
| Files and streaming | Archives and streamed data read, zlib and BCPack decoded in parallel | Loading is limited by writes into title memory |
| Input | XInput pads, keyboard, keyboard stand-in pad | Trigger rumble |
| Audio | Menu, video and loading sound; XMA voices and Opus streams decode | Reverb is silent; Opus results unverified against the title |
| GPU: menus | Draws, compute and presentation on host D3D12; only changed title memory is copied | Frame pacing drifts on one screen in release builds |
| GPU: world | Mesh pipelines, shader-written buffers, descriptor lifetimes, indirect execution (arguments clamped), pipelines made by worker threads and kept on disk | Most of the scene is still missing (untranslated shaders drop draws); frames take seconds; real queries, depth bounds, HDR and scaling are missing |
| Shaders | Console GPU code recompiled to DXIL at first use, cached on disk. Menus: all. World: mesh 8,191 of 8,506, vertex/pixel 10,617 of 12,346, compute 1,643 of 2,539 | 64-bit image atomics, merged geometry stages, the ray-tracing builder's shaders; some compute programs need a GPU with 64-lane waves |
| Textures | Console tiled layouts and detiling for the modes the title uses | Render-target metadata planes |
| Series S mode | Runs through the menus to the loading screen | The world, as above |

## Getting started

Requirements: Windows 10/11 x64, a CPU with AVX2, and a Direct3D 12 GPU with
shader model 6.6 (and mesh shaders for the world). Install Rust via rustup
and the Visual Studio C++ build tools with the Windows SDK. Rustup uses the
version pinned in `rust-toolchain.toml`.

The title also needs several GiB of available Windows commit at startup.
Check **Committed** under **Task Manager → Performance → Memory**. If
`crashes.log` reports Windows error 1455, close other applications or increase
the Windows paging file size, then retry. `victory run` prints the host's
commit headroom as its `commit` line; the title's first boot asks for 6.1 GiB
in one mapping.

Keep the game folder on an SSD if you can. From a hard drive the first
screen stays black, at 30 fps and about 3 draws/frame, for a minute and a
half or more while the title reads its files: the drive stays 100% busy with
small random reads, and nothing shows until they finish. Wait for it rather
than closing the window. A later launch is faster while Windows still holds
the files in its cache.

From the repository root, build and check the paths in Git Bash. Replace the
example path with the folder containing your title files. Keep that folder
outside the repository. The environment variable applies to this Bash shell.
For persistent local paths, copy `victory.example.toml` to `victory.toml` and
edit it.

```bash
export VICTORY_GAME_DIR='/d/Xbox Titles/My Game'
cargo xtask install-hooks                # asset-policy check on every commit
cp victory.example.toml victory.toml     # then set your paths (outside the repo)
cargo build --release -p victory
cargo test --workspace                   # needs no game data
./target/release/victory.exe config      # check your paths
./target/release/victory.exe run "$VICTORY_GAME_DIR/game_scarlett_master.exe" --execute
```

Replace the executable name if your title uses a different one. A bare file
name is looked up inside `game_dir`, so this works from any directory:

```bash
./target/release/victory.exe run game_scarlett_master.exe --execute
```

An absolute path, or a relative path that exists as given, is used as-is:

```bash
./target/release/victory.exe run '/d/Xbox Games/GTA VI/title.exe' --execute
```

### Selecting the Series S profile

GTA VI chooses its render configuration from the console model, not from the
display modes VIctory advertises through DXGI. To use the lower-resolution
profile, pass `--console series-s` when starting the title:

```bash
./target/release/victory.exe run '<game dir>/<the title's exe>' --execute --console series-s
```

In a reported diagnostic run, the Series X profile used a 3840×2160 scene
with 2560×1440 internal targets; the Series S profile used a 1920×1080 scene
with 1280×720 internal targets. The UI can still use a separate 3840×2160
presentation plane, so selecting Series S does not necessarily make the
window or every render target 1080p. Changing the DXGI mode list alone does
not change the scene resolution ([issue #203](https://github.com/Marvisak/VIctory/issues/203));
Series S multi-plane presentation is covered separately by
[PR #211](https://github.com/Marvisak/VIctory/pull/211).

The same report measured about 3.9 GB less committed process memory in the
menus/intro and about 3.6 GB less at the world peak with Series S enabled.
However, peak committed memory was still about 17.7 GB, and whole-system
GPU memory use still reached about 7.3 of 8 GB. Both profiles also lost the
GPU device 30–45 seconds after entering the world. These are measurements
from one Windows 10 / RTX 2070 SUPER 8 GB / 32 GB RAM diagnostic setup
(base commit `d2a4bae`, with PRs #175 and #211 applied), not general
requirements or a guarantee of stability. Series S can reduce memory
pressure, but it does not make a 16 GB RAM / 8 GB VRAM system reliably fit
the world or resolve the current GPU hang.
The window caption shows the framerate. Saves, settings and `crashes.log`
are kept under the data folder (default `%LOCALAPPDATA%\victory\data`), derived
data under the cache folder.

Input: XInput pads. With none connected the keyboard is a pad while the
title's window has focus:

| Keys | Pad |
|------|-----|
| W A S D / I J K L | left / right stick |
| arrows | D-pad |
| Enter, Backspace, X, Y | A, B, X, Y |
| Q / E, Z / C | shoulders, triggers |
| F / H | stick clicks |
| Esc, Tab | Menu, View |

## Commands

```
victory inspect <exe|dll> [--imports] [--exports]
victory census [paths...] [--missing] [--strict]
victory lib-exports <import.lib> [--list]
victory shader-info [paths...] [--verbose] [--disasm]
victory translate [paths...] [--validate] [--pipelines] [--list] [--jobs N]
victory translate [paths...] --descriptor-failures [--jobs N]
victory run <exe> [--execute] [--unimplemented trap|zero] [--strict]
            [--console series-x|series-s] [-- <title arguments>]
victory config
```

Use `--descriptor-failures` to count programs that resource tracking refuses because
a first-lane descriptor comes from a computed VGPR. Each row groups the defining
opcode of the first varying word, the opcode or source class of every varying
word in descriptor order, and the source classes of all descriptor words.

Logging: `VICTORY_LOG=victory_loader=trace,info` (a release build logs
info and above).

## Debugging

If the title presents one or two black frames and exits with `0x80000003`,
check the archive files before investigating rendering. The loader's import
report only counts bindings, so it can report zero missing imports with a
damaged archive. Check the SHA-256 of the files your run actually uses against
a known-good copy of the **same build**. For example, in Git Bash:

```bash
sha256sum '/c/path/to/title/anim.rpf'
```

If a hash differs, restore that file from your verified backup and rerun with
the same build of VIctory and the same options. A breakpoint can have other
causes. Include the archive verification outcome in your test report.

A release build has no debugging aids. Build them in with

```bash
cargo build --profile diag -p victory --features diagnostics   # target/diag/victory.exe
```

The on/off switches below (`...=1`) are off when unset or set to `0`.

| Variable | Effect |
|----------|--------|
| `VICTORY_LOG=<target>=debug,...` | debug and trace output |
| `VICTORY_STACKS=20,60` | every thread's stack at those seconds |
| `VICTORY_WATCH=<rva>,...` | logs a title u32 when it changes |
| `VICTORY_KEYS=30:Enter,35:X` | presses keys at those seconds, focus or not |
| `VICTORY_CRASH_SAVE=<rva>:<len>,...` | saves title memory to the cache folder on a crash |
| `VICTORY_CAPTURE=<dir>` | saves presented frames (powers of two, or `VICTORY_CAPTURE_FRAMES=<list>`) |
| `VICTORY_DUMP_TARGETS=1` | with the two above, saves every render target of those frames |
| `VICTORY_D3D12_DEBUG=1` | host debug layer |
| `VICTORY_D3D12_GBV=1` | with the above, GPU-based validation (bad descriptor use reported, not faulted; slow) |
| `VICTORY_D3D12_DRED=1` | on a GPU failure, the allocation hit and the commands in flight |
| `VICTORY_SKIP_PIPELINES=<hash>,...` | omit draws and dispatches, including indirect ones, whose pipeline contains a listed 16-digit shader hash |
| `VICTORY_GPU_TIMING=1` | with the above, each list of a submission run and timed on its own; slow lists logged with their slowest commands |
| `VICTORY_D3D12_WARP=1` | host device on the software adapter (slow; cannot reset the driver); `=0` opts out like an unset variable |
| `VICTORY_SHADER_LOOP_LIMIT=<n>` | translated shader loops stop after `n` iterations per lane instead of 2^20 (own shader cache entries); if a lower `n` makes world frames much faster, loops hitting the limit are what makes them slow |
| `VICTORY_AUDIO_CAPTURE=<file.wav>` | records what the title plays (our process only) |
| `VICTORY_XMA_DUMP=<dir>` | saves the first 32 XMA voices' context and input |

Every build appends crashes and host GPU failures to `crashes.log` in the
title's data folder.

[docs/debugging-notes-from-a-gta-v-port.md](docs/debugging-notes-from-a-gta-v-port.md)
collects what got another RAGE port from "the world loads" to "the game
runs": what the title waited for, and how each wait was found.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
