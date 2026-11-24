# JMAN730/victory assessment

Date: 2026-11-24. User requested duplicate checking and conditional merge
after verification. **Decision: defer a wholesale merge.** Normal validation
passes, but two additional focused checks reproduce failures.

## Exact scope

- Source: [JMAN730/victory](https://github.com/JMAN730/victory), main
  `179d34e2ba0de1357d040a8e996e6787148a1b04`.
- Upstream base: `ffe310eb2098e52169f4c44db5ffdabf43a72622`.
- Difference: 275 commits; 184 files, 34,578 insertions, 2,414 deletions.
- No source commit from this fork has been integrated into our branch.
- Original fork-review worktree remained clean. Failing sampler regression
  was added only in an isolated scratch worktree; no production code changed.

## Duplicate candidates

[Complete historical map](maintenance-jman-duplicates.md): all 445 PRs from
the fully paginated inventory were compared. Stable nonmerge patch history
fully covers 103 PRs, partially covers 20, has no individual patch match for
320, and identifies two changed heads requiring refresh.

These are history matches, not proof that later fork commits preserved the
behavior. In particular:

| Source PR | Fork finding |
| --- | --- |
| #449 | Exact patch-history match at 24f1e1d; null AnalyticsInfo output guard |
| #480 | Exact patch-history match at 60ef1ea0; strict percent escape decoding |
| #363 | Equivalent empty-config behavior recognized in broader config rewrite |
| #406 | Equivalent incomplete-relocation-entry handling recognized |
| #708 | Related save/cache separation, but different XDG fallback paths |
| #396 | Optional-header bounds fix absent; do not mark covered |
| #689 | Similar frame-test correction exists, but no exact historical patch match |

## Normal source validation

Economical verification worker: `gpt-6.1-sol`, low reasoning. Explicit Rust
1.98.1 x86_64 Windows MSVC; fresh external build target.

| Command | Exit |
| --- | --- |
| cargo clippy --workspace --all-targets --locked -- -D warnings | 0 |
| cargo clippy --workspace --all-targets --all-features --locked -- -D warnings | 0 |
| cargo test --workspace --locked | 0 |
| cargo xtask policy | 0 |
| cargo fmt --check | 0 |
| cargo build -p victory --release --locked | 0 |

Workspace reports 794 harness passes and 18 ignored: 14 benchmarks, three
external format/XMA inputs, and one DualSense hardware test. Four shader
differential tests returned early without console/PC compilers. Some other
successful tests can also return early when devices/voices are unavailable;
the captured default output does not prove those branches ran. Policy
accepted 213 files using the fork's checker, which adds three explicitly
named Python/Swift Metal test files to its accepted source paths.

No title/game execution or playability validation was performed. WARP is
selected by VICTORY_D3D12_WARP only in diagnostics builds; the default suite's
adapter identity was not established.

## Reproduced blocker: sampler table remapping

Priority: high. Source locations: `crates/victory-gpu/src/draw.rs:824-895`,
`descriptor.rs:175`, `descriptor/host.rs:118-132`, and shader
`emit/lower.rs:4159-4162`.

The capped sampler heap keeps dynamic slots starting at 1792. A sampler
table pointing at title sampler 2048 in a synthetic 2049-entry heap takes
the fresh-table path. That path uses view-descriptor spacing, including
returning `first * 32`, although sampler descriptors occupy 16 bytes and
the shader divides the byte offset by 16. The first remapped sampler is
therefore addressed as 3584 rather than 1792, and the shader heap-limit
clamp substitutes slot 0. Later entries also use the wrong spacing.

A new test in the scratch copy called the actual `State::tables`, using
the existing descriptor-heap and host-list code. It required successful host
initialization, rather than silently skipping, with diagnostics and WARP.
It released the heap before asserting. Exactly one test executed and failed:

```text
maintenance_capped_sampler_table_uses_sampler_increment
left:  [(0, 57344)]
right: [(0, 28672)]
exit: 101
```

Command: `cargo test -p victory-gpu --features diagnostics --locked
maintenance_capped_sampler_table_uses_sampler_increment -- --nocapture`.
No command list was submitted and no title inputs were used. The full test
patch and log remain outside the repository in the maintenance-data directory
as `jman-sampler-repro.patch` and `jman-sampler-repro.log`.

Acceptance: use sampler increments throughout fresh-table sizing, hashing,
source reads, entry placement, and returned byte offsets. Test both initial
and cached remaps, multiple sampler offsets, and normal view tables.

## Reproduced blocker: conditional Agility SDK linker failure

Priority: high. Source: `crates/victory-shader/build.rs:35-36` and
`crates/victory-shader/src/lib.rs:13`.

When the app-local runtime file exists, the build script emits unrestricted
link arguments exporting D3D12SDKVersion and D3D12SDKPath. The library
defines those symbols only under cfg(test). An external integration-test
binary links the normal library and does not provide those definitions.

Reproduction used a fresh, separate external target containing a zero-byte
synthetic file at `debug/D3D12/D3D12Core.dll`. Only existence is needed to
activate this build-script branch. The file was never loaded or executed.

```powershell
cargo +1.98.1-x86_64-pc-windows-msvc test -p victory-shader --test resource_tracking --no-run --locked
```

Result: exit 101; LNK2001 for D3D12SDKPath and D3D12SDKVersion, then LNK1120
for two unresolved externals. Full log: external `jman-agility-repro.log`.

Acceptance: export symbols only from targets that define them, or provide
the definitions in every opted-in host target without duplicate application
symbols. Validate unit tests, integration tests, and benches with and without
the app-local-runtime file; report copy failures rather than ignoring them.

## Other review concerns and coverage

- Core/PE/loader worker reviewed 24/24 assigned files, including five tests
  excluded by OCR defaults. Title-specific watchdog hashes, patch sites,
  and environment handling live in the generic loader; the user's workflow
  requires title-specific behavior in victory-game.
- Without LOCALAPPDATA, the fork changes cache defaults from
  XDG_CACHE_HOME/victory/cache to XDG_CACHE_HOME/victory. Pointing only
  VICTORY_DATA_DIR at the previous cache-root/victory/data is then rejected
  as inside the new cache root. Preserve both paths explicitly or provide
  migration guidance before replacing existing configuration behavior.
- Focused GPU review also identified mixed-size fence atomics. Upstream
  already used non-atomic writes to fence storage; this is not represented
  as an entirely new race. The fork's 32-bit store still conflicts with
  64-bit fence reads/watcher accesses without a shared synchronization
  protocol. See the [Rust atomic memory model](https://doc.rust-lang.org/std/sync/atomic/).
- Broader GPU/shader and remaining fork-file review is incomplete. OCR
  preview counted 184 changed files, 147 default reviewable and 37 excluded.
  The two reproduced blockers are sufficient to defer bulk integration;
  no claim of a completed 184-file audit is made.
- Our Windows MSVC host pin would also need reconciliation with newly
  imported Linux CI jobs. Preserve the user's orchestration file rather
  than adopting instructions pointing issue operations at another fork.

Continue the main maintenance queue with individually reviewed and tested
changes. Do not treat the 103 historical matches as accepted integrations.

## Portable sampler regression

Insert this synthetic test in the existing depth_tests module in draw.rs of the
pinned JMAN source, in an isolated checkout. Run with diagnostics and
VICTORY_D3D12_WARP=1 using the command above. It is expected to fail there;
it is not an accepted change to our implementation.

```rust
    #[test]
    fn maintenance_capped_sampler_table_uses_sampler_increment() {
        let host = crate::host::host().expect("a real host device is required for this regression probe");
        #[repr(C)]
        struct ViewHeap {
            desc: D3D12_DESCRIPTOR_HEAP_DESC,
            placement: [u64; 2],
        }
        let desc = ViewHeap {
            desc: D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
                NumDescriptors: 2049,
                Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
                NodeMask: 0,
            },
            placement: [0; 2],
        };
        let iid = ID3D12DescriptorHeap::IID;
        let mut heap = std::ptr::null();
        assert_eq!(crate::descriptor::create_heap(
            std::ptr::null(), (&raw const desc).cast(), (&raw const iid).cast(), &mut heap,
        ), crate::object::S_OK);
        let table = crate::descriptor::heap(heap).start + 2048 * 16;
        let used = UsedResource {
            kind: ResourceType::Sampler,
            root: RootType::Table,
            table_offset_dwords: 0,
            slot: 0,
            user_data: 0,
            dwords: 4,
            lower_bound: 0,
            stride: 0,
        };
        let list = host.new_list().expect("host command list");
        let mut state = State::default();
        state.set_heaps(&[heap]);
        state.set_table(false, 0, table);
        let remapped = state.tables(&list, &[used], false);
        list.close();
        drop(state);
        drop(list);
        let release = crate::object::slot::<extern "system" fn(This) -> u32>(heap, 0x10);
        assert_eq!(release(heap), 0);
        assert_eq!(remapped, vec![(0, 1792 * 16)]);
    }

```
