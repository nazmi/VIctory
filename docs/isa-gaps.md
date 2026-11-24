# Shader translation gaps, ranked by what the title actually hits

Evidence: one offline run of the translator over every shader container in the
title's `shaders.rpf`, with no GPU and no title execution:

```bash
victory translate --jobs 6 <game_dir>/shaders.rpf
```

26,503 containers (614 duplicates skipped), 26,084 programs, 40.6 million IR
instructions. Raw log kept outside the repository.

## Honest limits of this analysis

- **The RDNA 2 ISA PDF is not on this machine.** An earlier session read one
  locally; a search of this user's disk now finds no such file. Everything
  below is therefore derived from (a) this repository's own opcode tables
  (`crates/victory-shader/src/gfx10/`), (b) the translator's own refusal
  strings, and (c) the observed census. Where a claim depends on ISA semantics
  from general knowledge rather than the tree, it is marked
  `[unverified: ISA]`.
- The counts are *programs*, not distinct shaders, and are not weighted by how
  often the title runs each one. A count of 787 is 787 programs out of 25,322
  in the three stages counted; it says nothing about frame-time impact.
- The ranked list sums to 2,807, while 2,447 programs in the vertex/pixel,
  compute and mesh groups were not emitted. The difference is programs that
  report more than one distinct failure; the list is per-failure, not
  per-program.

## What translates today

| Group | Emitted | Total | Refused |
| --- | --- | --- | --- |
| Vertex and pixel | 13,042 | 14,027 | 985 |
| Compute | 1,453 | 2,582 | 1,129 |
| Mesh | 8,380 | 8,713 | 333 |
| Total, those groups | 22,875 | 25,322 | 2,447 |

(The 762 programs outside those groups are the merged LS/HS/ES/GS stages this
command does not group.)

Compute is the weakest stage: 56% of its programs translate, against 93% for
vertex/pixel and 96% for mesh. It holds 46% of all refusals while being only
10% of the programs, so it is over-represented by about four and a half times.
That matches the world-load run in
[world-load-diagnosis.md](world-load-diagnosis.md), where all 31 of the top
refusals were compute shaders.

## The gaps, in order of how many programs they cost

| # | Refusal | Programs | Where it is raised | What a fix needs |
| --- | --- | --- | --- | --- |
| 1 | a descriptor's address or format read as data | 787 | `emit/lower.rs:3826` (`OTHER`) | The program reads a descriptor's own address or format field as a value, not to access the resource. Needs the descriptor's bytes materialised as data (the console's 4-dword descriptor is in memory; ours is not). |
| 2 | a descriptor read only as data | 620 | `emit/lower.rs:3856` | Same family: the descriptor is loaded but never used to access a resource, so no host view exists to hand back. |
| 3 | a lane mask read as data | 270 | `emit/lower.rs:265` (`LANE_MASK`) | EXEC/VCC spilled to a VGPR lane and then read as an integer. Needs the mask tracked through the spill rather than refused. |
| 4 | a lane mask holding lanes elsewhere read as data | 263 | `emit/lower.rs` (same family) | The mask came from another lane's view (readlane/DPP), so its bits are not this lane's. |
| 5 | an address in an SGPR in a gap between root parameters | 195 | `resource.rs` `why_no_base` | Our declared root-parameter layout leaves a register undeclared that the shader uses as an address. The message (added in `bfed7870`) now names this case specifically. |
| 6 | a structured buffer of unknown stride | 171 | `emit/lower.rs:7627`, `7710`, `7775` | The FSSM used-resource record carries no usable `stride` (0, below 4, or not a dword multiple). The value is in the record (`native.rs` `UsedResource.stride`); the fix is either reading it correctly or deriving it from the descriptor. |
| 7 | an address not from a root parameter | 73 | `resource.rs` | The address's base traces to something that is not a user SGPR at all. |
| 8 | a select on an untracked mask | 61 | `emit/lower.rs` | `v_cndmask` on a mask the tracker never saw defined (a select or carry-in). |
| 9 | a cross-lane operation outside compute | 52 | `emit/lower.rs` | A wave operation in a pixel or mesh stage, where the console's lane mapping differs from the host's. |
| 10 | a whole-wave section step the simulation does not follow | 51 | `emit/lower.rs:859` | The whole-wave simulator (`emit/wave.rs`) understands only a handful of VALU ops inside an `s_or_saveexec -1` / `s_nand_saveexec 0` section; ordinary arithmetic, LDS or a non-constant EXEC write inside one makes every value leaving it opaque. |
| 11 | a descriptor chosen by control flow | 40 | `emit/lower.rs` | Two descriptors joined across a branch, where the join is not a shape the tracker models. |
| 12 | a descriptor read from the first lane of computed VGPRs | 38 | `resource.rs:2821` | A waterfall-style first-lane read whose address the tracker cannot follow. |
| 13 | barycentrics read directly | 34 | `emit/lower.rs` | The program reads the barycentric inputs as raw values rather than through an interpolation. |
| 14 | an address from a phi | 28 | `resource.rs` | Two different root parameters meeting at a phi; needs the join to keep both candidates. |
| 15 | the first half of an interpolation | 20 | `emit/lower.rs` | An interpolation whose half is emitted separately from its pair. |
| 16 | a constant buffer's descriptor read as data | 17 | `emit/lower.rs` | CBV descriptor read as data; same family as #1/#2. |
| 17 | a hardware SGPR input | 17 | `emit/lower.rs` | A hardware-initialised SGPR (e.g. scratch offset) read as a value. |
| 18 | a waterfall loop whose compares do not select the access's lanes | 15 | `resource.rs` | The waterfall loop's compare does not match the mask the access uses. |

### The dominant family

Items 1, 2, 16 and to a degree 11 and 12 are one theme: **the program reads
descriptor bits as data**. Together that is at least 1,424 of the 2,807
failures — more than half, and more than every instruction-encoding gap
combined.

This is the same area two upstream PRs were deferred over (#1067, #1084): they
restructure descriptor inference in `resource.rs`, the most contested function
in the tree, and each closed only two refusals of the 74 seen in one run. The
census says that judgement was too pessimistic about the *size* of the prize
(1,400 programs, not 2) while still being right about the *risk*: a wrong
descriptor answer mis-binds buffers silently rather than refusing. It needs a
dedicated, tested design, not a patch.

## Instruction-level gaps

These are single mnemonics the decoder recognises and the emitter does not, and
they are cheap because each is self-contained:

| Mnemonic | Programs | Note |
| --- | --- | --- |
| `v_cvt_f32_f64` | 4 | No handler in `emit/`. DXIL has f64; this needs a 64-bit source path (two VGPRs to one f64). |
| `s_getpc_b64` | 4 | Program-counter-relative addressing in the emitter. `resource.rs` already models `CODE` addresses, so the information exists. |
| `v_permlanex16_b32` | 1 | The x16 permute; `v_permlanex16` exists in the table but has no lowering. |

Also decoded but never named in `emit/` (no program in this census hit them, so
they are latent rather than urgent): `s_call_b64`, `s_setpc_b64`, `s_rfe_b64`,
`s_cbranch_execz`, `s_cbranch_execnz`, `ds_gws_init`, `ds_append`,
`ds_ordered_count`, `s_scratch_load_dword`, `s_memtime`, `s_memrealtime`,
`s_atc_probe`. Several families are handled by prefix and are *not* gaps
(`ds_read*`, `ds_write*`, `buffer_load*`, `buffer_store*`, `tbuffer_*`,
`image_*`, `s_buffer_load_dword*`).

## What to do next, in order

1. **`s_getpc_b64` and `v_cvt_f32_f64`** — 8 programs, two self-contained
   lowerings, no descriptor work. Cheap and low risk.
2. **`a structured buffer of unknown stride`** (171) — the stride is a field
   the parser already reads; this is a correctness bug in how it is matched,
   not a missing feature. Worth an afternoon with a synthetic fixture.
3. **The lane-mask family** (533 across two entries) — spill and cross-lane
   mask tracking, contained in `resource.rs` and the mask table.
4. **The descriptor-as-data family** (1,424+) — the real prize, and the real
   risk. Needs a design note first: how the host represents a descriptor's own
   four dwords as readable data, and how that interacts with the root
   signature. Do not start it as a patch.

## Verification

`victory translate` is offline and needs no GPU, so any of these can be
re-measured with the same command and the refusal counts compared directly.
A fix is done when its count reaches zero and the workspace gate is green; a
fix that lowers a count by converting a refusal into a *wrong value* is worse
than the refusal and must not be counted as progress.

## A gap this census cannot see: spilled user-data tables

The 31 compute refusals from the runtime run read "an address in an SGPR
before the first root parameter", and **none of those 31 shader hashes appears
in the offline census at all** — not in `shaders.rpf`, and not in `data.rpf`,
`levels.rpf`, `packs.rpf`, `navmeshes.rpf`, `anim.rpf` or the `pack/`
directory, all of which report zero containers or zero matches. So the runtime
is translating shaders the `translate` subcommand does not reach, and the
"before the first root parameter" message cannot be reproduced offline. Treat
that message as unconfirmed until it is.

A concrete candidate for what those shaders are comes from a third-party fork
(`github.com/fxceless/victory-fork`), whose `crates/victory-shader/src/spill.rs`
describes a construct this tree does not model at all:

> A compute program has as many user SGPRs as `COMPUTE_PGM_RSRC2` says. A root
> signature that needs more dwords than that keeps none of its parameters in
> registers: user-data dwords 0 and 1 then hold the 40-bit address of a table
> in memory (`s0`, and `s1 & 0xff` as bits 32-39), and the parameter the layout
> declares at user-data index N (16 or above) is at byte `4 * (N - 16)` of that
> table.

Our `resource.rs` maps every root parameter to user SGPRs through
`HwStage::user_sgpr(p.user_data)` and has no concept of a parameter living in a
memory table, so such a program's addresses resolve to registers that hold a
*pointer*, not a parameter — which is exactly the shape that would produce an
address whose base is not any declared parameter. The fork says it measured
this on the console's ray-tracing builder, `XboxBvhBuilder.dll`: 118 of 118 such
loads land on a declared parameter.

That is a hypothesis, not a confirmed diagnosis: nobody has yet shown one of the
31 offending containers, and the fork's claim about `XboxBvhBuilder.dll` is
its own. The way to settle it is to capture the refused containers (the GPU side
already logs each hash) and run one through the translator with its layout
printed.
