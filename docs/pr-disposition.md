# Disposition of every open upstream PR

Snapshot: **1057 open PRs**, retrieved 2026-11-24 21:33 UTC by paginated REST.
Every head is fetched into `refs/remotes/pr/<n>`, so any row can be inspected
locally. Generated from git and that snapshot; regenerate, do not hand-edit.

The integration ledger answers "what is in the branch". This answers "what
happened to each open PR". Per-worker accounting is in
[worker-accounting.md](worker-accounting.md).

## Summary

| Disposition | Count |
| --- | --- |
| integrated | 205 |
| rejected | 19 |
| deferred | 8 |
| held | 5 |
| duplicate | 4 |
| obsolete | 1 |
| already-present | 1 |
| not yet reviewed | 814 |
| total open | 1057 |

A further 18 PRs are in the branch and no longer open upstream;
they are in [pr-integration-ledger.md](pr-integration-ledger.md).

## Read this before trusting a row

**814 of 1057 have no reviewed disposition.** They are
listed rather than omitted so that silence is never read as approval.

`integrated` means the change is in **this branch**, reviewed and tested when
it landed. It does not mean the PR is merged upstream. `rejected`, `deferred`
and `held` mean it was read and refused for the stated reason.

## Every open PR

Newest first. `—` in the commit column means nothing is in the tree.

| PR | Head | State | Local commit | Reason | Title |
| --- | --- | --- | --- | --- | --- |
| #1773 | `496ae1c5e0ce` | not yet reviewed | — |  | raise the fault of an emulated clzero at the title's instruction |
| #1772 | `69662be1d56b` | not yet reviewed | — |  | HLE: handle verbatim/drive-root mounts and UNC/verbatim GetDiskFreeSpaceW roo… |
| #1770 | `1beaca67e124` | not yet reviewed | — |  | Descriptor tests: take the capped heap's design into account |
| #1769 | `7e9907c76f59` | not yet reviewed | — |  | Shaders: LDS accesses outside the program's allocation read 0 and drop their … |
| #1768 | `968950d98233` | not yet reviewed | — |  | GPU: retry a root signature without the heap-directly-indexed flags when the … |
| #1767 | `882856f0f8ff` | not yet reviewed | — |  | translate: report the validator's own reason, not the line that hides it |
| #1766 | `368643182465` | not yet reviewed | — |  | Shaders: cube-map face coordinates read denormals as zeros |
| #1765 | `d02e007b7973` | not yet reviewed | — |  | GPU tests: the castable pair is two host textures where the host refuses cast… |
| #1763 | `ca168ded3716` | not yet reviewed | — |  | GPU: refuse typed UAVs in unsupported formats regardless of family or castabi… |
| #1762 | `550cac2b9f37` | not yet reviewed | — |  | GPU: structured buffer views are raw views of the same bytes on the host (NVI… |
| #1761 | `d06bfe76c72d` | not yet reviewed | — |  | Input: trigger rumble through Windows.Gaming.Input with one pad conne… |
| #1759 | `dc80215d5c72` | not yet reviewed | — |  | Shaders: lower numerical Wave32 mesh masks at uniform entry |
| #1757 | `29d5f6e0dc7c` | not yet reviewed | — |  | Shaders: one arena per program for instruction operands and effects |
| #1756 | `ab3c38d3793b` | not yet reviewed | — |  | Shaders: image atomics on a handle of their own element type |
| #1752 | `1bf4933db92f` | not yet reviewed | — |  | Shader tests: a software adapter (a CI runner's) skips what only GPU hardware… |
| #1751 | `45215912b474` | not yet reviewed | — |  | GPU: capped view heaps keep the full dynamic range (NVIDIA) |
| #1750 | `b58b779fdedc` | not yet reviewed | — |  | GPU: a wait for a submission ends when the device is lost or the GPU stalls |
| #1749 | `27514c6a864c` | not yet reviewed | — |  | GPU: indirect mesh dispatches run as an indirect execution of one record |
| #1747 | `fbf1dc32bb0c` | not yet reviewed | — |  | Shaders: lower v_div_scale_f32 and v_div_fmas_f32 |
| #1743 | `332152b58105` | not yet reviewed | — |  | GPU tests: replay a world frame's copies of title memory (benchmark for the c… |
| #1741 | `d2c9a469e7ae` | not yet reviewed | — |  | Docs: AGENTS.md for AI agents working on VIctory |
| #1740 | `075e963a73c9` | not yet reviewed | — |  | GPU: occlusion queries and timestamps run on the host (FIND-8) |
| #1739 | `9c7ca2c630e7` | not yet reviewed | — |  | GPU: clear final pipeline stream padding after moves (on #1690) |
| #1738 | `d39a027f2eb9` | not yet reviewed | — |  | Shaders: preserve live interpolation first halves (on #453) |
| #1737 | `108b38be2ee7` | not yet reviewed | — |  | Shaders: honor permlane EXEC and FI/BC flags (on #1014) |
| #1735 | `8ec5de1ac765` | not yet reviewed | — |  | GPU: time commands by pipeline in VICTORY_GPU_TIMING |
| #1734 | `90ebae3dbc33` | integrated | — |  | victory-shader: make the test dispatches cheap - an event wait and reused com… |
| #1733 | `54f693e4c556` | duplicate | — | landed locally as 694c4944, verified at lower.rs:5387 | Shaders: a helper lane's v_readfirstlane reads its own value (#908) |
| #1732 | `3dc6c12410c9` | duplicate | — | landed locally as 67b24154, verified against the tree function | GPU: f32_to_half keeps NaN and rounds normals to nearest even (#843) |
| #1731 | `1f84c2330954` | integrated | — |  | GPU: occlusion queries measure on the host instead of reporting every draw vi… |
| #1730 | `87a93820d96d` | not yet reviewed | — |  | translate: --cache DIR fills the runtime's shader cache |
| #1729 | `23a5a6a6feca` | rejected | — | rejects v0 and returns an older readable version where HEAD refuses an unreadable newest one: a silent rollback, and it breaks two integrated tests | XGameSave: a stray or unreadable version folder never costs the save (#870) |
| #1728 | `463e835868ba` | integrated | — |  | Pipelines: VICTORY_PIPELINE_WORKERS sets the worker count |
| #1727 | `6cd8375a23d5` | integrated | — |  | GPU: 1D textures are 2D on the host, one texel high (GBV id=940) |
| #1726 | `d09f3aa86c4e` | integrated | `64fa2831` | in the branch | DirectStorage: read a page list naming page 0 again before failing its request |
| #1725 | `a6365c376b37` | integrated | — |  | GPU: placed copies take the bytes a GPU write leaves in their memory |
| #1724 | `674f3b9118d1` | not yet reviewed | — |  | Make the VIctory game window borderless by default |
| #1723 | `5542340831d3` | not yet reviewed | — |  | GPU diagnostics: pipeline statistics per noted command with VICTORY_GPU_TIMING |
| #1722 | `ad06ac715e28` | not yet reviewed | — |  | GPU: the host pipeline library works under Wine (null blob for an empty libra… |
| #1721 | `619a8e8b158c` | not yet reviewed | — |  | Launcher: a GUI that checks the game directory and runs victory.exe |
| #1720 | `2f61f7c0fc27` | integrated | `d101c0b0` | in the branch | report apu heap allocations that fail |
| #1719 | `771467458b5d` | integrated | `b7e3689c` | in the branch | DirectStorage: clear the factory out pointer on failure |
| #1718 | `c0228d939ec1` | integrated | `09f17be6` | in the branch | victory-shader: mask mesh primitive vertex indices to 9 bits (drop edge flags) |
| #1717 | `35fb57d417bd` | integrated | `105fbc6d` | in the branch | HLE: the retail console ID has the documented dotted form |
| #1716 | `758bf7fbc6a3` | not yet reviewed | — |  | victory-shader: fold constants and eliminate dead pure ops in bitcode |
| #1715 | `645bd5c0e200` | not yet reviewed | — |  | FidelityFX: DLSS frame generation on frames the title upscales (stacked on #1… |
| #1712 | `57c25604fa37` | integrated | `e2f67a49` | in the branch | victory-shader: support line-topology mesh shaders |
| #1710 | `747093145fa4` | integrated | `f9fe0e37` | in the branch | GPU: keep dropped draws in the full frame reporting interval |
| #1709 | `3821b53040ff` | not yet reviewed | — |  | Shaders: flush subnormals in f32 min/max/median and cube |
| #1707 | `423d66d3dfcc` | not yet reviewed | — |  | Docs: add AGENTS.md for agent contributors |
| #1704 | `7e6e5788202d` | not yet reviewed | — |  | FidelityFX: answer the upscale calls with DLSS through Streamline (stacked on… |
| #1703 | `120df922ba73` | integrated | `336bee70` | in the branch | README: the GPU timing switch needs DRED for the per-command notes |
| #1702 | `10b96dea7bb5` | integrated | `a9040302` | in the branch | HLE: add VICTORY_NO_HOST_PAD to hide host gamepads |
| #1701 | `2e33598b9292` | integrated | `5c61aa48` | in the branch | GPU: VICTORY_MODE overrides the reported console display mode |
| #1700 | `0e1024d695f7` | integrated | `aec7eb38` | in the branch | GPU: say when a draw's render target has no host copy or lost its permission |
| #1699 | `6fd640a2ae53` | rejected | — | reads src2 with half_src where src0/src1 use half_int_src; probe measured 15366 where hardware gives 0 | shader: lower v_add_nc_i16, v_sub_nc_i16, v_mad_i16, and v_mad_u16 with clamp |
| #1696 | `e29391f56c49` | integrated | `f1b9a490` | in the branch | Shaders: honor op_sel, omod and clamp on v_interp_p2_f16 |
| #1695 | `6493dd93100c` | integrated | `c21cee43` | in the branch | Shader tests: require a device readback in every waterfall test |
| #1694 | `da18cb8bab09` | held | — | would revert the single-rounding v_dot2_f32_f16 fix | shader: lower 6 RDNA2 integer dot-product instructions without INT64 |
| #1693 | `77d21c68af28` | integrated | `c4a9469c` | in the branch | victory-shader: lower TFE image loads/samples/gathers via CheckAccessFullyMap… |
| #1692 | `2b3da5eb353a` | not yet reviewed | — |  | [BUNDLE] GPU diagnostics: 16 PRs, #279 #491 #500 #537 #602 #669 #1064 #1081 #… |
| #1691 | `667e7dc9a4fd` | integrated | `76adf6a1` | in the branch | GPU: track D3D12 shadow-buffer states (#1670) |
| #1690 | `6c22344b1e4f` | integrated | `67be115d` | in the branch | GPU: zero pipeline library description padding |
| #1688 | `c7242f1c66f2` | integrated | `cd2b6c39` | in the branch | Shaders: lower v_mad_u32_u16, v_mad_i32_i16, and v_lerp_u8 |
| #1687 | `df9c42347055` | integrated | `fe245cf6` | in the branch | Shaders: lower image_gather4_c_lz(_o) and offset sample variants |
| #1686 | `14ca12162bed` | integrated | `621c3c17` | in the branch | Shaders: lower pixel shader SV_Coverage input via dx.op.coverage |
| #1685 | `d7536bdd9ca5` | integrated | `138e43c5` | in the branch | Shaders: lower v_cmp_{f,t}_{i64,u64} constant 64-bit compares |
| #1684 | `c0b3633e5762` | not yet reviewed | — |  | shader: lower 10 scalar bitops (bcnt0/ff0/flbit64/brev64/bitset64/bitreplicat… |
| #1682 | `3b0c7dad7330` | not yet reviewed | — |  | Shaders: interpolations with any i/j by hand; barycentrics read as data |
| #1681 | `9c447fbdd049` | not yet reviewed | — |  | GPU: verify exact dirty-page masks on D3D12 |
| #1680 | `2a686ebfb222` | integrated | `9e91c1d2` | in the branch | GPU: pipeline library loads take a lock for their name, not one for all |
| #1677 | `9145d9d3dabb` | integrated | `e832c2fb` | in the branch | HLE: show title windows through a stand-in that always reads its messages |
| #1676 | `00b4b1b3b4a6` | integrated | `29289af8` | in the branch | Shaders, CLI: name a host without mesh shaders in the pipeline census |
| #1675 | `5a3f25a48bcd` | duplicate | — | lowering already at lower.rs:3567 | shader: lower s_absdiff_i32 and s_pack_{ll,lh,hh}_b32_b16 |
| #1674 | `b9d3e8364e61` | not yet reviewed | — |  | Shaders: guard speculative and EXEC-predicated heap descriptor loads (#1595) |
| #1673 | `dda166c8f17e` | integrated | `7e80b534` | in the branch | HLE: XTaskQueueTerminate cancels only its own tasks on shared ports |
| #1669 | `70d3a821155f` | integrated | `ee61aa1c` | in the branch | shader: handle dword-straddling 16-bit typed buffer loads at byte offset 3 |
| #1668 | `c4d626b250d1` | integrated | `bc9c0f60` | in the branch | Shaders: read inline floats as f32 bits in 16-bit integer ops |
| #1667 | `eb7cc2d43711` | integrated | `13ab09bb` | in the branch | flush a kept save blob that had to be copied |
| #1666 | `a67255631622` | not yet reviewed | — |  | GPU: small first scratch chunk per list; log host memory against budget |
| #1665 | `6d760bc05a3a` | not yet reviewed | — |  | Docs: add Windows 10 and GPU compatibility guide |
| #1664 | `98b0ec8c60ff` | integrated | `4cf31e8e` | in the branch | hle: return S_OK with null handle when PowerSetRequest returns ERROR_NOT_SUPP… |
| #1663 | `20c6c269bf24` | held | — | drops wave32 forcing; out-of-bounds ADDTID returns 0 instead of refusing | Shaders: lower RDNA 2 ADDTID memory instructions to DXIL |
| #1662 | `e8bbc120b6d8` | held | — | same 248-as-f64 defect as #1654 | Shaders: lower RDNA 2 quad-byte SAD instructions to DXIL |
| #1661 | `07fc14b626b9` | not yet reviewed | — |  | GPU: route non-overlapping CopyMemoryToMemoryX through Step::Copy and scope i… |
| #1660 | `be824d9d2183` | integrated | `d32efcd9` | in the branch | GPU: serialize pipeline library atomic writes with unique temp files |
| #1659 | `79107fd52f9e` | not yet reviewed | — |  | shader: lower v_frexp_{exp,mant}_{f32,f16} in pure 32-bit DXIL |
| #1658 | `4fc9d70fee4c` | not yet reviewed | — |  | GPU: keep Staging::detile inside the staging buffer and harden committed() |
| #1657 | `932dcbf89f96` | integrated | `15316a22` | in the branch | XSystem: name the analytics family Windows.Xbox |
| #1655 | `4f1d0acf4d5c` | not yet reviewed | — |  | xtask: the Forge and claims (one branch of every pull request that builds; wh… |
| #1654 | `3a08221a6246` | held | — | inline constant 248 expanded from f64 among f32 siblings | shader: lower v_mad_u64_u32 and v_mad_i64_i32 without INT64 |
| #1653 | `1e79f148f66d` | not yet reviewed | — |  | victory-gpu: split the pipeline library into shards so a save writes only wha… |
| #1652 | `d43c8558abb3` | not yet reviewed | — |  | Add native GTA VI launcher and D3D12 rendering updates |
| #1651 | `955905163ecb` | not yet reviewed | — |  | GPU: save only changed host pipeline-library shards |
| #1650 | `6b12a361be06` | not yet reviewed | — |  | translate: identify unsupported ACCC/FSPX shader containers |
| #1649 | `36682459e8d1` | not yet reviewed | — |  | shader: lower s_cmov_b32, s_cmov_b64, and s_cmovk_i32 |
| #1648 | `c6ec4ad3bf29` | not yet reviewed | — |  | GPU: mesh shaders emulated as compute + unpack vertex on SM 6.6 cards without… |
| #1647 | `9cd8d23ea821` | not yet reviewed | — |  | shader: lower v_mul_hi_i32_i24 and v_mul_hi_u32_u24 without INT64 |
| #1646 | `64a703844408` | not yet reviewed | — |  | Shaders: fold constant bin/cmp/select/cast ops and identity ops at emit time |
| #1644 | `8bf13c49dfd2` | not yet reviewed | — |  | [BUNDLE] Unknown-stride buffers: #704 on #193, stacked on #1511 |
| #1643 | `c79f4c891254` | not yet reviewed | — |  | GPU: register pipeline demand after asynchronous compilation timeout |
| #1642 | `f876a822b059` | not yet reviewed | — |  | Shaders: preserve signaling NaNs in typed buffer loads |
| #1641 | `3271c00e8c9b` | not yet reviewed | — |  | GameInput: answer IUnknown and own-IID queries on the COM objects |
| #1640 | `b916a9c0d419` | not yet reviewed | — |  | GPU: constant buffer views of at most 64 KiB |
| #1639 | `0b7a13c68dfb` | not yet reviewed | — |  | PE: map header RVAs and read TLS data from the headers |
| #1638 | `ab4a6e645b1e` | not yet reviewed | — |  | [BUNDLE] NVIDIA stack: Impre-visible's tree (wave64 #189 + the world-run list… |
| #1636 | `40f30be0b52f` | not yet reviewed | — |  | Shaders: GFX10 FLAT offsets ignore bit 11 |
| #1635 | `6dcf40a4c8b4` | not yet reviewed | — |  | victory-gpu: drop an indirect call before its clamp, not after (1 400-1 750 a… |
| #1632 | `8de76a242c98` | not yet reviewed | — |  | [BUNDLE] World-run stack from Impre-visible's report: 12 PRs (#1077 #1175 #52… |
| #1631 | `e98c81a870db` | not yet reviewed | — |  | Tests: support VICTORY_HLE_PROBE_DLL and GNU delayload in hle-probe |
| #1630 | `d91d8084750f` | not yet reviewed | — |  | GPU: adapter choice and logging, presenter backoff, a texture-description lad… |
| #1629 | `111b77360b82` | not yet reviewed | — |  | GPU: zero 256-byte CBV scratch padding and bind null CBVs for empty/invalid v… |
| #1628 | `f016efe502f5` | not yet reviewed | — |  | GPU: log initial states and clear format when texture creation fails |
| #1626 | `f0967745efca` | not yet reviewed | — |  | GPU: throttle failed swap-chain creation and resize |
| #1625 | `1460a851c4b6` | not yet reviewed | — |  | CLI: compare runtime shader failures with offline coverage |
| #1623 | `e9254f23d3fe` | not yet reviewed | — |  | gfx10: keep an instruction's words inline instead of in a heap vector (decode… |
| #1621 | `53cf4ceed964` | not yet reviewed | — |  | bound the bytes one part's native blobs unpack to and hand out |
| #1619 | `1f47ee79d0a3` | not yet reviewed | — |  | Shader tests: pin IR reachability and PHI invariants |
| #1618 | `e62dd007815a` | not yet reviewed | — |  | victory menu: speak Portuguese where Windows does (stacked on #1609) |
| #1617 | `0d79788520a7` | not yet reviewed | — |  | [BUNDLE] #1296 (texture mip drop) resolved onto the GPU bundles by antikohza,… |
| #1614 | `50cbc893b057` | not yet reviewed | — |  | Shader tests: a cross-device VALU fuzzer (hardware adapters vs WARP) |
| #1613 | `e3d76bdaaf93` | not yet reviewed | — |  | GPU: no render-target view, barrier or clear for a texture without the render… |
| #1611 | `e9cb14ad3ea2` | not yet reviewed | — |  | run: stop with a clear message when the title's own DLLs are missing |
| #1610 | `47a43871e562` | not yet reviewed | — |  | [BUNDLE] Eight more fixes carried from contributors' forks: #745 #776 #787 #8… |
| #1609 | `1b9aad255916` | not yet reviewed | — |  | victory doctor: name the known problems a run shows, in plain words (stacked … |
| #1608 | `ced51de047fe` | not yet reviewed | — |  | Shaders: single-round v_fma_mixlo_f16 and v_fma_mixhi_f16 to binary16 |
| #1607 | `18c7561938da` | not yet reviewed | — |  | Formats: reject invalid GFX10 swizzle modes for 1D textures |
| #1606 | `cc94388e2e2b` | not yet reviewed | — |  | Shaders: lower remaining 32-bit integer and bitwise LDS atomics |
| #1604 | `e7134303bbb5` | not yet reviewed | — |  | GPU diagnostics: measure pipeline worker backlog during shared shader waits |
| #1603 | `9ec15138f66e` | not yet reviewed | — |  | [BUNDLE] Four fixes carried from Peppacodes' fork: #762 #764 #765 #832 |
| #1602 | `94a2e994f9d2` | not yet reviewed | — |  | Shader tests: f32/int conversions saturate, take NaN as 0, round to even |
| #1601 | `8a5515516d8c` | not yet reviewed | — |  | Shader tests: special values of v_rcp/v_rsq/v_sqrt/v_log/v_exp_f32 |
| #1599 | `1f6b392ca8bf` | not yet reviewed | — |  | Shaders: 64-bit integer compares on their dwords (AMD orders a shared-dword i… |
| #1598 | `07b5ecd09bd6` | not yet reviewed | — |  | Shaders: lower v_mad_f16 and v_mac_f16 with unfused binary16 product rounding |
| #1597 | `f4e166a84199` | not yet reviewed | — |  | Shaders: hidden descriptor words joined with data are an undefined path (+494… |
| #1594 | `3f6910ac3201` | not yet reviewed | — |  | CLI: preserve the title entry point exit status |
| #1593 | `5996b8740156` | not yet reviewed | — |  | [BUNDLE] Shader translation speed: 6 PRs, #139 #288 #369 #947 #1033 #1180, st… |
| #1592 | `6d174aeeeb93` | integrated | — |  | CLI: list shaders that complete emission |
| #1590 | `44e0c31d149b` | not yet reviewed | — |  | XMA tests: count frames only in the selected stream |
| #1589 | `60fa5d7ee343` | not yet reviewed | — |  | [BUNDLE] Wave64 compute on 32-lane hosts: #189 merged onto the shader stack, … |
| #1588 | `c36a25b8b780` | not yet reviewed | — |  | Shaders: single-round v_fma_mix*_f16 and unfused v_mad_f16/v_mac_f16 |
| #1587 | `4c8eb158d226` | not yet reviewed | — |  | Shaders: ballot divergent-loop masks at reconverged reads (#1498) |
| #1585 | `e25b88b16a94` | not yet reviewed | — |  | GPU diagnostics: report slots only inside recorded heap ranges |
| #1584 | `56387d3f69d7` | not yet reviewed | — |  | GPU: add opt-in idle shader cache trimming |
| #1583 | `c1c89516fb06` | not yet reviewed | — |  | GPU: texture read-back into a title buffer (CopyTextureRegion to a placed foo… |
| #1582 | `ab416b2d727e` | not yet reviewed | — |  | Shaders: enforce inactive-lane EXEC rules for DPP16, DPP8 and ds_swizzle_b32 |
| #1578 | `8631598388c2` | not yet reviewed | — |  | GPU: the "shader does not translate" warning names where its value was made |
| #1577 | `30ccd2307b7d` | not yet reviewed | — |  | Shaders: expand 64-bit inline constants as signed ints and binary64 floats |
| #1576 | `f32980a80aac` | not yet reviewed | — |  | HLE: implement IUnknown queries for COM objects |
| #1575 | `856f0ce0b1ec` | not yet reviewed | — |  | Shaders: sweep typed buffer formats and f16 ALU against exact references |
| #1574 | `ff5f46996635` | not yet reviewed | — |  | GPU descriptor: write null CBV when create_cbv description is null |
| #1572 | `0c1666198205` | not yet reviewed | — |  | Shaders: MUBUF sub-dword loads (buffer_load_ubyte, sbyte, ushort, sshort and … |
| #1571 | `9bfd43936615` | not yet reviewed | — |  | GPU: add diagnostics for shader dependency wait times |
| #1570 | `2f8898332c27` | not yet reviewed | — |  | Shaders: govern pixel discard by last exp VM (Valid Mask) and guard pixel MRT… |
| #1569 | `c55866929e05` | not yet reviewed | — |  | Shaders: lower v_swaprel_b32 |
| #1568 | `dde4577abe4e` | not yet reviewed | — |  | Shaders: model 64-bit carry sums and f16 reductions in whole-wave sections |
| #1566 | `f76393984981` | not yet reviewed | — |  | GPU: SetStablePowerState is accepted and ignored |
| #1565 | `883941b75287` | not yet reviewed | — |  | GPU: Evict succeeds, as the console's memory is never evicted |
| #1564 | `c6eb3fddffb8` | not yet reviewed | — |  | GPU: MakeResident succeeds, as the console's memory is always resident |
| #1563 | `93f718d7a94b` | not yet reviewed | — |  | Shaders: link mesh and pixel programs whose mesh outputs pass 32 KiB |
| #1562 | `bbdc3ca72499` | not yet reviewed | — |  | GPU: fences answer for ID3D12Fence1 as documented |
| #1560 | `db470df5325d` | not yet reviewed | — |  | PE: refuse an import address table inside the headers |
| #1559 | `acce0c03e468` | not yet reviewed | — |  | Shaders: a float inline constant in a 64-bit operand is the f64 value |
| #1558 | `6cc4929e7c93` | not yet reviewed | — |  | HLE: export constrained app notifications |
| #1557 | `1504ec9aa446` | not yet reviewed | — |  | Validate BC7 control-word subset counts |
| #1555 | `9fb81e1c17e4` | not yet reviewed | — |  | GPU: coalesce nearby shadow-buffer refreshes without spanning huge heap gaps |
| #1554 | `dce05bcc9508` | not yet reviewed | — |  | Shaders: flush subnormals in f32 MAD and MAC |
| #1551 | `2fc9b94e9780` | not yet reviewed | — |  | GPU: prioritize repeat-demanded pipelines with non-mesh tie-breaking and anti… |
| #1550 | `6c9e0ec115fd` | not yet reviewed | — |  | Shaders: a deep block's reach no longer hides a waterfall's proof |
| #1549 | `f078008b1123` | not yet reviewed | — |  | Formats: test texture layouts against addrlib's own sizes and addresses |
| #1548 | `2d6a0b1af382` | not yet reviewed | — |  | GPU: preserve full DYNAMIC descriptor range when host view heap is capped at … |
| #1547 | `bdf870eb4728` | not yet reviewed | — |  | GPU: invalidate HOST_WRITTEN textures when title memory is written |
| #1546 | `4a77d59f8365` | not yet reviewed | — |  | GPU: route indirect dispatch_mesh_x through State::execute_indirect |
| #1544 | `bf1c45b64bf5` | not yet reviewed | — |  | GPU: refresh and transition ResourceDescriptorHeap views before draws |
| #1543 | `e77f6331cecd` | not yet reviewed | — |  | GPU: avoid shift and arithmetic overflow panics in subresource and copyable_f… |
| #1542 | `b9301c6c330c` | not yet reviewed | — |  | [BUNDLE] GPU fixes 4: 10 PRs, #346 #431 #493 #967 #1082 #1143 #1165 #1173 #12… |
| #1541 | `85845466bd8e` | not yet reviewed | — |  | CLI: list-diff compares two translate listings program by program |
| #1540 | `1ccae2d0fb76` | not yet reviewed | — |  | Shaders: guard both back edges when a conditional branch targets two … |
| #1539 | `274f6988c977` | not yet reviewed | — |  | Loader: guard clwb_loop shortcut against u64 overflow and wraparound |
| #1538 | `73f0f7d68f77` | not yet reviewed | — |  | GPU: check subobject size against remaining stream bytes in create_st… |
| #1537 | `9ad1f21b64be` | not yet reviewed | — |  | HLE: update shared XMA context words atomically |
| #1536 | `50cbdd08d4f9` | not yet reviewed | — |  | Shaders: assemble dword-straddling global_load_ushort and global_load_sshort |
| #1529 | `460d0c1ed845` | not yet reviewed | — |  | Input: keyboard and mouse played like a PC game (rebindable [controls]) |
| #1525 | `bbe3b411766c` | not yet reviewed | — |  | Reuse fenced GPU upload scratch across command lists |
| #1524 | `8e14d23547c3` | not yet reviewed | — |  | Shaders: every DXIL loop gets one latch, and every block one back edge |
| #1521 | `780e6175f45b` | not yet reviewed | — |  | [BUNDLE] Bridge for #1368 and #1386: one storage test changes when both land,… |
| #1520 | `2190efeae1f3` | not yet reviewed | — |  | Draft: reproduce RTX 5080 world-transition GPU hang on combined renderer patc… |
| #1519 | `cb8dd75743eb` | not yet reviewed | — |  | Shaders: unpack 16-bit vertex inputs into the low half |
| #1517 | `c72555dbdc84` | not yet reviewed | — |  | Input: add Windows DualSense / DualSense Edge support |
| #1516 | `63be4e04826a` | not yet reviewed | — |  | Shaders: zero missing typed-buffer components |
| #1515 | `9ccb83c00a9b` | not yet reviewed | — |  | HLE: guard dialog commands before initialization |
| #1514 | `1288c632bb8f` | not yet reviewed | — |  | Add opt-in GPU memory synchronization tracing |
| #1511 | `4ac9e91c399f` | not yet reviewed | — |  | [BUNDLE] DXIL emission and container robustness: 5 PRs, #1145 #343 #1042 #130… |
| #1510 | `4ab20fc863c1` | not yet reviewed | — |  | GPU: the indirect budget is charged in record order and no longer wraps |
| #1506 | `42b7d5fe3bb7` | not yet reviewed | — |  | HLE: avoid following links during storage quota scans |
| #1505 | `ade80d28a758` | integrated | — |  | GPU: pipeline workers at the title's priority, not below it |
| #1504 | `56f7da33a064` | not yet reviewed | — |  | [BUNDLE] Computed barycentrics: #453 merged with #176 and #1293, stacked on #… |
| #1503 | `5a25850411e0` | not yet reviewed | — |  | GPU: fail fast when the host D3D12 runtime is unusable |
| #1502 | `3f204cd60fd9` | not yet reviewed | — |  | fix(gpu): transition shadow buffers before VB/IB binding |
| #1501 | `5b390eff2c04` | not yet reviewed | — |  | Shaders: DS ops in a program without LDS read 0 and drop writes |
| #1501 | `5b390eff2c04` | not yet reviewed | — |  | Shaders: DS ops in a program without LDS read 0 and drop writes |
| #1500 | `23400033753a` | not yet reviewed | — |  | GPU: copy title textures to the host once per change, and check them on sever… |
| #1497 | `39aa84849ab4` | not yet reviewed | — |  | HLE: fix DirectStorage file sharing and error capture |
| #1494 | `929e35b394ac` | not yet reviewed | — |  | formats: bound thick 3D mip tail by depth |
| #1493 | `cc0f47a4f4e5` | not yet reviewed | — |  | Shaders: key immediate samplers by comparison mode |
| #1492 | `ea33d12b3f70` | not yet reviewed | — |  | [BUNDLE] Shader instruction lowering: 21 PRs, #98 #156 #162 #168 #169 #368 #3… |
| #1491 | `28c91b91b015` | not yet reviewed | — |  | Shaders: the lane-condition search keeps the condition sets no lane meets |
| #1490 | `7f01c33c966c` | not yet reviewed | — |  | HLE: recycle freed XMem page numbers |
| #1489 | `e0a052c2bc07` | not yet reviewed | — |  | Shaders: lower image_gather4_c_lz and image_gather4_c_lz_o (MIMG 0x4f, 0x5f) |
| #1488 | `62c4ded0ffba` | not yet reviewed | — |  | Shaders: lower ds_add{_rtn}_f32 and ds_cmpst{_rtn}_f32 LDS atomics |
| #1487 | `1455884a28d7` | not yet reviewed | — |  | Shader tests: prefer high-performance hardware adapter in emit::host::Host::n… |
| #1486 | `ecdb14633b45` | not yet reviewed | — |  | Shaders: track source lane in whole-wave Sym::Own to prevent miscompiled shuf… |
| #1485 | `b77b148c629d` | not yet reviewed | — |  | Shaders: honor DPP16/DPP8 fi and source-lane EXEC activity |
| #1484 | `2b8b4d746fbe` | not yet reviewed | — |  | Shaders: prune unreachable blocks before SSA construction |
| #1483 | `0daabccbdec4` | not yet reviewed | — |  | Pick the discrete GPU on laptops; fix driver-dependent and wrong shader lower… |
| #1480 | `b32a5d593d46` | not yet reviewed | — |  | Shaders: unpack a16 image coordinates and gradients after whole 32-bi… |
| #1478 | `19052138d065` | not yet reviewed | — |  | Shaders: lower VOP3 integer clamp saturation for add/sub ops |
| #1477 | `2286e038cc60` | not yet reviewed | — |  | Shaders: key immediate sampler slots by descriptor and comparison mode |
| #1476 | `711d651ccab7` | not yet reviewed | — |  | Shaders: precompute MD5 round constants and avoid per-block Vec in md… |
| #1475 | `d8897b763962` | not yet reviewed | — |  | HLE: clear blocked DNS lookup results |
| #1474 | `1e923ad46234` | integrated | — |  | Shaders: set WAVE_OPS feature flag when lane_bit emits waveGetLaneIndex |
| #1473 | `826a97fe403d` | not yet reviewed | — |  | Shaders: track per-lane export state across mixed pixel-shader joins |
| #1472 | `c00db2cb3833` | not yet reviewed | — |  | GPU: flush only overlapping shadow refreshes before Step::GpuWrite |
| #1471 | `be3202de393b` | not yet reviewed | — |  | Shaders: fold empty loop latches containing only dead phis |
| #1469 | `ed9f8c6bdf6b` | not yet reviewed | — |  | Shaders: lower binary64 class comparisons |
| #1466 | `3ee8820bb5e3` | not yet reviewed | — |  | GPU: avoid faulting when zero-filling protected title memory |
| #1463 | `68f23d9e41f9` | not yet reviewed | — |  | GPU: cap command lists per submission (VICTORY_SUBMIT_MAX_RUNS) |
| #1462 | `9bab357ac855` | not yet reviewed | — |  | CLI: report emitted shader wave sizes in translate |
| #1461 | `f9822de8b349` | not yet reviewed | — |  | GPU: handle DXGI Present failures and occlusion |
| #1460 | `3e3a71a874c8` | not yet reviewed | — |  | Shaders: keep distinct critical-edge blocks for duplicate branch targets |
| #1459 | `a88abe80190b` | not yet reviewed | — |  | [BUNDLE] CLI and census: 11 PRs, #115 #151 #238 #353 #378 #476 #482 #535 #649… |
| #1457 | `ff3b10e753a3` | not yet reviewed | — |  | DirectStorage: keep raw reads running while staging drains |
| #1456 | `f86fd7601fe4` | not yet reviewed | — |  | [BUNDLE] Loader, PE reader and config: 20 PRs, #188 #213 #214 #219 #221 #311 … |
| #1455 | `a7636dd7b1a4` | not yet reviewed | — |  | HLE: allow an explicit maximum title window size |
| #1454 | `e49b332b075b` | not yet reviewed | — |  | [BUNDLE] Docs: README and docs additions, 10 PRs, #280 #323 #354 #424 #532 #6… |
| #1453 | `6f36cdf98252` | not yet reviewed | — |  | GPU: interpolate normalized rectangle vertex attributes |
| #1452 | `f5e1259b9b10` | not yet reviewed | — |  | [BUNDLE] Formats (XMA, BCPack, texture detiling): 12 PRs, #96 #172 #183 #351 … |
| #1451 | `f78bfa7ec648` | not yet reviewed | — |  | Shaders: interpolate by hand rows read flat and interpolated; zero and copied… |
| #1450 | `cfd9130783e6` | not yet reviewed | — |  | Formats texture: add thick-3D mip-tail regression tests matching AMD addrlib |
| #1449 | `1cae27ba7e39` | not yet reviewed | — |  | GPU: report host resources held by descriptor heap slots in diagnostics |
| #1448 | `a2700be14795` | not yet reviewed | — |  | GPU: invalidate root bindings when a root signature changes |
| #1447 | `604f24b4abc8` | not yet reviewed | — |  | hle: confine virtual-drive paths and harden storage/package inputs |
| #1446 | `8d3a025df199` | not yet reviewed | — |  | GPU list: bind initial pipeline state in create_list and create_list_x |
| #1445 | `139080a9ef9c` | not yet reviewed | — |  | HLE reuses freed physical page numbers |
| #1443 | `e3fe5bf94f84` | not yet reviewed | — |  | Shaders: lower scalar quadmask instructions |
| #1441 | `60bf416db31a` | not yet reviewed | — |  | Shaders: preserve the unwritten high half of packed D16 buffer loads |
| #1439 | `f9eb9798e792` | not yet reviewed | — |  | [BUNDLE] GPU pipeline library, shader cache, draw state and frame wait: 16 PR… |
| #1438 | `6bd4135121e1` | not yet reviewed | — |  | XPackage: wire XPackageEnumerateChunkAvailability |
| #1437 | `4cfae26174f4` | not yet reviewed | — |  | Shaders: preserve the unwritten high half of packed D16 image loads |
| #1436 | `08a41e838934` | not yet reviewed | — |  | Game: check record bounds and offset arithmetic in SGD5 parsing |
| #1435 | `f542636cfb8f` | not yet reviewed | — |  | HLE: implement XTaskQueueRegisterWaiter and XTaskQueueUnregisterWaiter |
| #1434 | `098065bb9ec6` | not yet reviewed | — |  | Loader: match title DLL filenames ignoring ASCII case |
| #1433 | `b3efa1c0e3b7` | not yet reviewed | — |  | Shaders: console flow for groups of several console waves on 32-lane hosts |
| #1432 | `ed397b1a7c17` | not yet reviewed | — |  | HLE: set ERROR_PROC_NOT_FOUND for missing pseudo-module exports |
| #1431 | `04b68eea6043` | not yet reviewed | — |  | HLE: reopen the emulated OEM key on a null or empty subkey |
| #1430 | `7af465164699` | not yet reviewed | — |  | Input: keep keyboard stand-in pad active while XInput slots are idle |
| #1429 | `48bf0d791c95` | not yet reviewed | — |  | GPU: persist early-pipeline keys even without a host pipeline library |
| #1428 | `e58d8043849b` | not yet reviewed | — |  | GPU: guard OMSetRenderTargets handle expansion on empty, null, or oversized c… |
| #1427 | `e4e24714d4bd` | not yet reviewed | — |  | GPU: turn zero-element buffer SRVs and UAVs into null descriptors |
| #1426 | `3fc262e22ace` | not yet reviewed | — |  | [BUNDLE] GPU host copies, descriptors, sampler heaps and footprints: 15 PRs, … |
| #1425 | `06a543e8a9d3` | not yet reviewed | — |  | Shaders: a refusal names the instruction that made its missing value |
| #1424 | `4ef937194602` | not yet reviewed | — |  | GPU skips empty host submissions |
| #1423 | `55f0dc4eb480` | not yet reviewed | — |  | victory-shader: lazily lower non-active console lane-mask SOPs bitwise |
| #1422 | `b70f51afaeb2` | not yet reviewed | — |  | GPU: ignore clear values on non-target textures and avoid caching failed host… |
| #1421 | `f371228a3101` | not yet reviewed | — |  | hle: parse prefixed title ids in manifests |
| #1420 | `9f57d9c0110d` | not yet reviewed | — |  | Shaders: a mask built up in a loop every lane runs the same way (stacked on #… |
| #1419 | `c8b7a3d59b1d` | not yet reviewed | — |  | HLE: pass non-code LoadLibraryEx mappings outside HostPolicy to the host |
| #1418 | `39d044fa6509` | not yet reviewed | — |  | [BUNDLE] GPU fixes (write watch, textures, device, DXGI, objects, primitives,… |
| #1417 | `329600213704` | not yet reviewed | — |  | Shaders: lower SV_RenderTargetArrayIndex and SV_ViewportArrayIndex exports |
| #1416 | `54dea150f6f4` | not yet reviewed | — |  | Shaders: return zero for inactive compute swizzle sources |
| #1415 | `61f2848bd45e` | not yet reviewed | — |  | CLI: reuse the D3D12 host across translation workers |
| #1414 | `c11dce0ce2d7` | not yet reviewed | — |  | GPU: avoid clobbering HostList prelude allocator when a list executes twice i… |
| #1412 | `86b29fa4cd5d` | not yet reviewed | — |  | Shaders: keep scalar comparisons between lane masks uniform |
| #1411 | `2b13ae419202` | not yet reviewed | — |  | HLE clears the result of blocked address lookups |
| #1409 | `de109f1d3d5f` | not yet reviewed | — |  | Shaders: read heap-loaded s_buffer_load raw; SampleLevel outside pixel shaders |
| #1408 | `a69e32405731` | not yet reviewed | — |  | Shaders: lower float wave prefix sums via lane reads |
| #1407 | `e19347488050` | not yet reviewed | — |  | Shaders: lane masks holding lanes elsewhere, and mask joins, read as numbers |
| #1406 | `87b42ecd18c9` | not yet reviewed | — |  | GPU: fix NULL/UNKNOWN SRV/UAVs on promoted-typeless textures and MSAA/array/3… |
| #1405 | `ad2025a433c9` | not yet reviewed | — |  | GPU: a view's own copy that shaders feed reads its buffer's copy on the GPU (… |
| #1404 | `6f0badc834c3` | not yet reviewed | — |  | Shaders: test raw VCC on s_cbranch_vccz and vccnz without EXEC masking |
| #1403 | `567e1345fb77` | not yet reviewed | — |  | CLI: report shader worker initialization failures |
| #1402 | `7e7c5be2db4f` | not yet reviewed | — |  | [BUNDLE] High-yield shader PRs (+295/+520/+153/+117 programs in #1247): #1067… |
| #1400 | `f2491a58f6c9` | not yet reviewed | — |  | GPU: split duplicate HostList entries into separate ExecuteCommandLists batch… |
| #1399 | `d01f9a6aeb8d` | not yet reviewed | — |  | Shader & GPU: support merged ES+GS geometry stages (vs_6_6 / gs_6_6) |
| #1398 | `38ddebf95426` | not yet reviewed | — |  | Shaders: honor clamp on unsigned VOP3 add and subtract |
| #1397 | `a78fc2cb8f3a` | not yet reviewed | — |  | Shader: propagate float values across selects/phis and cache bools across dom… |
| #1396 | `0fbc85a60eb1` | not yet reviewed | — |  | GPU: preserve sub-64 KiB alignments and check bounds in allocation_info |
| #1395 | `a91e9adde213` | not yet reviewed | — |  | Shaders: lower s_bfe_u64, s_bfe_i64 and s_bfm_b64 |
| #1394 | `71798281f998` | not yet reviewed | — |  | HLE: make XUserCompare antisymmetric |
| #1393 | `7a780a0ad707` | not yet reviewed | — |  | Shaders: vectorize wide s_buffer_load and global_load rawBufferLoad calls |
| #1392 | `9687a19581ca` | not yet reviewed | — |  | gpu: emit UAV barrier after a dropped compute dispatch |
| #1391 | `78c12677e54f` | not yet reviewed | — |  | Shaders: lower v_cvt_off_f32_i4 |
| #1390 | `97c8783f4a26` | not yet reviewed | — |  | GPU: keep A8_UNORM out of the R8 typeless family |
| #1389 | `9b39340bff50` | not yet reviewed | — |  | [BUNDLE] HLE audio and input (XMA, APU, WASAPI capture, GameInput): #194 #528… |
| #1388 | `9d84135faadd` | not yet reviewed | — |  | GPU: honor command-list 64-bit memory waits without blocking submissions (#70… |
| #1387 | `c77b2533559d` | not yet reviewed | — |  | Round v_fma_f16 once in DXIL lowering |
| #1386 | `bea3c810ef8b` | not yet reviewed | — |  | [BUNDLE] HLE host side (files, paths, host DLLs, display, xmem, net, speech):… |
| #1385 | `3e630d357c13` | not yet reviewed | — |  | Shaders: narrow mesh output bounds from GS_ALLOC_REQ M0 analysis (#201) |
| #1384 | `3215e71e9c0d` | not yet reviewed | — |  | Shaders: hardware test for the three-operand integer ops |
| #1383 | `a613b9e81b1d` | not yet reviewed | — |  | Preserve Win32 errors from DirectStorage OpenFile |
| #1383 | `a613b9e81b1d` | not yet reviewed | — |  | Preserve Win32 errors from DirectStorage OpenFile |
| #1382 | `bf8a9b23ec9c` | not yet reviewed | — |  | GPU: batch resource barriers, reuse draw scratch buffers, range-lock copy_sim… |
| #1381 | `f025d5d83c2a` | not yet reviewed | — |  | Shaders: hardware test for v_not, v_bfrev and the vector bit-scan ops |
| #1380 | `2f300c4cf173` | not yet reviewed | — |  | Net: allow local machine hostname in resolvable and loopback_name |
| #1379 | `d6c7078aebcc` | not yet reviewed | — |  | Shaders: hardware test for v_alignbit/alignbyte, v_bfi, v_bfm and v_bcnt |
| #1378 | `5c6dea37170f` | not yet reviewed | — |  | Shaders: hardware test for v_mul_lo_u32, v_mul_hi_u32 and v_mul_hi_i32 |
| #1377 | `c6ce95a142b9` | not yet reviewed | — |  | Shaders: hardware test for the 24-bit v_mul and v_mad forms |
| #1376 | `272c262daf6e` | not yet reviewed | — |  | Shader: lower scalar Root CBV/SRV loads and descriptor-as-data queries |
| #1374 | `5dd1e71379a5` | not yet reviewed | — |  | GPU: read same-list GPU writes in placed buffer-to-texture copies |
| #1373 | `56fbe7c23682` | not yet reviewed | — |  | HLE: guard against null GWLP_USERDATA in XGameUi dialog WM_COMMAND handler |
| #1372 | `fa22cba5df93` | not yet reviewed | — |  | Loader: lazily NOP-patch clwb/monitorx on #UD, emulate rdpid, batch IAT prote… |
| #1371 | `5c44562868df` | not yet reviewed | — |  | Shaders: compress replaced phi chains in remove_trivial_phis |
| #1370 | `44d9c2da1b65` | not yet reviewed | — |  | Shaders: zero missing channels on typed buffer loads |
| #1369 | `12a089334917` | not yet reviewed | — |  | GPU: query native format support and multisample quality levels |
| #1368 | `152368e11450` | not yet reviewed | — |  | [BUNDLE] HLE runtime APIs (XGameRuntime, XStore, XSystem, XPackage, XG, XUser… |
| #1367 | `62d367a561d8` | not yet reviewed | — |  | XPackage: wire XPackageUnregisterInstallationProgressChanged returning bool |
| #1366 | `4e155a36ad72` | not yet reviewed | — |  | Shaders: do not parse VS inputs when reading mesh shader stage I/O |
| #1365 | `a4c994bf574c` | not yet reviewed | — |  | Shaders: hardware test for s_bfm_b32 and s_bitset0/1_b32 |
| #1364 | `26f70987e6ac` | not yet reviewed | — |  | Shaders: hardware test for s_mul_i32, s_mul_hi_u32 and s_mul_hi_i32 |
| #1363 | `94874f2c8b08` | not yet reviewed | — |  | [BUNDLE] XAsync, kernel, lifecycle and DirectStorage HLE: #114 #122 #247 #109… |
| #1362 | `c98c50f9c66e` | not yet reviewed | — |  | Shaders: hardware test for s_lshl1..4_add_u32 |
| #1361 | `8d6d06c342ef` | not yet reviewed | — |  | Shaders: hardware test for the scalar bit-scan, reverse and abs ops |
| #1360 | `0e24b79ae220` | not yet reviewed | — |  | Shaders: hardware test for s_min_i32/u32 and s_max_i32/u32 |
| #1359 | `f2782f3d6319` | not yet reviewed | — |  | Shaders coalesce conditional branches to their fallthrough |
| #1358 | `144b6203c05a` | not yet reviewed | — |  | [BUNDLE] XGameSave hardening: #228 #399 #570 #590 #1006 #1110 #1123 |
| #1357 | `cdec26e9dd45` | not yet reviewed | — |  | HLE: native 4-motor Xbox impulse trigger haptics via Windows.Gaming.Input wit… |
| #1356 | `080456154250` | not yet reviewed | — |  | GPU: use RwLock for read-mostly tables and add per-HostList VA/view caches |
| #1355 | `2e602026c13a` | not yet reviewed | — |  | HLE & Formats: replace miniz_oxide with SIMD-accelerated zlib-rs |
| #1354 | `cd9699f6b2d0` | not yet reviewed | — |  | Shaders: infer heap UAVs program-wide and emit IsGloballyCoherent on glc UAVs |
| #1352 | `a695f884d660` | not yet reviewed | — |  | GPU: implement resource-backed texture UAV clears (#771) |
| #1351 | `5a44d97d1ceb` | not yet reviewed | — |  | GPU: deduplicate per-submission write-watch askings over nested buffers (#776) |
| #1350 | `2f27ea1e9eb8` | not yet reviewed | — |  | GPU: scan virtual regions on copy_mapped and write_mapped fallback (#783) |
| #1349 | `eb5fea7359f3` | not yet reviewed | — |  | GPU: reserve 2^18 dynamic slots on capped view heaps and trim fresh tables (#… |
| #1346 | `6db970c10a0f` | not yet reviewed | — |  | GPU: validate allocate_upload alignments and handle oversized requests |
| #1345 | `7b108912f0c4` | not yet reviewed | — |  | docs: record RTX 4060 story-mode validation in English and Portuguese |
| #1344 | `008900daa4bc` | not yet reviewed | — |  | GPU: read DRED constant-view words without VirtualQuery |
| #1342 | `8448e91f535f` | not yet reviewed | — |  | Shaders: lower buffer_atomic_{sub,csub,inc,dec,fcmpswap,fmin,fmax} |
| #1341 | `59e48eb90d83` | not yet reviewed | — |  | Shaders: lower v_ldexp_f32 (VOP3 0x362) |
| #1340 | `89189ecdbef2` | not yet reviewed | — |  | Shaders: mask M0[7:0] on v_movrel* and allow scalar/const src0 on v_movreld_b… |
| #1339 | `b686bcc5d929` | not yet reviewed | — |  | Shaders: support clamp output modifier on v_cvt_pkrtz_f16_f32 |
| #1338 | `e2dc6baec3a6` | not yet reviewed | — |  | Shaders: honor op_sel[3], clamp, and omod on v_interp_p2_f16 |
| #1337 | `7dfa86fa39ec` | not yet reviewed | — |  | Shaders: preserve mip point filtering in anisotropic static samplers |
| #1336 | `4eead6e1163c` | not yet reviewed | — |  | Shaders: fix RPO resource tracking, SMEM alignment, raw VCC branch, and readl… |
| #1335 | `6ab2927d8a0e` | not yet reviewed | — |  | test(shader): translate random GFX10 programs on every test run |
| #1333 | `2901bb670a2e` | not yet reviewed | — |  | Shaders: bindless through pointer chasing, bias means only the pointer's own … |
| #1332 | `6e3419128b13` | not yet reviewed | — |  | GPU: handle empty and inverted ranges in watch::writes, writing, and title_me… |
| #1331 | `84845210cc9e` | not yet reviewed | — |  | GPU: rebind replaced descriptor heaps by host identity |
| #1330 | `fb40a8f834e5` | not yet reviewed | — |  | Windows playability, shader translation, D3D12 stability, and HLE improvements |
| #1329 | `c52be85ed9f7` | not yet reviewed | — |  | HLE GameInput supports IUnknown queries |
| #1328 | `91823286336d` | not yet reviewed | — |  | Shaders: bitwise wave prefixes as WaveMultiPrefixOp |
| #1327 | `4f2eee6bb595` | not yet reviewed | — |  | Shaders: apply bitwise modifiers to v_cmp_class_f32 source operands |
| #1326 | `75d919f32dee` | not yet reviewed | — |  | Shaders: fold always-false and always-true compares to boolean constants |
| #1324 | `8f28c35af440` | not yet reviewed | — |  | Pipeline: accept existing entry when concurrent cache publication races on Wi… |
| #1322 | `7a037533f8ee` | not yet reviewed | — |  | GPU: copy title writes under ranges a submission's shaders write |
| #1320 | `70b92d9616c4` | not yet reviewed | — |  | GPU diagnostics: skip host pipeline creation by shader hash |
| #1319 | `643aef6460cf` | not yet reviewed | — |  | [BUNDLE] Lane-mask, scalar descriptor read and cube-sample shader fixes: #105… |
| #1315 | `f74eab0ff299` | not yet reviewed | — |  | Shaders, GPU: translate single-stream triangle geometry programs |
| #1314 | `f574bf2085ac` | not yet reviewed | — |  | Shaders: lower v_{fmaak,fmamk}_f16, v_cvt_pk_{i16_i32,u16_u32}, and v_lerp_u8 |
| #1313 | `7295d7e85cec` | not yet reviewed | — |  | Shaders: return zero for unused image query extents |
| #1312 | `d27838ecd9f3` | not yet reviewed | — |  | Shaders: mask v_readlane/v_writelane lane index and bypass EXEC on writelane |
| #1310 | `b2194106bb52` | not yet reviewed | — |  | feat: add native Linux support (loader, HLE, POSIX bridge) |
| #1309 | `0e8c0eb4c557` | not yet reviewed | — |  | HLE audio capture: fix WAV/QPC overflows, chunk silence writes, close target … |
| #1308 | `942a5673e358` | not yet reviewed | — |  | HLE xpackage: fix bool vtable slots, symlink dir walk, and estimate_download … |
| #1307 | `fc53aca4ec3b` | not yet reviewed | — |  | GPU dxgi: null out on NOT_FOUND, preserve display mode format, add GetDesc1 |
| #1306 | `29d78bae7044` | not yet reviewed | — |  | Loader: fix thunk block overflow, capacity enforcement, flush errors, and lea… |
| #1304 | `98716d1a58bf` | not yet reviewed | — |  | Loader: fix thread shutdown targeting, registry lifetime, and CRT detach orde… |
| #1303 | `428725aaed22` | not yet reviewed | — |  | Shaders: bounds-check DXIL header offsets and recognize dword zero padding |
| #1301 | `cacb8d2ff8d1` | not yet reviewed | — |  | Shaders: align GFX10 opcode tables with RDNA2 ISA and add synthetic tests |
| #1300 | `7383f54e2a5d` | not yet reviewed | — |  | Shaders: lower v_{min3,max3,med3}_{i16,u16} and v_mad_{u32_u16,i32_i16} |
| #1299 | `32eb93495919` | not yet reviewed | — |  | docs: explain Windows shader-test validator prerequisite |
| #1298 | `df8f906b1e59` | not yet reviewed | — |  | GPU: replace stale descriptors on null CBV writes |
| #1296 | `a6f3ccb6998f` | not yet reviewed | — |  | GPU: add VICTORY_TEXTURE_MIP_DROP and DXGI auto-default for <= 8 GB GPUs (fix… |
| #1295 | `6cf80f2c7288` | not yet reviewed | — |  | Shaders: decode 16-bit integer inline float constants as f32 bits |
| #1293 | `6a9cc8fef867` | not yet reviewed | — |  | Shaders: follow plain v_mov_b32 copies when pairing interpolations |
| #1292 | `36260aecf994` | not yet reviewed | — |  | Shaders: lower v_dot2_{i32_i16,u32_u16}, v_dot4_{i32_i8,u32_u8}, v_dot4c_i32_… |
| #1291 | `94dbc0a35496` | not yet reviewed | — |  | victory-shader: emit zero for padding halves in f16 pixel input interpolation |
| #1290 | `100de0e97a23` | not yet reviewed | — |  | Shader tests: a skipped device says why it was skipped |
| #1289 | `003944a950dd` | not yet reviewed | — |  | Shader tests: v_pk_mul_lo_u16 keeps each half to 16 bits |
| #1288 | `0d0f5f91f9b0` | not yet reviewed | — |  | Shaders: a typed fetch inside a dword reads the aligned pair |
| #1287 | `7566b6d42689` | not yet reviewed | — |  | Add experimental AMD frame generation test path |
| #1285 | `ac1a6e2a5181` | not yet reviewed | — |  | Shaders: fix 16-bit VALU SDWA placement/sext and inline float constants |
| #1284 | `bea3a581e6ec` | not yet reviewed | — |  | Add experimental PC FSR upscaling |
| #1283 | `b232b8838d52` | not yet reviewed | — |  | Clarify DRED reports when GPU progress is unknown |
| #1282 | `1ed26e50d860` | not yet reviewed | — |  | Shaders: fold conditional branches to their fallthrough |
| #1281 | `72e794b6294c` | not yet reviewed | — |  | GPU/Shader: declare wave64 wave size and strip unsupported video-texture flags |
| #1280 | `a93f1f12e0d2` | not yet reviewed | — |  | Shaders: gradient samples with a level clamp, and cube gradients in face space |
| #1279 | `4dd4cfc8c006` | not yet reviewed | — |  | GPU: an argument clamp Wine's compiler builds too |
| #1278 | `d464b0468d73` | not yet reviewed | — |  | Docs: add testing.md |
| #1277 | `673936697ca7` | not yet reviewed | — |  | GPU: remember which title memory is view memory until it is remapped |
| #1276 | `b95831e1b5bf` | not yet reviewed | — |  | Shaders: scalar loads from a code table at the first lane's address |
| #1275 | `cdbe17891855` | not yet reviewed | — |  | Shaders: apply v_mov_b32 and v_cndmask_b32 abs/neg modifiers bitwise |
| #1274 | `4f54c4775574` | not yet reviewed | — |  | HLE: complete XPackage progress callback unregistration |
| #1273 | `62946e6eefb4` | not yet reviewed | — |  | GPU: mirror placed unordered texture views |
| #1272 | `ccd13f80949a` | not yet reviewed | — |  | GPU: automatic texture mip-drop on E_OUTOFMEMORY and VICTORY_TEXTURE_MIP_DROP |
| #1271 | `25d9e2add72c` | not yet reviewed | — |  | Shaders: infer heap UAVs and mark glc UAV handles globally coherent |
| #1270 | `77eafaa11014` | not yet reviewed | — |  | Shaders: lower v_cndmask_b32 selects on uniform scalar masks |
| #1269 | `8c3092f43ca3` | not yet reviewed | — |  | Shaders: handle dead mask dwords at data joins and wave-uniform s_bcnt1 SCC |
| #1268 | `00c6c961f321` | not yet reviewed | — |  | GPU: skip empty command list submissions |
| #1267 | `4cefaac2be47` | not yet reviewed | — |  | GPU: keep only the bytes shaders may write, each once per submission |
| #1266 | `f14fdfba020b` | not yet reviewed | — |  | Tests: keep the D3D12 pipeline smoke check portable under Wine |
| #1265 | `25e58682283a` | not yet reviewed | — |  | GPU: VICTORY_WIDE_SCISSOR widens a scissor to its viewport (a diagnostic) |
| #1264 | `204ec23c8594` | not yet reviewed | — |  | HLE: map a page scatter in runs, not one page at a time |
| #1263 | `d4ee6a57f581` | not yet reviewed | — |  | Shader: the loop limit switch works in every build |
| #1262 | `c5aeedd064d9` | not yet reviewed | — |  | Shaders: double-precision add, mul, fma, min, max, compares and conversions (… |
| #1261 | `d493a131df47` | not yet reviewed | — |  | Shaders: an SGPR no instruction writes stays refused above the initialized on… |
| #1260 | `10706d2a5ce1` | not yet reviewed | — |  | GPU: one aligned copy per unaligned SRV range per list (follow-up to #986) |
| #1259 | `5be1fe4b658d` | not yet reviewed | — |  | Shaders: v_med3 orders -0 below +0, as RDNA2 does |
| #1258 | `4c748c619976` | not yet reviewed | — |  | Gather XMA frames in byte-sized chunks |
| #1257 | `5981894e7491` | not yet reviewed | — |  | Shaders: verify restored compute lane masks on D3D12 (stacked on #298) |
| #1256 | `18a1073d4982` | not yet reviewed | — |  | Shaders: decode cube-array slice and face in image lowering |
| #1255 | `9459e0dd095f` | not yet reviewed | — |  | Shaders: support clamped image sample variants and include cubemap kinds in s… |
| #1254 | `83a8ec3d081c` | not yet reviewed | — |  | GPU diagnostics: a black frame's trace keeps the draws that end the frame |
| #1253 | `c880bee775a3` | not yet reviewed | — |  | Shaders: follow scalar index arithmetic in waterfall table offsets |
| #1252 | `3596ccc2b422` | not yet reviewed | — |  | Update architecture.md |
| #1251 | `17a15daf7a8e` | not yet reviewed | — |  | Diagnostics: trace command-list waits and memory write progress |
| #1248 | `64a33ec757c1` | not yet reviewed | — |  | XNetworking: wire UDP port change callbacks and UTF-16 security queries |
| #1246 | `59035de94c4a` | not yet reviewed | — |  | GPU: grow per-list upload scratch on demand |
| #1245 | `e6dfe9c61daf` | not yet reviewed | — |  | Shaders: a biased sample of 16-bit coordinates reads its bias from its own VG… |
| #1244 | `7c27219d4d6e` | not yet reviewed | — |  | Validate VICTORY_PAIRS before translating shaders |
| #1243 | `9472139a2a7c` | not yet reviewed | — |  | HLE: the module tree gates for a host that is not Windows |
| #1241 | `f02c70246c43` | not yet reviewed | — |  | CI: build and test the host-independent crates on Linux |
| #1238 | `591932ac8ce4` | not yet reviewed | — |  | Grow DirectStorage BCPack intermediates as zlib inflates |
| #1237 | `a3346f97d6f4` | not yet reviewed | — |  | Shaders: implicit-level samples in compute take their gradients from the quad |
| #1236 | `ec836909edd4` | integrated | — |  | GPU diagnostics: stop inferring descriptor slots from fault addresses |
| #1233 | `312ff569264b` | not yet reviewed | — |  | Report bound descriptor tables accurately in DRED breadcrumbs |
| #1231 | `8996aeb8cc4e` | not yet reviewed | — |  | Shaders: uniform phis (loop counters) under v_readfirstlane addresses (stacke… |
| #1230 | `6f77efdb0c3f` | not yet reviewed | — |  | Shaders: path-sensitive reachability for missing values across phi joins (#73… |
| #1229 | `da253596bce8` | not yet reviewed | — |  | Shaders: handle redundant partial descriptor writes and lane-preserving VGPR … |
| #1228 | `63e6c9eca4d8` | not yet reviewed | — |  | GPU: order shader writes on command list pipeline syncs (#732) |
| #1227 | `c3beb4a2628b` | not yet reviewed | — |  | Improve shader mask handling and GPU diagnostics |
| #1225 | `b00573d87932` | not yet reviewed | — |  | Shaders: trace a code-table address read from the first lane of a uniform val… |
| #1224 | `9fb62942d3c7` | not yet reviewed | — |  | HLE: coalesce split placeholders on map and group runs in map_alias |
| #1223 | `ba532339235f` | not yet reviewed | — |  | Shaders: lower v_ldexp_f16 (VOP2 0x3b and VOP3 0x13b) |
| #1221 | `67c4f53109ab` | not yet reviewed | — |  | victory-shader: resolve Structured::other and constant masks in v_cndmask_b32 |
| #1220 | `b4e464738d6e` | not yet reviewed | — |  | GPU: a half exactly halfway between two rounds to the even one, not up |
| #1218 | `a19107bdfdf2` | not yet reviewed | — |  | xtask: wine-test runs the Windows tests on a Linux host under Wine |
| #1217 | `a2e532d155a2` | not yet reviewed | — |  | GPU: the copies timing no longer counts the flush before a title copy twice |
| #1215 | `6950319ce556` | not yet reviewed | — |  | GPU: copy back with NtWriteVirtualMemory, no VirtualQuery before a copy into … |
| #1214 | `643fc6dfecff` | not yet reviewed | — |  | HLE: write decoded Opus samples through unaligned output buffers |
| #1213 | `d30bed44dd92` | not yet reviewed | — |  | HLE: reject null handle in WinHttpSetOption proxy path and expand net.rs test… |
| #1212 | `d10138b98c1b` | not yet reviewed | — |  | HLE: expand synthetic unit tests for apu.rs (1 -> 19 tests) |
| #1211 | `7dffb2a6c245` | not yet reviewed | — |  | Shaders: lower v_mad_{u16,i16} and saturating clamp on 16-bit integer ops |
| #1210 | `fd5948937bc6` | not yet reviewed | — |  | GPU: upload a host-copied texture again once the title writes its memory |
| #1209 | `ae0b48e6daf7` | not yet reviewed | — |  | Shaders: fix GFX10 independent a16/g16 gradient dword packing in fields.rs wi… |
| #1208 | `084d8d2fd9db` | not yet reviewed | — |  | Shaders: PSV0 and signatures laid out as the validator expects |
| #1207 | `649c6e5274ef` | not yet reviewed | — |  | GPU: validate input layouts and aligned vertex offsets in pipeline/host.rs wi… |
| #1206 | `ef0c3e944916` | not yet reviewed | — |  | GPU: validate null args in create_deserializer to prevent d3d12 AV and add ro… |
| #1205 | `d3867e6899df` | not yet reviewed | — |  | Formats: add 7 invariant unit tests for xma/tables.rs |
| #1204 | `e3c9dd0d53d9` | not yet reviewed | — |  | GPU: guard zero/overflowing stride in expand_rects and add primitive.rs unit … |
| #1203 | `76291ef38c31` | not yet reviewed | — |  | GPU: expand synthetic unit tests for pipeline/make.rs (1 -> 9/12 tests) |
| #1202 | `57d00fdcb1b2` | not yet reviewed | — |  | Shaders: fix 4 DXIL validator bugs and add synthetic container/validator tests |
| #1201 | `98b761bd1f18` | not yet reviewed | — |  | Loader: fix 4 bounds and cleanup bugs in windows.rs with unit tests |
| #1200 | `f4c4bd47daf9` | not yet reviewed | — |  | GPU: fix 3 encoding, fallback, and slice-panic bugs in shader_cache.rs with u… |
| #1199 | `8466edd7bb1f` | not yet reviewed | — |  | HLE: reject null out in QueryApiImpl before lookup and add xgameruntime unit … |
| #1198 | `1bbae65b8036` | not yet reviewed | — |  | Shaders, GPU: emulate the BVH intersection instructions, and reach title memo… |
| #1197 | `89670357c408` | not yet reviewed | — |  | Shaders: lower v_sad_u8, v_msad_u8, v_sad_u16, and v_sad_u32 |
| #1196 | `a4a94d720299` | not yet reviewed | — |  | HLE: fix self-closing XML element body parsing in runtime.rs and add unit tes… |
| #1195 | `ed1d9c75e4e4` | not yet reviewed | — |  | Shaders: expand synthetic unit tests for emit/sampler.rs |
| #1194 | `d51dc42a9dfa` | not yet reviewed | — |  | HLE: expand synthetic unit test coverage for xstore.rs (1 -> 13 tests) |
| #1193 | `62abb6071a7d` | not yet reviewed | — |  | HLE: add synthetic unit tests for storage.rs and host.rs |
| #1192 | `02ea74cc21b6` | not yet reviewed | — |  | Loader: fix 4 bounds/overflow bugs in tls.rs and stacks.rs with unit tests |
| #1191 | `1456fe76cd8e` | not yet reviewed | — |  | GPU: fix texture footprint span and committed-range overflow with unit tests |
| #1190 | `d160854be046` | not yet reviewed | — |  | Shaders: lower v_frexp_mant_{f32,f16} and v_frexp_exp_i{32_f32,16_f16} |
| #1189 | `370c61bffc81` | not yet reviewed | — |  | GPU: make fence signaling monotonic, clamp sampler enums, and add unit tests |
| #1188 | `6765a7bdbb4a` | not yet reviewed | — |  | GPU: guard non-positive frames and add unit tests for copystat.rs |
| #1187 | `bcfd293095ca` | not yet reviewed | — |  | Keep unstaged DirectStorage reads moving under staging pressure |
| #1186 | `ce94115ef8aa` | not yet reviewed | — |  | GPU: fix 8 overflow, lookup, and snapshot bugs in shadow.rs with regression t… |
| #1185 | `4d9b271a868f` | not yet reviewed | — |  | Stream shader archive scans in bounded batches |
| #1183 | `15cba99a4ca6` | not yet reviewed | — |  | Keep shader metadata variants in the offline translation census |
| #1182 | `8c94b5c317dc` | not yet reviewed | — |  | Shaders: keep prior per-lane descriptor word across redundant masked moves (f… |
| #1181 | `715293e3e458` | not yet reviewed | — |  | HLE: keep DirectStorage file references alive for queued reads |
| #1180 | `9e5c1304ee02` | integrated | — |  | gfx10: resolve the opcode name once, at decode time (translation -4..-8%) |
| #1179 | `f60fb419cbff` | not yet reviewed | — |  | Input: choose the loose controls diagram by active gamepad (stacked on #312) |
| #1178 | `9b8949f4ad71` | not yet reviewed | — |  | Diagnostics: trace a DirectStorage request from submission to decoder outcome |
| #1177 | `142ec8dd4563` | integrated | — |  | Deduplicate image paths in victory census |
| #1176 | `4c2d610bc4f3` | not yet reviewed | — |  | Shaders: reads of uninitialized pixel SGPRs read as zero |
| #1175 | `bfe5d2dab276` | not yet reviewed | — |  | Shaders: verify compute writes through a spilled root UAV |
| #1174 | `299bc17db983` | not yet reviewed | — |  | HLE: implement IUnknown::QueryInterface and unit tests for xgame objects |
| #1173 | `0be58e62d241` | not yet reviewed | — |  | GPU: add synthetic unit tests for query.rs and signature.rs |
| #1171 | `131944d02dc0` | not yet reviewed | — |  | PE: add synthetic unit tests for implib.rs and layout.rs |
| #1170 | `9b1a0fe926a0` | not yet reviewed | — |  | Fix root-indexed descriptors in rebuilt resource tables |
| #1169 | `212778a4bec3` | not yet reviewed | — |  | HLE: implement remaining XAccessibility closed-caption, high-contrast, and sp… |
| #1168 | `3247043440ff` | not yet reviewed | — |  | GPU: implement SetPredication, SetMarker, BeginEvent, and EndEvent on command… |
| #1167 | `36ae257c97f8` | not yet reviewed | — |  | HLE: implement remaining XNetworking UDP port-change and UTF-16 security slots |
| #1166 | `d57ea6762607` | not yet reviewed | — |  | GPU: implement ID3D12GraphicsCommandList::ClearState (slot 0x58) |
| #1165 | `735f3af7cb95` | not yet reviewed | — |  | GPU: implement DiscardResource on command lists with region and state checks |
| #1164 | `9a173fc8f3bd` | not yet reviewed | — |  | Shaders: add hardware tests for v_cmp_class_f32, integer min3/max3/med3, and … |
| #1163 | `8a0ac650fdc8` | not yet reviewed | — |  | Core: normalize trailing .. and Windows verbatim paths in policy::resolve |
| #1162 | `12e9c6a3a52b` | not yet reviewed | — |  | GPU: implement ClearUnorderedAccessViewUint/Float for buffers and textures |
| #1161 | `aaacc2dcfe09` | integrated | — |  | docs(architecture): list BCPack as implemented in victory-formats |
| #1160 | `9782f640bf51` | not yet reviewed | — |  | gpu(indirect): support constant-prefixed multi-argument ExecuteIndirect signa… |
| #1159 | `121412bec21b` | not yet reviewed | — |  | loader(crash): verify unwind against known caller frames when RtlCaptureStack… |
| #1158 | `abefa49204d5` | not yet reviewed | — |  | GPU: guard zero/overflowing stride in expand_rects and add primitive.rs unit … |
| #1157 | `d67e37dcd97e` | not yet reviewed | — |  | Input: the mouse as the stand-in pad's camera (VICTORY_MOUSE) |
| #1156 | `2f2952693ab1` | not yet reviewed | — |  | Loader: continue past a title's int3 asserts, for finding the next blocker |
| #1155 | `5c375286acb5` | not yet reviewed | — |  | hle(display): fall back to no-op deferral when PowerSetRequest returns ERROR_… |
| #1152 | `e8fd20f85c83` | not yet reviewed | — |  | GPU: no blending on integer render targets in host pipelines (fixes #806) |
| #1149 | `4e35c96234d0` | not yet reviewed | — |  | Shaders: lower v_div_fixup_f32 and v_div_fixup_f16 |
| #1148 | `1661a3e0cfb3` | not yet reviewed | — |  | Shaders: lower image_atomic_inc, _dec, _fcmpswap, _fmin and _fmax |
| #1146 | `47c00d5ea6ef` | integrated | — |  | GPU diagnostics: create missing capture output directories |
| #1146 | `47c00d5ea6ef` | integrated | — |  | GPU diagnostics: create missing capture output directories |
| #1145 | `ea633e723d3c` | not yet reviewed | — |  | Shaders: fix duplicate latch folding, unreachable phi edges, and replace_cons… |
| #1144 | `48781527f53f` | not yet reviewed | — |  | Tests: show when shader differential checks are skipped |
| #1143 | `69a49fa21205` | not yet reviewed | — |  | GPU tests: verify a synthetic frame through PresentX |
| #1142 | `bf873ffd3b30` | not yet reviewed | — |  | Shaders: support saturating clamp on packed 16-bit integer ops (v_pk_{add,sub… |
| #1138 | `2eb4d0380571` | not yet reviewed | — |  | Shader tests: failures name the adapter that ran them |
| #1137 | `7d94b72060b9` | not yet reviewed | — |  | HLE: expand synthetic unit tests for apu.rs (1 -> 19 tests) |
| #1136 | `0dcf6d01adbb` | not yet reviewed | — |  | Shaders: lower v_pk_fmac_f16 (VOP2 0x3c and VOP3 0x13c) |
| #1135 | `af4fef40372e` | not yet reviewed | — |  | GPU: detile only texture subresources affected by writes |
| #1133 | `be0fe04e6782` | not yet reviewed | — |  | Nvidia/heap cap and barriers |
| #1130 | `98f073d0e42a` | not yet reviewed | — |  | Shaders: a descriptor read only as data takes the kind of its entry's… |
| #1129 | `d09c31b4c144` | not yet reviewed | — |  | GPU: give pipeline workers room for NVIDIA's shader compiler |
| #1127 | `a0b441dd1682` | not yet reviewed | — |  | Shaders: 64-bit image atomics lowering to dx.op.atomicBinOp.i64 and a… |
| #1126 | `bb65bc32a98e` | not yet reviewed | — |  | GPU: release descriptor tables after a rejected bind |
| #1125 | `efe102edac86` | not yet reviewed | — |  | Handle unsupported display deferrals in the HLE test |
| #1124 | `b3d8f95cab44` | not yet reviewed | — |  | Shadows: batch GPU copy-back writes safely |
| #1123 | `4e32dfc7866e` | not yet reviewed | — |  | Validate XGameSave blob count before reading |
| #1122 | `9ed6093372d9` | not yet reviewed | — |  | GPU: a write through one view of physical pages opens the guards of all its v… |
| #1114 | `89f3d3c574d1` | not yet reviewed | — |  | CLI: replay selected runtime shader hashes in offline translation |
| #1113 | `f4374838c3dd` | not yet reviewed | — |  | Formats: lay out packed RGB textures as horizontal texel pairs |
| #1112 | `4f5fa479c962` | not yet reviewed | — |  | Shaders: round normalized MTBUF loads without Int64ShaderOps |
| #1111 | `5f8800dd93d4` | not yet reviewed | — |  | fix(loader): preserve protection regions when binding unaligned imports |
| #1110 | `d2ce712926b2` | integrated | `6db9bb08` | in the branch | fix(hle): account for save replacements before clamping quota |
| #1109 | `a32eacdd5e0f` | integrated | `9d7735b8` | in the branch | fix(pipeline): retry collisions when creating cache staging files |
| #1108 | `a0206d1ad34a` | not yet reviewed | — |  | Shaders: split comparison and non-comparison immediate static samplers |
| #1106 | `0a1f602cef1f` | integrated | `e3d1b13e` | in the branch | test(shader): report forced WARP runs as software only |
| #1105 | `6273f06bb7de` | not yet reviewed | — |  | GPU: saturate COM AddRef instead of wrapping its reference count |
| #1104 | `da8875b82080` | not yet reviewed | — |  | Input: trigger rumble via XInputSetStateEx ordinal 100 |
| #1101 | `0b00f3f9ba93` | not yet reviewed | — |  | Revert 2 fix/format support native query |
| #1100 | `b82fb1257775` | not yet reviewed | — |  | GPU: saturate COM AddRef against a u32 wrap, with tests for object.rs |
| #1098 | `44ce06784b9f` | not yet reviewed | — |  | XAsync: canceling a call that already completed leaves it alone, as documented |
| #1097 | `ceac35bb8ce0` | not yet reviewed | — |  | specialize fixed-width runs in texture tiling and detiling |
| #1095 | `d25783b8d87d` | not yet reviewed | — |  | Shaders: compute signed overflow SCC for s_add_i32, s_addk_i32, and s_sub_i32 |
| #1094 | `d03d93ca2dcb` | not yet reviewed | — |  | GPU: cache the error of a container that does not translate |
| #1093 | `ef2f8fa16e00` | not yet reviewed | — |  | GPU: return the output pointer from GetDesc methods |
| #1092 | `499b963f90ac` | not yet reviewed | — |  | Improve texture format handling and GPU resource-state management |
| #1091 | `098a26aad5e3` | integrated | `07f2fb60` | in the branch | README: how to install Rust on Windows (#1087) |
| #1090 | `0b7067fb6deb` | not yet reviewed | — |  | Shaders: lower constant 64-bit integer compares without INT64 |
| #1088 | `59ab6b736af3` | not yet reviewed | — |  | Fix compute shader signed overflow and paired LDS writes |
| #1084 | `48a41fb850ac` | deferred | — | restructures descriptor inference in the contested function | Shaders: a select or carry-in on an untracked mask tests this lane's bit |
| #1083 | `dbea3a10d8f1` | not yet reviewed | — |  | Shaders: name what a scalar memory read is, and which descriptor dword it rea… |
| #1082 | `42961e8e577e` | not yet reviewed | — |  | GPU: log the TDR settings, and warn when a GPU hang would freeze the PC |
| #1081 | `ff2dff8e4ca6` | not yet reviewed | — |  | Shaders: leave loop-count breadcrumbs in a host buffer and name the runaway l… |
| #1080 | `a7847a520197` | integrated | `7abafc7b` | in the branch | HLE: constrain the title before suspending it (window close never suspended) |
| #1077 | `873548f2d39f` | not yet reviewed | — |  | Shaders, GPU: root UAVs (stacked on #489) |
| #1076 | `6a4cc18bf55c` | not yet reviewed | — |  | Fix NVIDIA Ada Lovelace (`RTX 4060`) D3D12 compatibility, 2s TDR submissions,… |
| #1074 | `ec337a025252` | not yet reviewed | — |  | Shaders: a lane mask spilled to a VGPR lane is still that mask |
| #1073 | `986fa6db9dc6` | not yet reviewed | — |  | Shaders: a scalar joining a lane mask and a number lowers in pixel and vertex… |
| #1067 | `6d30fd71c28f` | deferred | — | restructures descriptor inference in the contested function | Shaders: a descriptor only read as data takes its kind from its table |
| #1066 | `dc1aa378d990` | not yet reviewed | — |  | gpu, shader: Intel Arc DG2/Battlemage optimizations and hardware execution fa… |
| #1065 | `37f838d01548` | not yet reviewed | — |  | Shaders: use validator 1.8 for compute derivatives |
| #1064 | `dc134c34c905` | not yet reviewed | — |  | Docs: the GPU path, diagnosing GPU problems, and testing |
| #1059 | `3530ba983ede` | not yet reviewed | — |  | Loader: VICTORY_PATCH writes bytes into a loaded module after relocation |
| #1058 | `8770e1905c55` | integrated | — |  | CI: serialize Windows workspace tests |
| #1056 | `417f4e30d22f` | not yet reviewed | — |  | Shaders: scalar loads from a root view's address read its memory |
| #1055 | `21d1b4682735` | not yet reviewed | — |  | Shaders: scalar buffer loads from a UAV read it raw |
| #1053 | `393320dfa8cd` | integrated | `ecd3e625` | in the branch | Shaders: hardware test for v_cmp_class_f32 |
| #1052 | `41b2472e134c` | not yet reviewed | — |  | Shaders: hardware test for the integer v_min3/v_max3/v_med3 |
| #1050 | `259ba31e28ca` | integrated | `e3622af7` | in the branch | Shaders: negative-zero ordering test for the f16 min/max forms |
| #1048 | `7b57e8e8da92` | integrated | `e80a574e` | in the branch | Shaders: hardware test for s_bfe_u32 |
| #1046 | `5f9a0d247df7` | not yet reviewed | — |  | Shaders: hardware test for the s_cmpk_* scalar comparisons |
| #1045 | `bb3a37bc2516` | not yet reviewed | — |  | Shaders: cover integer three-way ops, f16 zero order, s_bfe_u32 and f32 class |
| #1043 | `f23c49321bb3` | not yet reviewed | — |  | Shaders: 16-bit compares ignore VOP3 op_sel |
| #1042 | `ad330155a5a0` | not yet reviewed | — |  | Shaders: emit canonical system-value semantic names in ISG1/OSG1 signature pa… |
| #1041 | `4111946bb8c9` | not yet reviewed | — |  | GPU: zero graphics/mesh PSO padding bytes and deduplicate pipeline library st… |
| #1040 | `08e4c89757d3` | not yet reviewed | — |  | GPU: drop unsupported UAV/RT flags on typed host textures and guard RTV creat… |
| #1033 | `266f6719266a` | not yet reviewed | — |  | victory-shader: build the per-instruction read/write lists once (ir::build -1… |
| #1032 | `b38a3c9fa1a6` | not yet reviewed | — |  | GPU: cap oversized sampler heaps |
| #1031 | `19b4cc43cc84` | not yet reviewed | — |  | cli: forward and display title_args in victory run |
| #1030 | `b187f7552d09` | rejected | — | adds a zip of game-derived data the asset policy forbids; body describes features absent from the diff | VIctory emulator patches for GTA VI (Xbox Series X Scarlett build 1.0… |
| #1025 | `ece73bb54d17` | not yet reviewed | — |  | GPU, HLE: an announced kernel read stays open until it lands |
| #1024 | `67125b3a891a` | not yet reviewed | — |  | GPU: copy-back writes only the GPU's bytes and marks only unchanged pages cur… |
| #1023 | `836490b9690c` | not yet reviewed | — |  | gpu: implement ordered 32-bit and 64-bit command-list memory waits |
| #1022 | `57e2cdeed86b` | integrated | `ab442f01` | in the branch | Shaders: signature rows of one register in the validator's order |
| #1021 | `3d71ad1a6547` | integrated | `b04607ca` | in the branch | Shaders: s_atc_probe's immediate is not a register |
| #1019 | `461d12adaaf4` | not yet reviewed | — |  | GPU: buffer views off their host copy's elements use a copy of just their byt… |
| #1017 | `b0933f503856` | not yet reviewed | — |  | Fix the Linux build and clippy gates |
| #1016 | `03a6f22906b4` | integrated | `efc14e0a` | in the branch | XMA: gather frame payloads one byte at a time |
| #1015 | `b8cfe8ce7d65` | not yet reviewed | — |  | Shaders: scalar loads through a heap buffer that vector accesses use read a b… |
| #1014 | `9d1c2999324f` | not yet reviewed | — |  | victory-shader: lower v_permlane16_b32 and v_permlanex16_b32 |
| #1013 | `cbbb17ea6962` | not yet reviewed | — |  | speed up shader structurisation and prioritize live pipeline compiles |
| #1012 | `0ba5b998e6df` | integrated | `8a37c05c` | in the branch | Shaders: descriptors joined by nested control flow |
| #1011 | `3b0243325256` | not yet reviewed | — |  | Add Linux GE-Proton workflow |
| #1010 | `79c9b93fb77d` | not yet reviewed | — |  | GPU: copy title memory into host memory without a system call |
| #1009 | `7ea91124f956` | not yet reviewed | — |  | xmem: keep the space below 4 GiB for the title's 32-bit memory |
| #1008 | `7101c9857a44` | not yet reviewed | — |  | Shaders: immediate samplers chosen by control flow |
| #1007 | `9f2177cce6b2` | not yet reviewed | — |  | Apple Silicon: unified experimental GPU and runtime integration |
| #1005 | `72b9709675e5` | not yet reviewed | — |  | Fix 12 verified regressions in shader translation, runtime, caching and textu… |
| #1004 | `cbc8c33dc679` | not yet reviewed | — |  | Shaders: lower image_get_lod on cube textures |
| #1002 | `b6db98554243` | not yet reviewed | — |  | victory-shader: hash the lowering tables with Fx instead of SipHash (lowering… |
| #1000 | `f821cb9d7550` | not yet reviewed | — |  | GPU: pipelines of different names load from the library at the same time |
| #999 | `3ca94561506c` | not yet reviewed | — |  | Shaders: search lane conditions once and redo a lowering once |
| #997 | `7d1a0a585b18` | rejected | — | closes the caller WinHTTP handle where the API keeps it | HLE: prevent automatic WinHTTP redirects |
| #996 | `0b004f2d531f` | not yet reviewed | — |  | victory-shader: lower a16 biased image samples and sub-dword LDS access |
| #995 | `78c8078a859f` | not yet reviewed | — |  | Shaders: use declared structured SRVs for scalar loads and record counts |
| #994 | `993dff8ef8a6` | not yet reviewed | — |  | Linux: run under Proton, and fix the world GPU faults |
| #993 | `64b80c310941` | integrated | `c87e75a3` | in the branch | Formats: share immutable texture swizzle tables |
| #991 | `ab3f3fcd5ee1` | not yet reviewed | — |  | merge overlapping gpu write-back ranges and coalesce watch opens |
| #990 | `8d67fa5b00eb` | not yet reviewed | — |  | Shaders: hardware test for v_cvt_f16_u16 and v_cvt_f16_i16 |
| #988 | `089bd768d5f1` | not yet reviewed | — |  | GPU: say when a fault lies past the end of its title memory copy |
| #986 | `d8d7c2c0e19b` | rejected | — | reads title memory at submit, missing an earlier same-list UAV write | GPU: copy unaligned buffer SRVs into element-aligned scratch memory |
| #985 | `60cf1d69616c` | not yet reviewed | — |  | GPU frame test: tolerate a late wake-up instead of requiring consecutive flips |
| #984 | `501164509d22` | not yet reviewed | — |  | Shaders: split flat byte-offset accesses to structured buffers |
| #983 | `486b54eda05f` | not yet reviewed | — |  | GPU: the early pipeline list keeps the most recently used keys |
| #982 | `b4abe5db60f1` | not yet reviewed | — |  | GPU: pool upload buffers across command list resets |
| #978 | `8d9e4f1205c6` | not yet reviewed | — |  | victory-shader: lower v_cvt_f32_f64 and image_sample_d_cl |
| #976 | `ad22f1fa46fb` | not yet reviewed | — |  | GPU: add check_feature_support unit tests for features 0, 1, 3, 4 (fixes #869) |
| #975 | `78e2d19505bb` | integrated | `22e61383` | in the branch | HLE: add XGSuggestSwizzleMode TEXTURE_1D branch validation (fixes #868) |
| #974 | `c1be75f7b799` | not yet reviewed | — |  | GPU: extend HostHeap wraparound search range to cursor bounds (fixes #871) |
| #973 | `58ee9ad9e864` | not yet reviewed | — |  | Core: fix policy resolve trailing and verbatim dotdot folding (fixes #793) |
| #972 | `886a867661bc` | not yet reviewed | — |  | HLE: enforce XSystemXboxLiveSandboxId minimum buffer size (fixes #862) |
| #970 | `60222b335719` | not yet reviewed | — |  | GPU: aspect-aware concrete format resolution for typeless views (fixes #882, … |
| #969 | `f3457f8350e9` | not yet reviewed | — |  | GPU: load the Agility SDK through ID3D12SDKConfiguration1 (#749) |
| #968 | `34114063dce1` | not yet reviewed | — |  | GPU: load the DirectX 12 Agility SDK, and map the console's packed 16… |
| #967 | `66d97eea7ea3` | integrated | `ee173bad` | in the branch | GPU: choose an adapter that can run the title, and name the ones that cannot |
| #963 | `adeb9ff49dbf` | integrated | `d9495f66` | in the branch | PE: read long-format import descriptors |
| #961 | `31568ced3e44` | not yet reviewed | — |  | Shaders, GPU: draw mesh pipelines without mesh shaders through a compute fall… |
| #960 | `8c12a98be191` | not yet reviewed | — |  | Game picker |
| #959 | `6614cc26b7e4` | held | — | names the GPU from adapter 0 but reads capabilities from the default device | victory: a menu when opened from Explorer, instead of help that closes at once |
| #958 | `7d0302b2d85e` | not yet reviewed | — |  | GPU: unit tests for the OPTIONS, ARCHITECTURE and MULTISAMPLE feature queries |
| #957 | `246ecea9631a` | duplicate | — | upstream xg.rs:697 already asserts the TEXTURE_1D swizzle | HLE: test that XGSuggestSwizzleMode picks linear for a 1D texture |
| #956 | `db86e1e387a6` | not yet reviewed | — |  | GPU: support multi-argument ExecuteIndirect signatures with root constants |
| #955 | `05c681c1b833` | not yet reviewed | — |  | Shaders, GPU: clamp descriptor heap indices to bound host heap guard slot |
| #953 | `c42cbb0737d4` | integrated | `07f604ce` | in the branch | Shaders: handle s_buffer_load SMEM uses in descriptor_data (#798) |
| #952 | `faccc4ef5645` | not yet reviewed | — |  | GPU: log the host's video-memory use and its budget |
| #949 | `db91a486f337` | not yet reviewed | — |  | victory-gpu: segregate single-slot and table dynamic descriptors (#747) |
| #948 | `35802af69d54` | not yet reviewed | — |  | victory-gpu: implement ClearUnorderedAccessViewUint/Float and DiscardResource |
| #947 | `80a235e0368e` | not yet reviewed | — |  | victory-shader: add forced-var propagation and UNSAT memoization to Bools::me… |
| #946 | `c1596049299b` | not yet reviewed | — |  | victory-shader: preserve helper-lane values on pixel waveReadLaneFirst |
| #943 | `12cfd38273db` | not yet reviewed | — |  | GPU: use VirtualQuery region fallback in copy_mapped and write_mapped |
| #941 | `60d2cf9cbcf5` | integrated | `d65d475e` | in the branch | Loader: a clwb in the last bytes of a page is emulated instead of faulting |
| #939 | `d4306f4753fd` | integrated | `4a8b49ec` | in the branch | HLE: XSystemGetXboxLiveSandboxId refuses a buffer under 16 bytes |
| #937 | `52f0a099ede8` | not yet reviewed | — |  | policy: resolve a .. in the not-yet-existing part of a path |
| #934 | `95cefdaca5f4` | integrated | `e28ee842` | in the branch | GPU tests: isolate write-watch fixture from reused addresses |
| #933 | `9914ea4be88a` | not yet reviewed | — |  | GPU: report allocated mips from GetDesc |
| #929 | `2b8756d301c4` | not yet reviewed | — |  | Audio: a silent playback endpoint when the host has no render device |
| #928 | `13185f50ac09` | not yet reviewed | — |  | GPU: GetDeviceRemovedReason reports the host device's state instead of E_NOTI… |
| #927 | `4ce99c162413` | not yet reviewed | — |  | GameInput: held keys are reported in the order they were pressed, as document… |
| #926 | `3cdbc0cabbe9` | not yet reviewed | — |  | Shaders: 16-bit compares read the low halves whatever op_sel says, as RDNA2 d… |
| #925 | `0d2bcd66fcbf` | not yet reviewed | — |  | contribute: depth bounds + waterfall queries + XNetworking (#849 #853 #851) |
| #922 | `f7bce85fcce3` | integrated | `846fcdee` | in the branch | Pipeline: delimit converter id in cache keys |
| #710 | `a5a40bc40675` | not yet reviewed | — |  | Shaders, GPU: read through addresses passed in two root constants (stacked on… |
| #709 | `e50ed36cd9e2` | not yet reviewed | — |  | Shaders: structured access through a root SRV or UAV uses its raw view (stack… |
| #708 | `a9698e7488ab` | integrated | `79381585` | in the branch | fix(config): keep the data directory out of the cache root |
| #707 | `7ca777e55543` | not yet reviewed | — |  | Shaders: lower tbuffer_store_format_* that need no conversion |
| #705 | `ee037b0cc3ae` | not yet reviewed | — |  | Fix/typed image atomic int64 |
| #704 | `1f3c184a36f2` | not yet reviewed | — |  | Shaders, GPU: unknown-stride buffers are read through a raw view of the same … |
| #703 | `712f89096fd7` | deferred | — | 1,000-line BCPack restructure competing with the merged bucket lookup | Formats: eliminate BCPack per-call allocations and optimize streaming decode |
| #702 | `0672d75ffe0c` | not yet reviewed | — |  | gpu: add native DXR 1.1 acceleration structure backend and OPTIONS5 query |
| #699 | `db0257e2c133` | already-present | — | regression already exists at lower.rs:7301 upstream | shader: a phi read only through another phi keeps its missing incoming value |
| #697 | `fa8d0845f7a4` | not yet reviewed | — |  | Shader: backward dead-code elimination in DXIL bitcode emission |
| #695 | `b91ff3c4829d` | integrated | `958693cf` | in the branch | gpu: VICTORY_D3D12_WARP=0 opts out of the software adapter |
| #694 | `efd02d561bdc` | rejected | — | would drop the LOD operand: wrong texels rather than a refusal | Shaders: support image_sample_d_cl and image_gather4_l lowering (#293, #653) |
| #693 | `1c94839f9a15` | deferred | — | reviewed version left shadows in COPY_SOURCE with no transition back | gpu: move shadow buffers to DEFAULT VRAM with pooled staging transfers |
| #692 | `e8ec70cbe59a` | not yet reviewed | — |  | style: cargo fmt across the workspace; fix clippy field-reassign-with-default |
| #691 | `796ace2e632e` | not yet reviewed | — |  | hle: speak accessibility narration through the host's WinRT synthesizer |
| #690 | `a35440366602` | not yet reviewed | — |  | gpu: expand console primitive topologies into host list index buffers |
| #688 | `2e42e8bf492d` | not yet reviewed | — |  | GPU: take back the dynamic descriptor slots of lists that ran when a heap run… |
| #687 | `73eba29bfaea` | not yet reviewed | — |  | hle: track title working-set memory, bound xasync threads, and map XUser devi… |
| #686 | `741d675c9028` | integrated | `aa0401c2` | in the branch | docs: check archive integrity after an early boot breakpoint |
| #684 | `9c2a364dde5d` | integrated | `2e672a6d` | in the branch | Shaders: find the PC test compiler independently of the validator |
| #683 | `81fb96eb74be` | integrated | `db9e8b9b` | in the branch | Shaders: zero unavailable mixed-precision FMA halves |
| #682 | `e1bd18f0e6d6` | not yet reviewed | — |  | Shaders: v_mul_legacy_f32 treats denormal operands as zero |
| #679 | `b96c85d47d7c` | not yet reviewed | — |  | run: refuse capture and dump paths inside the repo |
| #677 | `0284037e045c` | integrated | `de6e76fc` | in the branch | Build: thin LTO for release and diag builds |
| #674 | `b9e5f2e9ff3d` | integrated | `d28b71de` | in the branch | HLE (xg): validate null tiling output, slice bounds, and checked copy spans |
| #673 | `3ba3f79c1952` | integrated | `9a6bc8e0` | in the branch | Make translate failure examples stable across workers |
| #672 | `3a2631333edf` | integrated | `b746d051` | in the branch | CLI: report all varying descriptor-word opcodes in translation refusals |
| #670 | `44bbd6a5e988` | not yet reviewed | — |  | loader: use checked offsets and sizes in image preparation |
| #669 | `52d528ba0c02` | integrated | `f6aa6695` | in the branch | GPU: find a list in the DRED breadcrumbs by its name, not only its pointer |
| #668 | `5e9d60d134e2` | integrated | `3ad7f8ef` | in the branch | Shaders: support f16 omod and saturate/apply source modifiers in v_cvt_{u16,i… |
| #667 | `995f7f3ee720` | deferred | — | spans nine files; depends on descriptor-tracking shape changed by #672 | Shaders: an address in two root constants is a root view (#614) |
| #665 | `e950e8175634` | integrated | `7fc9f665` | in the branch | xtask: count \\x escapes and 0x hex arrays as encoded runs in the policy check |
| #664 | `988fd9a64718` | integrated | `d350b132` | in the branch | translate: refuse a VICTORY_EMIT_DUMP directory inside the repo |
| #663 | `d4dcd0b63ad1` | integrated | `3c1aca6c` | in the branch | media: apply the local-file rule to the source reader, sink writer and MFCrea… |
| #662 | `37d52cb78ddc` | not yet reviewed | — |  | DSP: refuse XDspDisconnect with active streams; XDSP_E_NO_DEVICE_RESOURCES pa… |
| #661 | `6325df752c56` | integrated | `d4426690` | in the branch | Shaders: fix missing and phantom operands in SP3 text |
| #660 | `19b02ece91ef` | not yet reviewed | — |  | Shaders: drop a wave64 mask whose low dword is seen with two high dwords |
| #659 | `98fe8fe840ba` | integrated | `2ece2f86` | in the branch | Shaders: refuse per-lane reads of mesh v5 in v5 writes and after joins |
| #658 | `0ecaa8cb92ac` | integrated | `7d5fa089` | in the branch | BCPack: read BC7 dictionary indices from the right offset |
| #656 | `3b001ed80c97` | integrated | `39a46807` | in the branch | Input: report the character each held key types |
| #655 | `1bd52dcc9e4d` | not yet reviewed | — |  | Shaders: guard conditional exports |
| #652 | `e3448dc4e976` | integrated | `6a836d63` | in the branch | GPU: use selected mip extents for MSAA resolves |
| #651 | `d615ee0fe0f9` | integrated | `ce478cfb` | in the branch | GPU: select the readable format and plane for depth/stencil shader views |
| #650 | `06657a8bebe8` | not yet reviewed | — |  | Shaders: bound root view accesses by the size the host binds |
| #649 | `43e2ca9de29f` | integrated | `d0952126` | in the branch | Show on-disk file size in victory inspect |
| #648 | `241b08f49439` | integrated | `d0462736` | in the branch | HLE: implement XUser maximum-users slot 0x30 (#348, #622) |
| #646 | `12eba495039d` | integrated | `42f24d56` | in the branch | Shaders: fix whole-wave EXEC inline constant sign-extension and prefix lane v… |
| #644 | `4eb145be7089` | integrated | `b33b0fed` | in the branch | GPU: spill oversized host root signatures' user data to root CBVs |
| #642 | `bdef690c7054` | integrated | `f5f97470` | in the branch | GPU: translate enhanced texture barriers in ID3D12GraphicsCommandList7::Barri… |
| #641 | `f96c56fb693b` | integrated | `15907832` | in the branch | GPU: upload partially-resident streamed textures |
| #640 | `446fef929c85` | integrated | `842ed08b` | in the branch | Retry failed pipeline cache saves |
| #638 | `16d1aad4d1a9` | not yet reviewed | — |  | GPU diagnostics: find what the world hang waits for, and look under a black f… |
| #636 | `7216ab711311` | integrated | `3412fd91` | in the branch | HLE: implement the RPC runtime's GUID calls |
| #633 | `90bc07545837` | not yet reviewed | — |  | HLE: GetModuleFileNameA answers in the ANSI code page |
| #632 | `3bd823210d3e` | not yet reviewed | — |  | Shaders: LDS reads and writes of a byte or half |
| #631 | `e009e13aa479` | integrated | `a157f3fe` | in the branch | Add unit tests for console.rs |
| #629 | `350f91c23a94` | integrated | `25e797e9` | in the branch | Reject multisampled linear texture layouts |
| #628 | `717e732f68c5` | not yet reviewed | — |  | Shaders: lower ds_min_f32 / ds_max_f32 (and _rtn) with RDNA2's NaN rules |
| #627 | `7dac1ae36b51` | not yet reviewed | — |  | feat(victory-shader): implement v_swaprel_b32, v_movrelsd_2_b32, and s_movrel… |
| #626 | `df9ab3c69172` | not yet reviewed | — |  | GPU: fall back to standard root signature flags when host runtime rejects 0xC… |
| #624 | `a548b62da776` | not yet reviewed | — |  | DirectStorage: report queue read failures |
| #623 | `7c01e3d05c4f` | not yet reviewed | — |  | Fix Windows 10 Game-Init crash and root-signature fallback |
| #621 | `7e5c7c8d516e` | not yet reviewed | — |  | Shaders: read explicit packed input padding as zero |
| #620 | `59aa44b0e0de` | integrated | `cfd019c2` | in the branch | HLE input: pass trigger motor levels to the host source |
| #618 | `691236d606b9` | integrated | `9c9e46f6` | in the branch | GPU: align appended input-element offsets like D3D12 |
| #617 | `81b81df35143` | integrated | `8042c6b7` | in the branch | Shaders: reuse lane guards for unchanged EXEC within a block |
| #616 | `e930351bebd6` | integrated | `1af23c07` | in the branch | GPU: rectangle draws check their bounds without 32-bit overflow |
| #615 | `a4974e4c21c1` | integrated | `bf57211f` | in the branch | GPU: VICTORY_* switches set to 0 are off (#610) |
| #613 | `7da55befe681` | not yet reviewed | — |  | HLE: serialize device callbacks across delivery threads |
| #609 | `6d8c72bc04ee` | not yet reviewed | — |  | GPU: retain and retire released host textures, staging, shadow buffers, and d… |
| #608 | `cc6b6984a700` | rejected | — | does not compile and deletes two existing tests | feat(victory-shader): implement s_movrels_b32 and s_movreld_b32 lowering |
| #606 | `95b58c1351de` | not yet reviewed | — |  | Shaders: ds_swizzle_b32 offsets from 0xC000 are the rotate and FFT modes |
| #605 | `e7f5738163df` | not yet reviewed | — |  | Shaders: v_cvt_rpi_i32_f32 rounds the exact sum; v_cvt_flr/rpi_i32_f32 satura… |
| #603 | `705a2f634d1c` | integrated | `dae0a292` | in the branch | Formats: inline repeated micro-tile copies when detiling textures |
| #602 | `380dab26ccbc` | not yet reviewed | — |  | GPU diagnostics: report retained copy memory |
| #601 | `30b962565ff2` | not yet reviewed | — |  | GPU: bound texture upload and capture slices to the allocated buffer |
| #599 | `38234626c7c1` | not yet reviewed | — |  | Shaders: lane-indexed ops on a wide host wave use the own console wave |
| #596 | `be92188eee0a` | not yet reviewed | — |  | Shaders: lower v_cvt_pknorm_{i16,u16}_{f32,f16} with exact rounding |
| #595 | `dfac3b46f19b` | not yet reviewed | — |  | GPU: run submissions asynchronously on a host fence timeline |
| #594 | `c3f1d7443d5f` | not yet reviewed | — |  | GPU: track texture subresource states |
| #592 | `35bf1530542f` | not yet reviewed | — |  | Shaders: second rows of packed 16-bit pixel inputs |
| #590 | `52ceb8f7173d` | not yet reviewed | — |  | HLE: reject duplicate blob changes in one save update |
| #589 | `ab2e7cc3f648` | integrated | `005d6219` | in the branch | fix(shader): skip unused image atomic descriptor reconstruction |
| #588 | `54c7c89e6f1c` | not yet reviewed | — |  | GPU: transition shadow buffers between UAV and vertex/index/CBV states |
| #587 | `1052a90a4072` | not yet reviewed | — |  | Shaders: guard mesh output bounds, init all primitives, and omit sync on dive… |
| #586 | `9b5e23997216` | not yet reviewed | — |  | Shaders: honor the mesh allocation request counts |
| #585 | `f800f1ca9cc3` | not yet reviewed | — |  | Shaders: lower untracked v_cndmask_b32/carry-in masks via lane_bit and descri… |
| #584 | `0bcf2df27621` | not yet reviewed | — |  | Shaders: clamp ResourceDescriptorHeap and SamplerDescriptorHeap indices to ho… |
| #583 | `07bae5d505f2` | not yet reviewed | — |  | fix(gpu): respect indirect count buffer, guard budget overflow, and bind null… |
| #582 | `7e78c81aaacb` | not yet reviewed | — |  | feat(gpu): wire up depth bounds testing on command lists and pipeline streams |
| #581 | `0a936b63e633` | not yet reviewed | — |  | GPU: typed null descriptors for empty table slots, depth-UAV scratch stand-in… |
| #580 | `7ca84cc13670` | integrated | `2db7897e` | in the branch | Docs: add a contributor onboarding guide |
| #579 | `1658c8452a4a` | integrated | `13008f98` | in the branch | GPU: VICTORY_SKIP_WAVE64 diagnostics switch for wave64 programs |
| #578 | `fb7facde2d44` | not yet reviewed | — |  | GPU: refuse wave64 mesh and compute programs on hosts without 64-lane waves |
| #577 | `40816378b598` | integrated | `e532ec98` | in the branch | dstorage: retain queued file references and validate null out pointers |
| #576 | `fde38b399f96` | not yet reviewed | — |  | shader: lower v_permlane16_b32 |
| #575 | `69e44f59cd9c` | rejected | — | does not compile: Mutex<Vec<ID3D12Resource>> is not Send | GPU: release resources the CPU is done with behind the queue |
| #574 | `253b9db4a8f3` | not yet reviewed | — |  | HLE: write at-rest gamepad state for non-pad readings |
| #573 | `1d09f19b2102` | not yet reviewed | — |  | Shaders: lower ds_bpermute_b32 |
| #572 | `0a1a702bb440` | integrated | `34370fce` | in the branch | Docs: add a PowerShell quickstart for running VIctory |
| #571 | `8ad9d9f75ec8` | not yet reviewed | — |  | Shaders: LDS byte and half accesses |
| #570 | `906ebe7b27e8` | not yet reviewed | — |  | HLE: reject saves whose committed container lacks a manifest |
| #569 | `aed9637927e7` | integrated | `0860ed7a` | in the branch | Shaders: biased samples with 16-bit coordinates |
| #563 | `10ac6ceac928` | integrated | `f90c170e` | in the branch | Shaders: cube samples with gradients, and image_sample_d_cl |
| #562 | `9ca05334eded` | not yet reviewed | — |  | GPU: reuse immutable descriptor tables across command lists |
| #561 | `3c02d8cf8947` | not yet reviewed | — |  | xmem: commit physical pages in pieces, and report the real count |
| #560 | `1aac20aa8fd9` | integrated | `fb3c0578` | in the branch | Loader: stage prepared images in per-writer files |
| #559 | `15eafe275313` | not yet reviewed | — |  | GPU: track mip states for 2D color targets |
| #558 | `fa83a31dfaab` | not yet reviewed | — |  | CLI: check every extension the title uses unchecked, not only AVX2 (depends o… |
| #555 | `d0dd9fc5c01c` | not yet reviewed | — |  | PE: keep every name for a shared export ordinal |
| #553 | `b796c8b8b80c` | integrated | `5f095aad` | in the branch | GPU: clear DXGI outputs on out-of-range enumeration |
| #551 | `967051859c6a` | not yet reviewed | — |  | Shaders: lower compute group info, wave id in workgroup, and sample coverage |
| #549 | `acc78da3c11c` | not yet reviewed | — |  | CI: add Dependabot configuration for GitHub Actions and Cargo |
| #547 | `19cf287d0d9c` | integrated | `a8b8e88d` | in the branch | QA: add rustfmt.toml to standardize workspace formatting |
| #546 | `a8bf0d3aa6ce` | not yet reviewed | — |  | GPU: prioritize pipeline work requested by draws |
| #545 | `d4c0e846b731` | rejected | — | a second divergent lint gate on stable | xtask: add qa command to run policy, rustfmt, and clippy checks |
| #544 | `f861613328da` | not yet reviewed | — |  | HLE: report queue termination after running callbacks |
| #543 | `c0d10021683b` | integrated | `95ee7d24` | in the branch | QA: add pull request template with asset policy and safety checklist |
| #542 | `31a4692a1581` | not yet reviewed | — |  | Shaders: keep buffer offsets below 2 GiB (NVIDIA + vkd3d-proton world device … |
| #541 | `cbd84266d515` | not yet reviewed | — |  | GPU: retain D3D12 object names in private data |
| #540 | `6b0a4123a253` | rejected | — | a second divergent lint gate on stable | CI: add asset-policy and formatting verification workflow |
| #539 | `404dbd97b6df` | not yet reviewed | — |  | Shaders: 64-bit image atomics |
| #538 | `65062e133c75` | integrated | `4efb34b6` | in the branch | xmem: report allocations the host refuses |
| #537 | `2021e0f26970` | integrated | `67f5d656` | in the branch | GPU: name the switch when a removal has no device-removed data |
| #536 | `7a5d85e6a286` | not yet reviewed | — |  | Audio: an XMA write block past the ring's end wraps before the first write |
| #535 | `f6585cc0b392` | not yet reviewed | — |  | App: enforce asset policy check in default_paths |
| #534 | `29046af7f589` | not yet reviewed | — |  | CLI: report shader scan input failures |
| #532 | `aa50c7d6481e` | integrated | `f4d0697a` | in the branch | Document the Arch MSVC build and Proton rendering on an RTX 4080 |
| #531 | `26912a953541` | not yet reviewed | — |  | HLE: check null and invalid handle in display deferral |
| #530 | `951ceb1eb8da` | integrated | `2f0a1740` | in the branch | GPU diagnostics: show one of the title's render targets in the window |
| #529 | `609c74fbd59a` | not yet reviewed | — |  | PE: preallocate symbol_key in CodeViewInfo |
| #528 | `33495ff068e0` | not yet reviewed | — |  | HLE: validate null pointer in ApuIsVirtualAddressValid and overflow in ApuAll… |
| #526 | `d46727fe79d9` | not yet reviewed | — |  | Linux: a native runtime, no Wine |
| #524 | `53282a65ad88` | not yet reviewed | — |  | Runtime: declare title-facing functions extern "win64" |
| #523 | `001226617527` | not yet reviewed | — |  | Build: compile and test the workspace on Linux again |
| #522 | `882d4f0f5fbe` | not yet reviewed | — |  | HLE: validate null package identifier in XPackage methods |
| #521 | `75b1b8ff6550` | not yet reviewed | — |  | Shaders: model the compute thread-group info SGPR's wave count |
| #519 | `53f77dd92a50` | not yet reviewed | — |  | HLE: validate pointers in XGameUi text entry result and size getters |
| #517 | `bab15db5c7a1` | not yet reviewed | — |  | CLI: report incomplete shader directory scans |
| #515 | `d9cb9085689f` | rejected | — | new tests reuse probe addresses against a process-global journal | GPU: ask for written pages allocation by allocation when a range spans several |
| #514 | `8540a0ec3423` | not yet reviewed | — |  | fix(gpu): recheck uncommitted blocks before skipping writes |
| #513 | `13aac0844da2` | obsolete | — | run::run() no longer takes title_args; the target block does not exist | App: forward title command-line arguments to run command |
| #512 | `829f774722a6` | not yet reviewed | — |  | HLE: resolve ordinals on pseudo-module handles |
| #511 | `ff96ffe9644c` | not yet reviewed | — |  | Shaders: coalesce aligned constant-buffer loads |
| #509 | `bfc60cfab5fb` | not yet reviewed | — |  | GPU: add an opt-in execution performance census |
| #506 | `e80778cffa94` | not yet reviewed | — |  | CLI: reject zero shader translation workers |
| #505 | `d4c8abf932a1` | integrated | `b9f70206` | in the branch | Tests: add synthetic Xbox runtime conformance under Wine |
| #504 | `b95632012574` | rejected | — | reports a D24S8 depth plane as R32_TYPELESS | GPU: handle depth/stencil copy footprints |
| #503 | `43710396895e` | not yet reviewed | — |  | HLE: pin a call's task queue at XAsyncBegin |
| #502 | `c2609ea3f9b7` | not yet reviewed | — |  | fmt: Format all files |
| #501 | `006eafcaf8a1` | not yet reviewed | — |  | gpu: probe a CPU-visible placed-buffer arena |
| #500 | `bdbaac636524` | integrated | `a5ccbffd` | in the branch | GPU diagnostics: pick capture frames by draw count |
| #499 | `5c211d04035c` | not yet reviewed | — |  | shader: lower v_sat_pk_u8_i16 |
| #498 | `4f48c59608ca` | not yet reviewed | — |  | shader: lower v_movrelsd_2_b32 and v_swaprel_b32 |
| #497 | `92e1d4d5f560` | not yet reviewed | — |  | GPU: keep what shaders may write by page, in reused page buffers |
| #494 | `0a96e1e09068` | integrated | `d5a3f66e` | in the branch | docs: add Indonesian translation (languages/README.id.md) |
| #493 | `c48b240addc0` | rejected | — | drops draws when no view heap is bound, with no test | GPU: bind root SRVs as one-view tables bounded to their host copy |
| #492 | `c7bb83fd37fa` | not yet reviewed | — |  | GPU: substitute R9G9B9E5_SHAREDEXP render targets and UAVs with R16G16B16A16_… |
| #491 | `027926b162ac` | not yet reviewed | — |  | GPU: remember pipelines whose creation crashed the process (VICTORY_PIPELINE_… |
| #490 | `1d46278d70c3` | not yet reviewed | — |  | GPU: ask whether guarded memory is a view once per block, not per asking |
| #489 | `fe7210ebbf7a` | rejected | — | assumes spill dword 0 is user SGPR 16 without checking | Shaders: spill-table loads read the arguments as they were on entry (stacked … |
| #488 | `fba33207c4ff` | integrated | `c8de2412` | in the branch | GPU: skip zero-sized frame captures |
| #487 | `ca362a42473b` | not yet reviewed | — |  | HLE: read manifest fields past inline XML comments |
| #486 | `6179b10dedff` | rejected | — | emits an export when its copy failed | shader: opt the test binaries into an app-local Agility SDK runtime |
| #485 | `d454508f71df` | not yet reviewed | — |  | GPU: preserve compatible read states for read-only depth targets |
| #484 | `3b7e1d5f950c` | not yet reviewed | — |  | Diagnostics/process memory snapshot |
| #483 | `65a161d3c83e` | not yet reviewed | — |  | Shaders, GPU: a heap index computed from data stays inside the heap |
| #482 | `c91e6b6b20e3` | integrated | `42c80756` | in the branch | App: reject wrapping RVAs in VICTORY_WATCH |
| #481 | `3f9e688eefd4` | rejected | — | deletes ci.yml and the --all-features clippy run | CI: Add GitHub Actions workflow |
| #480 | `75fe6c02b278` | integrated | `06daf0f7` | in the branch | HLE: require two hex digits in percent escapes |
| #478 | `7ac2760d317c` | integrated | `93324593` | in the branch | Audio capture: guard WASAPI timestamp errors/underflow, null 0-frame slice UB… |
| #477 | `bd59ba346da7` | integrated | `c26bcc20` | in the branch | Shaders: fix DXIL validator UB on null error blobs, unchecked GetStatus, and … |
| #476 | `a3f89eda5476` | integrated | `abfa4d3f` | in the branch | Census: deduplicate PE imports without hint |
| #475 | `2c22e2b2d23e` | integrated | `aa801cbb` | in the branch | XStore: wire offline *Result slots and fix OOB XStoreShowProductPageUIResult … |
| #474 | `e34cb5396aaa` | rejected | — | admits negative LOD bounds D3D12 forbids | GPU: sanitize CreateSamplerX LOD range, bias clamp, and invalid enum values |
| #472 | `f200a2a101e2` | not yet reviewed | — |  | GPU: grow the write snapshot in place, and cap idle pool memory |
| #470 | `e9f870735ad8` | not yet reviewed | — |  | shader: lower wave prefixes of min and max |
| #469 | `c47c963c4202` | not yet reviewed | — |  | GPU: give depth targets their unordered access through a twin texture |
| #465 | `bdced0f38f92` | not yet reviewed | — |  | Shaders: lower v_dot2_f32_f16 and v_dot2c_f32_f16 |
| #464 | `9dd1507872c6` | not yet reviewed | — |  | GPU: remember unmapped guarded view blocks instead of re-copying them every s… |
| #463 | `f47a36e83f86` | not yet reviewed | — |  | GPU: load an app-local D3D12 Agility SDK runtime when present |
| #461 | `020a79008b23` | not yet reviewed | — |  | GPU: keep the initial pipeline of new command lists |
| #460 | `8ce597b2beea` | integrated | `66a3c2c1` | in the branch | Metal: add native Apple Silicon GPU conformance harness |
| #458 | `360f38256dea` | not yet reviewed | — |  | Fix GPU hangs, memory leak, and mesh shader exports; render intro cre… |
| #457 | `08b681c2df8f` | not yet reviewed | — |  | fix(shader): accept overwritten VGPR inputs in counted indexed writes |
| #456 | `4b8a1ca6dd0d` | integrated | `35a3b2de` | in the branch | shader: raise v_interp_p1lv_f16 |
| #455 | `eb8067f2eae8` | not yet reviewed | — |  | HLE: honor the requested executable on relaunch |
| #454 | `85c04d129a0a` | rejected | — | patches code with no bounds check | Loader: opt-in VICTORY_NO_WATCHDOG hook for the title's watchdog (build 1.0.2… |
| #453 | `88b34a447350` | not yet reviewed | — |  | Shaders: pixel barycentrics through SV_Barycentrics (values and computed inte… |
| #452 | `5bcacf5cfdce` | not yet reviewed | — |  | Shaders: spin-wait loops stop after 4096 iterations; loop limit in release bu… |
| #451 | `8ff871b71c5a` | integrated | `e969a76c` | in the branch | HLE: keep fitted window dimensions positive |
| #450 | `11da59948592` | not yet reviewed | — |  | GPU: preserve typed null render target slots |
| #449 | `f625cc508c0d` | integrated | `c64f8aab` | in the branch | HLE: refuse a null AnalyticsInfo output pointer |
| #447 | `7d049431de9b` | integrated | `5a24976b` | in the branch | Shaders: a signed 64-bit operand sign-extends its 32-bit literal |
| #446 | `881caef98fa0` | integrated | `efff47d2` | in the branch | HLE: report required size on string getter buffer errors |
| #443 | `54f4329932b1` | not yet reviewed | — |  | Audio: keep XMA frames split across rolling inputs |
| #441 | `157e0ceea175` | not yet reviewed | — |  | run: let victory.toml pick the console |
| #439 | `28aca3b718f7` | not yet reviewed | — |  | Shaders: one console wave sharing LDS runs in the console flow on 32-lane hos… |
| #438 | `98e01ff90e15` | not yet reviewed | — |  | CI: consolidate Linux and native Apple Silicon offline validation |
| #437 | `7a29f5239145` | not yet reviewed | — |  | Shaders: preserve raw scalar buffer loads on NVIDIA |
| #436 | `e914ed456f97` | not yet reviewed | — |  | GPU: copy a shadow's edge pages whole so they are not copied again |
| #435 | `657efd827942` | not yet reviewed | — |  | GPU: coalesce identical write declarations within a submission |
| #434 | `48f92e23c152` | not yet reviewed | — |  | Shaders: a wave32 program's mask read picks its half of a 64-lane ballot |
| #431 | `23ced703f072` | not yet reviewed | — |  | GPU: free host textures only after running submissions, and make them on firs… |
| #430 | `129d2897c43f` | integrated | `b69d6856` | in the branch | DirectStorage: Close completes what was submitted |
| #428 | `a312333c28eb` | integrated | `ca8d2253` | in the branch | GameInput: next and previous readings of a disconnected device fail |
| #427 | `16d395ccc097` | not yet reviewed | — |  | GPU: say in device-removed reports which user data is a table |
| #426 | `d31cb6b55f12` | not yet reviewed | — |  | Shaders: lower native 16-bit buffer stores |
| #425 | `012ce71c5a8c` | integrated | `2c9c3325` | in the branch | Game: validate DXBC containers in SGD5 records |
| #424 | `5b9a9b37ef26` | integrated | `a751c8a3` | in the branch | README: quote the executable path in the PowerShell launch example |
| #422 | `a746d533f407` | integrated | `f7d2faf8` | in the branch | Shaders: narrow f32 to f16 to nearest even, as the console does |
| #421 | `83b2037e40ea` | not yet reviewed | — |  | HLE: clean dot parts in extended virtual-drive paths |
| #420 | `b4b9498abc00` | not yet reviewed | — |  | GPU: include root-selected entries in rebuilt descriptor tables |
| #416 | `16d21661cf55` | not yet reviewed | — |  | shader: implement v_cvt_f32_f64 instruction lowering |
| #415 | `6924b095e3bf` | integrated | `e9aecbd8` | in the branch | GPU: use native root type codes in the shader cache |
| #413 | `42c460aecc2f` | not yet reviewed | — |  | GPU: materialize bindless ResourceDescriptorHeap views, refreshing only chang… |
| #412 | `d2c7d4a329ab` | integrated | `1cbf4b9c` | in the branch | HLE: validate XPackage chunk selectors |
| #411 | `16a5893ea7fb` | not yet reviewed | — |  | GPU: require only the descriptor heaps used by the pipeline |
| #409 | `340499c44c7f` | not yet reviewed | — |  | GPU: match D3D12 copyable texture footprints |
| #404 | `d505b0e4c5e3` | not yet reviewed | — |  | shader: lower v_permlanex16_b32 |
| #402 | `022e9c82f0ee` | not yet reviewed | — |  | GPU: find a buffer holding an address past many smaller ones; reuse loose win… |
| #401 | `0b5838d4d271` | not yet reviewed | — |  | GPU: reduce work in texture refresh and readback |
| #398 | `22cc74ef4cad` | not yet reviewed | — |  | HLE: report the single supported offline user |
| #395 | `8c3e0d4ca82b` | not yet reviewed | — |  | shader: unpack compressed f16 pixel exports |
| #393 | `6e7186c09b15` | not yet reviewed | — |  | GPU: refresh a buffer copy's ranges by page; answer clean guarded views from … |
| #391 | `a3f219ecbdb7` | not yet reviewed | — |  | Shaders: refuse LDS past the host's group limit by name |
| #390 | `e125477cbd6d` | integrated | `589ea43d` | in the branch | Crash log: a panic carries our Rust stack |
| #389 | `c84e0e60bfc1` | not yet reviewed | — |  | Shader cache: a cache that cannot write still translates |
| #388 | `e54b6756f5b2` | not yet reviewed | — |  | GPU: refresh unwatched textures without a region query; detile into cached me… |
| #387 | `0899b7d7fd97` | not yet reviewed | — |  | GPU: preserve execution ordering for buffer copies |
| #385 | `ec086dca9069` | not yet reviewed | — |  | Shaders: track every incoming edge of a lane-mask phi |
| #383 | `b6af8b177e46` | not yet reviewed | — |  | Shader: LDS bytes and halves, constant sample offsets, descriptor-data reasons |
| #379 | `5ecb6abe769d` | not yet reviewed | — |  | Shaders: lower v_div_fixup_f16 |
| #378 | `2392dfd79a19` | integrated | `9a68bad9` | in the branch | CLI: include runtime shader hashes in the translation listing |
| #374 | `9d6bc0c7c681` | not yet reviewed | — |  | XAsync: a wait on a failed call during its cleanup waits for the status |
| #373 | `c000581df2cf` | integrated | `3fdb9879` | in the branch | Kernel: GetModuleFileName of a title image into a null buffer |
| #370 | `936e5e51c91c` | integrated | `099c2f65` | in the branch | XGameUi: a text entry's limit is the console's |
| #369 | `341782191c55` | not yet reviewed | — |  | Shaders: eliminate heap allocations during CFG traversal |
| #368 | `f47ebd83f314` | not yet reviewed | — |  | Shaders: lower buffer cache invalidations as no-ops |
| #364 | `e503756597ca` | integrated | `6de0fa33` | in the branch | XMA: accelerate VLC decoding with an eight-bit prefix lookup |
| #363 | `fd44561ea118` | integrated | `4dbc5a71` | in the branch | Config: ignore an empty VICTORY_CONFIG |
| #362 | `1e33e8838ca6` | not yet reviewed | — |  | Shader tests: use the Agility SDK runtime on Windows 10 (stacked on #175) |
| #361 | `3f5eff576032` | integrated | `e9ca0d50` | in the branch | Input: with no device, previous and next readings walk every device in time o… |
| #359 | `d549b6397e3f` | integrated | `e30d5d3b` | in the branch | HLE: a '>' in a quoted manifest attribute does not end its tag |
| #358 | `369dc3246931` | not yet reviewed | — |  | GPU: a shader cache that cannot keep an entry still gives its translation |
| #357 | `c6959acba383` | integrated | `83bd896d` | in the branch | HLE: load a host DLL outside the module cache's lock (loader-lock deadlock) |
| #354 | `9f28f6b9845b` | integrated | `f2f21ab5` | in the branch | docs: explain Series S render profile and memory limits |
| #353 | `e6ebec9ccf0b` | integrated | `086a3126` | in the branch | Shader info: show stage I/O even when instruction decoding fails |
| #352 | `f77bd28c6f4f` | not yet reviewed | — |  | GPU: mesh pipelines load from the pipeline library (name their topology) |
| #351 | `93a24b7cdaeb` | integrated | `e4b302b1` | in the branch | Formats: reject overlapping texture depth slices |
| #347 | `197c7acf1fb0` | not yet reviewed | — |  | GPU: ask for an empty pipeline library with a null blob |
| #345 | `e16140174dda` | integrated | `90a5dd2d` | in the branch | docs: add Brazilian Portuguese translation |
| #343 | `9459c12d2e3a` | integrated | `fd228fb5` | in the branch | Shaders: reject overlapping DXBC container parts |
| #342 | `4950d6171111` | not yet reviewed | — |  | Shaders: lower partial-channel typed buffer stores |
| #337 | `5cf259e5fe2f` | integrated | `feeff994` | in the branch | fix(hle): saturate storage quota conversion |
| #335 | `e3bef089855c` | integrated | `22309608` | in the branch | XStore: a store id without an installed package is an invalid argument |
| #334 | `c99e4290b628` | not yet reviewed | — |  | GPU: transition host copies of title memory between shader writes and reads |
| #333 | `a23184d50b67` | not yet reviewed | — |  | Shaders: keep native root parameters inside the SGPRs, clamp the thread-id co… |
| #332 | `5048d2d13f81` | not yet reviewed | — |  | Shaders: lower partial-channel image_store |
| #331 | `ccdecb808078` | not yet reviewed | — |  | GPU: sRGB and linear views of one texture read with their own curve |
| #329 | `02564c5d9561` | not yet reviewed | — |  | Shaders: recover record counts for descriptor-only structured buffers |
| #328 | `91e7742f9852` | integrated | `64d7d016` | in the branch | Shaders: lower MSAA image_load and image_get_resinfo |
| #326 | `c8d72832706c` | not yet reviewed | — |  | Shaders: DXBC/DXIL parser and sampler/SP3 robustness fixes |
| #324 | `c37793a059e3` | integrated | `f9eb3bf0` | in the branch | fix(pe): detect incomplete TLS callback arrays |
| #323 | `e08ac6a9dc6b` | integrated | `f71bd69d` | in the branch | docs: debugging notes from a GTA V port |
| #322 | `b58d198d954e` | integrated | `894b7462` | in the branch | GPU: copy into title memory without asking for its region first |
| #321 | `d674d9e2bbaa` | not yet reviewed | — |  | GPU: honour scheduled frame-event offsets |
| #320 | `41369c90250d` | not yet reviewed | — |  | Shaders: lane reads and writes where lanes branch differently run in the cons… |
| #319 | `b36bd4c1d655` | integrated | `f9800117` | in the branch | GPU: end a late flip wait on the tick it missed, not the next |
| #318 | `1d7bf9b0bdf3` | not yet reviewed | — |  | Shader: pack a16 image addresses as the hardware does; image_sample_d_cl |
| #317 | `8133754ea5af` | integrated | `f9b5d3b9` | in the branch | Shaders: support cube and MSAA dimensions in descriptor_data |
| #315 | `e93b803d2df8` | not yet reviewed | — |  | Formats: console formats 189/190 have texture layouts; rustdoc builds without… |
| #312 | `191e335d7848` | deferred | — | held as deferred with concrete review findings | HLE: SDL3 gamepad backend for the gameinput shim |
| #311 | `1251a24b78ee` | not yet reviewed | — |  | test(pe): cover header layout offsets |
| #310 | `1ea6427f0507` | integrated | `6d897c25` | in the branch | Policy: resolve cache paths inside Windows AppContainer |
| #309 | `64c7188a17f9` | not yet reviewed | — |  | Shaders: forward SGPRs reloaded from VGPR lanes as the spilled value |
| #308 | `6b303847ed87` | not yet reviewed | — |  | Shaders: follow lane writes and scalar steps in whole-wave sections |
| #306 | `e3067aeaa7d2` | not yet reviewed | — |  | Shader: fix biased 16-bit image samples and DXIL validator failures |
| #305 | `78dae8362697` | not yet reviewed | — |  | Shaders: lower 64-bit image atomics |
| #302 | `815178dcc01b` | not yet reviewed | — |  | Shaders: read root arguments the console spills to memory |
| #301 | `1af462451537` | not yet reviewed | — |  | GPU: non-stream graphics PSOs, stream subobjects 25..=28, texture-to-buffer r… |
| #299 | `63dd56433c68` | not yet reviewed | — |  | Docs: correct rumble status and the HDR note |
| #298 | `54f97dbb31b6` | not yet reviewed | — |  | Shaders: lane masks spilled to VGPR lanes and read back |
| #294 | `a4e0feb15080` | not yet reviewed | — |  | GPU & shader: full Root SRV extent refresh, safe fallback VAs, and Root UAVs |
| #292 | `e065026b6ed8` | not yet reviewed | — |  | Shader: lower WQM QuadAny, clamp LDS accesses, and guard loop budgets & barri… |
| #290 | `ae6ebd9ec270` | not yet reviewed | — |  | HLE tests: cover Windows path rules on every host |
| #288 | `f177aaeade9f` | integrated | `2f2fbe49` | in the branch | Shaders: cap BDD memo table and bail out early on overflow |
| #286 | `6afdb58eeb87` | integrated | `869cb862` | in the branch | Shaders: accept known D3D_MIN_PRECISION signature elements |
| #284 | `fa5189da66a9` | not yet reviewed | — |  | Shaders: RDNA2's legacy multiply-adds are the fused ones |
| #282 | `4d3d003c3c73` | not yet reviewed | — |  | GPU: run indirect rectangle-list draws (#255) |
| #281 | `d9bbff571eab` | not yet reviewed | — |  | feat(shader): add opt-in perspective-center barycentric inputs |
| #280 | `7ecc6209b8b3` | integrated | `25c43bc2` | in the branch | README: a hard drive keeps the first screen black for ~90 s |
| #279 | `02f0e29f8031` | not yet reviewed | — |  | GPU: name the commands at a DRED stop, and dump translated shaders |
| #278 | `d063cee543a2` | not yet reviewed | — |  | GPU: answer format-info and virtual-address feature queries |
| #275 | `96b282274c7a` | not yet reviewed | — |  | GPU: atomic adaptive PSO wait budget and non-blocking pipeline library saves |
| #273 | `ef08225cfde8` | not yet reviewed | — |  | SGD5: a marker inside a record's packed bytes is not another record |
| #270 | `92f2c60c8159` | integrated | `0739961f` | in the branch | GPU: every object answers for IUnknown |
| #269 | `9356beeae173` | not yet reviewed | — |  | Shaders: scalars leave divergent loops and branches as the wave's last path l… |
| #265 | `964e3adc6a62` | not yet reviewed | — |  | Shaders: private (scratch) memory as a per-lane array |
| #263 | `a67a5855f3f8` | not yet reviewed | — |  | Formats: reserve texture metadata memory in XG layouts |
| #261 | `a2243adc60d8` | integrated | `ee4a9ae6` | in the branch | GPU: copy-back puts back only what shaders wrote, in two more cases |
| #259 | `e232d3d7a210` | not yet reviewed | — |  | Shaders: query a descriptor's level count only when word 3 is read |
| #256 | `4c57d95c6c15` | not yet reviewed | — |  | fix(hle): skip unavailable host exports in net test |
| #254 | `b19ad9451e63` | not yet reviewed | — |  | Pipeline & CLI: stable shader cache key, in-flight dedup, and AOT warm-up |
| #250 | `6c2c14146c9e` | not yet reviewed | — |  | GPU & CLI: configurable/uncapped FPS limit, high-res pacing, high-refresh mod… |
| #247 | `5cff0007a525` | integrated | `2685c9ed` | in the branch | XAsync: a new block where a result was left behind begins a call |
| #246 | `0d6cf21506d1` | not yet reviewed | — |  | Policy and hooks: staged type changes, pinned exception, executable pre-commit |
| #244 | `7f95b87ea945` | integrated | `45a4455e` | in the branch | Shaders: share and serialize WARP host operations across threads |
| #242 | `6cb8fbe6b4d4` | not yet reviewed | — |  | GPU: make ExecuteIndirect clamping countBuffer-aware and non-destructive |
| #241 | `e8fbe9363d47` | not yet reviewed | — |  | Core host UX: XDG data dir, config-relative paths, and clearer VICTORY_LOG ha… |
| #240 | `723f4a296a00` | not yet reviewed | — |  | DSP: convolve reverb streams on the host |
| #238 | `534e1dd74c57` | not yet reviewed | — |  | CLI: check for AVX2 before running a title (VICTORY_SKIP_CPU_CHECK=1 to bypas… |
| #237 | `203d9f38407e` | not yet reviewed | — |  | HLE: offload async saves, reuse audio buffers, pin APU heap, and decouple inp… |
| #236 | `38bf927807fd` | not yet reviewed | — |  | XAsync: a canceled pending call's work touches neither freed state nor the ne… |
| #235 | `5d40ad817cc8` | not yet reviewed | — |  | Build: gate Windows-only paths so the workspace builds and tests on Linux |
| #234 | `e39fc9cbe8b9` | not yet reviewed | — |  | HLE: overlapped DirectStorage I/O and persistent xmem section views |
| #233 | `ededb1babb4d` | not yet reviewed | — |  | Shaders: wave64 compute on 32-lane hosts, in halves and in the console flow |
| #232 | `2c83383a429a` | rejected | — | replaces CI with 1,149 unverifiable lines | Linux/Wine testing: interim Linux CI subset; skip load_probe cargo build unde… |
| #231 | `e440f471b159` | not yet reviewed | — |  | Draft integration test of 138 PRs on Windows |
| #230 | `067ece6b1194` | not yet reviewed | — |  | GPU: skip unchanged texture uploads via prelude copy recording |
| #228 | `09ff84b6a7cb` | integrated | `f53da6cc` | in the branch | XGameSave: a failed create leaves a null handle, as documented |
| #226 | `6e01ea031553` | not yet reviewed | — |  | Shaders: test that an image read as float and used by an atomic is annotated … |
| #224 | `b87a9682e346` | integrated | `65c02b0e` | in the branch | HLE: report the host's commit headroom before the title starts |
| #223 | `714b81805f98` | not yet reviewed | — |  | Shaders: translate for the host's wave sizes |
| #222 | `9a77707b01c0` | not yet reviewed | — |  | apu: warn when the host has no audio output device |
| #221 | `acdcc41687be` | integrated | `18113a3c` | in the branch | Loader: suspend a thread before reading its TEB; skip bad dump times |
| #219 | `e57e76a7b405` | deferred | — | held as deferred with concrete review findings | fix(pe): validate section raw data bounds |
| #218 | `42bb9f9c08da` | not yet reviewed | — |  | APU: decode-convert and PCM rate conversion |
| #217 | `133e11455acc` | not yet reviewed | — |  | Wine: fall back where ws2_32 lacks exports; hook and cfg fixes |
| #216 | `58e267e23054` | not yet reviewed | — |  | Loader & HLE: harden VEH crash handler, PE prep/exports/TLS, and allow oleaut… |
| #214 | `514927d2842d` | integrated | `ba88b6ef` | in the branch | Loader: clzero loops in one exception; rdpru (#34 rebased, with a fix) |
| #213 | `7f00a62a4b86` | integrated | `106de91e` | in the branch | Crash reports: name the host's CPU and Windows build |
| #212 | `738b8b8ae7cb` | not yet reviewed | — |  | GPU, HLE: report host display HDR luminance and configure swap chain color sp… |
| #211 | `2fb7a8b44051` | not yet reviewed | — |  | GPU: composite multiple PresentX planes (fixes Series S black UI/menus) |
| #208 | `eb723596bd64` | not yet reviewed | — |  |  GPU: select high-performance D3D12 adapter and probe host caps |
| #207 | `11ca3ce4928c` | integrated | `188340ae` | in the branch | Media: a share is refused however it is spelled |
| #206 | `05981dd53b33` | not yet reviewed | — |  | GPU: make a structured buffer view off its host copy's elements a raw view |
| #202 | `dcf666486a3c` | integrated | `ae526377` | in the branch | GPU: skip unmapped regions after a failed bulk read into host memory |
| #200 | `93f052e23301` | not yet reviewed | — |  | Shaders: lower v_div_scale_f32, v_div_fmas_f32 and v_div_fixup_f32 |
| #197 | `64484b426ba8` | integrated | `ade65f82` | in the branch | Input: apply VICTORY_KEYS to a connected host pad too |
| #194 | `985d8d4b0253` | integrated | `7e18d623` | in the branch | Audio: an XMA loop's skip is of its first frame, not of its last |
| #193 | `f99b792e0794` | not yet reviewed | — |  | Shaders, GPU: add runtime stride table for structured buffers |
| #192 | `4a365e3a736a` | not yet reviewed | — |  | Shaders: replace single-wave Wave64 LDS lockstep with barriers and lower 32-l… |
| #190 | `ffa13108fef5` | integrated | `a4023b67` | in the branch | GPU: build the indirect-argument clamp with our own writer (no shader compile… |
| #189 | `54b333fc5c27` | not yet reviewed | — |  | Shaders: emulate Wave64 compute shaders on Wave32 and narrower hosts |
| #188 | `2a6e6a4bcea5` | integrated | `85834d42` | in the branch | Loader: find TLS-array load candidates sixteen offsets at a time (8-9x faster… |
| #186 | `3c7d845b3bf4` | integrated | `791580b7` | in the branch | Kernel: a title image's file name that fits clears the last error |
| #185 | `f173d3626e94` | not yet reviewed | — |  | GPU: maintain shadow-buffer coherency for directly indexed SRVs |
| #183 | `d4d1822049fe` | integrated | `a6df1a25` | in the branch | XMA: noise tests, and end a frame at its data instead of looping forever |
| #182 | `18275322e4d3` | integrated | `aa370a59` | in the branch | Files: `..` in a title path stops at the virtual drive's root |
| #180 | `aa692580ed69` | not yet reviewed | — |  | Shaders: twin descriptor loads and quad gradients for compute samples |
| #178 | `eb635f7af85f` | not yet reviewed | — |  | GPU: compile the indirect-argument clamp with DXC when D3DCompile refuses it … |
| #177 | `b7300eb5b86c` | integrated | `c68f6e1a` | in the branch | Shaders: key heap handles by element type (atomic on a float-read UAV) |
| #176 | `70278092ab8a` | not yet reviewed | — |  | Shaders: pair packed fp16 pixel inputs with unlisted signature rows |
| #175 | `19aec59ba293` | not yet reviewed | — |  | Windows 10: load the D3D12 Agility SDK runtime for shader model 6.6 |
| #174 | `882664e13cfa` | not yet reviewed | — |  | GPU: a frame wait takes its tick before it sleeps |
| #172 | `4eb0898b3cbd` | integrated | `5ba70891` | in the branch | XMA: cache IMDCT rotation tables per subframe size |
| #171 | `8f19f0dc169e` | not yet reviewed | — |  | Shaders: lower v_mad_u64_u32 and v_mad_i64_i32 |
| #169 | `645391bcce95` | not yet reviewed | — |  | Shaders: tbuffer_load_format_d16 loads pack two 16-bit channels per VGPR |
| #168 | `9483b1a96261` | not yet reviewed | — |  | Shaders: SCC of s_min, s_max and s_lshl1-4_add_u32 as the ISA sets it |
| #166 | `b81071c2fc6d` | not yet reviewed | — |  | Shaders: lower s_bitset{0,1}_b64, s_brev_b64 and s_bitreplicate_b64_b32 |
| #165 | `cee01b43e3a1` | not yet reviewed | — |  | Shaders: lower s_bcnt0_i32_b32/b64 and s_ff0_i32_b32/b64 |
| #164 | `825dbd98f714` | not yet reviewed | — |  | Shaders: integer memory atomics and sub-dword buffer accesses |
| #162 | `5d6889e8be76` | not yet reviewed | — |  | Shaders: lower v_sad_u8, v_sad_hi_u8, v_sad_u16, v_sad_u32 and v_msad_u8 |
| #160 | `5ce85c5e7087` | not yet reviewed | — |  | Shaders: texel offsets, comparison and gather forms of image samples |
| #159 | `f42e7d5e68b2` | not yet reviewed | — |  | GPU: occlusion, statistics and timestamp queries run on the host |
| #158 | `520d629ed178` | integrated | `9a6d9e67` | in the branch | GPU: the depth bounds test reaches the host |
| #157 | `13d984adfd77` | not yet reviewed | — |  | Shaders: lower 24-bit mul_hi, saturating 16-bit packs, integer dot pr… |
| #156 | `344704e02850` | not yet reviewed | — |  | Shaders: lower v_mul_hi_u32_u24 and v_mul_hi_i32_i24 |
| #153 | `3e98bd7e0ce7` | integrated | `c051b5da` | in the branch | GPU: seek directly to private data on final release |
| #151 | `d77408bb71e1` | integrated | `db44521f` | in the branch | Link victory.exe with a 16 MiB default thread stack |
| #150 | `fe014c1b50ed` | not yet reviewed | — |  | Input: read DualSense pads over HID when XInput has none |
| #148 | `a0380049aacd` | integrated | `3ff3ce73` | in the branch | Shaders: lower s_bcnt0_i32_b32/_b64, s_ff0_i32_b32/_b64 and s_flbit_i32_b64/_… |
| #145 | `ecc4eb668db7` | not yet reviewed | — |  | Shaders: read both ballot halves for wave32 host-wave mask ops |
| #143 | `525a713e8fcf` | not yet reviewed | — |  | GPU: hash and detile unwatched textures from a kernel-read copy |
| #142 | `26bdec947834` | not yet reviewed | — |  | shader: lower s_bitcmp{0,1}_b{32,64} and s_sext_i32_i{8,16} |
| #139 | `697c4bcd1b5e` | integrated | `0b3db22a` | in the branch | Shaders: reuse SSA sums when tracing memory addresses |
| #138 | `800227d59f9e` | not yet reviewed | — |  | Shaders: gradient samples on cube images |
| #137 | `31979ad2a749` | not yet reviewed | — |  | fix(gpu): preserve source and destination pixels in texture readbacks |
| #133 | `01ae55a3ba04` | integrated | `13e4e7fe` | in the branch | Files: a title run by its file name alone gets its own folder as G: |
| #132 | `b9ff219826e6` | deferred | — | held as deferred with concrete review findings | XTaskQueue: terminating a queue cancels its delayed callbacks too |
| #130 | `0ba0851dea0f` | not yet reviewed | — |  | Profile: opt-in VICTORY_PROFILE (frame times, memory, caches, loads; frames.c… |
| #126 | `e5b715983242` | integrated | `05c691d0` | in the branch | Shaders: walk a lane-mask diagram once in vars and given |
| #124 | `a6deb46db2fc` | not yet reviewed | — |  | GPU: reuse unchanged page checks across shadow copy-back ranges |
| #123 | `bc747281dcd6` | not yet reviewed | — |  | GPU: refuse raw buffer views that do not start on 16 bytes |
| #122 | `d01b17070dbf` | integrated | `461c174f` | in the branch | XAsync: end the call that finished, not whatever call the block holds now |
| #120 | `cb40cdcc21a7` | integrated | `9df49c0f` | in the branch | Shaders: lower s_absdiff_i32, s_sext_i32_i8/i16 and s_pack_ll/lh/hh_b32_b16 |
| #119 | `095e44a0bc15` | integrated | `4502afa1` | in the branch | GPU: query the allocation type when guarding mapped memory |
| #116 | `c853a6ab38df` | not yet reviewed | — |  | Shaders: byte and halfword LDS reads and writes |
| #115 | `b1bdd8994a45` | integrated | `b0d2e84f` | in the branch | Census: report directory traversal failures |
| #114 | `0b729de6ee7b` | integrated | `5d8955e0` | in the branch | XAsync: run a failed call's cleanup without the completion lock |
| #113 | `620afd5ec423` | not yet reviewed | — |  | GitHub: rebase issue forms and PR template on main |
| #112 | `89be65511431` | not yet reviewed | — |  | Build: Linux workspace, contributor setup and CI |
| #109 | `c2b4eef99423` | not yet reviewed | — |  | Shaders: rebuild a cube image descriptor for code that reads its dwords |
| #105 | `6e6d3fc027d0` | integrated | `4f6dac61` | in the branch | Shaders: a loop that branches back to the entry instruction translates |
| #101 | `9fbd7e9fc6ae` | not yet reviewed | — |  | Shaders: lower sub-dword LDS reads and writes |
| #100 | `f7d08ced116c` | not yet reviewed | — |  | GPU: cap a shader-visible sampler heap at 2,048 instead of losing it |
| #98 | `f468457a1859` | integrated | `5ceb0e1f` | in the branch | Shaders: model v_permlane16_b32 in whole-wave sections |
| #96 | `ba42db643c15` | integrated | `bbf60d7f` | in the branch | xma: start the frame after a continuation at the next packet that has frames |
| #92 | `4f2d5dbd8f1c` | integrated | `a47f488e` | in the branch | Shaders: lower v_add_nc_i32, v_sub_nc_i32, v_mul_lo_i32 and the signed 16-bit… |
| #91 | `ed5f98ff67e3` | not yet reviewed | — |  | Shaders: lower s_cmov_b32, s_cmov_b64 and s_cmovk_i32 |
| #89 | `081afbd5382b` | not yet reviewed | — |  | gpu: packed 16-bit render targets are R8G8B8A8 where the GPU cannot render to… |
| #86 | `a80eb17c4cea` | not yet reviewed | — |  | GPU: cap a sampler heap past the host's limit, with dynamic slots for tables … |
| #83 | `d8f063b04566` | not yet reviewed | — |  | GPU: bound the null sampler staging heap |
| #82 | `df5e928aa0c5` | integrated | `6399ea71` | in the branch | HLE: speak the title's narration with the host's speech synthesizer |
| #78 | `e6faad2c9fc5` | not yet reviewed | — |  | Shaders: inline float constants as f16 in v_fma_mix and v_pack_b32_f16 |
| #76 | `dea90ccc7e87` | integrated | `f464bd96` | in the branch | xmem: map a run of physical pages over several placeholders |
| #73 | `7cc244530270` | not yet reviewed | — |  | Shaders: support image_sample_b with a16 and _cl LOD-clamp variants |
| #72 | `521ceccd781e` | not yet reviewed | — |  | GPU: retry pipeline cache saves after write failures |
| #69 | `18f3937ed135` | not yet reviewed | — |  | Translate compute selects with numeric lane masks |
| #67 | `9ea5068e6075` | not yet reviewed | — |  | GPU: implement CopyTextureRegion texture-to-buffer readbacks |
| #55 | `56f1d413873d` | not yet reviewed | — |  | GPU: create host textures without the uses the host refuses |
| #52 | `628b95da5e73` | not yet reviewed | — |  | GPU: hold SHADER_HEAPS across descriptor mirror and check query offsets |
| #32 | `9c5328d4ef91` | not yet reviewed | — |  | GPU: fence waits, direct memory-write wakes, queue timeline and host list pool |
| #24 | `d5f41f54c49b` | integrated | `d819400b` | in the branch | GPU: run indirect mesh dispatches (DispatchMeshX) through indirect execution |
| #12 | `c63262f1e977` | not yet reviewed | — |  | Check the required package archive before title startup |

