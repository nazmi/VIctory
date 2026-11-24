# Why the world does not load

Evidence: one real run of `victory run game_scarlett_master.exe --execute`
(build `b99f876b`, title `7952987A`, Xbox Series X profile), from the captured
log in `latest dump.txt` (local, gitignored — 3,357 lines). Everything below
quotes that log; nothing here is inferred from the title's code, which we do
not have.

## What the run actually achieved

| | |
| --- | --- |
| Frames present | 3,910 |
| Shaders translated | 596 |
| Host pipelines built | 251 |
| Shaders **refused** | 74 (distinct) |
| Title DLLs loaded | 4 (bvh builder, FidelityFX loader, libhttpclient, bink2) |
| Unresolved imports | 0 |

So the loader and the HLE are in good shape: every import bound, four title
DLLs mapped, menus ran, and the run reached the world's own GPU work (mesh plus
pixel pipelines were being bound at frame 3910, and 83 distinct mesh/pixel
pipeline pairs appear in one submission).

It then died:

```
first-chance exception 0xc0000005 at victory-image+0x23cf683
  (write 0x0: nothing mapped there)
  #0 +0x23cf683   #1 +0x2215c35   #2 +0x2215eca   #3 KERNEL32+0x1259d
```

A **write to address 0** from title code, on a thread whose start routine is
`+0x2215c35` (the same address the loader's own watchdog patch targets, i.e. a
CRT thread entry). The title then spent ten seconds in its Breakpad handler
trying to upload a minidump to `submit.backtrace.io`, which our network policy
blocked, and the process ended.

## The one concrete lead

A worker thread writing to null immediately after a batch of compute shaders
were dropped is the signature of work whose shader never materialised: the
dispatch is dropped, its output buffer stays in whatever state it had, and the
consumer dereferences a null it was given.

The refusals are therefore not cosmetic. Ranked by how often the run hit them:

| Refusal | Count | Share |
| --- | --- | --- |
| `an address in an SGPR that is no root parameter` | 31 | 42% |
| `image_gather4_l is not supported` | 7 | 9% |
| `a 64-bit image atomic is not supported` | 7 | 9% |
| `a whole-wave section step the simulation does not follow` | 6 | 8% |
| `a structured buffer of unknown stride` | 5 | 7% |
| descriptor read from the first lane of computed VGPRs | 4 | 5% |
| lane-mask join of an untracked mask | 3 | 4% |
| lane mask read as data | 2 | 3% |
| descriptor's address or format read as data | 2 | 3% |
| descriptor read only as data | 2 | 3% |
| five singletons (see the log) | 5 | 7% |

## The shape of the top refusal

Two facts from the log change what the 31 refusals mean:

1. **Every one of them is a compute shader.**
2. **Every one fails at offset `0x2c`, `0x30` or `0x40`** — the prologue, within
   the shader's first sixteen instructions.

Thirty-one *distinct* shaders all failing at the same three offsets is not
thirty-one problems; it is one systematic one. A prologue failure means our
tracking does not recognise the address the shader builds from its very first
instructions, before any of its own logic runs. The most likely cause is that a
root parameter's declared SGPR span in our layout is too small, so a register
the shader treats as part of that parameter's address falls outside every
parameter we declared.

If that is right, the same cause may also explain the crash: a compute family
of 31 kernels failing together is exactly the kind of subsystem a worker thread
would wait on and then dereference a null from.

## What was done about it tonight

**Fixed — 64-bit image atomics (7 refusals).** PR #705 is integrated
(`cd1bbc84`). It adds the two feature bits DXC calls
`AtomicInt64OnTypedResource` (SFI0 bit 22) and
`AtomicInt64OnDescriptorHeapResource` (bit 24), and lowers the operations
through `atomicBinOp` / `atomicCompareExchange` on 64-bit values. Its own test
asserts both bits are set after lowering, and the shader suite passes.

**Diagnosed, not yet fixed — the top refusal (31 refusals).** The refusal text
was misleading: it said "no root parameter" for cases where the parameter
exists but the address came from its second or later dword. It now names the
actual case (root descriptor's high half / root constant past the first dword /
descriptor heap index / no parameter at all). This is diagnostics only — no
lowering behaviour changed — because guessing at address semantics here would
risk silently mis-binding every buffer in the world.

**Deliberately not merged.** PR #694 would make `image_gather4_l` (7 refusals)
translate by dropping the LOD operand. That produces wrong texels rather than a
refusal, which is worse than the current state; DXIL's `textureGather` has no
LOD parameter, so a correct lowering needs either four `sampleLevel` taps
assembled into the 2×2 quad or a mip-specific view. Left out on purpose.

## What to do next, in order

1. **Re-run with the new refusal text.** The 31 prologue refusals now say
   which case they are. If they read "an SGPR that is no root parameter", our
   layout is not covering a register the shader uses — a parameter's SGPR span
   is too small — and that is the bug to fix first, because it takes out a
   whole subsystem at once. If instead they read "a root constant past the
   first dword", the fix is the address rule #667 already applies to one shape.
   Either way one run decides it; no guessing needed.
2. **Capture what the null write is.** Set `VICTORY_WATCH` on nothing and
   instead run with `VICTORY_CRASH_SAVE` around the faulting RVA to dump the
   registers and the thread's stack (`--features diagnostics` build). The log
   already gives the faulting RIP (`+0x23cf683`), so a stack dump there would
   show whether the null came from a dropped dispatch.
3. **Then `image_gather4_l`** with a real LOD strategy, and the whole-wave
   section step (6), which is likely a small simulation gap rather than an
   instruction gap.
4. **`structured buffer of unknown stride`** (5) deserves a look next: a
   GPU-side fix exists in PR #704, which is large and stacked, but the refusal
   itself is probably a tracking gap that could be closed in the lowerer.

## Secondary observations from the same run

Three GPU-side warnings appear once each, in the world, and none of them is
explained away:

- `view the host does not allow unordered=true format=53 flags=2` — a UAV view
  was requested on a format the host refuses to create as unordered.
- `buffer view not on an element of its host copy offset=20968080 element=32`
- `unordered buffer view not on an element of its host copy offset=98679456
  element=20`

The last two say a buffer view's offset does not land on an element boundary of
its host copy. That is the kind of mismatch that produces wrong data rather
than a refusal, so it is worth chasing once the refusals are down: a shader can
happily read shifted structure fields and then take a wild branch.

## Honest limits

- One run, one machine (Ryzen 9 7940HS, Radeon 780M, Windows 10.0.22631). No
  second capture to compare against.
- No title access: the causal chain from "shader dropped" to "null write" is a
  hypothesis consistent with the evidence, not a proven one. Point 2 above is
  how it gets proven or refuted.
- The two "method not implemented" warnings are **console** device vtable slots
  (0x1c8, 0x218) three seconds into startup, far from the crash at 16:13:54.
  They are recorded but not implicated.
- Two more declared shader gaps were reviewed and **deferred rather than
  merged**: #1067 (a descriptor read only as data takes its kind from its
  table) and #1084 (a select or carry-in on an untracked mask) each restructure
  descriptor inference in the most contested function in the tree. Each closes
  only two of the 74 refusals, and both conflict there; merging them blind
  would risk the descriptor path that everything else depends on.
- Everything above is on the local integration branch; nothing here changes the
  upstream tree.
