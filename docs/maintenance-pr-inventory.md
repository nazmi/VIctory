# Open upstream PR inventory

Retrieved 2026-11-24T11:51:11.9051768Z through 2026-11-24T11:51:36.6100229Z.
Query: `GET repos/Marvisak/VIctory/pulls?state=open&per_page=100`, using gh api --paginate --slurp.
Pagination exhausted successfully: 5 pages (100, 100, 100, 100, 45), 445 open PRs. Complete for the retrieval snapshot; recheck current state and exact head before work.

Inventory membership and preliminary workflow labels do not imply completed review, passing tests, acceptance, or integration. Review-passed labels identify completed static review only; tests remain pending.

| PR | Title | Exact head SHA | Draft | Disposition |
|---|---|---|---|---|
| [#710](https://github.com/Marvisak/VIctory/pull/710) | Shaders, GPU: read through addresses passed in two root constants (stacked on #489) | `a5a40bc40675c3fe63d808c3726f0eff54d71f4c` | false | queued-unreviewed |
| [#709](https://github.com/Marvisak/VIctory/pull/709) | Shaders: structured access through a root SRV or UAV uses its raw view (stacked on #294) | `e50ed36cd9e20ded929b3bb23a53a8b54b9a21bb` | false | queued-unreviewed |
| [#708](https://github.com/Marvisak/VIctory/pull/708) | fix(config): keep the data directory out of the cache root | `a9698e7488ab2a7d1bdbf0e6b736ab689b6b2ca2` | false | review-passed-tests-pending |
| [#707](https://github.com/Marvisak/VIctory/pull/707) | Shaders: lower tbuffer_store_format_* that need no conversion | `7ca777e555435962d724766dfc5f267c9bc44dde` | false | queued-unreviewed |
| [#705](https://github.com/Marvisak/VIctory/pull/705) | Fix/typed image atomic int64 | `4efabc45ca5d29a260833c502c96030d08e4956c` | false | queued-unreviewed |
| [#704](https://github.com/Marvisak/VIctory/pull/704) | Shaders, GPU: unknown-stride buffers are read through a raw view of the same bytes (stacked on #193) | `7f77ccad8a95c13cd9d7e86a418bdf1ce9bbea53` | false | queued-unreviewed |
| [#703](https://github.com/Marvisak/VIctory/pull/703) | Formats: eliminate BCPack per-call allocations and optimize streaming decode | `d45b04b0ab07daf732a2cd9dc85df9d5c9471de8` | false | queued-unreviewed |
| [#702](https://github.com/Marvisak/VIctory/pull/702) | gpu: add native DXR 1.1 acceleration structure backend and OPTIONS5 query | `0237ecadb7791eb62a061206f54fb02143578f6b` | false | queued-unreviewed |
| [#699](https://github.com/Marvisak/VIctory/pull/699) | shader: a phi read only through another phi keeps its missing incoming value | `db0257e2c133ccdcb4605f0a90bcf03cb1847cad` | false | already-present (#612) |
| [#697](https://github.com/Marvisak/VIctory/pull/697) | Shader: backward dead-code elimination in DXIL bitcode emission | `2eea8e2e817032c7574c64b2adb8fa870504d75a` | false | queued-unreviewed |
| [#695](https://github.com/Marvisak/VIctory/pull/695) | gpu: VICTORY_D3D12_WARP=0 opts out of the software adapter | `b91ff3c4829d2d9cf82f08032f6437b4fb667f74` | false | queued-unreviewed |
| [#694](https://github.com/Marvisak/VIctory/pull/694) | Shaders: support image_sample_d_cl and image_gather4_l lowering (#293, #653) | `0beec4d64361a698b7550b30232df8e9c4114984` | false | queued-unreviewed |
| [#693](https://github.com/Marvisak/VIctory/pull/693) | gpu: move shadow buffers to DEFAULT VRAM with pooled staging transfers | `ae39e9c14bd417d81ecf6c292ed6bb93b6f718ce` | false | queued-unreviewed |
| [#692](https://github.com/Marvisak/VIctory/pull/692) | style: cargo fmt across the workspace; fix clippy field-reassign-with-default | `e8ec70cbe59a7905194d80a9f68ffd8f5105eced` | false | queued-unreviewed |
| [#691](https://github.com/Marvisak/VIctory/pull/691) | hle: speak accessibility narration through the host's WinRT synthesizer | `796ace2e632e2fb8793951b14a58eec73b8e4720` | false | queued-unreviewed |
| [#690](https://github.com/Marvisak/VIctory/pull/690) | gpu: expand console primitive topologies into host list index buffers | `79a476f57cb2934352a10273bcec6cce9919c4b8` | false | queued-unreviewed |
| [#689](https://github.com/Marvisak/VIctory/pull/689) | GPU: let the frame-wait test tolerate a loaded scheduler | `e1c3557bdc67f31e13d04a00d2d9ad1ebc65c7e3` | false | integrated-local |
| [#688](https://github.com/Marvisak/VIctory/pull/688) | GPU: take back the dynamic descriptor slots of lists that ran when a heap runs short | `bf4e37ed9b7c296d4fce59f27ab8eb77de441a94` | false | queued-unreviewed |
| [#687](https://github.com/Marvisak/VIctory/pull/687) | hle: track title working-set memory, bound xasync threads, and map XUser devices | `b7d3263d15cfa782233e7bf9956a061e4a27f902` | false | queued-unreviewed |
| [#686](https://github.com/Marvisak/VIctory/pull/686) | docs: check archive integrity after an early boot breakpoint | `741d675c90287a82052f1c5bbbae82d699d95a28` | false | queued-unreviewed |
| [#684](https://github.com/Marvisak/VIctory/pull/684) | Shaders: find the PC test compiler independently of the validator | `9c2a364dde5dd53ffd790725851f6cdcfed83360` | false | queued-unreviewed |
| [#683](https://github.com/Marvisak/VIctory/pull/683) | Shaders: zero unavailable mixed-precision FMA halves | `0b4863361e7cffa7c82e2b0a301cddede7702363` | false | queued-unreviewed |
| [#682](https://github.com/Marvisak/VIctory/pull/682) | Shaders: v_mul_legacy_f32 treats denormal operands as zero | `e1bd18f0e6d6178f7df373a76a0221c67914ae4c` | false | queued-unreviewed |
| [#679](https://github.com/Marvisak/VIctory/pull/679) | run: refuse capture and dump paths inside the repo | `b96c85d47d7c5d9b2e7f1673e0a59deb36c4aa31` | false | queued-unreviewed |
| [#677](https://github.com/Marvisak/VIctory/pull/677) | Build: thin LTO for release and diag builds | `0284037e045c50e457f3885427c7ca72a232ffe9` | false | queued-unreviewed |
| [#674](https://github.com/Marvisak/VIctory/pull/674) | HLE (xg): validate null tiling output, slice bounds, and checked copy spans | `b9e5f2e9ff3d03c1cc3f89402aacbb63eb4c559e` | false | queued-unreviewed |
| [#673](https://github.com/Marvisak/VIctory/pull/673) | Make translate failure examples stable across workers | `3ba3f79c1952da6db19fb7960a960e41d3e225c2` | false | queued-unreviewed |
| [#672](https://github.com/Marvisak/VIctory/pull/672) | CLI: report all varying descriptor-word opcodes in translation refusals | `ad434b4c037bdb4399c9519c6fc21ee3d42dce5d` | false | queued-unreviewed |
| [#670](https://github.com/Marvisak/VIctory/pull/670) | loader: use checked offsets and sizes in image preparation | `44bbd6a5e988ca9f9a02c130ce0e2e42c53c27d4` | false | queued-unreviewed |
| [#669](https://github.com/Marvisak/VIctory/pull/669) | GPU: find a list in the DRED breadcrumbs by its name, not only its pointer | `52d528ba0c028f9525450354acbfaa5d39c9918d` | false | queued-unreviewed |
| [#668](https://github.com/Marvisak/VIctory/pull/668) | Shaders: support f16 omod and saturate/apply source modifiers in v_cvt_{u16,i16}_f16 | `38a41cb0c9aad593b494fd8285c5a49d68f75c61` | false | queued-unreviewed |
| [#667](https://github.com/Marvisak/VIctory/pull/667) | Shaders: an address in two root constants is a root view (#614) | `995f7f3ee720270108dfe93decb0f8c3c53f613b` | false | queued-unreviewed |
| [#665](https://github.com/Marvisak/VIctory/pull/665) | xtask: count \x escapes and 0x hex arrays as encoded runs in the policy check | `e950e81756340b10c3ef87d9c1789ae482f1a262` | false | queued-unreviewed |
| [#664](https://github.com/Marvisak/VIctory/pull/664) | translate: refuse a VICTORY_EMIT_DUMP directory inside the repo | `988fd9a64718ee9a235cff3b60192b6b86006c4d` | false | queued-unreviewed |
| [#663](https://github.com/Marvisak/VIctory/pull/663) | media: apply the local-file rule to the source reader, sink writer and MFCreateFile | `d4dcd0b63ad1d17d057ef8cd76583e581565146c` | false | queued-unreviewed |
| [#662](https://github.com/Marvisak/VIctory/pull/662) | DSP: refuse XDspDisconnect with active streams; XDSP_E_NO_DEVICE_RESOURCES past the stream limit | `37d52cb78ddc09cc5f8c967e190ebc9a51e79fe8` | false | queued-unreviewed |
| [#661](https://github.com/Marvisak/VIctory/pull/661) | Shaders: fix missing and phantom operands in SP3 text | `6325df752c5638365c3e5a2b2f827ba0f3f145cb` | false | queued-unreviewed |
| [#660](https://github.com/Marvisak/VIctory/pull/660) | Shaders: drop a wave64 mask whose low dword is seen with two high dwords | `d92ebe9c9dc9eda57fdf54681a2b21f4058a4265` | false | queued-unreviewed |
| [#659](https://github.com/Marvisak/VIctory/pull/659) | Shaders: refuse per-lane reads of mesh v5 in v5 writes and after joins | `98fe8fe840bab7ad086c8126ea9d5dd856fb2e89` | false | queued-unreviewed |
| [#658](https://github.com/Marvisak/VIctory/pull/658) | BCPack: read BC7 dictionary indices from the right offset | `0ecaa8cb92ac86e9721f9a0870788fe326b645bd` | false | queued-unreviewed |
| [#656](https://github.com/Marvisak/VIctory/pull/656) | Input: report the character each held key types | `3b001ed80c97a9da158696ae06c4961548953a53` | false | queued-unreviewed |
| [#655](https://github.com/Marvisak/VIctory/pull/655) | Shaders: guard conditional exports | `b9c7699cc5cd014c9dd7ac170948bd3595a0ea8d` | false | queued-unreviewed |
| [#652](https://github.com/Marvisak/VIctory/pull/652) | GPU: use selected mip extents for MSAA resolves | `ea09cb04bc0cf14ee4caae7d71f3bf51ae787115` | false | queued-unreviewed |
| [#651](https://github.com/Marvisak/VIctory/pull/651) | GPU: select the readable format and plane for depth/stencil shader views | `d615ee0fe0f90750320575e2501726486a0454ca` | false | queued-unreviewed |
| [#650](https://github.com/Marvisak/VIctory/pull/650) | Shaders: bound root view accesses by the size the host binds | `06657a8bebe86f562cf1d55796de565b9a9d329b` | false | queued-unreviewed |
| [#649](https://github.com/Marvisak/VIctory/pull/649) | Show on-disk file size in victory inspect | `43e2ca9de29f5833941970e82ed89872c9f86a81` | false | queued-unreviewed |
| [#648](https://github.com/Marvisak/VIctory/pull/648) | HLE: implement XUser maximum-users slot 0x30 (#348, #622) | `241b08f4943973800de7c867d332b32d14436cf5` | false | queued-unreviewed |
| [#646](https://github.com/Marvisak/VIctory/pull/646) | Shaders: fix whole-wave EXEC inline constant sign-extension and prefix lane validation | `12eba495039d8018eb6959be95fcc10e387b7e33` | false | queued-unreviewed |
| [#645](https://github.com/Marvisak/VIctory/pull/645) | Shaders: trace multi-term, scaled, masked, and phi-joined addresses in resource tracking | `499bd9860a323e509ebcdd61a7644f11ba3f27e2` | false | queued-unreviewed |
| [#644](https://github.com/Marvisak/VIctory/pull/644) | GPU: spill oversized host root signatures' user data to root CBVs | `6fa53a51c06f3b3d4d60518371694008bab89df8` | false | queued-unreviewed |
| [#643](https://github.com/Marvisak/VIctory/pull/643) | GPU: share live host textures across aliased addresses and canonicalize view descriptors | `e7b2182e539aed2303dab794e23de991abae3faf` | false | queued-unreviewed |
| [#642](https://github.com/Marvisak/VIctory/pull/642) | GPU: translate enhanced texture barriers in ID3D12GraphicsCommandList7::Barrier | `6b987062d20a3106da4f3d5ff5342cce14c6e22a` | false | queued-unreviewed |
| [#641](https://github.com/Marvisak/VIctory/pull/641) | GPU: upload partially-resident streamed textures | `f887f6a3f7bcd083c072871fedc8a66d2a5b9fa8` | false | queued-unreviewed |
| [#640](https://github.com/Marvisak/VIctory/pull/640) | Retry failed pipeline cache saves | `446fef929c8502b6c47b7dc70d86e5ad5e587d38` | false | queued-unreviewed |
| [#638](https://github.com/Marvisak/VIctory/pull/638) | GPU diagnostics: find what the world hang waits for, and look under a black frame | `d6e50b83b38a786ebeeeded45bea28566ffe5c48` | false | queued-unreviewed |
| [#636](https://github.com/Marvisak/VIctory/pull/636) | HLE: implement the RPC runtime's GUID calls | `7216ab7113115ea1accfabc6e3a00ee700c7c846` | false | queued-unreviewed |
| [#635](https://github.com/Marvisak/VIctory/pull/635) | GPU: buffer views past the end of their host copy are null, views running past it are cut | `cfe7610b771e5569e03413ab6f13f88ec19a7951` | false | queued-unreviewed |
| [#633](https://github.com/Marvisak/VIctory/pull/633) | HLE: GetModuleFileNameA answers in the ANSI code page | `90bc075458372756f52c63648a6bc00bdbdebce0` | false | queued-unreviewed |
| [#632](https://github.com/Marvisak/VIctory/pull/632) | Shaders: LDS reads and writes of a byte or half | `203b11b36d2a15e262dac41d8bbe6c63a75bb7ac` | false | queued-unreviewed |
| [#631](https://github.com/Marvisak/VIctory/pull/631) | Add unit tests for console.rs | `e009e13aa4793a2671a962a9bad90bbd586a5cd2` | false | queued-unreviewed |
| [#629](https://github.com/Marvisak/VIctory/pull/629) | Reject multisampled linear texture layouts | `350f91c23a9402b2a556a84c208a80030910ae3f` | false | coordinator-review |
| [#627](https://github.com/Marvisak/VIctory/pull/627) | feat(victory-shader): implement v_swaprel_b32, v_movrelsd_2_b32, and s_movrelsd_2_b32 lowering | `66a3ec58b3abd1ab9245f272acb0739b5907a985` | false | queued-unreviewed |
| [#628](https://github.com/Marvisak/VIctory/pull/628) | Shaders: lower ds_min_f32 / ds_max_f32 (and _rtn) with RDNA2's NaN rules | `717e732f68c5aaea2c855ed253a6c35b48bd953c` | false | queued-unreviewed |
| [#626](https://github.com/Marvisak/VIctory/pull/626) | GPU: fall back to standard root signature flags when host runtime rejects 0xC01/0xC00 | `df9ab3c69172c64cd8638344390577334954bd04` | false | queued-unreviewed |
| [#624](https://github.com/Marvisak/VIctory/pull/624) | DirectStorage: report queue read failures | `8632869429b76638bf314891d70b3922652b154c` | false | queued-unreviewed |
| [#623](https://github.com/Marvisak/VIctory/pull/623) | Fix Windows 10 Game-Init crash and root-signature fallback | `7c01e3d05c4fab45b506f4d6d9cc42d9a48d0d90` | false | queued-unreviewed |
| [#621](https://github.com/Marvisak/VIctory/pull/621) | Shaders: read explicit packed input padding as zero | `7e5c7c8d516e8224d149d494c23f772c01bf30ac` | false | queued-unreviewed |
| [#620](https://github.com/Marvisak/VIctory/pull/620) | HLE input: pass trigger motor levels to the host source | `59aa44b0e0de8f9dc7a4c02df1a839af72519696` | false | queued-unreviewed |
| [#618](https://github.com/Marvisak/VIctory/pull/618) | GPU: align appended input-element offsets like D3D12 | `691236d606b9de8cbccfa8540e2b27ca6070f8ae` | false | queued-unreviewed |
| [#617](https://github.com/Marvisak/VIctory/pull/617) | Shaders: reuse lane guards for unchanged EXEC within a block | `81b81df351437c50f4321286c725de51664c7b1b` | false | queued-unreviewed |
| [#616](https://github.com/Marvisak/VIctory/pull/616) | GPU: rectangle draws check their bounds without 32-bit overflow | `94bc7cf85130b589c8c5b592697aa6cb92666172` | false | queued-unreviewed |
| [#615](https://github.com/Marvisak/VIctory/pull/615) | GPU: VICTORY_* switches set to 0 are off (#610) | `a4974e4c21c16e2305942e37a868647d42eefe37` | false | queued-unreviewed |
| [#613](https://github.com/Marvisak/VIctory/pull/613) | HLE: serialize device callbacks across delivery threads | `2750ab1f82aa869a8b4f7ebebc3f3af4b7da9a82` | false | queued-unreviewed |
| [#609](https://github.com/Marvisak/VIctory/pull/609) | GPU: retain and retire released host textures, staging, shadow buffers, and descriptor resources across submissions | `6d8c72bc04ee7d31c654e089213dbf258a662ce6` | false | queued-unreviewed |
| [#608](https://github.com/Marvisak/VIctory/pull/608) | feat(victory-shader): implement s_movrels_b32 and s_movreld_b32 lowering | `cc6b6984a700f220565099017a763482fcb46e38` | false | queued-unreviewed |
| [#606](https://github.com/Marvisak/VIctory/pull/606) | Shaders: ds_swizzle_b32 offsets from 0xC000 are the rotate and FFT modes | `da1a1f7ba3b4eb28cc46e9e2352b7e32e1bd5c4b` | false | queued-unreviewed |
| [#605](https://github.com/Marvisak/VIctory/pull/605) | Shaders: v_cvt_rpi_i32_f32 rounds the exact sum; v_cvt_flr/rpi_i32_f32 saturate a NaN by its sign | `c7e4caffa410bb4e5799680867a95f5a017ba4a6` | false | queued-unreviewed |
| [#603](https://github.com/Marvisak/VIctory/pull/603) | Formats: inline repeated micro-tile copies when detiling textures | `705a2f634d1c3ada825922a6a1c5befca15d599b` | false | queued-unreviewed |
| [#602](https://github.com/Marvisak/VIctory/pull/602) | GPU diagnostics: report retained copy memory | `4367d6c70a584e7253075e5a3281dd75da3f7413` | false | queued-unreviewed |
| [#601](https://github.com/Marvisak/VIctory/pull/601) | GPU: bound texture upload and capture slices to the allocated buffer | `48995162e7e27c2e11651361c19360350fc5cc66` | false | queued-unreviewed |
| [#599](https://github.com/Marvisak/VIctory/pull/599) | Shaders: lane-indexed ops on a wide host wave use the own console wave | `38234626c7c17cb6966a51c2ecba89e0bd1f78a7` | false | queued-unreviewed |
| [#596](https://github.com/Marvisak/VIctory/pull/596) | Shaders: lower v_cvt_pknorm_{i16,u16}_{f32,f16} with exact rounding | `be92188eee0a56b3a5150260a8f0786a456f87b5` | false | queued-unreviewed |
| [#595](https://github.com/Marvisak/VIctory/pull/595) | GPU: run submissions asynchronously on a host fence timeline | `c89f08fe866dab895998df79e7c65b6f091f5d95` | false | queued-unreviewed |
| [#594](https://github.com/Marvisak/VIctory/pull/594) | GPU: track texture subresource states | `c3f1d7443d5f9ce6c0a19f15fe5fee84c0396780` | false | queued-unreviewed |
| [#592](https://github.com/Marvisak/VIctory/pull/592) | Shaders: second rows of packed 16-bit pixel inputs | `35bf1530542f9a463a233d14394e8c6734ae0958` | false | queued-unreviewed |
| [#591](https://github.com/Marvisak/VIctory/pull/591) | GPU diagnostics: trace each draw and dispatch of captured frames with its targets and drop reason | `ec2e1841c39c1cf7636adf2f575fa59ce8109de6` | false | queued-unreviewed |
| [#590](https://github.com/Marvisak/VIctory/pull/590) | HLE: reject duplicate blob changes in one save update | `32e0690e7d64f0013cc09281c6e70d5253dc4a3c` | false | queued-unreviewed |
| [#589](https://github.com/Marvisak/VIctory/pull/589) | fix(shader): skip unused image atomic descriptor reconstruction | `ab2e7cc3f648fe78e851ff1769d50774728edc1f` | false | queued-unreviewed |
| [#588](https://github.com/Marvisak/VIctory/pull/588) | GPU: transition shadow buffers between UAV and vertex/index/CBV states | `54c7c89e6f1c0e65c09d9960a1e93fad9b35aa9f` | false | queued-unreviewed |
| [#587](https://github.com/Marvisak/VIctory/pull/587) | Shaders: guard mesh output bounds, init all primitives, and omit sync on divergent s_barrier | `1052a90a407242812ea02caffe806b92c5f462ae` | false | queued-unreviewed |
| [#586](https://github.com/Marvisak/VIctory/pull/586) | Shaders: honor the mesh allocation request counts | `9b5e23997216f2d6ffb155086bfb313b4f0d93fe` | false | queued-unreviewed |
| [#585](https://github.com/Marvisak/VIctory/pull/585) | Shaders: lower untracked v_cndmask_b32/carry-in masks via lane_bit and descriptor-only GetDimensions | `f800f1ca9cc34cbc0bd4660bf387bb4eca1d106e` | false | queued-unreviewed |
| [#584](https://github.com/Marvisak/VIctory/pull/584) | Shaders: clamp ResourceDescriptorHeap and SamplerDescriptorHeap indices to host limits | `0bcf2df2762118baee53d32b859f35010122a46d` | false | queued-unreviewed |
| [#583](https://github.com/Marvisak/VIctory/pull/583) | fix(gpu): respect indirect count buffer, guard budget overflow, and bind null root views | `07bae5d505f2ff04d5365d535ce4c774f4f4ff30` | false | queued-unreviewed |
| [#582](https://github.com/Marvisak/VIctory/pull/582) | feat(gpu): wire up depth bounds testing on command lists and pipeline streams | `7e78c81aaacbdb00d12bb806bd3f62c1ffb33436` | false | queued-unreviewed |
| [#581](https://github.com/Marvisak/VIctory/pull/581) | GPU: typed null descriptors for empty table slots, depth-UAV scratch stand-in, and packed 16-bit fallback | `0a936b63e633c6f569b659f1e454c2cf0827190d` | false | queued-unreviewed |
| [#580](https://github.com/Marvisak/VIctory/pull/580) | Docs: add a contributor onboarding guide | `7ca84cc136702dd70ed2d48898ac77131862d4ef` | false | queued-unreviewed |
| [#579](https://github.com/Marvisak/VIctory/pull/579) | GPU: VICTORY_SKIP_WAVE64 diagnostics switch for wave64 programs | `1658c8452a4ac556f0e7fc26cc934c816ce29e7c` | false | queued-unreviewed |
| [#578](https://github.com/Marvisak/VIctory/pull/578) | GPU: refuse wave64 mesh and compute programs on hosts without 64-lane waves | `72926906b5cc429a92fe1be794866da3ac2993be` | false | queued-unreviewed |
| [#577](https://github.com/Marvisak/VIctory/pull/577) | dstorage: retain queued file references and validate null out pointers | `40816378b59840cdd6db42728b8d78690d2411eb` | false | queued-unreviewed |
| [#576](https://github.com/Marvisak/VIctory/pull/576) | shader: lower v_permlane16_b32 | `fde38b399f96111ca6a3c4c73a07b9b09e8303eb` | false | queued-unreviewed |
| [#575](https://github.com/Marvisak/VIctory/pull/575) | GPU: release resources the CPU is done with behind the queue | `69e44f59cd9c6f47aa64af36301fadcfd4decae5` | false | queued-unreviewed |
| [#574](https://github.com/Marvisak/VIctory/pull/574) | HLE: write at-rest gamepad state for non-pad readings | `c70c47854eb4a8b743d11c5335f1ab0a942fcc35` | false | queued-unreviewed |
| [#573](https://github.com/Marvisak/VIctory/pull/573) | Shaders: lower ds_bpermute_b32 | `1d09f19b210284f9c65c558719481dd707187384` | false | queued-unreviewed |
| [#572](https://github.com/Marvisak/VIctory/pull/572) | Docs: add a PowerShell quickstart for running VIctory | `0a1a702bb440fa3fe14a9aa53532a102da5b01d7` | false | queued-unreviewed |
| [#571](https://github.com/Marvisak/VIctory/pull/571) | Shaders: LDS byte and half accesses | `ce45c678b9fe78d0a25f7d68ff2e69910a3e7d4e` | false | queued-unreviewed |
| [#570](https://github.com/Marvisak/VIctory/pull/570) | HLE: reject saves whose committed container lacks a manifest | `edbd1a7bb3eecb309a8233608b4ffa2fb4ca2034` | false | queued-unreviewed |
| [#569](https://github.com/Marvisak/VIctory/pull/569) | Shaders: biased samples with 16-bit coordinates | `aed9637927e7ce80380ba29fa247b428f9408d26` | false | queued-unreviewed |
| [#568](https://github.com/Marvisak/VIctory/pull/568) | GPU: give table slots without a view a null descriptor of the kind the shader reads | `22e6676cad17ef86398fb2fe8407fd6e6f3fc781` | false | queued-unreviewed |
| [#563](https://github.com/Marvisak/VIctory/pull/563) | Shaders: cube samples with gradients, and image_sample_d_cl | `10ac6ceac9284caa6dd5d4bad3166687f8d0c1ac` | false | queued-unreviewed |
| [#562](https://github.com/Marvisak/VIctory/pull/562) | GPU: reuse immutable descriptor tables across command lists | `b82f29c12fb8b7d590da8f6342709331ad9e2d81` | true | queued-unreviewed |
| [#561](https://github.com/Marvisak/VIctory/pull/561) | xmem: commit physical pages in pieces, and report the real count | `3c02d8cf89470f487bab9c3a207da75084aca8b2` | false | queued-unreviewed |
| [#560](https://github.com/Marvisak/VIctory/pull/560) | Loader: stage prepared images in per-writer files | `49df7e1f31ef0359bae7fb78020e73750be790db` | false | queued-unreviewed |
| [#559](https://github.com/Marvisak/VIctory/pull/559) | GPU: track mip states for 2D color targets | `ca896f660c89b0a8ca68fb80a585dd78d2a5e31e` | false | queued-unreviewed |
| [#558](https://github.com/Marvisak/VIctory/pull/558) | CLI: check every extension the title uses unchecked, not only AVX2 (depends on #238) | `fa83a31dfaab405fac650407c9e9c680421caa12` | false | queued-unreviewed |
| [#555](https://github.com/Marvisak/VIctory/pull/555) | PE: keep every name for a shared export ordinal | `b71c25220e5c0ea82071b8e1a6bf6ac71e415785` | false | queued-unreviewed |
| [#554](https://github.com/Marvisak/VIctory/pull/554) | Shaders: one interpolation mode per signature row | `9152a1beefa059fecf07c0d62b89c805a7171d69` | true | queued-unreviewed |
| [#553](https://github.com/Marvisak/VIctory/pull/553) | GPU: clear DXGI outputs on out-of-range enumeration | `b796c8b8b80cfa4506f254474ab9fd7260bfd8e6` | false | queued-unreviewed |
| [#551](https://github.com/Marvisak/VIctory/pull/551) | Shaders: lower compute group info, wave id in workgroup, and sample coverage | `f96ce292750a9ab3df336f7465c543bda428f154` | false | queued-unreviewed |
| [#550](https://github.com/Marvisak/VIctory/pull/550) | GPU: never give a shader a table offset past the host heap | `a8be52867677a3f0c989fa3ed4264cb4b9a961fe` | false | queued-unreviewed |
| [#549](https://github.com/Marvisak/VIctory/pull/549) | CI: add Dependabot configuration for GitHub Actions and Cargo | `e04d70ede964210b379929fb918d325e4b5363a3` | false | queued-unreviewed |
| [#547](https://github.com/Marvisak/VIctory/pull/547) | QA: add rustfmt.toml to standardize workspace formatting | `55aea2c4de6f697208f7f97700b2e9f2f71a421a` | false | queued-unreviewed |
| [#546](https://github.com/Marvisak/VIctory/pull/546) | GPU: prioritize pipeline work requested by draws | `a8bf0d3aa6ce61f70331fbc85863d463eb86e885` | true | queued-unreviewed |
| [#545](https://github.com/Marvisak/VIctory/pull/545) | xtask: add qa command to run policy, rustfmt, and clippy checks | `366f485aaa45056e0394ae0bc7de41edf0bc12df` | false | queued-unreviewed |
| [#544](https://github.com/Marvisak/VIctory/pull/544) | HLE: report queue termination after running callbacks | `2a554637194db770b849ce1674e02e8645e63e13` | false | queued-unreviewed |
| [#543](https://github.com/Marvisak/VIctory/pull/543) | QA: add pull request template with asset policy and safety checklist | `c9e5db87b4c68ba1860445a302e7d6feabe83469` | false | queued-unreviewed |
| [#542](https://github.com/Marvisak/VIctory/pull/542) | Shaders: keep buffer offsets below 2 GiB (NVIDIA + vkd3d-proton world device loss) | `31a4692a1581a4ed20d33cd22b5164a0ce240ce8` | false | queued-unreviewed |
| [#541](https://github.com/Marvisak/VIctory/pull/541) | GPU: retain D3D12 object names in private data | `cbd84266d5159172146839cf1bdc09de07e1b803` | false | queued-unreviewed |
| [#540](https://github.com/Marvisak/VIctory/pull/540) | CI: add asset-policy and formatting verification workflow | `5b7eb8a168e9d12a8c451b563b9396575c07b6f3` | false | queued-unreviewed |
| [#539](https://github.com/Marvisak/VIctory/pull/539) | Shaders: 64-bit image atomics | `719079f2b73831a51d5dbe4218948eeaa9c447db` | false | queued-unreviewed |
| [#538](https://github.com/Marvisak/VIctory/pull/538) | xmem: report allocations the host refuses | `65062e133c75c3b6e984ac51e904512d41f71f93` | false | queued-unreviewed |
| [#537](https://github.com/Marvisak/VIctory/pull/537) | GPU: name the switch when a removal has no device-removed data | `2021e0f26970a164dbb58db0a266423057f58564` | false | queued-unreviewed |
| [#536](https://github.com/Marvisak/VIctory/pull/536) | Audio: an XMA write block past the ring's end wraps before the first write | `7a5d85e6a2867081ed54515ce7f33bae3fcb643d` | false | queued-unreviewed |
| [#535](https://github.com/Marvisak/VIctory/pull/535) | App: enforce asset policy check in default_paths | `ebfa8ae0c641395bb57362996a2e56f598e1422a` | false | queued-unreviewed |
| [#534](https://github.com/Marvisak/VIctory/pull/534) | CLI: report shader scan input failures | `e1680afdbe1bbfb2b4e8c567a2c306b78c8b6905` | false | queued-unreviewed |
| [#532](https://github.com/Marvisak/VIctory/pull/532) | Document the Arch MSVC build and Proton rendering on an RTX 4080 | `aa50c7d6481e584b47115849ac3b383a8fc529a9` | false | queued-unreviewed |
| [#531](https://github.com/Marvisak/VIctory/pull/531) | HLE: check null and invalid handle in display deferral | `7f0053be9ea83cd6d7902832ac8d764a28683556` | false | queued-unreviewed |
| [#530](https://github.com/Marvisak/VIctory/pull/530) | GPU diagnostics: show one of the title's render targets in the window | `951ceb1eb8dac4034bf2ecec8e9b32a5df1010a6` | false | queued-unreviewed |
| [#529](https://github.com/Marvisak/VIctory/pull/529) | PE: preallocate symbol_key in CodeViewInfo | `9cf9f2f4aefd5dbfd4b76f2294a3d3ba9382548d` | false | queued-unreviewed |
| [#528](https://github.com/Marvisak/VIctory/pull/528) | HLE: validate null pointer in ApuIsVirtualAddressValid and overflow in ApuAlloc | `b1539b75a2c8d43651e06c7c287fce723c56d483` | false | queued-unreviewed |
| [#526](https://github.com/Marvisak/VIctory/pull/526) | Linux: a native runtime, no Wine | `d46727fe79d9ec071468a7d675e555019e114c5a` | false | queued-unreviewed |
| [#524](https://github.com/Marvisak/VIctory/pull/524) | Runtime: declare title-facing functions extern "win64" | `53282a65ad88ff5e826585a3340ebaf806f41ca0` | false | queued-unreviewed |
| [#523](https://github.com/Marvisak/VIctory/pull/523) | Build: compile and test the workspace on Linux again | `0012266175278c7b220c00f09e2a7bef08aa63df` | false | queued-unreviewed |
| [#522](https://github.com/Marvisak/VIctory/pull/522) | HLE: validate null package identifier in XPackage methods | `e5419f49097a486babf6b86c881ef6b2a1944b04` | false | queued-unreviewed |
| [#521](https://github.com/Marvisak/VIctory/pull/521) | Shaders: model the compute thread-group info SGPR's wave count | `75b1b8ff6550f99acd5494843551bf913bd7334a` | false | queued-unreviewed |
| [#519](https://github.com/Marvisak/VIctory/pull/519) | HLE: validate pointers in XGameUi text entry result and size getters | `4ab8bca10ef4701ab89c7c4a39f3f7070cccdb88` | false | queued-unreviewed |
| [#517](https://github.com/Marvisak/VIctory/pull/517) | CLI: report incomplete shader directory scans | `5f211143d5fc66c8aaa8253c02174677f8d7df6f` | false | queued-unreviewed |
| [#516](https://github.com/Marvisak/VIctory/pull/516) | Core: use XDG_DATA_HOME for default data_dir on non-Windows hosts | `0e48e5a9a3af681ca7507553692d9168209e2d27` | false | queued-unreviewed |
| [#515](https://github.com/Marvisak/VIctory/pull/515) | GPU: ask for written pages allocation by allocation when a range spans several | `d9cb9085689f6ccbe31262d74f59998037a10e22` | false | queued-unreviewed |
| [#514](https://github.com/Marvisak/VIctory/pull/514) | fix(gpu): recheck uncommitted blocks before skipping writes | `8540a0ec34234ed35700dd98ce338402f507855e` | false | queued-unreviewed |
| [#513](https://github.com/Marvisak/VIctory/pull/513) | App: forward title command-line arguments to run command | `c3f374fd47bddd3d10dd30bfd757a961c01ddf25` | false | queued-unreviewed |
| [#512](https://github.com/Marvisak/VIctory/pull/512) | HLE: resolve ordinals on pseudo-module handles | `d3e33eae80fb4907a265e62eafc2c0399298f9e7` | false | queued-unreviewed |
| [#511](https://github.com/Marvisak/VIctory/pull/511) | Shaders: coalesce aligned constant-buffer loads | `0b3c1fd1c66df7ab01af05214b75666b5bbb8223` | false | queued-unreviewed |
| [#509](https://github.com/Marvisak/VIctory/pull/509) | GPU: add an opt-in execution performance census | `99a24ecf13dd4c3e77ed576b0e89ee3d42a219e1` | false | queued-unreviewed |
| [#506](https://github.com/Marvisak/VIctory/pull/506) | CLI: reject zero shader translation workers | `e80778cffa944cecfdc0775c10b466d6887edd95` | false | queued-unreviewed |
| [#505](https://github.com/Marvisak/VIctory/pull/505) | Tests: add synthetic Xbox runtime conformance under Wine | `d4c8abf932a1a5b62b75b4eeb5c2fb39c55f13aa` | false | queued-unreviewed |
| [#504](https://github.com/Marvisak/VIctory/pull/504) | GPU: handle depth/stencil copy footprints | `b95632012574cd1609c94c0f5a7cf8095a971b65` | false | queued-unreviewed |
| [#503](https://github.com/Marvisak/VIctory/pull/503) | HLE: pin a call's task queue at XAsyncBegin | `3e4d52c4a10d75d8e8b7abb038dc77bc42bd81c7` | false | queued-unreviewed |
| [#502](https://github.com/Marvisak/VIctory/pull/502) | fmt: Format all files | `c2609ea3f9b77dd945689c511ab83516a5c74204` | false | queued-unreviewed |
| [#501](https://github.com/Marvisak/VIctory/pull/501) | gpu: probe a CPU-visible placed-buffer arena | `e5990eab6001e9b05bfcafb299258a93091b3c1e` | false | queued-unreviewed |
| [#500](https://github.com/Marvisak/VIctory/pull/500) | GPU diagnostics: pick capture frames by draw count | `be6df3a3e3ef917ffea4c2d70c0c32c1cef30083` | false | queued-unreviewed |
| [#499](https://github.com/Marvisak/VIctory/pull/499) | shader: lower v_sat_pk_u8_i16 | `5c211d04035c11fa47a267dbb9ded2f12d62a47c` | false | queued-unreviewed |
| [#498](https://github.com/Marvisak/VIctory/pull/498) | shader: lower v_movrelsd_2_b32 and v_swaprel_b32 | `4f48c59608ca01359f1058ab4fa6bc9175862e57` | false | queued-unreviewed |
| [#497](https://github.com/Marvisak/VIctory/pull/497) | GPU: keep what shaders may write by page, in reused page buffers | `5ff3a3585a91738688249def26e08b9bc3a05d40` | false | queued-unreviewed |
| [#494](https://github.com/Marvisak/VIctory/pull/494) | docs: add Indonesian translation (languages/README.id.md) | `0a96e1e090689104c84f76b64423ec37e8ee958d` | false | queued-unreviewed |
| [#493](https://github.com/Marvisak/VIctory/pull/493) | GPU: bind root SRVs as one-view tables bounded to their host copy | `c48b240addc031cb3126c8dbf3fc39de63507b37` | false | queued-unreviewed |
| [#492](https://github.com/Marvisak/VIctory/pull/492) | GPU: substitute R9G9B9E5_SHAREDEXP render targets and UAVs with R16G16B16A16_FLOAT | `b57e10d719b3a3ffb7a8fd04e63a94ea088787d7` | false | queued-unreviewed |
| [#491](https://github.com/Marvisak/VIctory/pull/491) | GPU: remember pipelines whose creation crashed the process (VICTORY_PIPELINE_JOURNAL) | `24c419b0307e9a06ab98d8c18f56e1f31e4db259` | false | queued-unreviewed |
| [#490](https://github.com/Marvisak/VIctory/pull/490) | GPU: ask whether guarded memory is a view once per block, not per asking | `e6f91597e0f1fa560346ed0e88c3b77a4641a237` | false | queued-unreviewed |
| [#489](https://github.com/Marvisak/VIctory/pull/489) | Shaders: spill-table loads read the arguments as they were on entry (stacked on #302) | `279eac1a1b896de9c55eccf385b17d3ebbe320a2` | false | queued-unreviewed |
| [#488](https://github.com/Marvisak/VIctory/pull/488) | GPU: skip zero-sized frame captures | `62cf5dd6b8bdbe2933e9da0970f1b11afa9fde6f` | false | queued-unreviewed |
| [#487](https://github.com/Marvisak/VIctory/pull/487) | HLE: read manifest fields past inline XML comments | `be484a14522fc736429db9fdda97464e5560347c` | false | queued-unreviewed |
| [#486](https://github.com/Marvisak/VIctory/pull/486) | shader: opt the test binaries into an app-local Agility SDK runtime | `6179b10dedfffc7e759fc0ab6bdbf0989679359c` | false | queued-unreviewed |
| [#485](https://github.com/Marvisak/VIctory/pull/485) | GPU: preserve compatible read states for read-only depth targets | `5805f73666c589010fa156479d9e277c5a23b68c` | false | queued-unreviewed |
| [#484](https://github.com/Marvisak/VIctory/pull/484) | Diagnostics/process memory snapshot | `3b7e1d5f950c270818cbe8d7e9c08721935deba4` | false | queued-unreviewed |
| [#483](https://github.com/Marvisak/VIctory/pull/483) | Shaders, GPU: a heap index computed from data stays inside the heap | `65a161d3c83e8c5c0d904c7e92591b05ba4ad23c` | false | queued-unreviewed |
| [#482](https://github.com/Marvisak/VIctory/pull/482) | App: reject wrapping RVAs in VICTORY_WATCH | `c91e6b6b20e3f9e4561286a7e23fb1c196397df9` | false | queued-unreviewed |
| [#481](https://github.com/Marvisak/VIctory/pull/481) | CI: Add GitHub Actions workflow | `9a06b6275093597e8e4d8deba16013aa8bba6372` | false | queued-unreviewed |
| [#480](https://github.com/Marvisak/VIctory/pull/480) | HLE: require two hex digits in percent escapes | `75fe6c02b278f18eab33ad893bda1dd68d176287` | false | review-passed-tests-pending |
| [#478](https://github.com/Marvisak/VIctory/pull/478) | Audio capture: guard WASAPI timestamp errors/underflow, null 0-frame slice UB, unbounded silence allocation, and WAV header u32 overflow | `7ac2760d317c62ac05622f5e2edc2c7d1586ff54` | false | queued-unreviewed |
| [#477](https://github.com/Marvisak/VIctory/pull/477) | Shaders: fix DXIL validator UB on null error blobs, unchecked GetStatus, and COM/DLL handle leaks | `bd59ba346da72d106ef47c2c837779ef8d8e5abd` | false | queued-unreviewed |
| [#476](https://github.com/Marvisak/VIctory/pull/476) | Census: deduplicate PE imports without hint | `a3f89eda547690efa8542b8b993b9de894f05bb1` | false | queued-unreviewed |
| [#475](https://github.com/Marvisak/VIctory/pull/475) | XStore: wire offline *Result slots and fix OOB XStoreShowProductPageUIResult slot | `2c22e2b2d23ef2c6e79e67d5cff51d5c8ec3475d` | false | queued-unreviewed |
| [#474](https://github.com/Marvisak/VIctory/pull/474) | GPU: sanitize CreateSamplerX LOD range, bias clamp, and invalid enum values | `e34cb5396aaa9b1d1d58024e25e1fac1a958a4ef` | false | queued-unreviewed |
| [#472](https://github.com/Marvisak/VIctory/pull/472) | GPU: grow the write snapshot in place, and cap idle pool memory | `f200a2a101e2a61999a8a9f46117231f7fcde71a` | false | queued-unreviewed |
| [#470](https://github.com/Marvisak/VIctory/pull/470) | shader: lower wave prefixes of min and max | `95212d93b4fb94acbb3deb1b4525113ef0f34ccb` | false | queued-unreviewed |
| [#469](https://github.com/Marvisak/VIctory/pull/469) | GPU: give depth targets their unordered access through a twin texture | `c47c963c420292e1655435248b59d5d6d81f9fe1` | false | queued-unreviewed |
| [#465](https://github.com/Marvisak/VIctory/pull/465) | Shaders: lower v_dot2_f32_f16 and v_dot2c_f32_f16 | `1d201237cd90088828b5da3fdb69f8004ce96e80` | false | queued-unreviewed |
| [#464](https://github.com/Marvisak/VIctory/pull/464) | GPU: remember unmapped guarded view blocks instead of re-copying them every submission | `41af42e5fd3a5d68df60a1bb60b325dc8ab45ad1` | false | queued-unreviewed |
| [#463](https://github.com/Marvisak/VIctory/pull/463) | GPU: load an app-local D3D12 Agility SDK runtime when present | `f47a36e83f86f6b377aa6741d9dd2fee22fca551` | false | queued-unreviewed |
| [#461](https://github.com/Marvisak/VIctory/pull/461) | GPU: keep the initial pipeline of new command lists | `bf84506212307603110cf06d27388c7ee9aa871f` | false | queued-unreviewed |
| [#460](https://github.com/Marvisak/VIctory/pull/460) | Metal: add native Apple Silicon GPU conformance harness | `8ce597b2beea8aa5073391010f012755c7a7fc76` | false | queued-unreviewed |
| [#458](https://github.com/Marvisak/VIctory/pull/458) | Fix GPU hangs, memory leak, and mesh shader exports; render intro cre… | `360f38256dea4e80f5b3afd84227b61462b9b6a9` | false | queued-unreviewed |
| [#457](https://github.com/Marvisak/VIctory/pull/457) | fix(shader): accept overwritten VGPR inputs in counted indexed writes | `efcd753ae24b337a70468e00035a108607504461` | false | queued-unreviewed |
| [#456](https://github.com/Marvisak/VIctory/pull/456) | shader: raise v_interp_p1lv_f16 | `4b8a1ca6dd0d30a7c39d6d633d97e850dae2bccf` | false | queued-unreviewed |
| [#455](https://github.com/Marvisak/VIctory/pull/455) | HLE: honor the requested executable on relaunch | `a9bc94e5e4a378835265c0150f89f58ead24d9b1` | false | queued-unreviewed |
| [#454](https://github.com/Marvisak/VIctory/pull/454) | Loader: opt-in VICTORY_NO_WATCHDOG hook for the title's watchdog (build 1.0.2.1) | `d2c93fac49c2138402742128b953b45e2299a7b5` | false | queued-unreviewed |
| [#453](https://github.com/Marvisak/VIctory/pull/453) | Shaders: pixel barycentrics through SV_Barycentrics (values and computed interpolation) | `88b34a447350eeb1e747be84e4204dbb3955c58c` | false | queued-unreviewed |
| [#452](https://github.com/Marvisak/VIctory/pull/452) | Shaders: spin-wait loops stop after 4096 iterations; loop limit in release builds | `3ede93437dc27bcd116f8a67bebe75212ce4bbe1` | false | queued-unreviewed |
| [#451](https://github.com/Marvisak/VIctory/pull/451) | HLE: keep fitted window dimensions positive | `8ff871b71c5a73b2ce0a457ea33bd87a61f3a1fd` | false | queued-unreviewed |
| [#450](https://github.com/Marvisak/VIctory/pull/450) | GPU: preserve typed null render target slots | `93edd8e0acfa8ac4d28fdec6c1893b2d28d297fb` | false | queued-unreviewed |
| [#449](https://github.com/Marvisak/VIctory/pull/449) | HLE: refuse a null AnalyticsInfo output pointer | `f625cc508c0d3a1b3f124cd3714dde681440d8c4` | false | review-passed-tests-pending |
| [#447](https://github.com/Marvisak/VIctory/pull/447) | Shaders: a signed 64-bit operand sign-extends its 32-bit literal | `7d049431de9b12873d801de2b8a54ab4007fcde9` | false | queued-unreviewed |
| [#446](https://github.com/Marvisak/VIctory/pull/446) | HLE: report required size on string getter buffer errors | `881caef98fa0e2e08c4b37fd1f6e2d759aa2f40c` | false | queued-unreviewed |
| [#443](https://github.com/Marvisak/VIctory/pull/443) | Audio: keep XMA frames split across rolling inputs | `f546d6501d40349f923eb64f9df59b1c079e89f4` | false | queued-unreviewed |
| [#441](https://github.com/Marvisak/VIctory/pull/441) | run: let victory.toml pick the console | `157e0ceea175558534dcb6cf0051b2148b9a46d1` | false | queued-unreviewed |
| [#440](https://github.com/Marvisak/VIctory/pull/440) | GPU: track each write range once per submission | `0ba3e44575a2590bfc583e8e4d232fcfbcbb5c84` | false | queued-unreviewed |
| [#439](https://github.com/Marvisak/VIctory/pull/439) | Shaders: one console wave sharing LDS runs in the console flow on 32-lane hosts | `c153b21c9f0ea9909a424fe7f56530c8eabbde3f` | false | queued-unreviewed |
| [#438](https://github.com/Marvisak/VIctory/pull/438) | CI: native Apple Silicon builds and synthetic offline shader tests | `4e107dd3c8a05a63fcb23e59896947f479522719` | false | queued-unreviewed |
| [#437](https://github.com/Marvisak/VIctory/pull/437) | Shaders: preserve raw scalar buffer loads on NVIDIA | `598d22182abe38e5500d5b4da3d5b1229893a737` | true | queued-unreviewed |
| [#436](https://github.com/Marvisak/VIctory/pull/436) | GPU: copy a shadow's edge pages whole so they are not copied again | `e914ed456f975ccfe94ecf7d727cadb27477993f` | false | queued-unreviewed |
| [#435](https://github.com/Marvisak/VIctory/pull/435) | GPU: coalesce identical write declarations within a submission | `657efd8279422a5e35b51d9240350b223ecfb4e8` | false | queued-unreviewed |
| [#434](https://github.com/Marvisak/VIctory/pull/434) | Shaders: a wave32 program's mask read picks its half of a 64-lane ballot | `48f92e23c152fbef4f4434b8b9341e357e2220aa` | false | queued-unreviewed |
| [#431](https://github.com/Marvisak/VIctory/pull/431) | GPU: free host textures only after running submissions, and make them on first use | `23ced703f072075936abe863d8d6f1eb2990309c` | false | queued-unreviewed |
| [#430](https://github.com/Marvisak/VIctory/pull/430) | DirectStorage: Close completes what was submitted | `129d2897c43fed991ffb1160b49e85a6b1a1aba0` | false | queued-unreviewed |
| [#428](https://github.com/Marvisak/VIctory/pull/428) | GameInput: next and previous readings of a disconnected device fail | `a312333c28ebfd4944d2943b8cb642f4a306469c` | false | queued-unreviewed |
| [#427](https://github.com/Marvisak/VIctory/pull/427) | GPU: say in device-removed reports which user data is a table | `16d395ccc0974d634158a404b5cdbb2c0d5760a8` | false | queued-unreviewed |
| [#426](https://github.com/Marvisak/VIctory/pull/426) | Shaders: lower native 16-bit buffer stores | `e421755a6f1ccbd306a60429032eb4171180d817` | false | queued-unreviewed |
| [#425](https://github.com/Marvisak/VIctory/pull/425) | Game: validate DXBC containers in SGD5 records | `012ce71c5a8ce7ed19c9d443a691136924e6ebdc` | false | queued-unreviewed |
| [#424](https://github.com/Marvisak/VIctory/pull/424) | README: quote the executable path in the PowerShell launch example | `5b9a9b37ef2691dbdc40e7fc1637e0abdee1dac3` | false | queued-unreviewed |
| [#422](https://github.com/Marvisak/VIctory/pull/422) | Shaders: narrow f32 to f16 to nearest even, as the console does | `a746d533f4077cf6c9888942b3f4b08144a1bb06` | false | queued-unreviewed |
| [#421](https://github.com/Marvisak/VIctory/pull/421) | HLE: clean dot parts in extended virtual-drive paths | `1d0f48a93f04dde16dd9d414737254739f1800b6` | false | queued-unreviewed |
| [#420](https://github.com/Marvisak/VIctory/pull/420) | GPU: include root-selected entries in rebuilt descriptor tables | `b4b9498abc00f5437b1e870476cbb6607215c3c3` | false | queued-unreviewed |
| [#416](https://github.com/Marvisak/VIctory/pull/416) | shader: implement v_cvt_f32_f64 instruction lowering | `7516f947b2e5d92c42b557b90459a0ba3265132b` | false | queued-unreviewed |
| [#415](https://github.com/Marvisak/VIctory/pull/415) | GPU: use native root type codes in the shader cache | `6924b095e3bfc128a3c85b3edc3332cd94274f5a` | false | queued-unreviewed |
| [#413](https://github.com/Marvisak/VIctory/pull/413) | GPU: materialize bindless ResourceDescriptorHeap views, refreshing only changed slots | `51f447b56a307bd5b469f31f803ac8be73fab3ac` | false | queued-unreviewed |
| [#412](https://github.com/Marvisak/VIctory/pull/412) | HLE: validate XPackage chunk selectors | `d2c7d4a329abb59d669867c5b5020776eb7963a9` | false | queued-unreviewed |
| [#411](https://github.com/Marvisak/VIctory/pull/411) | GPU: require only the descriptor heaps used by the pipeline | `16a5893ea7fb94d1492e726c38f53caa70c79932` | false | queued-unreviewed |
| [#409](https://github.com/Marvisak/VIctory/pull/409) | GPU: match D3D12 copyable texture footprints | `340499c44c7f52b47b856b2bd97dd7626a059b5d` | false | queued-unreviewed |
| [#408](https://github.com/Marvisak/VIctory/pull/408) | xgamesave: defer incomplete legacy migrations | `44d20a326f78d9c0bfe98145dc9dd9fa2064c644` | false | queued-unreviewed |
| [#406](https://github.com/Marvisak/VIctory/pull/406) | loader: ignore incomplete relocation entries | `b33c704e3335ac2644faafc1ff92bac899e65195` | false | queued-unreviewed |
| [#405](https://github.com/Marvisak/VIctory/pull/405) | Fix clippy field_reassign_with_default in xmem | `49f113baf8b1aacb84c171922def2f1c7c1a9d17` | false | queued-unreviewed |
| [#404](https://github.com/Marvisak/VIctory/pull/404) | shader: lower v_permlanex16_b32 | `d505b0e4c5e3d29a7d027e639d70651386f71b2a` | false | queued-unreviewed |
| [#403](https://github.com/Marvisak/VIctory/pull/403) | xtaskqueue: preserve other work on shared ports | `4ee6efa4a0c686d1def48c4bfebd45caa0aa9d75` | false | queued-unreviewed |
| [#402](https://github.com/Marvisak/VIctory/pull/402) | GPU: find a buffer holding an address past many smaller ones; reuse loose windows that hold a range | `022e9c82f0ee147a16b5a06c391de5a8d9beeab3` | false | queued-unreviewed |
| [#401](https://github.com/Marvisak/VIctory/pull/401) | GPU: reduce work in texture refresh and readback | `0b5838d4d2711021beca723254075f67a248d4c2` | false | queued-unreviewed |
| [#399](https://github.com/Marvisak/VIctory/pull/399) | xgamesave: preserve older versions on manifest error | `54f8cee58682bcdac13067542b07d1116fb8b66e` | false | queued-unreviewed |
| [#398](https://github.com/Marvisak/VIctory/pull/398) | HLE: report the single supported offline user | `1d6f71e78730821f89f10c65fba14713d62b05fd` | false | queued-unreviewed |
| [#396](https://github.com/Marvisak/VIctory/pull/396) | pe: bound data directories to the optional header | `10e9f56a50214df67dca98b19daac0d8e9b04075` | false | coordinator-review |
| [#395](https://github.com/Marvisak/VIctory/pull/395) | shader: unpack compressed f16 pixel exports | `c0d37697514e46c72a2693e216c1804a4b1a6be9` | false | queued-unreviewed |
| [#393](https://github.com/Marvisak/VIctory/pull/393) | GPU: refresh a buffer copy's ranges by page; answer clean guarded views from the journal | `6e7186c09b15f25822b05707f0b7ded00f57a747` | false | queued-unreviewed |
| [#391](https://github.com/Marvisak/VIctory/pull/391) | Shaders: refuse LDS past the host's group limit by name | `a3f219ecbdb7601fbee235515d41d1114eaf2195` | true | queued-unreviewed |
| [#390](https://github.com/Marvisak/VIctory/pull/390) | Crash log: a panic carries our Rust stack | `e125477cbd6df1267ea4e5c099d13ab5e9e03dba` | true | queued-unreviewed |
| [#389](https://github.com/Marvisak/VIctory/pull/389) | Shader cache: a cache that cannot write still translates | `c84e0e60bfc13de5e603c445ebaaf827690ae6cd` | true | queued-unreviewed |
| [#388](https://github.com/Marvisak/VIctory/pull/388) | GPU: refresh unwatched textures without a region query; detile into cached memory | `e54b6756f5b2a9c218f8f8c7ab4f8931d7ab9030` | false | queued-unreviewed |
| [#387](https://github.com/Marvisak/VIctory/pull/387) | GPU: preserve execution ordering for buffer copies | `df2276ba063ce0a23b34a422d2f81a95d8e7af9c` | true | queued-unreviewed |
| [#385](https://github.com/Marvisak/VIctory/pull/385) | Shaders: track every incoming edge of a lane-mask phi | `a34512768a10ab8996e36f583d6b33f9129b17bd` | false | queued-unreviewed |
| [#383](https://github.com/Marvisak/VIctory/pull/383) | Shader: LDS bytes and halves, constant sample offsets, descriptor-data reasons | `b6af8b177e46e14b0ac9e372c287a8db63c83630` | false | queued-unreviewed |
| [#380](https://github.com/Marvisak/VIctory/pull/380) | github: add a bug report issue form | `43551948876eb828dacfc8cbcee0e37f4a7f8da1` | false | queued-unreviewed |
| [#379](https://github.com/Marvisak/VIctory/pull/379) | Shaders: lower v_div_fixup_f16 | `5ecb6abe769d2507a64324637c34d1597ab27e39` | false | queued-unreviewed |
| [#378](https://github.com/Marvisak/VIctory/pull/378) | CLI: include runtime shader hashes in the translation listing | `2392dfd79a190bd3a8a0ab568b28d306cda37980` | false | queued-unreviewed |
| [#377](https://github.com/Marvisak/VIctory/pull/377) | policy: fail when staged content cannot be read | `4da59d10713631d263ebd27b039474eeeee585d6` | false | queued-unreviewed |
| [#374](https://github.com/Marvisak/VIctory/pull/374) | XAsync: a wait on a failed call during its cleanup waits for the status | `9d6bc0c7c6811c012f39501aa297ddd261d3870f` | false | queued-unreviewed |
| [#373](https://github.com/Marvisak/VIctory/pull/373) | Kernel: GetModuleFileName of a title image into a null buffer | `c000581df2cf12aaadfe49025f9abba8b5993c8e` | false | queued-unreviewed |
| [#370](https://github.com/Marvisak/VIctory/pull/370) | XGameUi: a text entry's limit is the console's | `936e5e51c91c7b380a4729aeaa2255824ce8085a` | false | queued-unreviewed |
| [#369](https://github.com/Marvisak/VIctory/pull/369) | Shaders: eliminate heap allocations during CFG traversal | `341782191c557d278810a613d4dbf8cf47421094` | false | queued-unreviewed |
| [#368](https://github.com/Marvisak/VIctory/pull/368) | Shaders: lower buffer cache invalidations as no-ops | `42c86c36d1f25e486c347279024d71dc70aefcde` | false | queued-unreviewed |
| [#364](https://github.com/Marvisak/VIctory/pull/364) | XMA: accelerate VLC decoding with an eight-bit prefix lookup | `e503756597cafb3ac6c259ec82458fa5c61215aa` | false | queued-unreviewed |
| [#363](https://github.com/Marvisak/VIctory/pull/363) | Config: ignore an empty VICTORY_CONFIG | `4874c3fdc18e01d0a1973fc62558669ed9e18ead` | false | queued-unreviewed |
| [#362](https://github.com/Marvisak/VIctory/pull/362) | Shader tests: use the Agility SDK runtime on Windows 10 (stacked on #175) | `1e33e8838ca6761eee0e51d138a446ff43b3a718` | false | queued-unreviewed |
| [#361](https://github.com/Marvisak/VIctory/pull/361) | Input: with no device, previous and next readings walk every device in time order | `3f5eff57603270823920e15b93479dc168b08eb9` | false | queued-unreviewed |
| [#360](https://github.com/Marvisak/VIctory/pull/360) | shader: support image_sample_o and image_sample_lz_o texel offset opcodes | `38ac9ef132e26e7fc39d6f3a67e5623208852332` | false | queued-unreviewed |
| [#359](https://github.com/Marvisak/VIctory/pull/359) | HLE: a '>' in a quoted manifest attribute does not end its tag | `d549b6397e3f40e77f9da670344aa15f12b26abb` | false | queued-unreviewed |
| [#358](https://github.com/Marvisak/VIctory/pull/358) | GPU: a shader cache that cannot keep an entry still gives its translation | `369dc3246931ec4d0d9a1e0aaec0dae2da717d1c` | false | queued-unreviewed |
| [#357](https://github.com/Marvisak/VIctory/pull/357) | HLE: load a host DLL outside the module cache's lock (loader-lock deadlock) | `c6959acba38304852c185bcd7bc2ba1478d6e0b2` | false | queued-unreviewed |
| [#354](https://github.com/Marvisak/VIctory/pull/354) | docs: explain Series S render profile and memory limits | `9f28f6b9845b06b68c7981fcbed93f9f62ba8993` | false | queued-unreviewed |
| [#353](https://github.com/Marvisak/VIctory/pull/353) | Shader info: show stage I/O even when instruction decoding fails | `e6ebec9ccf0bc27cfe0b5509ec3d07a0969eb25f` | false | queued-unreviewed |
| [#352](https://github.com/Marvisak/VIctory/pull/352) | GPU: mesh pipelines load from the pipeline library (name their topology) | `f77bd28c6f4f1ae10f151d5ac4d6ca5dc003e1a9` | false | queued-unreviewed |
| [#351](https://github.com/Marvisak/VIctory/pull/351) | Formats: reject overlapping texture depth slices | `93a24b7cdaeb0e7f132f711e9cbd9ba405b0f639` | false | queued-unreviewed |
| [#349](https://github.com/Marvisak/VIctory/pull/349) | HLE tests: Windows 10's winhttp.dll may lack WinHttpGetProxySettingsEx (#118) | `14e78f8c4e1e319ec38e1bcceca4bfc3807ddd56` | false | queued-unreviewed |
| [#347](https://github.com/Marvisak/VIctory/pull/347) | GPU: ask for an empty pipeline library with a null blob | `197c7acf1fb0a1bd51db4f54d6ae934f91edcf25` | false | queued-unreviewed |
| [#346](https://github.com/Marvisak/VIctory/pull/346) | GPU: expand indirect rectangle-list draws on the GPU | `8ef6bfea1682c903c7b7ee6b887595bfa86838a7` | false | queued-unreviewed |
| [#345](https://github.com/Marvisak/VIctory/pull/345) | docs: add Brazilian Portuguese translation | `177783ad0c44c201bb3715c61a1911234712cf1a` | false | queued-unreviewed |
| [#343](https://github.com/Marvisak/VIctory/pull/343) | Shaders: reject overlapping DXBC container parts | `9459c12d2e3a3d48ac2e073e7d3d9a888bfa913d` | false | queued-unreviewed |
| [#342](https://github.com/Marvisak/VIctory/pull/342) | Shaders: lower partial-channel typed buffer stores | `4950d617111160ecc8f517652ea2ea6ec780cd9e` | false | queued-unreviewed |
| [#337](https://github.com/Marvisak/VIctory/pull/337) | fix(hle): saturate storage quota conversion | `5cf259e5fe2f9361fffbc00e9b5bd46067b6686d` | false | queued-unreviewed |
| [#335](https://github.com/Marvisak/VIctory/pull/335) | XStore: a store id without an installed package is an invalid argument | `e3bef089855cc4b463a0ce7e1a70f6abf7c859e5` | false | queued-unreviewed |
| [#334](https://github.com/Marvisak/VIctory/pull/334) | GPU: transition host copies of title memory between shader writes and reads | `af2aecf6c130e6c931b9ca58a15718530c16c3ba` | false | queued-unreviewed |
| [#333](https://github.com/Marvisak/VIctory/pull/333) | Shaders: keep native root parameters inside the SGPRs, clamp the thread-id count, and name each blob refusal | `a23184d50b67fcdff5c0c16f665cc5b791303476` | false | queued-unreviewed |
| [#332](https://github.com/Marvisak/VIctory/pull/332) | Shaders: lower partial-channel image_store | `a5b0dcbec0783dc070a51b3b12737d570fb5999e` | false | queued-unreviewed |
| [#331](https://github.com/Marvisak/VIctory/pull/331) | GPU: sRGB and linear views of one texture read with their own curve | `ccdecb80807826e49ce82a5d9aa6e7814d2b8376` | false | queued-unreviewed |
| [#329](https://github.com/Marvisak/VIctory/pull/329) | Shaders: recover record counts for descriptor-only structured buffers | `02564c5d9561021eb6cfba01e91b1898192a60ac` | false | queued-unreviewed |
| [#328](https://github.com/Marvisak/VIctory/pull/328) | Shaders: lower MSAA image_load and image_get_resinfo | `91e7742f9852128ae456224c53993fd661b6457c` | false | queued-unreviewed |
| [#326](https://github.com/Marvisak/VIctory/pull/326) | Shaders: DXBC/DXIL parser and sampler/SP3 robustness fixes | `c8d72832706ccc402df0664cc353c898b2ecf5f8` | false | queued-unreviewed |
| [#324](https://github.com/Marvisak/VIctory/pull/324) | fix(pe): detect incomplete TLS callback arrays | `8fc3a1bb2f37c6220c8f9a397ff5d1a99f6e34a7` | false | queued-unreviewed |
| [#323](https://github.com/Marvisak/VIctory/pull/323) | docs: debugging notes from a GTA V port | `e08ac6a9dc6bfe1b679ff6be7ef3ff4d1c76be70` | false | queued-unreviewed |
| [#322](https://github.com/Marvisak/VIctory/pull/322) | GPU: copy into title memory without asking for its region first | `b58d198d954e63f3aa7e2a8e856bf70fbb0af291` | false | queued-unreviewed |
| [#321](https://github.com/Marvisak/VIctory/pull/321) | GPU: honour scheduled frame-event offsets | `3ff556668001e3e76d5ef580eaa0b964f1eabaee` | false | queued-unreviewed |
| [#320](https://github.com/Marvisak/VIctory/pull/320) | Shaders: lane reads and writes where lanes branch differently run in the console flow | `0fa6af437d02ceb643e32bf9fc32415a5ae00b0a` | false | queued-unreviewed |
| [#319](https://github.com/Marvisak/VIctory/pull/319) | GPU: end a late flip wait on the tick it missed, not the next | `b36bd4c1d655f4d2f2069e4369a402a2828091e2` | false | queued-unreviewed |
| [#318](https://github.com/Marvisak/VIctory/pull/318) | Shader: pack a16 image addresses as the hardware does; image_sample_d_cl | `1d7bf9b0bdf3b445b6d13a6d920e7dd7e59a6dd1` | false | queued-unreviewed |
| [#317](https://github.com/Marvisak/VIctory/pull/317) | Shaders: support cube and MSAA dimensions in descriptor_data | `8133754ea5af95033702f53c22f23d1cf6b1c4f3` | false | queued-unreviewed |
| [#315](https://github.com/Marvisak/VIctory/pull/315) | Formats: console formats 189/190 have texture layouts; rustdoc builds without warnings | `e93b803d2df8e0fdcae0e00fa70fc59dd900c5cb` | false | queued-unreviewed |
| [#312](https://github.com/Marvisak/VIctory/pull/312) | HLE: SDL3 gamepad backend for the gameinput shim | `191e335d784888d37d7c0626a95c377e38ba87e5` | false | queued-unreviewed |
| [#311](https://github.com/Marvisak/VIctory/pull/311) | test(pe): cover header layout offsets | `1251a24b78ee00b7fc180e9ed7b41e4e64257bac` | false | queued-unreviewed |
| [#310](https://github.com/Marvisak/VIctory/pull/310) | Policy: resolve cache paths inside Windows AppContainer | `1ea6427f05078f9771d724ba180c8a826c9bf009` | false | queued-unreviewed |
| [#309](https://github.com/Marvisak/VIctory/pull/309) | Shaders: forward SGPRs reloaded from VGPR lanes as the spilled value | `64c7188a17f998756edf57a1c6aa803f4e2c7022` | false | queued-unreviewed |
| [#308](https://github.com/Marvisak/VIctory/pull/308) | Shaders: follow lane writes and scalar steps in whole-wave sections | `3fba97c7c5ced2cb7f91883d31b82f72063d79b2` | false | queued-unreviewed |
| [#306](https://github.com/Marvisak/VIctory/pull/306) | Shader: fix biased 16-bit image samples and DXIL validator failures | `a8430f220700b523660359ce5f2d162ed89eee9a` | false | queued-unreviewed |
| [#305](https://github.com/Marvisak/VIctory/pull/305) | Shaders: lower 64-bit image atomics | `78dae836269750512b7b645ef715a5e45dcbd336` | false | queued-unreviewed |
| [#302](https://github.com/Marvisak/VIctory/pull/302) | Shaders: read root arguments the console spills to memory | `815178dcc01be31f110c56127feacc0720dd89dd` | false | queued-unreviewed |
| [#301](https://github.com/Marvisak/VIctory/pull/301) | GPU: non-stream graphics PSOs, stream subobjects 25..=28, texture-to-buffer readbacks, and sRGB view preservation | `1af462451537270dc9b0d508fb5e70bf535622be` | false | queued-unreviewed |
| [#299](https://github.com/Marvisak/VIctory/pull/299) | Docs: correct rumble status and the HDR note | `2fc6db0b2cda3efe8ead51eef04de3782349e048` | false | queued-unreviewed |
| [#298](https://github.com/Marvisak/VIctory/pull/298) | Shaders: lane masks spilled to VGPR lanes and read back | `54f97dbb31b6adc8fe7135141842160ee160199f` | false | queued-unreviewed |
| [#294](https://github.com/Marvisak/VIctory/pull/294) | GPU & shader: full Root SRV extent refresh, safe fallback VAs, and Root UAVs | `0050ce50fd58c14501eb0390993bac35c0f8386c` | false | queued-unreviewed |
| [#292](https://github.com/Marvisak/VIctory/pull/292) | Shader: lower WQM QuadAny, clamp LDS accesses, and guard loop budgets & barriers | `a2a1b1e9445242777a6a566637432e601a0c600d` | false | queued-unreviewed |
| [#290](https://github.com/Marvisak/VIctory/pull/290) | HLE tests: cover Windows path rules on every host | `e9caf0addb17a79b874cdcc0c5ec1430d88d4d9d` | false | queued-unreviewed |
| [#288](https://github.com/Marvisak/VIctory/pull/288) | Shaders: cap BDD memo table and bail out early on overflow | `176c766ab1814ab4be0602709f2f80abb466fd84` | false | queued-unreviewed |
| [#286](https://github.com/Marvisak/VIctory/pull/286) | Shaders: accept known D3D_MIN_PRECISION signature elements | `6afdb58eeb875499ff79b3f8434500894df2ace4` | false | queued-unreviewed |
| [#284](https://github.com/Marvisak/VIctory/pull/284) | Shaders: RDNA2's legacy multiply-adds are the fused ones | `fa5189da66a91611c41fc1bdafd3675e15eb50f1` | false | queued-unreviewed |
| [#282](https://github.com/Marvisak/VIctory/pull/282) | GPU: run indirect rectangle-list draws (#255) | `4d3d003c3c73e163ff23e91335829209e7243b83` | false | queued-unreviewed |
| [#281](https://github.com/Marvisak/VIctory/pull/281) | feat(shader): add opt-in perspective-center barycentric inputs | `d9bbff571eabf7defe6b106349acda49d1eaa3f8` | true | queued-unreviewed |
| [#280](https://github.com/Marvisak/VIctory/pull/280) | README: a hard drive keeps the first screen black for ~90 s | `7ecc6209b8b3a81a541f0773f730407d37edd2ed` | false | queued-unreviewed |
| [#279](https://github.com/Marvisak/VIctory/pull/279) | GPU: name the commands at a DRED stop, and dump translated shaders | `02f0e29f8031d7b97353e9b90f6bbab5495b6b10` | false | queued-unreviewed |
| [#278](https://github.com/Marvisak/VIctory/pull/278) | GPU: answer format-info and virtual-address feature queries | `cbf3584c32daf165cb12d2c90d84717b71f85409` | false | queued-unreviewed |
| [#275](https://github.com/Marvisak/VIctory/pull/275) | GPU: atomic adaptive PSO wait budget and non-blocking pipeline library saves | `96b282274c7a34900436f859989d1f70c57ed5f0` | false | queued-unreviewed |
| [#273](https://github.com/Marvisak/VIctory/pull/273) | SGD5: a marker inside a record's packed bytes is not another record | `ef08225cfde81df12b7c97126a51091b38624c4d` | false | queued-unreviewed |
| [#270](https://github.com/Marvisak/VIctory/pull/270) | GPU: every object answers for IUnknown | `92f2c60c8159e34a47570aa81f3d25624cc6db5c` | false | queued-unreviewed |
| [#269](https://github.com/Marvisak/VIctory/pull/269) | Shaders: scalars leave divergent loops and branches as the wave's last path left them | `9356beeae173723585c2aeac91f68c34862c00a4` | false | queued-unreviewed |
| [#265](https://github.com/Marvisak/VIctory/pull/265) | Shaders: private (scratch) memory as a per-lane array | `d9c04c674eeb3f3ca0a032461ba413634d4e7ca4` | false | queued-unreviewed |
| [#263](https://github.com/Marvisak/VIctory/pull/263) | Formats: reserve texture metadata memory in XG layouts | `e5eaf4adb69c8ed75c2c0b41531b4a55b02cfb1b` | false | queued-unreviewed |
| [#261](https://github.com/Marvisak/VIctory/pull/261) | GPU: copy-back puts back only what shaders wrote, in two more cases | `a2243adc60d86e1540989447981a8e8e5c27f2fe` | false | queued-unreviewed |
| [#259](https://github.com/Marvisak/VIctory/pull/259) | Shaders: query a descriptor's level count only when word 3 is read | `e232d3d7a210d81691fd3d644520817e22cfb48f` | false | queued-unreviewed |
| [#256](https://github.com/Marvisak/VIctory/pull/256) | fix(hle): skip unavailable host exports in net test | `4c57d95c6c15aac53f91a9cd803f586ec0a43eac` | false | queued-unreviewed |
| [#254](https://github.com/Marvisak/VIctory/pull/254) | Pipeline & CLI: stable shader cache key, in-flight dedup, and AOT warm-up | `0e6b2b4db5eee53aa3dde1e4eaafcf4e80f5aab9` | false | queued-unreviewed |
| [#251](https://github.com/Marvisak/VIctory/pull/251) | Find free descriptor tables that span the search cursor | `deb58a669560673b3ba6dc53903b52028b28ffe0` | false | queued-unreviewed |
| [#250](https://github.com/Marvisak/VIctory/pull/250) | GPU & CLI: configurable/uncapped FPS limit, high-res pacing, high-refresh modes, and tearing swapchain | `031c5c14818dfa4a2cd4993f79d6f0a508265749` | false | queued-unreviewed |
| [#247](https://github.com/Marvisak/VIctory/pull/247) | XAsync: a new block where a result was left behind begins a call | `5cff0007a525b83f5c69912d7a38f53c6dafcfef` | false | queued-unreviewed |
| [#246](https://github.com/Marvisak/VIctory/pull/246) | Policy and hooks: staged type changes, pinned exception, executable pre-commit | `0d6cf21506d14c27b14e0c5eb86baaf6e6698594` | false | queued-unreviewed |
| [#244](https://github.com/Marvisak/VIctory/pull/244) | Shaders: share and serialize WARP host operations across threads | `7f95b87ea94514a02d180d562922f6fd25704454` | false | queued-unreviewed |
| [#242](https://github.com/Marvisak/VIctory/pull/242) | GPU: make ExecuteIndirect clamping countBuffer-aware and non-destructive | `ee61c57e44c59235c06fa813f557040ba8be52c2` | false | queued-unreviewed |
| [#241](https://github.com/Marvisak/VIctory/pull/241) | Core host UX: XDG data dir, config-relative paths, and clearer VICTORY_LOG handling | `e8fbe9363d475bf392627d69837be765445b6db2` | false | queued-unreviewed |
| [#240](https://github.com/Marvisak/VIctory/pull/240) | DSP: convolve reverb streams on the host | `2d1036d73f99fc364241d20e877f24d19d5f5533` | false | queued-unreviewed |
| [#238](https://github.com/Marvisak/VIctory/pull/238) | CLI: check for AVX2 before running a title (VICTORY_SKIP_CPU_CHECK=1 to bypass) | `397005f6ddc943fc727245c9791241674217aa9c` | false | queued-unreviewed |
| [#237](https://github.com/Marvisak/VIctory/pull/237) | HLE: offload async saves, reuse audio buffers, pin APU heap, and decouple input rumble/dialogs | `203d9f38407edeb74f37ae03f7a44b323cf402fb` | false | queued-unreviewed |
| [#236](https://github.com/Marvisak/VIctory/pull/236) | XAsync: a canceled pending call's work touches neither freed state nor the next call | `f0393ddd90c7fbb24485d5ac6c6d7f6a9a40a611` | false | queued-unreviewed |
| [#235](https://github.com/Marvisak/VIctory/pull/235) | Build: gate Windows-only paths so the workspace builds and tests on Linux | `5d40ad817cc836d7431c98a0fc945bde906e2919` | false | queued-unreviewed |
| [#234](https://github.com/Marvisak/VIctory/pull/234) | HLE: overlapped DirectStorage I/O and persistent xmem section views | `423d3f4e1f9b000525eb8f1d22b75ec2e97e0fee` | false | queued-unreviewed |
| [#233](https://github.com/Marvisak/VIctory/pull/233) | Shaders: wave64 compute on 32-lane hosts, in halves and in the console flow | `d1fecaa1ef231f8ad24736d58e9d8e1e149938bd` | false | queued-unreviewed |
| [#232](https://github.com/Marvisak/VIctory/pull/232) | Linux/Wine testing: interim Linux CI subset; skip load_probe cargo build under Wine | `2c83383a429aa7b01df6324724383da7d72aa10e` | false | queued-unreviewed |
| [#231](https://github.com/Marvisak/VIctory/pull/231) | Draft integration test of 138 PRs on Windows | `e440f471b159e0ac940612d0115e2e2be2cd6154` | true | queued-unreviewed |
| [#230](https://github.com/Marvisak/VIctory/pull/230) | GPU: skip unchanged texture uploads via prelude copy recording | `6a96adbfb8fcb001686dc251a24cda3dd34ee51c` | false | queued-unreviewed |
| [#228](https://github.com/Marvisak/VIctory/pull/228) | XGameSave: a failed create leaves a null handle, as documented | `09ff84b6a7cb5cc6f612697866113d3d791ab2ae` | false | queued-unreviewed |
| [#226](https://github.com/Marvisak/VIctory/pull/226) | Shaders: test that an image read as float and used by an atomic is annotated twice | `6e01ea031553e1bd19b37ffae5cc2facaa18920b` | false | queued-unreviewed |
| [#224](https://github.com/Marvisak/VIctory/pull/224) | HLE: report the host's commit headroom before the title starts | `b87a9682e3462491bed43e99d91fcc51b39ba7a9` | false | queued-unreviewed |
| [#223](https://github.com/Marvisak/VIctory/pull/223) | Shaders: translate for the host's wave sizes | `714b81805f98a48976e98863aa352172af31565b` | false | queued-unreviewed |
| [#222](https://github.com/Marvisak/VIctory/pull/222) | apu: warn when the host has no audio output device | `9a77707b01c040e7c4480fb96b2c6e17b79b04d8` | false | queued-unreviewed |
| [#221](https://github.com/Marvisak/VIctory/pull/221) | Loader: suspend a thread before reading its TEB; skip bad dump times | `acdcc41687be2ecd2bf2efc43ab7449329483382` | false | queued-unreviewed |
| [#219](https://github.com/Marvisak/VIctory/pull/219) | fix(pe): validate section raw data bounds | `e57e76a7b4058e50d665ddcc9a11eb3cd4de4e7d` | false | queued-unreviewed |
| [#218](https://github.com/Marvisak/VIctory/pull/218) | APU: decode-convert and PCM rate conversion | `717560849f7866ce2fe01d46cb14849a32958689` | false | queued-unreviewed |
| [#217](https://github.com/Marvisak/VIctory/pull/217) | Wine: fall back where ws2_32 lacks exports; hook and cfg fixes | `dbbe37de47c1b0b392b7f64a480db454ca3cdae3` | false | queued-unreviewed |
| [#216](https://github.com/Marvisak/VIctory/pull/216) | Loader & HLE: harden VEH crash handler, PE prep/exports/TLS, and allow oleaut32 | `6269b57090f44a54be9b4b861f2e83b13a20ae73` | false | queued-unreviewed |
| [#215](https://github.com/Marvisak/VIctory/pull/215) | Reuse one CPU read buffer for watched textures | `0f7052e38ac18d58612619c3f612f9050b9769e1` | false | queued-unreviewed |
| [#214](https://github.com/Marvisak/VIctory/pull/214) | Loader: clzero loops in one exception; rdpru (#34 rebased, with a fix) | `9de712f5a1c6dde04306c9815a99f8e35a77f34d` | false | queued-unreviewed |
| [#213](https://github.com/Marvisak/VIctory/pull/213) | Crash reports: name the host's CPU and Windows build | `7f00a62a4b861f0a051cc600470ce0570ab43c6f` | false | queued-unreviewed |
| [#212](https://github.com/Marvisak/VIctory/pull/212) | GPU, HLE: report host display HDR luminance and configure swap chain color space | `4b18e64f23d42e2130e0bd502473bf5fe0f341e7` | false | queued-unreviewed |
| [#211](https://github.com/Marvisak/VIctory/pull/211) | GPU: composite multiple PresentX planes (fixes Series S black UI/menus) | `24a387d700bc18e84ea413f2b9f34d564fdf0a54` | false | queued-unreviewed |
| [#208](https://github.com/Marvisak/VIctory/pull/208) | GPU: select high-performance D3D12 adapter and probe host caps | `eb723596bd64760b49be8269f310e1393bc6f8ed` | false | queued-unreviewed |
| [#207](https://github.com/Marvisak/VIctory/pull/207) | Media: a share is refused however it is spelled | `11ca3ce4928cf9ac2f77fcdc1c095d41c62fcbce` | false | queued-unreviewed |
| [#206](https://github.com/Marvisak/VIctory/pull/206) | GPU: make a structured buffer view off its host copy's elements a raw view | `05981dd53b338ef0dabf30d4b728ccb41b0772d9` | true | queued-unreviewed |
| [#202](https://github.com/Marvisak/VIctory/pull/202) | GPU: skip unmapped regions after a failed bulk read into host memory | `dcf666486a3cb1fd47ad60437ef110d94c9f810e` | false | queued-unreviewed |
| [#200](https://github.com/Marvisak/VIctory/pull/200) | Shaders: lower v_div_scale_f32, v_div_fmas_f32 and v_div_fixup_f32 | `770627325f2c8ac40851b67afcce8b558ec63036` | false | queued-unreviewed |
| [#197](https://github.com/Marvisak/VIctory/pull/197) | Input: apply VICTORY_KEYS to a connected host pad too | `64484b426ba8e5f9571c6398ab33ea573997d4d7` | false | queued-unreviewed |
| [#194](https://github.com/Marvisak/VIctory/pull/194) | Audio: an XMA loop's skip is of its first frame, not of its last | `985d8d4b025384b5f13a3177ef906c8b6cce6840` | false | queued-unreviewed |
| [#193](https://github.com/Marvisak/VIctory/pull/193) | Shaders, GPU: add runtime stride table for structured buffers | `f99b792e0794a904c1669e709177b32c6c85e268` | false | queued-unreviewed |
| [#192](https://github.com/Marvisak/VIctory/pull/192) | Shaders: replace single-wave Wave64 LDS lockstep with barriers and lower 32-lane swizzle/DPP at WaveSize(32) | `4a365e3a736a92af82947dbfb1c6617ca1a4311d` | false | queued-unreviewed |
| [#191](https://github.com/Marvisak/VIctory/pull/191) | Add a Linux launcher and smoke test for Proton | `e3757d306b69e01231b80cf4a11fd77c53819ec7` | false | queued-unreviewed |
| [#190](https://github.com/Marvisak/VIctory/pull/190) | GPU: build the indirect-argument clamp with our own writer (no shader compiler at run time) | `ffa13108fef5f2bf646ba5db8845cacc67f2a38a` | false | queued-unreviewed |
| [#189](https://github.com/Marvisak/VIctory/pull/189) | Shaders: emulate Wave64 compute shaders on Wave32 and narrower hosts | `d0fd9bf4b9a2b63d303e692c82c9fc8669642cc2` | false | queued-unreviewed |
| [#188](https://github.com/Marvisak/VIctory/pull/188) | Loader: find TLS-array load candidates sixteen offsets at a time (8-9x faster scan) | `2a6e6a4bcea51c3fc4df3c3b6823a15f501b4039` | false | queued-unreviewed |
| [#186](https://github.com/Marvisak/VIctory/pull/186) | Kernel: a title image's file name that fits clears the last error | `3c7d845b3bf4c729c9f507f310d6c89775e3117d` | false | queued-unreviewed |
| [#185](https://github.com/Marvisak/VIctory/pull/185) | GPU: maintain shadow-buffer coherency for directly indexed SRVs | `a6367ba0993998e5b85520ef54232ec98d9997e1` | true | queued-unreviewed |
| [#183](https://github.com/Marvisak/VIctory/pull/183) | XMA: noise tests, and end a frame at its data instead of looping forever | `d4d1822049fed053c6188ea0b016b0976a71bf41` | false | queued-unreviewed |
| [#182](https://github.com/Marvisak/VIctory/pull/182) | Files: `..` in a title path stops at the virtual drive's root | `18275322e4d3ea3a36b01b7e91f192e2ad742cdc` | false | queued-unreviewed |
| [#180](https://github.com/Marvisak/VIctory/pull/180) | Shaders: twin descriptor loads and quad gradients for compute samples | `aa692580ed69e4fa20b8312df55027108fe8d7c7` | false | queued-unreviewed |
| [#178](https://github.com/Marvisak/VIctory/pull/178) | GPU: compile the indirect-argument clamp with DXC when D3DCompile refuses it (Wine/Proton) | `eb635f7af85f70590edbde8ed1b58cba55603627` | false | queued-unreviewed |
| [#177](https://github.com/Marvisak/VIctory/pull/177) | Shaders: key heap handles by element type (atomic on a float-read UAV) | `b7300eb5b86cc37027d197d56b4eaf5f8b08d060` | false | queued-unreviewed |
| [#176](https://github.com/Marvisak/VIctory/pull/176) | Shaders: pair packed fp16 pixel inputs with unlisted signature rows | `f0e7745b7f3887d244e5d4854909c58c8db8e6bd` | false | queued-unreviewed |
| [#175](https://github.com/Marvisak/VIctory/pull/175) | Windows 10: load the D3D12 Agility SDK runtime for shader model 6.6 | `19aec59ba2931333e4218fe16ab7851b110122bd` | false | queued-unreviewed |
| [#174](https://github.com/Marvisak/VIctory/pull/174) | GPU: a frame wait takes its tick before it sleeps | `882664e13cfa557fb46d41e182c3267b01097485` | false | queued-unreviewed |
| [#172](https://github.com/Marvisak/VIctory/pull/172) | XMA: cache IMDCT rotation tables per subframe size | `4eb0898b3cbd6d0bbbffad540aca5d7e3531ac58` | false | queued-unreviewed |
| [#171](https://github.com/Marvisak/VIctory/pull/171) | Shaders: lower v_mad_u64_u32 and v_mad_i64_i32 | `f4a24e1e185da318c5074b4a337b7a6c9673449e` | false | queued-unreviewed |
| [#169](https://github.com/Marvisak/VIctory/pull/169) | Shaders: tbuffer_load_format_d16 loads pack two 16-bit channels per VGPR | `a103937b9412f7bbb609b614363a76e6179178af` | false | queued-unreviewed |
| [#168](https://github.com/Marvisak/VIctory/pull/168) | Shaders: SCC of s_min, s_max and s_lshl1-4_add_u32 as the ISA sets it | `ae8967c37e1dd834ea038d4519a346495d676182` | false | queued-unreviewed |
| [#166](https://github.com/Marvisak/VIctory/pull/166) | Shaders: lower s_bitset{0,1}_b64, s_brev_b64 and s_bitreplicate_b64_b32 | `bd034c0f84d6ad79320becd037a040aeb14dd228` | false | queued-unreviewed |
| [#165](https://github.com/Marvisak/VIctory/pull/165) | Shaders: lower s_bcnt0_i32_b32/b64 and s_ff0_i32_b32/b64 | `50d8a98c3c3fc4406b067f74273434207226b46e` | false | queued-unreviewed |
| [#164](https://github.com/Marvisak/VIctory/pull/164) | Shaders: integer memory atomics and sub-dword buffer accesses | `e469b7c8a996a297dc7def07e48a7c70f0f039f2` | false | queued-unreviewed |
| [#162](https://github.com/Marvisak/VIctory/pull/162) | Shaders: lower v_sad_u8, v_sad_hi_u8, v_sad_u16, v_sad_u32 and v_msad_u8 | `318082f99ccd4e2910895a3ab8c98add0751ba8c` | false | queued-unreviewed |
| [#160](https://github.com/Marvisak/VIctory/pull/160) | Shaders: texel offsets, comparison and gather forms of image samples | `6ad7e152e65c7cb0095eb08eaf7824dbd6b9e895` | false | queued-unreviewed |
| [#159](https://github.com/Marvisak/VIctory/pull/159) | GPU: occlusion, statistics and timestamp queries run on the host | `f42e7d5e68b23b610d68253603caad81d0e166d7` | false | queued-unreviewed |
| [#158](https://github.com/Marvisak/VIctory/pull/158) | GPU: the depth bounds test reaches the host | `520d629ed1787321f88b1e3ba4d9e8aa3187791d` | false | queued-unreviewed |
| [#157](https://github.com/Marvisak/VIctory/pull/157) | Shaders: lower 24-bit mul_hi, saturating 16-bit packs, integer dot pr… | `13d984adfd77f09b884af6a508802fbf0c9720ef` | false | queued-unreviewed |
| [#156](https://github.com/Marvisak/VIctory/pull/156) | Shaders: lower v_mul_hi_u32_u24 and v_mul_hi_i32_i24 | `8142a9458ded53121c05858be4f1408a91aab3d9` | false | queued-unreviewed |
| [#154](https://github.com/Marvisak/VIctory/pull/154) | Shaders: use DXIL IsNaN for unordered/ordered and self float compares | `d5fca303b4171bd7078a617e2485deff18088f7b` | false | queued-unreviewed |
| [#153](https://github.com/Marvisak/VIctory/pull/153) | GPU: seek directly to private data on final release | `3e98bd7e0ce79762162ac958e3ab67cc85de00c9` | false | queued-unreviewed |
| [#151](https://github.com/Marvisak/VIctory/pull/151) | Link victory.exe with a 16 MiB default thread stack | `d77408bb71e1822105ae8e88bbe942d8e5c39d1a` | false | queued-unreviewed |
| [#150](https://github.com/Marvisak/VIctory/pull/150) | Input: read DualSense pads over HID when XInput has none | `fe014c1b50ed72e16dca5f101fe1a2219fea6518` | false | queued-unreviewed |
| [#148](https://github.com/Marvisak/VIctory/pull/148) | Shaders: lower s_bcnt0_i32_b32/_b64, s_ff0_i32_b32/_b64 and s_flbit_i32_b64/_i64 | `a0380049aacd0a0f04ac854faf41c9ded8c83a51` | false | queued-unreviewed |
| [#145](https://github.com/Marvisak/VIctory/pull/145) | Shaders: read both ballot halves for wave32 host-wave mask ops | `ecc4eb668db7e61b42e90a7bf06517fe34cc062a` | false | queued-unreviewed |
| [#143](https://github.com/Marvisak/VIctory/pull/143) | GPU: hash and detile unwatched textures from a kernel-read copy | `525a713e8fcf4971348e14f25454d3236a94c2f9` | false | queued-unreviewed |
| [#142](https://github.com/Marvisak/VIctory/pull/142) | shader: lower s_bitcmp{0,1}_b{32,64} and s_sext_i32_i{8,16} | `badfee5c480bb68d6c6e889d2b339d24eb252ea1` | false | queued-unreviewed |
| [#139](https://github.com/Marvisak/VIctory/pull/139) | Shaders: reuse SSA sums when tracing memory addresses | `697c4bcd1b5ef084ada3fa344db92768ef0a6ba2` | false | queued-unreviewed |
| [#138](https://github.com/Marvisak/VIctory/pull/138) | Shaders: gradient samples on cube images | `800227d59f9e97f5037d647a0435369db670838d` | false | queued-unreviewed |
| [#137](https://github.com/Marvisak/VIctory/pull/137) | fix(gpu): preserve pixels outside texture readback regions | `219e3cefc1b234fe6f8c93252a6ceae735a6897f` | true | queued-unreviewed |
| [#133](https://github.com/Marvisak/VIctory/pull/133) | Files: a title run by its file name alone gets its own folder as G: | `01ae55a3ba04c39a0e8851cc00da16af7a49adbd` | false | queued-unreviewed |
| [#132](https://github.com/Marvisak/VIctory/pull/132) | XTaskQueue: terminating a queue cancels its delayed callbacks too | `b9ff219826e61ba6306f6a1378b613319f70da47` | false | queued-unreviewed |
| [#130](https://github.com/Marvisak/VIctory/pull/130) | Profile: opt-in VICTORY_PROFILE (frame times, memory, caches, loads; frames.csv, a Perfetto trace and an on-screen overlay) | `0ba0851dea0f9452c104c4f71888232dd1958fd1` | false | queued-unreviewed |
| [#126](https://github.com/Marvisak/VIctory/pull/126) | Shaders: walk a lane-mask diagram once in vars and given | `e5b715983242c8c31296d3195dce74a22792d72d` | false | queued-unreviewed |
| [#124](https://github.com/Marvisak/VIctory/pull/124) | GPU: reuse unchanged page checks across shadow copy-back ranges | `a6deb46db2fcb4b09c3a158170da88a49e22a740` | false | queued-unreviewed |
| [#123](https://github.com/Marvisak/VIctory/pull/123) | GPU: refuse raw buffer views that do not start on 16 bytes | `bc747281dcd62bccfd36f4692755a12378b32e0d` | false | queued-unreviewed |
| [#122](https://github.com/Marvisak/VIctory/pull/122) | XAsync: end the call that finished, not whatever call the block holds now | `d01b17070dbf5f181e327ee540510cec73facbe9` | false | queued-unreviewed |
| [#120](https://github.com/Marvisak/VIctory/pull/120) | Shaders: lower s_absdiff_i32, s_sext_i32_i8/i16 and s_pack_ll/lh/hh_b32_b16 | `cb40cdcc21a77510b5b747c578059c1e11fff4ba` | false | queued-unreviewed |
| [#119](https://github.com/Marvisak/VIctory/pull/119) | GPU: query the allocation type when guarding mapped memory | `095e44a0bc15f7879a01ee92a30ae3a5fc94b7fb` | false | queued-unreviewed |
| [#116](https://github.com/Marvisak/VIctory/pull/116) | Shaders: byte and halfword LDS reads and writes | `8504777b2b6db044217247d0acea14918ca2da21` | false | queued-unreviewed |
| [#115](https://github.com/Marvisak/VIctory/pull/115) | Census: report directory traversal failures | `b1bdd8994a456755255b1e096f641ef47ff719c9` | false | queued-unreviewed |
| [#114](https://github.com/Marvisak/VIctory/pull/114) | XAsync: run a failed call's cleanup without the completion lock | `0b729de6ee7bdca1bca6fc0f3ea00b368991a3f9` | false | queued-unreviewed |
| [#113](https://github.com/Marvisak/VIctory/pull/113) | GitHub: rebase issue forms and PR template on main | `620afd5ec423cb8de4cded683befee79beb05011` | false | queued-unreviewed |
| [#112](https://github.com/Marvisak/VIctory/pull/112) | Build: Linux workspace, contributor setup and CI | `ee63908d50295af044b7950ae5693e49fe88e357` | false | queued-unreviewed |
| [#110](https://github.com/Marvisak/VIctory/pull/110) | Shaders: use DXIL IsNaN in med3 so Intel Xe does not fold fcmp uno | `d53fa58b0b8ecb999e1bcd5e7e382271b30a3362` | false | queued-unreviewed |
| [#109](https://github.com/Marvisak/VIctory/pull/109) | Shaders: rebuild a cube image descriptor for code that reads its dwords | `c2b4eef99423ee6d6af6aae4fe3baa045e5fa2a4` | false | queued-unreviewed |
| [#105](https://github.com/Marvisak/VIctory/pull/105) | Shaders: a loop that branches back to the entry instruction translates | `6e6d3fc027d097afcad8db1807f76a203cca7a50` | false | queued-unreviewed |
| [#101](https://github.com/Marvisak/VIctory/pull/101) | Shaders: lower sub-dword LDS reads and writes | `4baaee7a5eef38a0e57c4a14e70e183842791579` | false | queued-unreviewed |
| [#100](https://github.com/Marvisak/VIctory/pull/100) | GPU: cap a shader-visible sampler heap at 2,048 instead of losing it | `f7d08ced116c2243c2d1686cb593e86ad63a3406` | false | queued-unreviewed |
| [#98](https://github.com/Marvisak/VIctory/pull/98) | Shaders: model v_permlane16_b32 in whole-wave sections | `f468457a18590bea0e9c21d630c5597fa4649517` | false | queued-unreviewed |
| [#96](https://github.com/Marvisak/VIctory/pull/96) | xma: start the frame after a continuation at the next packet that has frames | `ba42db643c15c1b7fb17ba40464dddda36c1db60` | false | queued-unreviewed |
| [#94](https://github.com/Marvisak/VIctory/pull/94) | GPU: prefer high-performance D3D12 adapter and support VICTORY_ADAPTER | `513eb91273e7ced089520278545ffe58b192e1ee` | false | queued-unreviewed |
| [#92](https://github.com/Marvisak/VIctory/pull/92) | Shaders: lower v_add_nc_i32, v_sub_nc_i32, v_mul_lo_i32 and the signed 16-bit add and sub | `4f2d5dbd8f1cf04414dd2d6c07671d34478bae0c` | false | queued-unreviewed |
| [#91](https://github.com/Marvisak/VIctory/pull/91) | Shaders: lower s_cmov_b32, s_cmov_b64 and s_cmovk_i32 | `ed5f98ff67e3c09cde81b36c28bff216cc16a079` | false | queued-unreviewed |
| [#89](https://github.com/Marvisak/VIctory/pull/89) | gpu: packed 16-bit render targets are R8G8B8A8 where the GPU cannot render to them | `081afbd5382bb6aabaec5b28864c11cf00a15725` | false | queued-unreviewed |
| [#86](https://github.com/Marvisak/VIctory/pull/86) | GPU: cap a sampler heap past the host's limit, with dynamic slots for tables past it | `a80eb17c4cead4e251d7ca96d1326d901dd92fe4` | false | queued-unreviewed |
| [#83](https://github.com/Marvisak/VIctory/pull/83) | GPU: bound the null sampler staging heap | `d8f063b04566e39dfa32617387e8ab60b4cf689d` | false | queued-unreviewed |
| [#82](https://github.com/Marvisak/VIctory/pull/82) | HLE: speak the title's narration with the host's speech synthesizer | `df5e928aa0c5572dfc5289065a5b89795be27b94` | false | queued-unreviewed |
| [#78](https://github.com/Marvisak/VIctory/pull/78) | Shaders: inline float constants as f16 in v_fma_mix and v_pack_b32_f16 | `e6faad2c9fc52f82a72308f4fef4fd4838d4e332` | false | queued-unreviewed |
| [#76](https://github.com/Marvisak/VIctory/pull/76) | xmem: map a run of physical pages over several placeholders | `dea90ccc7e874bb4f08b4b56615d1a0822e15210` | false | queued-unreviewed |
| [#73](https://github.com/Marvisak/VIctory/pull/73) | Shaders: support image_sample_b with a16 and _cl LOD-clamp variants | `3eab56ad5f6ed7f530643db139001623af7e6f2d` | false | queued-unreviewed |
| [#72](https://github.com/Marvisak/VIctory/pull/72) | GPU: retry pipeline cache saves after write failures | `521ceccd781e95d36a74ad196920104bf424ed00` | false | queued-unreviewed |
| [#69](https://github.com/Marvisak/VIctory/pull/69) | Translate compute selects with numeric lane masks | `18f3937ed13546b35b25df66c79193ec1ffa8f0d` | false | queued-unreviewed |
| [#67](https://github.com/Marvisak/VIctory/pull/67) | GPU: implement CopyTextureRegion texture-to-buffer readbacks | `b5c38fa9c25cfdbfbc656d2f1e8371a450743315` | false | queued-unreviewed |
| [#55](https://github.com/Marvisak/VIctory/pull/55) | GPU: create host textures without the uses the host refuses | `56f1d413873df6f9313647e86e7e21bf692d9683` | false | queued-unreviewed |
| [#52](https://github.com/Marvisak/VIctory/pull/52) | GPU: hold SHADER_HEAPS across descriptor mirror and check query offsets | `628b95da5e738046d00995743e07137e48936ba6` | false | queued-unreviewed |
| [#32](https://github.com/Marvisak/VIctory/pull/32) | GPU: fence waits, direct memory-write wakes, queue timeline and host list pool | `9c5328d4ef91c56bb10bea6474f6f802b9d4d652` | false | queued-unreviewed |
| [#24](https://github.com/Marvisak/VIctory/pull/24) | GPU: run indirect mesh dispatches (DispatchMeshX) through indirect execution | `d5f41f54c49b20f31f8ffaa74a6963fa109c99f8` | false | queued-unreviewed |
| [#17](https://github.com/Marvisak/VIctory/pull/17) | CI: check the formatting with cargo fmt on all PRs | `b07dfe3d4e7f2531db3228a26f699c6693f5e331` | false | queued-unreviewed |
| [#12](https://github.com/Marvisak/VIctory/pull/12) | Check the required package archive before title startup | `c63262f1e977bdfb3d14c58f3f3bad0d245bbad0` | false | queued-unreviewed |
| [#9](https://github.com/Marvisak/VIctory/pull/9) | GitHub: issue forms, PR template and a PR description check | `75272ae60d3e5d8550c4ac4feee0a1448f8963c3` | false | queued-unreviewed |
