# Where a frame's time goes

Measured 2026-11-25 on the reference machine (Radeon 780M, 32 GB shared with
the CPU, Windows), `--execute`, Series X profile, diagnostics build. One run
at a time. The title renders **3840x2160** internally whatever the profile,
window size or reported display mode (`small-gpu-handoff.md`).

## What is logged, and when

Every frame, in the release build too, `victory_gpu::host` logs one line
every five seconds:

```
<fps> fps, <ms> ms/frame (<cpu> preparing: copies <ms> ms <MB> MB, textures
<ms> ms <MB> MB; <ms> on the GPU; <ms> copying back <MB> MB), <n> draws/frame,
dropped <n> without a host pipeline (<n> being made) and <n> indirect;
<n> root-view slots/frame
```

That covers frames per second, milliseconds per frame split into the four
phases, and draws per frame. In the diagnostics build two more lines follow:
`copies per frame: ...` (the copy machinery, below) and the new
`refresh copies: <n> per frame, <KB> each, <MB> per frame`, which is the
number that matters for the copy path: a refresh costs one kernel call per
run whatever the run's size.

## The breakdown

Steady state in the intro at 4K, one frame of 1,366 ms:

| Phase | Time | Share |
| --- | --- | --- |
| on the GPU | 592 ms | 43% |
| preparing submissions, of which copies | 629 ms (308 ms, 328 MB) | 46% (23%) |
| copying back | 81 ms | 6% |
| unaccounted (submit, present, CPU) | ~64 ms | 5% |

That sums to 1,302 of 1,366 ms, so the four counters account for the frame.

The GPU share is real GPU work at 3840x2160 on a 780M. Nothing on the CPU
side can be made to pay for it; only rendering fewer pixels changes it.

The copy share is the shadow system keeping copies of title memory current for
the GPU, and it is the part with slack in it:

- **19-20 GB of ranges are asked about per frame** to find **191-328 MB that
  actually changed**. The asking is cheap (41-66 ms of `WatchUs`); the copying
  is not.
- **10,000-20,000 refreshes across 29-36 submissions per frame.** The same big
  title buffers are read by most lists of a submission, and each submission
  asks again.
- **Copy throughput varies by 10x**: 1 GB/s in one run (328 MB in 308 ms),
  89 MB/s in another (191 MB in 2,145 ms). The bytes are the same size; the
  difference is memory pressure (67,000 to 315,000 page faults per frame) and
  it is why a 4K working set on a shared-memory machine degrades far faster
  than the byte counts suggest.

## What was tried and rejected

**Coalescing nearby changed runs.** Hypothesis: the copies are thousands of
4 KiB runs, each a kernel `ReadProcessMemory` call, so merging runs separated
by up to 64 KiB into one call would cut the call count. Measured A/B, same
build, same run length:

| `VICTORY_COPY_SLACK` | calls/frame | KB each | MB/frame |
| --- | --- | --- | --- |
| 0 (only changed runs) | 3,512 / 5,738 | 57.5 / 50.9 | 202 / 292 |
| 65,536 (merge within 64 KiB) | 4,540 / 4,025 | 49.0 / 56.1 | 223 / 226 |

The average run is already ~50 KB, so there is nothing to merge and the
difference is run-to-run noise. The change was reverted; only the counters
that showed this were kept.

**Plain `memcpy` instead of the kernel call.** `VICTORY_COPY_MODE=plain` runs
for eight seconds and dies with a segmentation fault: title memory has
unmapped pages (tiled memory, page mappings) and a plain copy faults on them.
The kernel call is required, not incidental.

## Where a real win is

Rendering less. The GPU share, the copy sizes and the memory pressure all
scale with the 3840x2160 target, and the title ignores `VICTORY_DISPLAY` and
`VICTORY_WINDOW_MAX` when it sizes it. Capping the render size at resource
creation, with the views and the present copy scaled to match, moves all three
numbers at once — that is a feature to build and verify, not a flag.
