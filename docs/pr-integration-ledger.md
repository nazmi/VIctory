# PR integration ledger

Which upstream pull requests are in this branch, which are not, and why.

Generated from git, not typed by hand: one row per commit between upstream
`ffe310eb` and the branch tip, joined to PR numbers through the
`(cherry picked from commit <sha>)` trailers and the local integration tables
in [maintenance-log.md](maintenance-log.md). Regenerate it whenever commits are
added; do not edit the tables by hand.

## Summary

For a disposition on **every** currently open PR — including the ones with
no decision yet — see [pr-disposition.md](pr-disposition.md). This file only
lists what is in the branch.

| | Count |
| --- | --- |
| Upstream PRs integrated locally | 182 |
| Local fixes with no upstream PR | 72 |
| Maintenance records (docs only) | 21 |
| Total commits since upstream base | 271 |

Nothing here has been pushed upstream: every row is a local integration on
`integration/maintenance`, and no upstream PR has been opened, merged or
closed by this work.

## Not integrated, with the reason

| PR | Disposition | Reason |
| --- | --- | --- |
| #1675 | duplicate | `s_absdiff_i32` / `s_sext_i32_i8`/`i16` / `s_pack_{ll,lh,hh}_b32_b16` are already lowered at `lower.rs:3567`. Its device-value test is stronger than our translation-only test and is worth salvaging separately. |
| #1656 | rejected (defect) | `pooled()` returns a length-0 `Vec` (`v.clear()`), and the new copy-back code indexes it (`&mut piece[..n]`), so it panics on the first piece of every write-back. Its only new test is `#[ignore]`d. Needs a `resize` and a non-ignored `copy_back` test. |
| #1694 | held | Wholesale apply would revert the single-rounding `v_dot2_f32_f16` fix we already committed; its `neg_hi` bit 2 handling also turns a refusal into a plausible wrong value. |
| #1684 | held (partial) | Partly already in the tree; re-applying collides with arms we carry. |
| #1654 | held pending fix | Bit-64 and clamp rules verified correct, but the inline constant 248 is expanded from the f64 `FRAC_1_PI*0.5` while 240-247 are widened f32s — a silent wrong value for that one encoding. Also the `binaryWithTwoOuts` high/low order is unverified. |
| #1662 | held pending fix | Same 248-as-f64 defect as #1654; window and clamp logic otherwise verified. |
| #1663 | held | ADDTID logic is right, but it stops forcing wave32 for shaders whose LDS access may be a `ds_bpermute`/`ds_swizzle`, and an out-of-bounds ADDTID returns 0 instead of refusing. |
| #1669 | queued | Reviewed as a candidate; not integrated in this wave. |
| #694 | rejected | Would make `image_gather4_l` translate by dropping the LOD operand: wrong texels rather than a refusal. |
| #1067 | deferred | Restructures descriptor inference in the most contested function, for two refusals. |
| #1084 | deferred | Same contested region as #1067, for two refusals. |
| #693 | deferred | Moves shadow buffers to VRAM, which attacks the world's memory duplication directly, but the reviewed version left uploaded shadows in `COPY_SOURCE` with no transition back to `COMMON`. |

## Upstream PRs integrated locally

| PR | Upstream head | Local commit | Date | Subject |
| --- | --- | --- | --- | --- |
| #17 | `b07dfe3d4e7f` | `7ea49a89` | 2026-11-24 | CI: check the formatting with cargo fmt |
| #24 | `d5f41f54c49b` | `d819400b` | 2026-11-24 | Run indirect mesh dispatches through indirect execution |
| #76 | `dea90ccc7e87` | `f464bd96` | 2026-11-24 | xmem: map a run of physical pages over several placeholders |
| #82 | `df5e928aa0c5` | `6399ea71` | 2026-11-24 | HLE: speak the title's narration with the host's speech synthesizer |
| #92 | `4f2d5dbd8f1c` | `a47f488e` | 2026-11-24 | shader: lower v_add_nc_i32, v_sub_nc_i32, v_mul_lo_i32 and v_add_nc_i16, v_sub_nc_i16 |
| #96 | `ba42db643c15` | `bbf60d7f` | 2026-11-24 | xma: start the frame after a continuation at the next packet that has frames |
| #98 | `f468457a1859` | `5ceb0e1f` | 2026-11-24 | fix: model v_permlane16_b32 in whole-wave sections |
| #105 | `6e6d3fc027d0` | `4f6dac61` | 2026-11-24 | fix: accept shader loops that branch to the first instruction |
| #114 | `0b729de6ee7b` | `5d8955e0` | 2026-11-24 | XAsync: run a failed call's cleanup without the completion lock |
| #115 | `b1bdd8994a45` | `b0d2e84f` | 2026-11-24 | Census: report directory traversal failures |
| #119 | `095e44a0bc15` | `4502afa1` | 2026-11-24 | Query allocation type directly when guarding mapped memory |
| #120 | `cb40cdcc21a7` | `9df49c0f` | 2026-11-24 | Shaders: lower s_absdiff_i32, s_sext_i32_i8/i16 and s_pack_ll/lh/hh_b32_b16 |
| #122 | `d01b17070dbf` | `461c174f` | 2026-11-24 | XAsync: end the call that finished, not whatever call the block holds now |
| #126 | `e5b715983242` | `05c691d0` | 2026-11-24 | Shaders: walk a lane-mask diagram once in vars and given |
| #133 | `01ae55a3ba04` | `13e4e7fe` | 2026-11-24 | Files: a title run by its file name alone gets its own folder as G: |
| #139 | `697c4bcd1b5e` | `0b3db22a` | 2026-11-24 | Shaders: reuse SSA sums when tracing memory addresses |
| #148 | `a0380049aacd` | `3ff3ce73` | 2026-11-24 | Shaders: lower s_bcnt0_i32_b32/_b64, s_ff0_i32_b32/_b64 and s_flbit_i32_b64/_i64 |
| #151 | `d77408bb71e1` | `db44521f` | 2026-11-24 | Link victory.exe with a 16 MiB default thread stack |
| #153 | `3e98bd7e0ce7` | `c051b5da` | 2026-11-24 | GPU: seek directly to private data on final release |
| #158 | `520d629ed178` | `9a6d9e67` | 2026-11-24 | GPU: the depth bounds test reaches the host |
| #172 | `4eb0898b3cbd` | `5ba70891` | 2026-11-24 | XMA: build the inverse transform's rotations once per subframe size |
| #177 | `b7300eb5b86c` | `c68f6e1a` | 2026-11-24 | Shaders: test that an image read as float and used by an atomic is annotated twice |
| #182 | `18275322e4d3` | `aa370a59` | 2026-11-24 | Files: `..` in a title path stops at the virtual drive's root |
| #183 | `d4d1822049fe` | `a6df1a25` | 2026-11-24 | XMA: noise tests, and end a frame at its data instead of looping forever |
| #186 | `3c7d845b3bf4` | `791580b7` | 2026-11-24 | Kernel: a title image's file name that fits clears the last error |
| #188 | `2a6e6a4bcea5` | `85834d42` | 2026-11-24 | Loader: find TLS-array load candidates sixteen offsets at a time |
| #190 | `ffa13108fef5` | `a4023b67` | 2026-11-24 | GPU: build the indirect-argument clamp with our own writer |
| #194 | `985d8d4b0253` | `7e18d623` | 2026-11-24 | Audio: an XMA loop's skip is of its first frame, not of its last |
| #197 | `64484b426ba8` | `ade65f82` | 2026-11-24 | Input: apply VICTORY_KEYS to a connected host pad too |
| #202 | `dcf666486a3c` | `ae526377` | 2026-11-24 | GPU: skip unmapped regions after a failed bulk read into host memory |
| #207 | `11ca3ce4928c` | `188340ae` | 2026-11-24 | Media: a share is refused however it is spelled |
| #213 | `7f00a62a4b86` | `106de91e` | 2026-11-24 | Crash reports: name the host's CPU and Windows build |
| #214 | `9de712f5a1c6` | `ba88b6ef` | 2026-11-24 | Loader: clzero loops in one exception; rdpru |
| #221 | `acdcc41687be` | `18113a3c` | 2026-11-24 | Loader: suspend a thread before reading its TEB; skip bad dump times |
| #224 | `b87a9682e346` | `65c02b0e` | 2026-11-24 | HLE: report the host's commit headroom before the title starts |
| #228 | `09ff84b6a7cb` | `f53da6cc` | 2026-11-24 | XGameSave: a failed create leaves a null handle, as documented |
| #244 | `7f95b87ea945` | `45a4455e` | 2026-11-24 | Shaders: share and serialize WARP host operations across threads |
| #247 | `5cff0007a525` | `2685c9ed` | 2026-11-24 | XAsync: a new block where a result was left behind begins a call |
| #261 | `a2243adc60d8` | `ee4a9ae6` | 2026-11-24 | GPU: copy-back puts back only what shaders wrote, in two more cases |
| #270 | `92f2c60c8159` | `0739961f` | 2026-11-24 | GPU: every object answers for IUnknown |
| #280 | `7ecc6209b8b3` | `25c43bc2` | 2026-11-23 | README: the first screen stays black for a while when the game is on a hard drive |
| #286 | `6afdb58eeb87` | `869cb862` | 2026-11-24 | Shaders: accept known D3D_MIN_PRECISION signature elements |
| #288 | `f177aaeade9f` | `2f2fbe49` | 2026-11-24 | Shaders: cap BDD memo table and bail out early on overflow |
| #310 | `1ea6427f0507` | `6d897c25` | 2026-11-24 | Policy: resolve cache paths inside Windows AppContainer |
| #317 | `8133754ea5af` | `f9b5d3b9` | 2026-11-24 | Shaders: support cube and MSAA dimensions in descriptor_data |
| #319 | `b36bd4c1d655` | `f9800117` | 2026-11-24 | GPU: end a late flip wait on the tick it missed, not the one after |
| #322 | `b58d198d954e` | `894b7462` | 2026-11-24 | GPU: copy into title memory without asking for its region first |
| #323 | `e08ac6a9dc6b` | `f71bd69d` | 2026-11-24 | docs: center the GTA V port notes on "world loads, game does not start" |
| #324 | `8fc3a1bb2f37` | `f9eb3bf0` | 2026-11-24 | fix(pe): detect incomplete TLS callback arrays |
| #328 | `91e7742f9852` | `64d7d016` | 2026-11-24 | Shaders: lower MSAA image_load and image_get_resinfo |
| #335 | `e3bef089855c` | `22309608` | 2026-11-24 | XStore: a store id without an installed package is an invalid argument |
| #337 | `` | `feeff994` | 2026-11-24 | fix(hle): saturate storage quota conversion |
| #343 | `9459c12d2e3a` | `fd228fb5` | 2026-11-24 | fix(shader): reject overlapping DXBC parts |
| #345 | `` | `90a5dd2d` | 2026-11-24 | docs(pt-BR): Brazilian Portuguese translation and install guide |
| #351 | `93a24b7cdaeb` | `e4b302b1` | 2026-11-24 | fix(formats): reject overlapping texture depth slices |
| #353 | `e6ebec9ccf0b` | `086a3126` | 2026-11-24 | Shader info: show stage I/O even when instruction decoding fails |
| #354 | `9f28f6b9845b` | `f2f21ab5` | 2026-11-24 | docs: explain Series S render profile and memory limits |
| #357 | `c6959acba383` | `83bd896d` | 2026-11-24 | HLE: load a host DLL outside the module cache's lock |
| #359 | `d549b6397e3f` | `e30d5d3b` | 2026-11-24 | HLE: a '>' in a quoted manifest attribute does not end its tag |
| #361 | `3f5eff576032` | `e9ca0d50` | 2026-11-24 | input: with no device, previous and next readings walk every device in time order |
| #363 | `4874c3fdc18e` | `4dbc5a71` | 2026-11-23 | Config: ignore an empty VICTORY_CONFIG |
| #364 | `e503756597ca` | `6de0fa33` | 2026-11-24 | perf(xma): accelerate VLC decoding with prefix lookup |
| #370 | `936e5e51c91c` | `099c2f65` | 2026-11-24 | XGameUi: a text entry's limit is the console's |
| #373 | `c000581df2cf` | `3fdb9879` | 2026-11-24 | Kernel: GetModuleFileName of a title image into a null buffer |
| #377 | `4da59d107136` | `56252f48` | 2026-11-24 | policy: fail when staged content cannot be read |
| #378 | `2392dfd79a19` | `9a68bad9` | 2026-11-24 | CLI: show runtime hashes in shader listing |
| #380 | `43551948876e` | `6856bcb3` | 2026-11-24 | github: add a bug report issue form |
| #390 | `e125477cbd6d` | `589ea43d` | 2026-11-25 | Crash log: a panic carries our Rust stack |
| #396 | `10e9f56a5021` | `27dd4a73` | 2026-11-24 | pe: bound data directories to the optional header |
| #399 | `54f8cee58682` | `e7ac86c6` | 2026-11-24 | xgamesave: preserve older versions on manifest error |
| #406 | `b33c704e3335` | `0c8cd0e9` | 2026-11-24 | loader: ignore incomplete relocation entries |
| #412 | `d2c7d4a329ab` | `1cbf4b9c` | 2026-11-24 | HLE: validate XPackage chunk selectors |
| #415 | `6924b095e3bf` | `e9aecbd8` | 2026-11-24 | GPU: use native root type codes in the shader cache |
| #422 | `a746d533f407` | `f7d2faf8` | 2026-11-24 | Shaders: narrow f32 to f16 to nearest even, as the console does |
| #424 | `5b9a9b37ef26` | `a751c8a3` | 2026-11-23 | README: quote the executable path in the PowerShell launch example |
| #425 | `012ce71c5a8c` | `2c9c3325` | 2026-11-24 | Game: validate DXBC containers in SGD5 records |
| #428 | `a312333c28eb` | `ca8d2253` | 2026-11-24 | GameInput: next and previous readings of a disconnected device fail |
| #430 | `129d2897c43f` | `b69d6856` | 2026-11-24 | DirectStorage: Close completes what was submitted |
| #446 | `881caef98fa0` | `efff47d2` | 2026-11-24 | HLE: report required size on string getter buffer errors |
| #447 | `7d049431de9b` | `5a24976b` | 2026-11-24 | Shaders: a signed 64-bit operand sign-extends its 32-bit literal |
| #449 | `f625cc508c0d` | `c64f8aab` | 2026-11-24 | HLE: refuse a null AnalyticsInfo output pointer |
| #451 | `8ff871b71c5a` | `e969a76c` | 2026-11-24 | HLE: keep fitted window dimensions positive |
| #456 | `4b8a1ca6dd0d` | `35a3b2de` | 2026-11-24 | shader: raise v_interp_p1lv_f16 |
| #460 | `8ce597b2beea` | `66a3c2c1` | 2026-11-24 | Metal: add native Apple Silicon GPU conformance harness |
| #475 | `2c22e2b2d23e` | `aa801cbb` | 2026-11-24 | XStore: wire offline *Result slots and fix OOB XStoreShowProductPageUIResult slot |
| #476 | `a3f89eda5476` | `abfa4d3f` | 2026-11-24 | Census: deduplicate PE imports without hint |
| #477 | `bd59ba346da7` | `c26bcc20` | 2026-11-24 | Shaders: fix DXIL validator UB on null error blobs, unchecked GetStatus, and COM/DLL handle leaks |
| #478 | `7ac2760d317c` | `93324593` | 2026-11-24 | Audio capture: guard WASAPI timestamp errors/underflow, null 0-frame slice UB, unbounded silence al… |
| #480 | `75fe6c02b278` | `06daf0f7` | 2026-11-24 | HLE: require two hex digits in percent escapes |
| #482 | `c91e6b6b20e3` | `42c80756` | 2026-11-24 | App: reject wrapping RVAs in VICTORY_WATCH |
| #488 | `fba33207c4ff` | `c8de2412` | 2026-11-24 | GPU: skip zero-sized frame captures |
| #494 | `0a96e1e09068` | `d5a3f66e` | 2026-11-24 | docs: add Indonesian translation (languages/README.id.md) |
| #500 | `bdbaac636524` | `a5ccbffd` | 2026-11-24 | GPU diagnostics: pick capture frames by draw count |
| #505 | `d4c8abf932a1` | `b9f70206` | 2026-11-24 | tests: add synthetic Windows runtime conformance under Wine |
| #530 | `951ceb1eb8da` | `2f0a1740` | 2026-11-24 | GPU diagnostics: show one of the title's render targets in the window |
| #532 | `aa50c7d6481e` | `f4d0697a` | 2026-11-24 | Record cutscene rendering evidence and the separate Proton startup failure |
| #537 | `2021e0f26970` | `67f5d656` | 2026-11-24 | GPU: name the switch when a removal has no device-removed data |
| #538 | `65062e133c75` | `4efb34b6` | 2026-11-24 | xmem: report allocations the host refuses |
| #543 | `` | `95ee7d24` | 2026-11-24 | QA: update formatting checkbox wording in PR template |
| #547 | `19cf287d0d9c` | `a8b8e88d` | 2026-11-24 | QA: add rustfmt.toml to standardize workspace formatting |
| #553 | `b796c8b8b80c` | `5f095aad` | 2026-11-24 | GPU: clear DXGI outputs on out-of-range enumeration |
| #560 | `1aac20aa8fd9` | `fb3c0578` | 2026-11-24 | fix(loader): stage prepared images in per-writer files |
| #563 | `10ac6ceac928` | `f90c170e` | 2026-11-24 | Shaders: cube samples with gradients, and image_sample_d_cl |
| #569 | `aed9637927e7` | `0860ed7a` | 2026-11-24 | Shaders: biased samples with 16-bit coordinates |
| #572 | `0a1a702bb440` | `34370fce` | 2026-11-24 | Docs: add a PowerShell quickstart for running VIctory |
| #577 | `40816378b598` | `e532ec98` | 2026-11-24 | dstorage: keep queued file references alive and validate null out pointers |
| #579 | `1658c8452a4a` | `13008f98` | 2026-11-24 | GPU: VICTORY_SKIP_WAVE64 omits draws of wave64 programs of listed stages |
| #580 | `` | `2db7897e` | 2026-11-24 | Docs: keep unrelated formatting out of contributor changes |
| #589 | `ab2e7cc3f648` | `005d6219` | 2026-11-24 | fix(shader): skip unused image atomic descriptor reconstruction |
| #603 | `705a2f634d1c` | `dae0a292` | 2026-11-24 | Inline uniform micro-tile runs in texture copies |
| #615 | `a4974e4c21c1` | `bf57211f` | 2026-11-24 | GPU: VICTORY_* switches set to 0 are off |
| #616 | `94bc7cf85130` | `1af23c07` | 2026-11-24 | GPU: rectangle draws check their bounds without 32-bit overflow |
| #617 | `81b81df35143` | `8042c6b7` | 2026-11-24 | Shaders: reuse lane guards for unchanged EXEC within a block |
| #618 | `691236d606b9` | `9c9e46f6` | 2026-11-24 | GPU: align appended input-element offsets like D3D12 |
| #620 | `59aa44b0e0de` | `cfd019c2` | 2026-11-24 | HLE input: pass trigger motor levels to the host source |
| #629 | `350f91c23a94` | `25e797e9` | 2026-11-24 | Formats: reject multisampled linear texture layouts |
| #631 | `e009e13aa479` | `a157f3fe` | 2026-11-24 | Add tests for Console struct functionality |
| #636 | `7216ab711311` | `3412fd91` | 2026-11-24 | HLE: implement the RPC runtime's GUID calls |
| #640 | `446fef929c85` | `842ed08b` | 2026-11-24 | Retry failed pipeline cache saves |
| #641 | `f887f6a3f7bc` | `15907832` | 2026-11-24 | GPU: upload partially-resident streamed textures |
| #642 | `` | `f5f97470` | 2026-11-24 | Test: use exact UNORM clear values in enhanced barrier test |
| #643 | `` | `d1ba3944` | 2026-11-24 | Test: use exact UNORM clear values in aliased texture test |
| #644 | `6fa53a51c06f` | `b33b0fed` | 2026-11-24 | GPU: spill oversized host root signatures' user data to root CBVs |
| #646 | `12eba495039d` | `42f24d56` | 2026-11-24 | Shaders: fix whole-wave EXEC inline constant sign-extension and prefix lane validation |
| #648 | `241b08f49439` | `d0462736` | 2026-11-24 | HLE: implement XUser maximum-users slot 0x30 (#348, #622) |
| #649 | `43e2ca9de29f` | `d0952126` | 2026-11-24 | Show on-disk file size in victory inspect |
| #651 | `d615ee0fe0f9` | `ce478cfb` | 2026-11-24 | GPU: select the readable format and plane for depth/stencil SRVs |
| #652 | `ea09cb04bc0c` | `6a836d63` | 2026-11-24 | GPU: use selected mip extents for MSAA resolves |
| #656 | `3b001ed80c97` | `39a46807` | 2026-11-24 | Input: report the character each held key types |
| #658 | `0ecaa8cb92ac` | `7d5fa089` | 2026-11-24 | BCPack: read BC7 dictionary indices from the right offset |
| #659 | `98fe8fe840ba` | `2ece2f86` | 2026-11-24 | Shaders: refuse per-lane reads of mesh v5 in v5 writes and after joins |
| #661 | `6325df752c56` | `d4426690` | 2026-11-24 | Shaders: print only the operands an instruction has in SP3 text |
| #663 | `d4dcd0b63ad1` | `3c1aca6c` | 2026-11-24 | media: apply the local-file rule to the source reader and MFCreateFile |
| #664 | `988fd9a64718` | `d350b132` | 2026-11-24 | translate: refuse a VICTORY_EMIT_DUMP directory inside the repo |
| #665 | `e950e8175634` | `7fc9f665` | 2026-11-24 | xtask: count \x escapes and 0x hex arrays as encoded runs in the policy check |
| #668 | `5e9d60d134e2` | `3ad7f8ef` | 2026-11-24 | shader: support f16 omod and saturate/apply modifiers in v_cvt_{u16,i16}_f16 |
| #669 | `` | `f6aa6695` | 2026-11-24 | GPU: match a breadcrumb node's name followed by the debug layer's note; drop the position fallback |
| #672 | `` | `b746d051` | 2026-11-24 | CLI: report all varying descriptor-word opcodes in translation refusals |
| #673 | `3ba3f79c1952` | `9a6bc8e0` | 2026-11-24 | Make translate failure examples stable across workers |
| #674 | `b9e5f2e9ff3d` | `d28b71de` | 2026-11-24 | xg: validate null tiling output, slice bounds, and checked copy spans |
| #677 | `0284037e045c` | `de6e76fc` | 2026-11-24 | Build: thin LTO for release and diag builds |
| #683 | `81fb96eb74be` | `db9e8b9b` | 2026-11-24 | Shaders: zero unavailable mixed-precision FMA halves |
| #684 | `9c2a364dde5d` | `2e672a6d` | 2026-11-24 | Shaders: find the PC test compiler independently of the validator |
| #686 | `741d675c9028` | `aa0401c2` | 2026-11-24 | docs: check archive integrity after early boot breakpoint |
| #689 | `e1c3557bdc67` | `f137b301` | 2026-11-24 | GPU: let the frame-wait test tolerate a loaded scheduler |
| #695 | `b91ff3c4829d` | `958693cf` | 2026-11-24 | gpu: VICTORY_D3D12_WARP=0 opts out of the software adapter |
| #708 | `a9698e7488ab` | `79381585` | 2026-11-24 | fix(config): keep the data directory out of the cache root |
| #922 | `f7bce85fcce3` | `846fcdee` | 2026-11-24 | Pipeline: delimit converter id in cache keys |
| #934 | `95cefdaca5f4` | `e28ee842` | 2026-11-24 | Isolate the write-watch fixture from reused virtual addresses |
| #939 | `d4306f4753fd` | `4a8b49ec` | 2026-11-24 | HLE: XSystemGetXboxLiveSandboxId refuses a buffer under 16 bytes |
| #941 | `60d2cf9cbcf5` | `d65d475e` | 2026-11-24 | Loader: a clwb in the last bytes of a page is emulated instead of faulting |
| #953 | `c42cbb0737d4` | `07f604ce` | 2026-11-24 | Shaders: handle s_buffer_load SMEM uses in descriptor_data |
| #963 | `adeb9ff49dbf` | `d9495f66` | 2026-11-24 | PE: read long-format import descriptors |
| #967 | `66d97eea7ea3` | `ee173bad` | 2026-11-24 | GPU: choose an adapter that can run the title, and name the ones that cannot |
| #975 | `78e2d19505bb` | `22e61383` | 2026-11-24 | HLE: exercise XGSuggestSwizzleMode TEXTURE_1D swizzle branch |
| #992 | `87f7cb894637` | `ccbaa676` | 2026-11-24 | Formats: XMA bit reads of 0 or more than 32 bits do not overflow a shift |
| #993 | `64b80c310941` | `c87e75a3` | 2026-11-24 | Formats: share immutable texture swizzle tables |
| #1006 | `6a2c18e2f5c8` | `4e9aff86` | 2026-11-24 | XGameSave: a folder named like a version, such as v01, does not remove the save |
| #1012 | `0ba5b998e6df` | `8a37c05c` | 2026-11-25 | Shaders: descriptors joined by nested control flow |
| #1016 | `03a6f22906b4` | `efc14e0a` | 2026-11-25 | XMA: gather frame payloads one byte at a time |
| #1021 | `3d71ad1a6547` | `b04607ca` | 2026-11-25 | Shaders: s_atc_probe's immediate is not a register |
| #1022 | `57e2cdeed86b` | `ab442f01` | 2026-11-25 | Shaders: signature rows of one register in the validator's order |
| #1039 | `0ca1e6ad753e` | `694c4944` | 2026-11-25 | Shaders: helper lanes take their own value in a pixel shader's first-lane read (#908) |
| #1048 | `7b57e8e8da92` | `e80a574e` | 2026-11-24 | Shaders: hardware test for s_bfe_u32 |
| #1050 | `259ba31e28ca` | `e3622af7` | 2026-11-25 | Shaders: negative-zero ordering test for the f16 min/max forms |
| #1053 | `393320dfa8cd` | `ecd3e625` | 2026-11-25 | Shaders: hardware test for v_cmp_class_f32 |
| #1057 | `a793ad86a83b` | `e4a0c8c5` | 2026-11-25 | victory-shader: lower all-zero and all-one constant lane masks |
| #1069 | `5f026f30774b` | `67b24154` | 2026-11-25 | GPU: f32_to_half keeps NaN and rounds ties to even (fixes #843) |
| #1070 | `5229ab872aeb` | `96592464` | 2026-11-25 | Pipeline: an entry published by another writer during conversion is used (fixes #890) |
| #1071 | `d9741ec66ce5` | `a541c722` | 2026-11-25 | Net: connect to a zero address disconnects a datagram socket instead of being refused (fixes #891) |
| #1072 | `5075e1ec8b65` | `b941c1db` | 2026-11-25 | HLE: XApu writes decoded audio to an unaligned output buffer as bytes (fixes #839) |
| #1080 | `a7847a520197` | `7abafc7b` | 2026-11-25 | HLE: constrain the title before suspending it |
| #1091 | `098a26aad5e3` | `07f2fb60` | 2026-11-25 | README: how to install Rust on Windows (#1087) |
| #1106 | `0a1f602cef1f` | `e3d1b13e` | 2026-11-25 | test(shader): report forced WARP runs as software only |
| #1109 | `a32eacdd5e0f` | `9d7735b8` | 2026-11-25 | fix(pipeline): retry collisions when creating cache staging files |
| #1110 | `d2ce712926b2` | `6db9bb08` | 2026-11-25 | fix(hle): account for save replacements before clamping quota |
| #1657 | `932dcbf89f96` | `15316a22` | 2026-11-24 | XSystem: name the analytics family Windows.Xbox |
| #1660 | `be824d9d2183` | `d32efcd9` | 2026-11-25 | GPU: serialize pipeline library atomic writes with unique temp files |
| #1664 | `98b0ec8c60ff` | `4cf31e8e` | 2026-11-24 | hle: return S_OK with null handle when PowerSetRequest returns ERROR_NOT_SUPPORTED |
| #1667 | `eb7cc2d43711` | `13ab09bb` | 2026-11-24 | flush a kept save blob that had to be copied |
| #1668 | `d027c4fb3819` | `c13f6037` | 2026-11-24 | Shaders: read inline floats as f32 bits in 16-bit integer ops |
| #1685 | `04e65b64f027` | `f4bcf683` | 2026-11-24 | Shaders: lower v_cmp_{f,t}_{i64,u64} constant 64-bit compares |

## Local fixes with no upstream PR

Changes made here rather than taken from a PR: corrections caught while
integrating, lint/format repairs, and the ISA work written in-tree.

| Local commit | Date | Subject |
| --- | --- | --- |
| `bfea43d0` | 2026-11-24 | build: pin development toolchain to Windows MSVC |
| `188dbb7f` | 2026-11-24 | Shaders: compressed 16-bit exports store the right half |
| `dd710b77` | 2026-11-24 | fix(hle): keep apostrophes intact when splitting relaunch args |
| `4398be82` | 2026-11-24 | shader: fold an inclusive float wave prefix with FMin/FMax |
| `97efa8de` | 2026-11-24 | fix(hle): resolve ordinals on pseudo-module handles |
| `4a81ddfa` | 2026-11-24 | HLE: validate pointers in XGameUi text entry result and size getters |
| `68ae6861` | 2026-11-24 | HLE: validate null pointer in ApuIsVirtualAddressValid and overflow in ApuAlloc |
| `3a957618` | 2026-11-24 | PE: preallocate symbol_key in CodeViewInfo |
| `2902ad00` | 2026-11-24 | HLE: check null and invalid handle in display deferral |
| `ec55ee70` | 2026-11-24 | Audio: an XMA write block past the ring's end wraps before the first write |
| `f9c608d9` | 2026-11-24 | fix(hle): reject saves whose committed container lacks a manifest |
| `4eeba29e` | 2026-11-24 | Shaders: a halfword at the last LDS dword's byte 3 drops its high byte |
| `c4412880` | 2026-11-24 | fix(hle): pin a call's task queue at XAsyncBegin |
| `b26e3c22` | 2026-11-24 | HLE: validate null package identifier in XPackage methods |
| `91975ac7` | 2026-11-24 | fix(pe): keep every name for a shared export ordinal |
| `eef50a92` | 2026-11-24 | fix(hle): write at-rest gamepad state for non-pad readings |
| `bd0e87b2` | 2026-11-24 | tests: satisfy the workspace lints in two integrated PR tests |
| `20228f6c` | 2026-11-24 | Shaders: add unit test for sub-dword LDS reads and writes |
| `27cf574b` | 2026-11-24 | DSP: refuse XDspDisconnect while streams are still active |
| `775a02f8` | 2026-11-24 | tests: parse current exe for ImageInfo in dir64_relocations test |
| `9c21600d` | 2026-11-24 | tests: keep the wave's new tests lint-clean |
| `4777058a` | 2026-11-24 | fix(hle): reject duplicate blob changes in one save update |
| `7b82ae45` | 2026-11-24 | fix(hle): serialize device callbacks across delivery threads |
| `863c9a42` | 2026-11-24 | Shaders: read explicit packed input padding as zero |
| `c2920339` | 2026-11-24 | Shaders: a NaN quotient in v_div_fixup_f16 overflows to infinity |
| `538866a9` | 2026-11-24 | Shaders: round v_dot2_f32_f16's sum once, as RDNA2 does |
| `61e76503` | 2026-11-24 | fix(shader): accept overwritten VGPR inputs in counted indexed writes |
| `dcce7e86` | 2026-11-24 | feat(shader): lower buffer cache invalidations as no-ops |
| `74d8dc87` | 2026-11-24 | fix(audio): gather spanning XMA frames through the packet cursor |
| `54de4f02` | 2026-11-24 | DirectStorage: report queue read failures |
| `f87b875b` | 2026-11-24 | Shaders: follow phis and copies before accepting a thread-group info read |
| `5577329c` | 2026-11-24 | Shaders: scope wave64 ds_bpermute_b32 to 32-lane halves |
| `c0bb75f3` | 2026-11-24 | Shaders: lower v_cvt_pknorm_{i16,u16}_{f32,f16} with exact rounding |
| `768cc06d` | 2026-11-24 | Shaders: v_mul_legacy_f32 treats denormal operands as zero |
| `d6c2769e` | 2026-11-24 | tests: keep the wave's new tests lint-clean |
| `2c6aff28` | 2026-11-24 | GPU: name a list's lease `dynamic` and give it `push`, so code written for the slot list compiles |
| `a2226677` | 2026-11-24 | run: a bare title name is looked up in the configured game_dir |
| `bab91c23` | 2026-11-24 | dstorage: serialise the tests that share the driver's scheduler |
| `b8e51047` | 2026-11-24 | XAsync: a canceled pending call's work touches neither freed state nor the next call |
| `49d9f393` | 2026-11-24 | CLI: add AVX2 pre-flight CPU check before executing titles |
| `99eb9327` | 2026-11-24 | feat(gpu): answer format-info and virtual-address feature queries |
| `65e35978` | 2026-11-24 | GPU: keep the frame wait's tick for the diagnostics log |
| `e420fd36` | 2026-11-24 | GPU: skip texture-to-buffer readback when host texel size differs |
| `4148054b` | 2026-11-24 | style: run cargo fmt across the workspace; fix clippy field-reassign-with-default |
| `d6d02744` | 2026-11-24 | CI: check the formatting with cargo fmt |
| `5541a92b` | 2026-11-24 | Shaders: keep the v_cvt_f16_u16/i16 test to integers f16 holds exactly |
| `69c467b7` | 2026-11-24 | GPU: name the user-data kind `root`, not `kind`, in op_text |
| `9b703856` | 2026-11-24 | diagnostics: preserve page fault baseline on query failure |
| `56961c3b` | 2026-11-24 | App: enforce ensure_outside_repo on configured paths while exempting CLI paths |
| `b1bafdaa` | 2026-11-24 | lint: satisfy the gate after the wave's integrations |
| `b9dd41c8` | 2026-11-24 | fix(cli): move scan test modules after production items |
| `312dd157` | 2026-11-24 | Shaders: v_swap_b32 with a non-VGPR source is a move into the destination |
| `0a091477` | 2026-11-24 | CI: configure Dependabot for security-only alerts without PR noise |
| `e37f8d9b` | 2026-11-24 | Shaders: tbuffer_load_format_d16 loads pack two 16-bit channels per VGPR |
| `92a5b9c1` | 2026-11-24 | GPU diagnostics: trace dispatches dropped because a table could not be made |
| `c1f36fee` | 2026-11-24 | tests: give the two synthetic shaders the diagnostics-only wave64 field |
| `cea6a039` | 2026-11-24 | Merge upstream main into pr-view-bounds |
| `dc866aa8` | 2026-11-24 | Shaders: an address in two root constants is a root view (#614) |
| `6c2a179c` | 2026-11-24 | lint: collapse the nested if PR #963 introduced in implib's long-format branch |
| `2b7cbae1` | 2026-11-24 | GPU: copy title memory into host memory without a system call |
| `055b89cb` | 2026-11-25 | Shaders: name the pick function type in the integer min3/max3/med3 test |
| `8f416804` | 2026-11-25 | GPU: log requested descriptor heap sizes |
| `38f4784d` | 2026-11-25 | Merge remote-tracking branch 'origin/main' into fix/xgamesave-read-count |
| `b6d51880` | 2026-11-25 | Shaders: use validator 1.8 for compute derivatives |
| `2c1f3461` | 2026-11-25 | Shaders: a lane mask spilled to a VGPR lane is still that mask |
| `528526c4` | 2026-11-25 | style: rustfmt the two files PRs #1070 and #1074 left unformatted |
| `feea90e4` | 2026-11-25 | Shaders: scalar loads through a heap buffer that vector loads read use a buffer view |
| `9861b0cc` | 2026-11-25 | GPU, HLE: an announced kernel read stays open until it lands |
| `cd1bbc84` | 2026-11-25 | Shaders: 64-bit image atomics carry the typed and heap feature bits |
| `27ab3167` | 2026-11-25 | Shaders: name which root-parameter case an address refusal is |
| `bfed7870` | 2026-11-25 | Shaders: split the address refusal into the three cases it covers |
| `92098a10` | 2026-11-25 | Shaders: lower pixel shader SV_Coverage input via dx.op.coverage |

## How to verify a row

```bash
# the local commit and the upstream commit it came from
git log -1 --format='%H%n%B' <local>
# the change itself
git show <local>
```

A row states only that the change is present locally. It does not mean the
upstream PR is merged upstream, and a green test run here is not evidence about
the upstream tree.

## Codex continuation: 2026-11-25

| PR | Source head | Candidate | Local no-ff merge | Evidence |
| --- | --- | --- | --- | --- |
| #1557 | `1504ec9aa4465bfe99dcf9311ea0b5091ef1a892` | `c159fa22` | `9e0d40a2` | Full 1-file review; source24/candidate45 formats tests; negative control; complete integration gate passed. |
