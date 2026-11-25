# Disposition of current open upstream PRs

Snapshot: **713 open PRs**, retrieved 2026-11-25 with paginated REST:
8 pages (100 entries on each of the first seven, 13 on the last).
The raw snapshot and generator remain outside the repository:
`C:/Users/nazmi/victory-runs/codex-open-prs-20261125.json` and
`C:/Users/nazmi/victory-runs/codex-generate-dispositions.py`.
Review decisions from this continuation are in `pr-review-decisions.json`.
Do not overwrite this table with the previous snapshot's generator.

The [previous snapshot](pr-disposition-20261124.md) preserves every prior row,
including PRs now closed upstream. Closure is not evidence of local integration.
A changed head invalidates its previous reviewed disposition until rechecked;
18 previously reviewed heads changed in this refresh. The old maintenance log's
576 pending estimate counted numbers only; the SHA-aware initial count is 594.

`integrated-local` means reviewed and tested on this branch, with a dedicated
`--no-ff` merge. Historical `integrated` entries retain their old evidence at an
unchanged source SHA; they do not mean merged upstream. Each head is rechecked
before acting. `not yet reviewed` is explicitly unfinished, never approval.

## Summary

| Disposition | Count |
| --- | --- |
| deferred | 4 |
| duplicate | 9 |
| held | 5 |
| integrated | 96 |
| integrated-local | 3 |
| not yet reviewed | 591 |
| rejected | 5 |
| total open | 713 |

## Every open PR

| PR | Head | State | Local commit | Reason | Title |
| --- | --- | --- | --- | --- | --- |
| #1935 | `01cc8c7b0974` | not yet reviewed | ? | New since prior snapshot | XMem: recycle freed physical page numbers on allocation |
| #1934 | `5fb9f47a55ee` | not yet reviewed | ? | New since prior snapshot | GPU: resolve initial texture uploads and host_written at submission (#1891) |
| #1933 | `c2a36241d316` | not yet reviewed | ? | New since prior snapshot | victory-gpu: keep depth-twin UAVs in concrete UAV formats and support default views |
| #1932 | `ce91efa14bf1` | not yet reviewed | ? | New since prior snapshot | DStorage: preserve GetLastError across heap allocation in open_file and permit shared write/delete |
| #1931 | `cea29aa3a302` | not yet reviewed | ? | New since prior snapshot | GPU: cache shader resource metadata with DXIL |
| #1930 | `e462eb1b9b8b` | not yet reviewed | ? | New since prior snapshot | XPackage: wire XPackageEnumerateChunkAvailability (slot 13 / 0x68) |
| #1929 | `b9b6dab9eb61` | not yet reviewed | ? | New since prior snapshot | Shaders: wave64 compute on 32-lane hosts, in halves and in the console flow (rebase of #233) |
| #1928 | `d8c8ae6b7562` | not yet reviewed | ? | New since prior snapshot | Storage: do not follow directory symlinks or junctions when computing used bytes |
| #1927 | `5b42f8ffdd9c` | not yet reviewed | ? | New since prior snapshot | Shaders: d16 format loads saturate 32-bit integer channels |
| #1926 | `644aed492e9f` | not yet reviewed | ? | New since prior snapshot | GPU: boot on NVIDIA when a format cannot be a typed UAV |
| #1924 | `1aa7225fb861` | not yet reviewed | ? | New since prior snapshot | Shader &amp; GPU: merged geometry stages (rebase of #1399, with the batched GSVS write fix) |
| #1923 | `4f1228c1fd62` | not yet reviewed | ? | New since prior snapshot | GPU: run submissions asynchronously on a host fence timeline (rebase of #595) |
| #1922 | `24a7bc20b857` | not yet reviewed | ? | New since prior snapshot | GPU: refuse wave64 mesh and compute programs on hosts without 64-lane waves (rebase of #578) |
| #1920 | `664d7e0a2890` | not yet reviewed | ? | New since prior snapshot | Shaders: v_mad_f32 and v_mac_f32 multiply apart from their add |
| #1919 | `6997ffa3f7c2` | not yet reviewed | ? | New since prior snapshot | Shader tests: the runner's Basic Render Driver counts as software, which its flags don't say |
| #1917 | `8a037ac6c3a9` | not yet reviewed | ? | New since prior snapshot | FidelityFX: add opt-in AMD frame generation to host upscaling |
| #1916 | `38ea240deb37` | not yet reviewed | ? | New since prior snapshot | GPU: clear descriptors for null constant buffers (re-lands #1522) |
| #1915 | `fd1adb35d475` | not yet reviewed | ? | New since prior snapshot | Shaders: lower 32-bit and 64-bit global atomics (cmpswap, csub, inc, dec, float, _x2) |
| #1914 | `a2cead0e8d00` | not yet reviewed | ? | New since prior snapshot | GPU: an empty pipeline library is not asked for pipelines it cannot have |
| #1913 | `27ccf626a4f5` | not yet reviewed | ? | New since prior snapshot | GPU tests: a batch's hold on a copy ends before its pool is freed |
| #1912 | `0ababf50f752` | not yet reviewed | ? | New since prior snapshot | test(shader): initialize Root UAV buffers before dispatch |
| #1908 | `1a0d45c20487` | not yet reviewed | ? | New since prior snapshot | Keep the shader cache warm across emitter test edits |
| #1907 | `a64b6c67d5b9` | not yet reviewed | ? | New since prior snapshot | victory-shader: honor unwritten OSG1 channels and output modes in rectangle_gs |
| #1906 | `c84cbaaf8c20` | not yet reviewed | ? | New since prior snapshot | GPU: view addresses for tables made afresh (dynamic slots) [draft] |
| #1904 | `ffc081baea0e` | not yet reviewed | ? | New since prior snapshot | GPU: VICTORY_LIBRARY_TIMING measures the host FidelityFX upscaler's GPU and CPU cost |
| #1903 | `cbafed53ac2b` | not yet reviewed | ? | New since prior snapshot | Shader: the ray test's triangle multiplies stay apart from their adds |
| #1900 | `3bdccbca0d36` | not yet reviewed | ? | New since prior snapshot | Tests: the dot products run with operands loaded at run time, not literals |
| #1899 | `de9ca0c3842a` | not yet reviewed | ? | New since prior snapshot | Shaders: lower 64-bit texture atomics to DXIL i64 atomics |
| #1898 | `5a96f441d0bb` | not yet reviewed | ? | New since prior snapshot | GPU: dependency-aware two-stage shader translation and pipeline scheduling (#1509) |
| #1897 | `58e0f3cc538f` | not yet reviewed | ? | New since prior snapshot | end the stack walk where rtlvirtualunwind faults |
| #1896 | `51f3cb87cf39` | not yet reviewed | ? | New since prior snapshot | Shader: the packed dot over an inline constant is taken by hand from … |
| #1895 | `ba1a666ddd51` | not yet reviewed | ? | New since prior snapshot | GPU: preserve integer contrast in R32 captures |
| #1894 | `18f21fcc531b` | not yet reviewed | ? | New since prior snapshot | GPU: pool mapped upload buffers across command list resets |
| #1893 | `211364a7ad3a` | not yet reviewed | ? | New since prior snapshot | GPU: implement IDXGIAdapter2::GetDesc2 |
| #1892 | `e840e3188f8d` | not yet reviewed | ? | New since prior snapshot | Tests: every GPU test module takes the shared lock, which diff_tests did not |
| #1890 | `d047e12a2ef0` | not yet reviewed | ? | New since prior snapshot | Shaders: console wave64 compute on 32-lane hosts (split waves) |
| #1889 | `13afa69850c8` | not yet reviewed | ? | New since prior snapshot | Shader tests: VICTORY_ADAPTER picks the GPU the hardware checks run on |
| #1888 | `6f00956c3046` | not yet reviewed | ? | New since prior snapshot | GPU: cache alias-copy footprint plans and pool temporary copy buffers |
| #1887 | `9ae875de12cc` | not yet reviewed | ? | New since prior snapshot | Shader tests: the test host's structured buffer views are raw (#1835) |
| #1886 | `77329c18dc8b` | not yet reviewed | ? | New since prior snapshot | Shaders: lower pixel SV_Coverage input via dx.op.coverage |
| #1885 | `5aa10e54597d` | not yet reviewed | ? | New since prior snapshot | Shaders, GPU: one back edge per loop header, and a large stack for pipeline threads |
| #1884 | `a175ffff06b7` | not yet reviewed | ? | New since prior snapshot | Perf/memory guard batching |
| #1883 | `a31d8ccbd051` | not yet reviewed | ? | New since prior snapshot | Shaders: lane masks a traversal keeps as numbers |
| #1880 | `69ef33e0af3a` | not yet reviewed | ? | New since prior snapshot | Audio: refill XMA rings from a decoded reserve on the title's thread |
| #1879 | `5a8fc617d7bb` | not yet reviewed | ? | New since prior snapshot | Shader tests: keep dot4 test operands runtime for RDNA1 |
| #1878 | `3bd5c6eec18b` | not yet reviewed | ? | New since prior snapshot | Shaders: hardware test for v_cvt_f32_ubyte0..3 |
| #1877 | `4e6e37fa9c04` | not yet reviewed | ? | New since prior snapshot | Shaders: hardware test for the 16-bit integer ops |
| #1876 | `b3eb725aa38e` | not yet reviewed | ? | New since prior snapshot | Shaders: hardware test for the VOP2 shifts, min/max and logic ops |
| #1875 | `035433cd8480` | not yet reviewed | ? | New since prior snapshot | Shaders: hardware test for the scalar add/sub carry chain and the SOPK forms |
| #1874 | `480da975e201` | not yet reviewed | ? | New since prior snapshot | Shaders: hardware test for the 32-bit scalar logic ops and s_ashr_i32 |
| #1873 | `bbbb52aec478` | not yet reviewed | ? | New since prior snapshot | GPU: share a scratch buffer for texture detiling instead of retaining one per Staging |
| #1872 | `654301a843f1` | not yet reviewed | ? | New since prior snapshot | GPU: the depth twin's converter builds its root signature when the compiler drops it (Wine) [draft] |
| #1871 | `1076ad969925` | not yet reviewed | ? | New since prior snapshot | Shaders: lower indexed tbuffer_load_format_* and indexed buffer_store_{byte,short} |
| #1869 | `b118e79363ac` | not yet reviewed | ? | New since prior snapshot | Perf/indexed copyback ranges |
| #1868 | `3d579a76fea2` | not yet reviewed | ? | New since prior snapshot | FidelityFX: write an RGB9E5 output NVIDIA cannot store to through an FP16 texture (fixes a device removal) |
| #1867 | `eabd870b2268` | not yet reviewed | ? | New since prior snapshot | GPU: merge shadow write-back bytes in AVX2 vectors with scalar fallback |
| #1866 | `c3826cb7c1eb` | not yet reviewed | ? | New since prior snapshot | Game: preserve sample masks in prebuilt graphics pipelines |
| #1865 | `b58ed3cba616` | not yet reviewed | ? | New since prior snapshot | GPU: pool title-memory dirty bitmaps, write page tables directly, and skip zero bitmap words |
| #1864 | `4268b6dd800e` | not yet reviewed | ? | New since prior snapshot | Shaders: global byte and half stores keep the rest of their dword |
| #1863 | `0e4c06ce318a` | not yet reviewed | ? | New since prior snapshot | fix(census): report unreadable files during PE detection |
| #1862 | `aab3845a5687` | not yet reviewed | ? | New since prior snapshot | GPU: index bindless texture slots for targeted release and preserve u… |
| #1861 | `a38aa27365a7` | not yet reviewed | ? | New since prior snapshot | fix |
| #1859 | `8c77217c4523` | not yet reviewed | ? | New since prior snapshot | GPU: reject overflowing and non-buffer descriptor spans before shadow… |
| #1858 | `2abb2301ed6b` | not yet reviewed | ? | New since prior snapshot | Shader tests: the permute test skips a device with shorter waves |
| #1857 | `6758d28b6cda` | not yet reviewed | ? | New since prior snapshot | Shader tests: 64-bit image atomics skip devices without typed 64-bit heap atomics |
| #1856 | `23e3b5ab998e` | not yet reviewed | ? | New since prior snapshot | Shaders: programs without FMad disable math refactoring |
| #1855 | `dca80ab6e028` | not yet reviewed | ? | New since prior snapshot | victory-gpu: clamp constant buffer views to their registered buffer |
| #1853 | `59b8454a8fab` | not yet reviewed | ? | New since prior snapshot | Shaders: extend image_gather4_l(_o) to all address modes, borders, a16, d16, unorm, and 2D arrays |
| #1852 | `12301e72095e` | not yet reviewed | ? | New since prior snapshot | GPU: publish descriptor address snapshots before releasing writer lock |
| #1849 | `de95e5150ce9` | not yet reviewed | ? | New since prior snapshot | GPU: free memory is not the runtime's to watch (fixes a world-load deadlock under Wine) |
| #1847 | `485a943942b0` | not yet reviewed | ? | New since prior snapshot | Shaders: reconstruct image descriptor dword 0 as a heap-slot proxy |
| #1846 | `5d4445d68cf1` | not yet reviewed | ? | New since prior snapshot | Shaders: worklist register liveness with 96 independent synthetic CFG… |
| #1845 | `c624d98f7774` | not yet reviewed | ? | New since prior snapshot | NVIDIA D3D12 fixes & shader translation improvements (dynamic descriptors, Wave32 compute, untracked masks) |
| #1843 | `3502e6c54ae7` | not yet reviewed | ? | New since prior snapshot | GPU: unordered views only in formats the host stores to (NVIDIA device removal on B5G5R5A1) |
| #1841 | `ee2cf2609c60` | not yet reviewed | ? | New since prior snapshot | Shaders: honor FP32 rounding when packing a byte |
| #1840 | `7811cf9b0b24` | not yet reviewed | ? | New since prior snapshot | Shaders: discover irreducible loops with iterative SCC decomposition |
| #1839 | `e6397f594b28` | not yet reviewed | ? | New since prior snapshot | GPU: an unimplemented method logs the four argument registers |
| #1838 | `5b40efdaa3ee` | not yet reviewed | ? | New since prior snapshot | XGameSave: an old version another process holds open no longer fails the save |
| #1837 | `3dcdfeec9386` | not yet reviewed | ? | New since prior snapshot | Display: the title's screenshots save the last presented frame |
| #1834 | `3f20c559beb5` | not yet reviewed | ? | New since prior snapshot | Shaders: read a VALU opcode's name without a heap vector |
| #1833 | `f52b80061522` | not yet reviewed | ? | New since prior snapshot | Shaders: walk a live set's registers a bit at a time |
| #1832 | `5abc1cc48939` | not yet reviewed | ? | New since prior snapshot | GPU: drop the unordered-access permission a format cannot carry, before any call names it |
| #1830 | `f148f005111f` | not yet reviewed | ? | New since prior snapshot | Config: `victory config` exits nonzero on an asset policy violation |
| #1828 | `3a85ae51fb06` | not yet reviewed | ? | New since prior snapshot | GPU: a present waits for the previous frame's submissions, not its own |
| #1827 | `708ca1bf6b25` | not yet reviewed | ? | New since prior snapshot | HLE: enumerate the current installation for XPackageKind::Game |
| #1826 | `dbb04eb13867` | not yet reviewed | ? | New since prior snapshot | GPU: build an object name only where the log takes it |
| #1825 | `3a3f9a26aa83` | not yet reviewed | ? | New since prior snapshot | GPU: keep the view-stride table once per list, not per draw stage |
| #1824 | `4fbaa83516ad` | not yet reviewed | ? | New since prior snapshot | Shaders: declare each dx.op call once |
| #1823 | `60561410ec3a` | not yet reviewed | ? | New since prior snapshot | Shaders: bound a signature part's rows by the bytes it has |
| #1822 | `5942e74b8ce4` | not yet reviewed | ? | New since prior snapshot | Speech: the title's narration plays through the host's SAPI voice |
| #1821 | `7c6f102516c2` | not yet reviewed | ? | New since prior snapshot | CLI: add offline shader and GPU regression comparison report with syn… |
| #1819 | `d7a5c3c6e014` | not yet reviewed | ? | New since prior snapshot | Shaders: translated loops are left after 2^14 iterations (was 2^20) |
| #1818 | `24a7d7068812` | not yet reviewed | ? | New since prior snapshot | GPU: preserve disjoint shadow refresh ranges |
| #1817 | `77dfdf2a1c07` | not yet reviewed | ? | New since prior snapshot | GPU: enforce logical indirect buffer spans |
| #1816 | `a688849e11a4` | not yet reviewed | ? | New since prior snapshot | GPU: account for delivered bytes in protected bulk copyback |
| #1814 | `32ad8f6534da` | not yet reviewed | ? | New since prior snapshot | PE tests: cover ImportLibrary::parse short imports and archive errors |
| #1813 | `6511c8529214` | not yet reviewed | ? | New since prior snapshot | GPU tests: verify indirect::bound clamping and budget on the GPU |
| #1812 | `99ae318f0af3` | not yet reviewed | ? | New since prior snapshot | Shader tests: run v_mul_hi_u32 and v_mul_hi_i32 on the GPU |
| #1811 | `aa2a55fdd08b` | not yet reviewed | ? | New since prior snapshot | Shader tests: run v_perm_b32 on the GPU |
| #1810 | `e85d368734a5` | not yet reviewed | ? | New since prior snapshot | Shader tests: run 24-bit multiply and multiply-add ops on the GPU |
| #1809 | `15d77558ebf4` | not yet reviewed | ? | New since prior snapshot | CLI: list pipeline keys without initializing caches |
| #1807 | `c3320ad5b110` | not yet reviewed | ? | New since prior snapshot | Refactor/iterative shader bdd |
| #1806 | `1c6593384f8f` | not yet reviewed | ? | New since prior snapshot | CLI: discover pipeline packs in archives without shader containers |
| #1805 | `703d5204cee3` | not yet reviewed | ? | New since prior snapshot | CLI: write pipeline key listings in stable pack order |
| #1804 | `3b3c1f5dece5` | not yet reviewed | ? | New since prior snapshot | Shader tests: run v_bfm_b32, v_bcnt_u32_b32, v_ffbl_b32, and v_cvt_pk_u8_f32 on the GPU |
| #1803 | `9407dd68ee0f` | not yet reviewed | ? | New since prior snapshot | GPU: copy depth twins from pre-list states in prelude and route depth UAV barriers to twin planes |
| #1802 | `70b895d308b6` | not yet reviewed | ? | New since prior snapshot | Perf/shader relative shift bounds |
| #1800 | `2a4698a66b91` | not yet reviewed | ? | New since prior snapshot | GPU: log title slots past a capped heap once per 64 Ki, not each |
| #1799 | `07c9dd472abc` | not yet reviewed | ? | New since prior snapshot | fix(shader,gpu): eliminate 22s s_barrier world TDRs, zero invalid indirect mesh args, and reach Vice City opening credits & world frames |
| #1798 | `63fefac00749` | not yet reviewed | ? | New since prior snapshot | run: boot a separated exe/dump layout from the dump |
| #1797 | `e5c608d9fee8` | not yet reviewed | ? | New since prior snapshot | XMA: reject frame starts in packet headers |
| #1796 | `5edb19078fa0` | not yet reviewed | ? | New since prior snapshot | Core: report unreadable implicit configuration files |
| #1795 | `8da35725f551` | not yet reviewed | ? | New since prior snapshot | Shaders: LDS halfwords straddling two dwords, and hardware tests of byte and halfword LDS accesses |
| #1794 | `72ebd96862cd` | not yet reviewed | ? | New since prior snapshot | Shaders: support BVH a16 ray directions and unsorted box nodes |
| #1793 | `7b622911c739` | not yet reviewed | ? | New since prior snapshot | FidelityFX: Series X switches to the host upscaler and its render scale (rework of #1284) |
| #1792 | `898b5512ac39` | not yet reviewed | ? | New since prior snapshot | Formats: reject texture mip counts beyond the full chain |
| #1791 | `6ef0c4ecc697` | not yet reviewed | ? | New since prior snapshot | Shader tests: keep the FSPX dot4 test off folded constants |
| #1790 | `8682e4a7fb6d` | not yet reviewed | ? | New since prior snapshot | Shader tests: dot products take their operands at run time |
| #1789 | `d09311e7c141` | not yet reviewed | ? | New since prior snapshot | fix(core): keep persistent data outside the cache [draft] |
| #1787 | `040f9d433926` | not yet reviewed | ? | New since prior snapshot | GPU: textures drop unordered access in formats the host cannot write (NVIDIA B5G5R5A1) |
| #1786 | `8e57b7352f61` | not yet reviewed | ? | New since prior snapshot | Input: support DualShock 4 over HID and preserve keyboard controls |
| #1784 | `79d6f3631c52` | not yet reviewed | ? | New since prior snapshot | Test Wave32 spilled lane masks at control-flow joins |
| #1780 | `151fa7d7316d` | not yet reviewed | ? | New since prior snapshot | GPU: check every unordered texture view against the host's typed-store support (#1754) |
| #1779 | `a63eb93747e2` | not yet reviewed | ? | New since prior snapshot | GPU: view heaps on hosts capped below 2^21 descriptors |
| #1778 | `d9c329fe93a7` | not yet reviewed | ? | New since prior snapshot | GPU: bind a sampler heap of our own when the title binds none |
| #1777 | `3635e704e7d7` | not yet reviewed | ? | New since prior snapshot | GPU: size the view heap clamp from what the host takes (NVIDIA 1,000,000 cap) |
| #1776 | `36b4ac48b832` | not yet reviewed | ? | New since prior snapshot | GPU: exclude uncastable formats (68, 69, 89) from castable texture lists |
| #1775 | `b0799091b4f3` | not yet reviewed | ? | New since prior snapshot | GPU: unordered views only in formats the host can store (#1753) |
| #1774 | `6c182d81526e` | not yet reviewed | ? | New since prior snapshot | victory: meshdump surveys mesh and compute programs for group shape, LDS and divergent barriers |
| #1773 | `35ead9126cff` | not yet reviewed | ? | Head changed since `496ae1c5e0ce`; prior not yet reviewed evidence needs revalidation | raise the fault of an emulated clzero at the title's instruction |
| #1772 | `69662be1d56b` | not yet reviewed | â€” |  | HLE: handle verbatim/drive-root mounts and UNC/verbatim GetDiskFreeSpaceW roots |
| #1770 | `1beaca67e124` | not yet reviewed | â€” |  | Descriptor tests: take the capped heap's design into account |
| #1769 | `cd90d74741fb` | not yet reviewed | ? | Head changed since `7e9907c76f59`; prior not yet reviewed evidence needs revalidation | Shaders: LDS accesses outside the program's allocation read 0 and drop their writes |
| #1768 | `968950d98233` | not yet reviewed | â€” |  | GPU: retry a root signature without the heap-directly-indexed flags when the host refuses them |
| #1767 | `85ce678419ba` | not yet reviewed | ? | Head changed since `882856f0f8ff`; prior not yet reviewed evidence needs revalidation | translate: report the validator's own reason, not the line that hides it |
| #1766 | `368643182465` | not yet reviewed | â€” |  | Shaders: cube-map face coordinates read denormals as zeros |
| #1765 | `d02e007b7973` | not yet reviewed | â€” |  | GPU tests: the castable pair is two host textures where the host refuses castable formats |
| #1763 | `dbd4fc2ea78c` | not yet reviewed | ? | Head changed since `ca168ded3716`; prior not yet reviewed evidence needs revalidation | GPU: refuse typed UAVs in unsupported formats regardless of family or castability |
| #1762 | `550cac2b9f37` | not yet reviewed | â€” |  | GPU: structured buffer views are raw views of the same bytes on the host (NVIDIA reads 0 through raw loads of structured views) |
| #1761 | `d06bfe76c72d` | not yet reviewed | â€” |  | Input: trigger rumble through Windows.Gaming.Input with one pad conne… |
| #1759 | `dc80215d5c72` | not yet reviewed | â€” |  | Shaders: lower numerical Wave32 mesh masks at uniform entry |
| #1757 | `e18d930287b2` | not yet reviewed | ? | Head changed since `29d5f6e0dc7c`; prior not yet reviewed evidence needs revalidation | Shaders: one arena per program for instruction operands and effects |
| #1756 | `ab3c38d3793b` | not yet reviewed | â€” |  | Shaders: image atomics on a handle of their own element type |
| #1751 | `45215912b474` | not yet reviewed | â€” |  | GPU: capped view heaps keep the full dynamic range (NVIDIA) |
| #1750 | `b58b779fdedc` | not yet reviewed | â€” |  | GPU: a wait for a submission ends when the device is lost or the GPU stalls |
| #1749 | `27514c6a864c` | not yet reviewed | â€” |  | GPU: indirect mesh dispatches run as an indirect execution of one record |
| #1747 | `9cd8d7466921` | not yet reviewed | ? | Head changed since `fbf1dc32bb0c`; prior not yet reviewed evidence needs revalidation | Shaders: lower v_div_scale_f32 and v_div_fmas_f32 |
| #1743 | `332152b58105` | not yet reviewed | â€” |  | GPU tests: replay a world frame's copies of title memory (benchmark for the copy PRs) |
| #1741 | `d2c9a469e7ae` | not yet reviewed | â€” |  | Docs: AGENTS.md for AI agents working on VIctory |
| #1739 | `9c7ca2c630e7` | not yet reviewed | â€” |  | GPU: clear final pipeline stream padding after moves (on #1690) |
| #1738 | `d39a027f2eb9` | not yet reviewed | â€” |  | Shaders: preserve live interpolation first halves (on #453) |
| #1737 | `108b38be2ee7` | not yet reviewed | â€” |  | Shaders: honor permlane EXEC and FI/BC flags (on #1014) |
| #1735 | `8ec5de1ac765` | not yet reviewed | â€” |  | GPU: time commands by pipeline in VICTORY_GPU_TIMING |
| #1734 | `0d4ea57f4490` | not yet reviewed | ? | Head changed since `90ebae3dbc33`; prior integrated evidence needs revalidation | victory-shader: make the test dispatches cheap - an event wait and reused command objects (emit suite -33% wall, -48% CPU) |
| #1732 | `5c781cf159c6` | not yet reviewed | ? | Head changed since `3dc6c12410c9`; prior duplicate evidence needs revalidation | GPU: f32_to_half keeps NaN and rounds normals to nearest even (#843) |
| #1731 | `150964d218ad` | not yet reviewed | ? | Head changed since `1f84c2330954`; prior integrated evidence needs revalidation | GPU: occlusion queries measure on the host instead of reporting every draw visible |
| #1730 | `87a93820d96d` | not yet reviewed | â€” |  | translate: --cache DIR fills the runtime's shader cache |
| #1728 | `463e835868ba` | integrated | â€” |  | Pipelines: VICTORY_PIPELINE_WORKERS sets the worker count |
| #1727 | `6cd8375a23d5` | integrated | â€” |  | GPU: 1D textures are 2D on the host, one texel high (GBV id=940) |
| #1725 | `a6365c376b37` | integrated | â€” |  | GPU: placed copies take the bytes a GPU write leaves in their memory |
| #1723 | `5542340831d3` | not yet reviewed | â€” |  | GPU diagnostics: pipeline statistics per noted command with VICTORY_GPU_TIMING |
| #1722 | `08b526d98c2d` | integrated-local | `a202f7ff` | Null empty-library blob; recovered per-name locks and snapshots retained; 1/1 files reviewed; source WARP and candidate/full integration gates pass; no-ff merge | GPU: the host pipeline library works under Wine (null blob for an empty library) |
| #1721 | `428c5abf6889` | not yet reviewed | ? | Head changed since `619a8e8b158c`; prior not yet reviewed evidence needs revalidation | Launcher: a GUI that checks the game directory and runs victory.exe |
| #1720 | `7a990741f749` | not yet reviewed | ? | Head changed since `2f61f7c0fc27`; prior integrated evidence needs revalidation | report apu heap allocations that fail |
| #1719 | `771467458b5d` | integrated | `b7e3689c` | in the branch | DirectStorage: clear the factory out pointer on failure |
| #1717 | `35fb57d417bd` | integrated | `105fbc6d` | in the branch | HLE: the retail console ID has the documented dotted form |
| #1716 | `758bf7fbc6a3` | not yet reviewed | â€” |  | victory-shader: fold constants and eliminate dead pure ops in bitcode |
| #1715 | `b3db101a0d86` | not yet reviewed | ? | Head changed since `645bd5c0e200`; prior not yet reviewed evidence needs revalidation | FidelityFX: DLSS frame generation on frames the title upscales (stacked on #1704) |
| #1712 | `57c25604fa37` | integrated | `e2f67a49` | in the branch | victory-shader: support line-topology mesh shaders |
| #1710 | `747093145fa4` | integrated | `f9fe0e37` | in the branch | GPU: keep dropped draws in the full frame reporting interval |
| #1704 | `be878d3c5130` | not yet reviewed | ? | Head changed since `7e6e5788202d`; prior not yet reviewed evidence needs revalidation | FidelityFX: answer the upscale calls with DLSS through Streamline, RGB9E5 outputs included |
| #1703 | `120df922ba73` | integrated | `336bee70` | in the branch | README: the GPU timing switch needs DRED for the per-command notes |
| #1702 | `7046f976589b` | not yet reviewed | ? | Head changed since `10b96dea7bb5`; prior integrated evidence needs revalidation | HLE: add VICTORY_NO_HOST_PAD to hide host gamepads |
| #1701 | `c985ce117efa` | not yet reviewed | ? | Head changed since `2e33598b9292`; prior integrated evidence needs revalidation | GPU: VICTORY_MODE overrides the reported console display mode |
| #1700 | `0e1024d695f7` | integrated | `aec7eb38` | in the branch | GPU: say when a draw's render target has no host copy or lost its permission |
| #1699 | `6fd640a2ae53` | rejected | â€” | reads src2 with half_src where src0/src1 use half_int_src; probe measured 15366 where hardware gives 0 | shader: lower v_add_nc_i16, v_sub_nc_i16, v_mad_i16, and v_mad_u16 with clamp |
| #1694 | `da18cb8bab09` | held | â€” | would revert the single-rounding v_dot2_f32_f16 fix | shader: lower 6 RDNA2 integer dot-product instructions without INT64 |
| #1693 | `77d21c68af28` | integrated | `c4a9469c` | in the branch | victory-shader: lower TFE image loads/samples/gathers via CheckAccessFullyMapped |
| #1692 | `a100b0e3184f` | not yet reviewed | ? | Head changed since `2b3da5eb353a`; prior not yet reviewed evidence needs revalidation | [BUNDLE] GPU diagnostics: 3 PRs, #669 #1251 #602 (rebuilt on main 70564ae) |
| #1691 | `667e7dc9a4fd` | integrated | `76adf6a1` | in the branch | GPU: track D3D12 shadow-buffer states (#1670) |
| #1690 | `d721660ea155` | not yet reviewed | ? | Head changed since `6c22344b1e4f`; prior integrated evidence needs revalidation | GPU: zero pipeline library description padding |
| #1688 | `27e971581623` | not yet reviewed | ? | Head changed since `c7242f1c66f2`; prior integrated evidence needs revalidation | Shaders: lower v_mad_u32_u16, v_mad_i32_i16, and v_lerp_u8 |
| #1687 | `3089d4c23465` | not yet reviewed | ? | Head changed since `df9c42347055`; prior integrated evidence needs revalidation | Shaders: lower image_gather4_c_lz(_o) and offset sample variants |
| #1686 | `14ca12162bed` | integrated | `621c3c17` | in the branch | Shaders: lower pixel shader SV_Coverage input via dx.op.coverage |
| #1685 | `d7536bdd9ca5` | integrated | `138e43c5` | in the branch | Shaders: lower v_cmp_{f,t}_{i64,u64} constant 64-bit compares |
| #1684 | `c0b3633e5762` | not yet reviewed | â€” |  | shader: lower 10 scalar bitops (bcnt0/ff0/flbit64/brev64/bitset64/bitreplicate) without INT64 |
| #1682 | `3b0c7dad7330` | not yet reviewed | â€” |  | Shaders: interpolations with any i/j by hand; barycentrics read as data |
| #1681 | `9c447fbdd049` | not yet reviewed | â€” |  | GPU: verify exact dirty-page masks on D3D12 |
| #1676 | `00b4b1b3b4a6` | integrated | `29289af8` | in the branch | Shaders, CLI: name a host without mesh shaders in the pipeline census |
| #1675 | `5a3f25a48bcd` | duplicate | â€” | lowering already at lower.rs:3567 | shader: lower s_absdiff_i32 and s_pack_{ll,lh,hh}_b32_b16 |
| #1674 | `b9d3e8364e61` | not yet reviewed | â€” |  | Shaders: guard speculative and EXEC-predicated heap descriptor loads (#1595) |
| #1673 | `0ef707e53d38` | not yet reviewed | ? | Head changed since `dda166c8f17e`; prior integrated evidence needs revalidation | HLE: XTaskQueueTerminate cancels only its own tasks on shared ports |
| #1669 | `70d3a821155f` | integrated | `ee61aa1c` | in the branch | shader: handle dword-straddling 16-bit typed buffer loads at byte offset 3 |
| #1668 | `c4d626b250d1` | integrated | `bc9c0f60` | in the branch | Shaders: read inline floats as f32 bits in 16-bit integer ops |
| #1666 | `a67255631622` | not yet reviewed | â€” |  | GPU: small first scratch chunk per list; log host memory against budget |
| #1665 | `6d760bc05a3a` | not yet reviewed | â€” |  | Docs: add Windows 10 and GPU compatibility guide |
| #1664 | `98b0ec8c60ff` | integrated | `4cf31e8e` | in the branch | hle: return S_OK with null handle when PowerSetRequest returns ERROR_NOT_SUPPORTED |
| #1663 | `20c6c269bf24` | held | â€” | drops wave32 forcing; out-of-bounds ADDTID returns 0 instead of refusing | Shaders: lower RDNA 2 ADDTID memory instructions to DXIL |
| #1662 | `e8bbc120b6d8` | held | â€” | same 248-as-f64 defect as #1654 | Shaders: lower RDNA 2 quad-byte SAD instructions to DXIL |
| #1661 | `07fc14b626b9` | not yet reviewed | â€” |  | GPU: route non-overlapping CopyMemoryToMemoryX through Step::Copy and scope invalidation to written ranges |
| #1659 | `79107fd52f9e` | not yet reviewed | â€” |  | shader: lower v_frexp_{exp,mant}_{f32,f16} in pure 32-bit DXIL |
| #1657 | `932dcbf89f96` | integrated | `15316a22` | in the branch | XSystem: name the analytics family Windows.Xbox |
| #1655 | `4f1d0acf4d5c` | not yet reviewed | â€” |  | xtask: the Forge and claims (one branch of every pull request that builds; who works on what) |
| #1654 | `3a08221a6246` | held | â€” | inline constant 248 expanded from f64 among f32 siblings | shader: lower v_mad_u64_u32 and v_mad_i64_i32 without INT64 |
| #1653 | `1e79f148f66d` | not yet reviewed | â€” |  | victory-gpu: split the pipeline library into shards so a save writes only what changed (was 600 MB per save) |
| #1652 | `d43c8558abb3` | not yet reviewed | â€” |  | Add native GTA VI launcher |
| #1651 | `955905163ecb` | not yet reviewed | â€” |  | GPU: save only changed host pipeline-library shards |
| #1650 | `6b12a361be06` | not yet reviewed | â€” |  | translate: identify unsupported ACCC/FSPX shader containers [draft] |
| #1649 | `36682459e8d1` | not yet reviewed | â€” |  | shader: lower s_cmov_b32, s_cmov_b64, and s_cmovk_i32 |
| #1648 | `7fe7d0e08d58` | not yet reviewed | ? | Head changed since `c6ec4ad3bf29`; prior not yet reviewed evidence needs revalidation | GPU: mesh shaders emulated as compute + unpack vertex on SM 6.6 cards without a mesh tier |
| #1647 | `9cd8d23ea821` | not yet reviewed | â€” |  | shader: lower v_mul_hi_i32_i24 and v_mul_hi_u32_u24 without INT64 |
| #1646 | `64a703844408` | not yet reviewed | â€” |  | Shaders: fold constant bin/cmp/select/cast ops and identity ops at emit time |
| #1644 | `8bf13c49dfd2` | not yet reviewed | â€” |  | [BUNDLE] Unknown-stride buffers: #704 on #193, stacked on #1511 |
| #1643 | `c79f4c891254` | not yet reviewed | â€” |  | GPU: register pipeline demand after asynchronous compilation timeout |
| #1642 | `f876a822b059` | not yet reviewed | â€” |  | Shaders: preserve signaling NaNs in typed buffer loads |
| #1641 | `3271c00e8c9b` | not yet reviewed | â€” |  | GameInput: answer IUnknown and own-IID queries on the COM objects |
| #1639 | `0b7a13c68dfb` | integrated | â€” |  | PE: map header RVAs and read TLS data from the headers |
| #1638 | `ab4a6e645b1e` | not yet reviewed | â€” |  | [BUNDLE] NVIDIA stack: Impre-visible's tree (wave64 #189 + the world-run list, 12 PRs) checked together, stacked on #1589 |
| #1632 | `8de76a242c98` | not yet reviewed | â€” |  | [BUNDLE] World-run stack from Impre-visible's report: 12 PRs (#1077 #1175 #523 #530 #1040 #1406 #1392 #1407 #1516 #1388 #1189 #542), stacked on #1511 |
| #1631 | `e98c81a870db` | not yet reviewed | â€” |  | Tests: support VICTORY_HLE_PROBE_DLL and GNU delayload in hle-probe |
| #1630 | `0bf29939318e` | not yet reviewed | ? | Head changed since `d91d8084750f`; prior not yet reviewed evidence needs revalidation | GPU: adapter choice and logging, presenter backoff, a texture-description ladder, and a display-deferral test a headless session can pass |
| #1629 | `111b77360b82` | not yet reviewed | â€” |  | GPU: zero 256-byte CBV scratch padding and bind null CBVs for empty/invalid views |
| #1628 | `f016efe502f5` | integrated | â€” |  | GPU: log initial states and clear format when texture creation fails |
| #1625 | `1460a851c4b6` | not yet reviewed | â€” |  | CLI: compare runtime shader failures with offline coverage |
| #1623 | `13eece9a0d34` | not yet reviewed | ? | Head changed since `e9254f23d3fe`; prior not yet reviewed evidence needs revalidation | gfx10: keep an instruction's words inline instead of in a heap vector (decode -18..-45%, translation -1..-5%) |
| #1621 | `dec3a4c08a38` | not yet reviewed | ? | Head changed since `53cf4ceed964`; prior not yet reviewed evidence needs revalidation | bound the bytes one part's native blobs unpack to and hand out |
| #1619 | `1f47ee79d0a3` | not yet reviewed | â€” |  | Shader tests: pin IR reachability and PHI invariants |
| #1618 | `e62dd007815a` | not yet reviewed | â€” |  | victory menu: speak Portuguese where Windows does (stacked on #1609) |
| #1617 | `0d79788520a7` | not yet reviewed | â€” |  | [BUNDLE] #1296 (texture mip drop) resolved onto the GPU bundles by antikohza, stacked on #1542 |
| #1614 | `50cbc893b057` | not yet reviewed | â€” |  | Shader tests: a cross-device VALU fuzzer (hardware adapters vs WARP) |
| #1613 | `e3d76bdaaf93` | not yet reviewed | â€” |  | GPU: no render-target view, barrier or clear for a texture without the render-target use |
| #1610 | `47a43871e562` | not yet reviewed | â€” |  | [BUNDLE] Eight more fixes carried from contributors' forks: #745 #776 #787 #805 #845 #852 #915 #918 |
| #1609 | `1b9aad255916` | not yet reviewed | â€” |  | victory doctor: name the known problems a run shows, in plain words (stacked on #959) |
| #1608 | `ced51de047fe` | not yet reviewed | â€” |  | Shaders: single-round v_fma_mixlo_f16 and v_fma_mixhi_f16 to binary16 |
| #1606 | `cc94388e2e2b` | not yet reviewed | â€” |  | Shaders: lower remaining 32-bit integer and bitwise LDS atomics |
| #1603 | `9ec15138f66e` | not yet reviewed | â€” |  | [BUNDLE] Four fixes carried from Peppacodes' fork: #762 #764 #765 #832 |
| #1602 | `94a2e994f9d2` | not yet reviewed | â€” |  | Shader tests: f32/int conversions saturate, take NaN as 0, round to even |
| #1599 | `1f6b392ca8bf` | not yet reviewed | â€” |  | Shaders: 64-bit integer compares on their dwords (AMD orders a shared-dword i64 compare wrong) |
| #1598 | `07b5ecd09bd6` | not yet reviewed | â€” |  | Shaders: lower v_mad_f16 and v_mac_f16 with unfused binary16 product rounding |
| #1597 | `f4e166a84199` | not yet reviewed | â€” |  | Shaders: hidden descriptor words joined with data are an undefined path (+494 programs, stacked on #1402) |
| #1593 | `15c14c142ecb` | not yet reviewed | ? | Head changed since `5996b8740156`; prior not yet reviewed evidence needs revalidation | [BUNDLE] Shader translation speed: 4 PRs, #288 #369 #1033 #1180 (rebuilt on main 70564ae) |
| #1590 | `44e0c31d149b` | integrated | â€” |  | XMA tests: count frames only in the selected stream |
| #1588 | `c36a25b8b780` | not yet reviewed | â€” |  | Shaders: single-round v_fma_mix*_f16 and unfused v_mad_f16/v_mac_f16 |
| #1587 | `8d1d5378408e` | not yet reviewed | ? | Head changed since `4c8eb158d226`; prior not yet reviewed evidence needs revalidation | Shaders: ballot divergent-loop masks at reconverged reads (#1498) |
| #1584 | `56387d3f69d7` | not yet reviewed | â€” |  | GPU: add opt-in idle shader cache trimming |
| #1582 | `ab416b2d727e` | not yet reviewed | â€” |  | Shaders: enforce inactive-lane EXEC rules for DPP16, DPP8 and ds_swizzle_b32 |
| #1578 | `8631598388c2` | not yet reviewed | â€” |  | GPU: the "shader does not translate" warning names where its value was made |
| #1577 | `30ccd2307b7d` | not yet reviewed | â€” |  | Shaders: expand 64-bit inline constants as signed ints and binary64 floats |
| #1576 | `29dd901bf67f` | not yet reviewed | ? | Head changed since `f32980a80aac`; prior not yet reviewed evidence needs revalidation | HLE: implement IUnknown queries for COM objects |
| #1575 | `856f0ce0b1ec` | not yet reviewed | â€” |  | Shaders: sweep typed buffer formats and f16 ALU against exact references |
| #1571 | `9bfd43936615` | not yet reviewed | â€” |  | GPU: add diagnostics for shader dependency wait times |
| #1570 | `2f8898332c27` | not yet reviewed | â€” |  | Shaders: govern pixel discard by last exp VM (Valid Mask) and guard pixel MRT exports |
| #1569 | `23fac9398050` | not yet reviewed | ? | Head changed since `c55866929e05`; prior not yet reviewed evidence needs revalidation | Shaders: lower v_swaprel_b32 |
| #1568 | `dde4577abe4e` | not yet reviewed | â€” |  | Shaders: model 64-bit carry sums and f16 reductions in whole-wave sections |
| #1566 | `f76393984981` | integrated | â€” |  | GPU: SetStablePowerState is accepted and ignored |
| #1565 | `883941b75287` | integrated | â€” |  | GPU: Evict succeeds, as the console's memory is never evicted |
| #1564 | `c6eb3fddffb8` | integrated | â€” |  | GPU: MakeResident succeeds, as the console's memory is always resident |
| #1563 | `93f718d7a94b` | not yet reviewed | â€” |  | Shaders: link mesh and pixel programs whose mesh outputs pass 32 KiB |
| #1560 | `db470df5325d` | not yet reviewed | â€” |  | PE: refuse an import address table inside the headers |
| #1557 | `1504ec9aa446` | integrated-local | `9e0d40a2` | BC7 mode/subset validation; all-zero sentinel preserved; 1/1 files reviewed; source/candidate and full integration gates pass; negative control fails; no-ff merge | Validate BC7 control-word subset counts |
| #1555 | `9fb81e1c17e4` | not yet reviewed | â€” |  | GPU: coalesce nearby shadow-buffer refreshes without spanning huge heap gaps |
| #1554 | `dce05bcc9508` | not yet reviewed | â€” |  | Shaders: flush subnormals in f32 MAD and MAC |
| #1548 | `2d6a0b1af382` | not yet reviewed | â€” |  | GPU: preserve full DYNAMIC descriptor range when host view heap is capped at PORTABLE_VIEWS |
| #1547 | `bdf870eb4728` | not yet reviewed | â€” |  | GPU: invalidate HOST_WRITTEN textures when title memory is written |
| #1546 | `4a77d59f8365` | not yet reviewed | â€” |  | GPU: route indirect dispatch_mesh_x through State::execute_indirect |
| #1543 | `e77f6331cecd` | not yet reviewed | â€” |  | GPU: avoid shift and arithmetic overflow panics in subresource and copyable_footprints (victory-gpu) |
| #1542 | `8df5c34ab76b` | not yet reviewed | ? | Head changed since `b9301c6c330c`; prior not yet reviewed evidence needs revalidation | [BUNDLE] GPU fixes 4: 3 PRs, #1082 #1173 #1265 (rebuilt on main 70564ae) |
| #1541 | `85845466bd8e` | not yet reviewed | â€” |  | CLI: list-diff compares two translate listings program by program |
| #1540 | `1ccae2d0fb76` | not yet reviewed | â€” |  | Shaders: guard both back edges when a conditional branch targets two … |
| #1537 | `9ad1f21b64be` | not yet reviewed | â€” |  | HLE: update shared XMA context words atomically |
| #1536 | `4efcbed6bc72` | not yet reviewed | ? | Head changed since `50cbdd08d4f9`; prior not yet reviewed evidence needs revalidation | Shaders: assemble dword-straddling global_load_ushort and global_load_sshort |
| #1529 | `08bed55f7a38` | not yet reviewed | ? | Head changed since `460d0c1ed845`; prior not yet reviewed evidence needs revalidation | Input: keyboard and mouse played like a PC game (rebindable [controls]) |
| #1525 | `861d3241a253` | not yet reviewed | ? | Head changed since `bbe3b411766c`; prior not yet reviewed evidence needs revalidation | Reuse fenced GPU upload scratch across command lists |
| #1519 | `cb8dd75743eb` | not yet reviewed | â€” |  | Shaders: unpack 16-bit vertex inputs into the low half |
| #1517 | `c72555dbdc84` | not yet reviewed | â€” |  | Input: add Windows DualSense / DualSense Edge support |
| #1516 | `63be4e04826a` | not yet reviewed | â€” |  | Shaders: zero missing typed-buffer components |
| #1514 | `1288c632bb8f` | not yet reviewed | â€” |  | Add opt-in GPU memory synchronization tracing |
| #1511 | `c4a10939a570` | not yet reviewed | ? | Head changed since `4ac9e91c399f`; prior not yet reviewed evidence needs revalidation | [BUNDLE] DXIL emission and container robustness: 3 PRs, #343 #1303 #477 (rebuilt on main 70564ae) |
| #1505 | `d4ef6a596be3` | not yet reviewed | ? | Head changed since `ade80d28a758`; prior integrated evidence needs revalidation | GPU: pipeline workers at the title's priority, not below it |
| #1503 | `5a25850411e0` | not yet reviewed | â€” |  | GPU: fail fast when the host D3D12 runtime is unusable |
| #1501 | `5b390eff2c04` | not yet reviewed | â€” |  | Shaders: DS ops in a program without LDS read 0 and drop writes |
| #1493 | `e96d689e17e1` | not yet reviewed | ? | Head changed since `cc0f47a4f4e5`; prior not yet reviewed evidence needs revalidation | Shaders: declare SamplerComparisonState for immediate comparison samplers and add GPU test |
| #1492 | `8fb97d2fbf4c` | not yet reviewed | ? | Head changed since `ea33d12b3f70`; prior not yet reviewed evidence needs revalidation | [BUNDLE] Shader instruction lowering: 10 PRs, #156 #162 #169 #368 #379 #990 #1021 #1048 #1052 #1053 (rebuilt on main 70564ae) |
| #1489 | `e0a052c2bc07` | not yet reviewed | â€” |  | Shaders: lower image_gather4_c_lz and image_gather4_c_lz_o (MIMG 0x4f, 0x5f) |
| #1488 | `62c4ded0ffba` | not yet reviewed | â€” |  | Shaders: lower ds_add{_rtn}_f32 and ds_cmpst{_rtn}_f32 LDS atomics |
| #1487 | `1455884a28d7` | not yet reviewed | â€” |  | Shader tests: prefer high-performance hardware adapter in emit::host::Host::new |
| #1486 | `ecdb14633b45` | not yet reviewed | â€” |  | Shaders: track source lane in whole-wave Sym::Own to prevent miscompiled shuffles |
| #1483 | `0daabccbdec4` | not yet reviewed | â€” |  | Pick the discrete GPU on laptops; fix driver-dependent and wrong shader lowering |
| #1473 | `826a97fe403d` | not yet reviewed | â€” |  | Shaders: track per-lane export state across mixed pixel-shader joins |
| #1472 | `c00db2cb3833` | not yet reviewed | â€” |  | GPU: flush only overlapping shadow refreshes before Step::GpuWrite |
| #1469 | `ed9f8c6bdf6b` | not yet reviewed | â€” |  | Shaders: lower binary64 class comparisons |
| #1466 | `3ee8820bb5e3` | not yet reviewed | â€” |  | GPU: avoid faulting when zero-filling protected title memory |
| #1463 | `68f23d9e41f9` | not yet reviewed | â€” |  | GPU: cap command lists per submission (VICTORY_SUBMIT_MAX_RUNS) |
| #1462 | `9bab357ac855` | not yet reviewed | â€” |  | CLI: report emitted shader wave sizes in translate |
| #1461 | `f9822de8b349` | not yet reviewed | â€” |  | GPU: handle DXGI Present failures and occlusion |
| #1460 | `3e3a71a874c8` | not yet reviewed | â€” |  | Shaders: keep distinct critical-edge blocks for duplicate branch targets |
| #1457 | `ff3b10e753a3` | not yet reviewed | â€” |  | DirectStorage: keep raw reads running while staging drains |
| #1449 | `1cae27ba7e39` | not yet reviewed | â€” |  | GPU: report host resources held by descriptor heap slots in diagnostics |
| #1448 | `a2700be14795` | not yet reviewed | â€” |  | GPU: invalidate root bindings when a root signature changes |
| #1445 | `139080a9ef9c` | not yet reviewed | â€” |  | HLE reuses freed physical page numbers |
| #1441 | `60bf416db31a` | not yet reviewed | â€” |  | Shaders: preserve the unwritten high half of packed D16 buffer loads |
| #1439 | `05ec5a983be5` | not yet reviewed | ? | Head changed since `f9eb9798e792`; prior not yet reviewed evidence needs revalidation | [BUNDLE] GPU pipeline library, shader cache, draw state and frame wait: 5 PRs, #1000 #626 #642 #174 #1279 (rebuilt on main 70564ae) |
| #1437 | `4cfae26174f4` | not yet reviewed | â€” |  | Shaders: preserve the unwritten high half of packed D16 image loads |
| #1436 | `08a41e838934` | not yet reviewed | â€” |  | Game: check record bounds and offset arithmetic in SGD5 parsing |
| #1435 | `f542636cfb8f` | not yet reviewed | â€” |  | HLE: implement XTaskQueueRegisterWaiter and XTaskQueueUnregisterWaiter |
| #1434 | `098065bb9ec6` | not yet reviewed | â€” |  | Loader: match title DLL filenames ignoring ASCII case |
| #1433 | `b3efa1c0e3b7` | not yet reviewed | â€” |  | Shaders: console flow for groups of several console waves on 32-lane hosts |
| #1432 | `ed397b1a7c17` | not yet reviewed | â€” |  | HLE: set ERROR_PROC_NOT_FOUND for missing pseudo-module exports |
| #1431 | `04b68eea6043` | not yet reviewed | â€” |  | HLE: reopen the emulated OEM key on a null or empty subkey |
| #1430 | `7af465164699` | not yet reviewed | â€” |  | Input: keep keyboard stand-in pad active while XInput slots are idle |
| #1429 | `48bf0d791c95` | not yet reviewed | â€” |  | GPU: persist early-pipeline keys even without a host pipeline library |
| #1428 | `e58d8043849b` | not yet reviewed | â€” |  | GPU: guard OMSetRenderTargets handle expansion on empty, null, or oversized calls |
| #1426 | `2f996192dbd7` | not yet reviewed | ? | Head changed since `3fc262e22ace`; prior not yet reviewed evidence needs revalidation | [BUNDLE] GPU host copies, descriptors, sampler heaps and footprints: 5 PRs, #322 #982 #409 #474 #1206 (rebuilt on main 70564ae) |
| #1425 | `06a543e8a9d3` | not yet reviewed | â€” |  | Shaders: a refusal names the instruction that made its missing value |
| #1424 | `4ef937194602` | integrated | â€” |  | GPU skips empty host submissions |
| #1422 | `b70f51afaeb2` | not yet reviewed | â€” |  | GPU: ignore clear values on non-target textures and avoid caching failed host textures in PLACED_TEXTURES |
| #1420 | `9f57d9c0110d` | not yet reviewed | â€” |  | Shaders: a mask built up in a loop every lane runs the same way (stacked on #1407) |
| #1419 | `c8b7a3d59b1d` | not yet reviewed | â€” |  | HLE: pass non-code LoadLibraryEx mappings outside HostPolicy to the host |
| #1418 | `e0e1790dfffd` | not yet reviewed | ? | Head changed since `39d044fa6509`; prior not yet reviewed evidence needs revalidation | [BUNDLE] GPU fixes (write watch, textures, device, DXGI, objects, primitives, capture): 9 PRs, #553 #153 #270 #1158 #1204 #1220 #488 #1146 #631 (rebuilt on main 70564ae) |
| #1417 | `329600213704` | not yet reviewed | â€” |  | Shaders: lower SV_RenderTargetArrayIndex and SV_ViewportArrayIndex exports |
| #1416 | `54dea150f6f4` | not yet reviewed | â€” |  | Shaders: return zero for inactive compute swizzle sources |
| #1415 | `61f2848bd45e` | not yet reviewed | â€” |  | CLI: reuse the D3D12 host across translation workers |
| #1414 | `c11dce0ce2d7` | not yet reviewed | â€” |  | GPU: avoid clobbering HostList prelude allocator when a list executes twice in one submission |
| #1412 | `86b29fa4cd5d` | not yet reviewed | â€” |  | Shaders: keep scalar comparisons between lane masks uniform |
| #1408 | `a69e32405731` | not yet reviewed | â€” |  | Shaders: lower float wave prefix sums via lane reads |
| #1407 | `e19347488050` | not yet reviewed | â€” |  | Shaders: lane masks holding lanes elsewhere, and mask joins, read as numbers |
| #1406 | `87b42ecd18c9` | not yet reviewed | â€” |  | GPU: fix NULL/UNKNOWN SRV/UAVs on promoted-typeless textures and MSAA/array/3D NULL RTV/DSVs |
| #1405 | `ad2025a433c9` | not yet reviewed | â€” |  | GPU: a view's own copy that shaders feed reads its buffer's copy on the GPU (stacked on #334 and #1019) |
| #1403 | `567e1345fb77` | not yet reviewed | â€” |  | CLI: report shader worker initialization failures |
| #1399 | `e80b16676006` | not yet reviewed | ? | Head changed since `d01f9a6aeb8d`; prior not yet reviewed evidence needs revalidation | Shader & GPU: support merged ES+GS geometry stages (vs_6_6 / gs_6_6) |
| #1398 | `38ddebf95426` | not yet reviewed | â€” |  | Shaders: honor clamp on unsigned VOP3 add and subtract |
| #1397 | `a78fc2cb8f3a` | not yet reviewed | â€” |  | Shader: propagate float values across selects/phis and cache bools across dominators |
| #1396 | `0fbc85a60eb1` | not yet reviewed | â€” |  | GPU: preserve sub-64 KiB alignments and check bounds in allocation_info |
| #1395 | `a8299456fc12` | not yet reviewed | ? | Head changed since `a91e9adde213`; prior not yet reviewed evidence needs revalidation | Shaders: lower s_bfe_u64, s_bfe_i64 and s_bfm_b64 |
| #1394 | `71798281f998` | integrated | â€” |  | HLE: make XUserCompare antisymmetric |
| #1393 | `7a780a0ad707` | not yet reviewed | â€” |  | Shaders: vectorize wide s_buffer_load and global_load rawBufferLoad calls |
| #1388 | `9d84135faadd` | not yet reviewed | â€” |  | GPU: honor command-list 64-bit memory waits without blocking submissions (#700) |
| #1387 | `c77b2533559d` | not yet reviewed | â€” |  | Round v_fma_f16 once in DXIL lowering |
| #1385 | `3e630d357c13` | not yet reviewed | â€” |  | Shaders: narrow mesh output bounds from GS_ALLOC_REQ M0 analysis (#201) |
| #1383 | `a613b9e81b1d` | not yet reviewed | â€” |  | Preserve Win32 errors from DirectStorage OpenFile |
| #1382 | `bf8a9b23ec9c` | not yet reviewed | â€” |  | GPU: batch resource barriers, reuse draw scratch buffers, range-lock copy_simple, and pool list uploads |
| #1374 | `5dd1e71379a5` | not yet reviewed | â€” |  | GPU: read same-list GPU writes in placed buffer-to-texture copies |
| #1372 | `fa22cba5df93` | not yet reviewed | â€” |  | Loader: lazily NOP-patch clwb/monitorx on #UD, emulate rdpid, batch IAT protection, and content-hash cached DLLs |
| #1369 | `12a089334917` | not yet reviewed | â€” |  | GPU: query native format support and multisample quality levels |
| #1365 | `a4c994bf574c` | not yet reviewed | â€” |  | Shaders: hardware test for s_bfm_b32 and s_bitset0/1_b32 |
| #1364 | `26f70987e6ac` | not yet reviewed | â€” |  | Shaders: hardware test for s_mul_i32, s_mul_hi_u32 and s_mul_hi_i32 |
| #1363 | `8e6d0b71b6f9` | not yet reviewed | ? | Head changed since `94874f2c8b08`; prior not yet reviewed evidence needs revalidation | [BUNDLE] XAsync, kernel, lifecycle and DirectStorage HLE: #114 #122 #247 #1098 #186 #373 #512 #451 #1080 #430 #577 #1238 |
| #1362 | `c98c50f9c66e` | integrated | â€” |  | Shaders: hardware test for s_lshl1..4_add_u32 |
| #1361 | `8d6d06c342ef` | not yet reviewed | â€” |  | Shaders: hardware test for the scalar bit-scan, reverse and abs ops |
| #1357 | `cdec26e9dd45` | not yet reviewed | â€” |  | HLE: native 4-motor Xbox impulse trigger haptics via Windows.Gaming.Input with XInput fallback |
| #1356 | `080456154250` | not yet reviewed | â€” |  | GPU: use RwLock for read-mostly tables and add per-HostList VA/view caches |
| #1355 | `2e602026c13a` | not yet reviewed | â€” |  | HLE & Formats: replace miniz_oxide with SIMD-accelerated zlib-rs |
| #1349 | `eb5fea7359f3` | not yet reviewed | â€” |  | GPU: reserve 2^18 dynamic slots on capped view heaps and trim fresh tables (#747) |
| #1342 | `8448e91f535f` | not yet reviewed | â€” |  | Shaders: lower buffer_atomic_{sub,csub,inc,dec,fcmpswap,fmin,fmax} |
| #1341 | `59e48eb90d83` | not yet reviewed | â€” |  | Shaders: lower v_ldexp_f32 (VOP3 0x362) |
| #1340 | `89189ecdbef2` | not yet reviewed | â€” |  | Shaders: mask M0[7:0] on v_movrel* and allow scalar/const src0 on v_movreld_b32 |
| #1339 | `b686bcc5d929` | not yet reviewed | â€” |  | Shaders: support clamp output modifier on v_cvt_pkrtz_f16_f32 |
| #1338 | `e2dc6baec3a6` | not yet reviewed | â€” |  | Shaders: honor op_sel[3], clamp, and omod on v_interp_p2_f16 |
| #1336 | `4eead6e1163c` | not yet reviewed | â€” |  | Shaders: fix RPO resource tracking, SMEM alignment, raw VCC branch, and readlane/writelane semantics (#1249) |
| #1335 | `6ab2927d8a0e` | not yet reviewed | â€” |  | test(shader): translate random GFX10 programs on every test run |
| #1331 | `84845210cc9e` | not yet reviewed | â€” |  | GPU: rebind replaced descriptor heaps by host identity |
| #1329 | `c52be85ed9f7` | not yet reviewed | â€” |  | HLE GameInput supports IUnknown queries |
| #1328 | `91823286336d` | not yet reviewed | â€” |  | Shaders: bitwise wave prefixes as WaveMultiPrefixOp |
| #1327 | `4f2eee6bb595` | not yet reviewed | â€” |  | Shaders: apply bitwise modifiers to v_cmp_class_f32 source operands |
| #1324 | `8f28c35af440` | not yet reviewed | â€” |  | Pipeline: accept existing entry when concurrent cache publication races on Windows |
| #1320 | `70b92d9616c4` | not yet reviewed | â€” |  | GPU diagnostics: skip host pipeline creation by shader hash |
| #1315 | `f74eab0ff299` | not yet reviewed | â€” |  | Shaders, GPU: translate single-stream triangle geometry programs |
| #1314 | `f574bf2085ac` | not yet reviewed | â€” |  | Shaders: lower v_{fmaak,fmamk}_f16, v_cvt_pk_{i16_i32,u16_u32}, and v_lerp_u8 |
| #1313 | `7295d7e85cec` | not yet reviewed | â€” |  | Shaders: return zero for unused image query extents |
| #1312 | `31fd70e195d8` | not yet reviewed | ? | Head changed since `d27838ecd9f3`; prior not yet reviewed evidence needs revalidation | Shaders: mask v_readlane/v_writelane lane index and bypass EXEC on writelane |
| #1306 | `29d78bae7044` | not yet reviewed | â€” |  | Loader: fix thunk block overflow, capacity enforcement, flush errors, and leaks |
| #1304 | `98716d1a58bf` | not yet reviewed | â€” |  | Loader: fix thread shutdown targeting, registry lifetime, and CRT detach ordering |
| #1303 | `428725aaed22` | not yet reviewed | â€” |  | Shaders: bounds-check DXIL header offsets and recognize dword zero padding |
| #1300 | `7383f54e2a5d` | not yet reviewed | â€” |  | Shaders: lower v_{min3,max3,med3}_{i16,u16} and v_mad_{u32_u16,i32_i16} |
| #1298 | `df8f906b1e59` | not yet reviewed | â€” |  | GPU: replace stale descriptors on null CBV writes |
| #1295 | `6cf80f2c7288` | not yet reviewed | â€” |  | Shaders: decode 16-bit integer inline float constants as f32 bits |
| #1292 | `36260aecf994` | not yet reviewed | â€” |  | Shaders: lower v_dot2_{i32_i16,u32_u16}, v_dot4_{i32_i8,u32_u8}, v_dot4c_i32_i8, and v_dot8_u32_u4 |
| #1289 | `003944a950dd` | not yet reviewed | â€” |  | Shader tests: v_pk_mul_lo_u16 keeps each half to 16 bits |
| #1288 | `0d0f5f91f9b0` | not yet reviewed | â€” |  | Shaders: a typed fetch inside a dword reads the aligned pair |
| #1285 | `ac1a6e2a5181` | not yet reviewed | â€” |  | Shaders: fix 16-bit VALU SDWA placement/sext and inline float constants |
| #1283 | `b232b8838d52` | not yet reviewed | â€” |  | Clarify DRED reports when GPU progress is unknown |
| #1281 | `72e794b6294c` | not yet reviewed | â€” |  | GPU/Shader: declare wave64 wave size and strip unsupported video-texture flags |
| #1279 | `4dd4cfc8c006` | not yet reviewed | â€” |  | GPU: an argument clamp Wine's compiler builds too |
| #1278 | `d464b0468d73` | not yet reviewed | â€” |  | Docs: add testing.md |
| #1275 | `cdbe17891855` | not yet reviewed | â€” |  | Shaders: apply v_mov_b32 and v_cndmask_b32 abs/neg modifiers bitwise |
| #1273 | `62946e6eefb4` | not yet reviewed | â€” |  | GPU: mirror placed unordered texture views |
| #1272 | `856e6d889a2d` | not yet reviewed | ? | Head changed since `ccd13f80949a`; prior not yet reviewed evidence needs revalidation | GPU: automatic texture mip-drop on E_OUTOFMEMORY and VICTORY_TEXTURE_MIP_DROP |
| #1271 | `25d9e2add72c` | not yet reviewed | â€” |  | Shaders: infer heap UAVs and mark glc UAV handles globally coherent |
| #1267 | `17548c0d66f3` | not yet reviewed | ? | Head changed since `4cefaac2be47`; prior not yet reviewed evidence needs revalidation | GPU: keep only the bytes shaders may write, each once per submission |
| #1264 | `81c4ffd32b4c` | not yet reviewed | ? | Head changed since `204ec23c8594`; prior not yet reviewed evidence needs revalidation | HLE: map a page scatter in runs, not one page at a time |
| #1263 | `cb6244a17f35` | not yet reviewed | ? | Head changed since `d4ee6a57f581`; prior not yet reviewed evidence needs revalidation | Shader: the loop limit switch works in every build |
| #1262 | `70fa9b72485b` | not yet reviewed | ? | Head changed since `c5aeedd064d9`; prior not yet reviewed evidence needs revalidation | Shaders: double-precision add, mul, fma, min, max, compares and conversions (stacked on #416) |
| #1261 | `d493a131df47` | not yet reviewed | â€” |  | Shaders: an SGPR no instruction writes stays refused above the initialized ones |
| #1259 | `f50ca31ec1e2` | not yet reviewed | ? | Head changed since `5be1fe4b658d`; prior not yet reviewed evidence needs revalidation | Shaders: v_med3 orders -0 below +0, as RDNA2 does |
| #1256 | `18a1073d4982` | not yet reviewed | â€” |  | Shaders: decode cube-array slice and face in image lowering |
| #1254 | `83a8ec3d081c` | not yet reviewed | â€” |  | GPU diagnostics: a black frame's trace keeps the draws that end the frame |
| #1253 | `c880bee775a3` | not yet reviewed | â€” |  | Shaders: follow scalar index arithmetic in waterfall table offsets |
| #1251 | `17a15daf7a8e` | not yet reviewed | â€” |  | Diagnostics: trace command-list waits and memory write progress |
| #1244 | `7c27219d4d6e` | not yet reviewed | â€” |  | Validate VICTORY_PAIRS before translating shaders |
| #1243 | `9472139a2a7c` | not yet reviewed | â€” |  | HLE: the module tree gates for a host that is not Windows |
| #1241 | `f02c70246c43` | not yet reviewed | â€” |  | CI: build and test the host-independent crates on Linux |
| #1238 | `591932ac8ce4` | not yet reviewed | â€” |  | Grow DirectStorage BCPack intermediates as zlib inflates |
| #1237 | `a3346f97d6f4` | not yet reviewed | â€” |  | Shaders: implicit-level samples in compute take their gradients from the quad |
| #1233 | `3575db403056` | not yet reviewed | ? | Head changed since `312ff569264b`; prior not yet reviewed evidence needs revalidation | Report bound descriptor tables accurately in DRED breadcrumbs |
| #1228 | `5a1556e109f9` | not yet reviewed | ? | Head changed since `63e6c9eca4d8`; prior not yet reviewed evidence needs revalidation | GPU: order shader writes on command list pipeline syncs (#732) |
| #1227 | `c3beb4a2628b` | not yet reviewed | â€” |  | Improve shader mask handling and GPU diagnostics |
| #1224 | `9fb62942d3c7` | not yet reviewed | â€” |  | HLE: coalesce split placeholders on map and group runs in map_alias |
| #1223 | `ba532339235f` | not yet reviewed | â€” |  | Shaders: lower v_ldexp_f16 (VOP2 0x3b and VOP3 0x13b) |
| #1218 | `a19107bdfdf2` | not yet reviewed | â€” |  | xtask: wine-test runs the Windows tests on a Linux host under Wine |
| #1217 | `a2e532d155a2` | duplicate | â€” | already local as 3ab92874, with the started reset made unconditional | GPU: the copies timing no longer counts the flush before a title copy twice |
| #1215 | `6950319ce556` | not yet reviewed | â€” |  | GPU: copy back with NtWriteVirtualMemory, no VirtualQuery before a copy into the title (copy back 1.5 s -> 0.13 s) |
| #1213 | `d30bed44dd92` | not yet reviewed | â€” |  | HLE: reject null handle in WinHttpSetOption proxy path and expand net.rs tests (1 -> 17) |
| #1212 | `d10138b98c1b` | not yet reviewed | â€” |  | HLE: expand synthetic unit tests for apu.rs (1 -> 19 tests) |
| #1211 | `7dffb2a6c245` | not yet reviewed | â€” |  | Shaders: lower v_mad_{u16,i16} and saturating clamp on 16-bit integer ops |
| #1210 | `fd5948937bc6` | not yet reviewed | â€” |  | GPU: upload a host-copied texture again once the title writes its memory |
| #1209 | `ae0b48e6daf7` | not yet reviewed | â€” |  | Shaders: fix GFX10 independent a16/g16 gradient dword packing in fields.rs with 21 tests |
| #1207 | `649c6e5274ef` | not yet reviewed | â€” |  | GPU: validate input layouts and aligned vertex offsets in pipeline/host.rs with unit tests |
| #1205 | `d3867e6899df` | not yet reviewed | â€” |  | Formats: add 7 invariant unit tests for xma/tables.rs |
| #1203 | `76291ef38c31` | not yet reviewed | â€” |  | GPU: expand synthetic unit tests for pipeline/make.rs (1 -> 9/12 tests) |
| #1201 | `98b761bd1f18` | not yet reviewed | â€” |  | Loader: fix 4 bounds and cleanup bugs in windows.rs with unit tests |
| #1199 | `8466edd7bb1f` | not yet reviewed | â€” |  | HLE: reject null out in QueryApiImpl before lookup and add xgameruntime unit tests |
| #1196 | `a4a94d720299` | not yet reviewed | â€” |  | HLE: fix self-closing XML element body parsing in runtime.rs and add unit tests |
| #1195 | `ed1d9c75e4e4` | not yet reviewed | â€” |  | Shaders: expand synthetic unit tests for emit/sampler.rs |
| #1194 | `d51dc42a9dfa` | not yet reviewed | â€” |  | HLE: expand synthetic unit test coverage for xstore.rs (1 -> 13 tests) |
| #1193 | `62abb6071a7d` | not yet reviewed | â€” |  | HLE: add synthetic unit tests for storage.rs and host.rs |
| #1192 | `02ea74cc21b6` | not yet reviewed | â€” |  | Loader: fix 4 bounds/overflow bugs in tls.rs and stacks.rs with unit tests |
| #1190 | `d160854be046` | not yet reviewed | â€” |  | Shaders: lower v_frexp_mant_{f32,f16} and v_frexp_exp_i{32_f32,16_f16} |
| #1185 | `4d9b271a868f` | not yet reviewed | â€” |  | Stream shader archive scans in bounded batches |
| #1185 | `4d9b271a868f` | not yet reviewed | â€” |  | Stream shader archive scans in bounded batches |
| #1183 | `15cba99a4ca6` | not yet reviewed | â€” |  | Keep shader metadata variants in the offline translation census |
| #1179 | `f60fb419cbff` | not yet reviewed | â€” |  | Input: choose the loose controls diagram by active gamepad (stacked on #312) [draft] |
| #1178 | `9b8949f4ad71` | not yet reviewed | â€” |  | Diagnostics: trace a DirectStorage request from submission to decoder outcome |
| #1174 | `299bc17db983` | not yet reviewed | â€” |  | HLE: implement IUnknown::QueryInterface and unit tests for xgame objects |
| #1168 | `3247043440ff` | not yet reviewed | â€” |  | GPU: implement SetPredication, SetMarker, BeginEvent, and EndEvent on command lists |
| #1166 | `d57ea6762607` | not yet reviewed | â€” |  | GPU: implement ID3D12GraphicsCommandList::ClearState (slot 0x58) |
| #1157 | `d67e37dcd97e` | not yet reviewed | â€” |  | Input: the mouse as the stand-in pad's camera (VICTORY_MOUSE) |
| #1156 | `2f2952693ab1` | not yet reviewed | â€” |  | Loader: continue past a title's int3 asserts, for finding the next blocker |
| #1155 | `5c375286acb5` | not yet reviewed | â€” |  | hle(display): fall back to no-op deferral when PowerSetRequest returns ERROR_NOT_SUPPORTED |
| #1152 | `70bbe993c62f` | not yet reviewed | ? | Head changed since `e8fd20f85c83`; prior not yet reviewed evidence needs revalidation | GPU: no blending on integer render targets in host pipelines (fixes #806) |
| #1148 | `1661a3e0cfb3` | not yet reviewed | â€” |  | Shaders: lower image_atomic_inc, _dec, _fcmpswap, _fmin and _fmax |
| #1145 | `ea633e723d3c` | not yet reviewed | â€” |  | Shaders: fix duplicate latch folding, unreachable phi edges, and replace_const caching in DXIL bitcode |
| #1144 | `48781527f53f` | integrated | â€” |  | Tests: show when shader differential checks are skipped |
| #1143 | `69a49fa21205` | not yet reviewed | â€” |  | GPU tests: verify a synthetic frame through PresentX |
| #1142 | `bf873ffd3b30` | not yet reviewed | â€” |  | Shaders: support saturating clamp on packed 16-bit integer ops (v_pk_{add,sub,mad}_{u16,i16}) |
| #1138 | `2eb4d0380571` | not yet reviewed | â€” |  | Shader tests: failures name the adapter that ran them |
| #1135 | `af4fef40372e` | not yet reviewed | â€” |  | GPU: detile only texture subresources affected by writes |
| #1133 | `be0fe04e6782` | not yet reviewed | â€” |  | Nvidia/heap cap and barriers |
| #1129 | `d09c31b4c144` | integrated | â€” |  | GPU: give pipeline workers room for NVIDIA's shader compiler |
| #1126 | `bb65bc32a98e` | not yet reviewed | â€” |  | GPU: release descriptor tables after a rejected bind |
| #1124 | `b3d8f95cab44` | not yet reviewed | â€” |  | Shadows: batch GPU copy-back writes safely |
| #1123 | `4e32dfc7866e` | not yet reviewed | â€” |  | Validate XGameSave blob count before reading |
| #1114 | `89f3d3c574d1` | not yet reviewed | â€” |  | CLI: replay selected runtime shader hashes in offline translation |
| #1113 | `f4374838c3dd` | not yet reviewed | â€” |  | Formats: lay out packed RGB textures as horizontal texel pairs |
| #1112 | `4f5fa479c962` | not yet reviewed | â€” |  | Shaders: round normalized MTBUF loads without Int64ShaderOps |
| #1111 | `5f8800dd93d4` | not yet reviewed | â€” |  | fix(loader): preserve protection regions when binding unaligned imports |
| #1110 | `d2ce712926b2` | integrated | `6db9bb08` | in the branch | fix(hle): account for save replacements before clamping quota |
| #1106 | `0a1f602cef1f` | integrated | `e3d1b13e` | in the branch | test(shader): report forced WARP runs as software only |
| #1095 | `d25783b8d87d` | not yet reviewed | â€” |  | Shaders: compute signed overflow SCC for s_add_i32, s_addk_i32, and s_sub_i32 |
| #1090 | `9a2513e56526` | not yet reviewed | ? | Head changed since `0b7067fb6deb`; prior not yet reviewed evidence needs revalidation | Shaders: lower constant 64-bit integer compares without INT64 |
| #1088 | `59ab6b736af3` | not yet reviewed | â€” |  | Fix compute shader signed overflow and paired LDS writes |
| #1084 | `48a41fb850ac` | deferred | â€” | restructures descriptor inference in the contested function | Shaders: a select or carry-in on an untracked mask tests this lane's bit |
| #1083 | `dbea3a10d8f1` | not yet reviewed | â€” |  | Shaders: name what a scalar memory read is, and which descriptor dword it reads |
| #1076 | `6a4cc18bf55c` | not yet reviewed | â€” |  | Fix NVIDIA Ada Lovelace (`RTX 4060`) D3D12 compatibility, 2s TDR submissions, and 16 GB host OOM |
| #1065 | `37f838d01548` | not yet reviewed | â€” |  | Shaders: use validator 1.8 for compute derivatives |
| #1059 | `3530ba983ede` | not yet reviewed | â€” |  | Loader: VICTORY_PATCH writes bytes into a loaded module after relocation |
| #1058 | `8770e1905c55` | integrated | â€” |  | CI: serialize Windows workspace tests |
| #1056 | `417f4e30d22f` | not yet reviewed | â€” |  | Shaders: scalar loads from a root view's address read its memory |
| #1055 | `21d1b4682735` | integrated | â€” |  | Shaders: scalar buffer loads from a UAV read it raw |
| #1046 | `5f9a0d247df7` | not yet reviewed | â€” |  | Shaders: hardware test for the s_cmpk_* scalar comparisons |
| #1033 | `266f6719266a` | not yet reviewed | â€” |  | victory-shader: build the per-instruction read/write lists once (ir::build -13%) |
| #1032 | `b38a3c9fa1a6` | not yet reviewed | â€” |  | GPU: cap oversized sampler heaps |
| #1024 | `67125b3a891a` | not yet reviewed | â€” |  | GPU: copy-back writes only the GPU's bytes and marks only unchanged pages current |
| #1023 | `836490b9690c` | not yet reviewed | â€” |  | gpu: implement ordered 32-bit and 64-bit command-list memory waits |
| #1017 | `daf2e8d9083e` | not yet reviewed | ? | Head changed since `b0933f503856`; prior not yet reviewed evidence needs revalidation | Fix the Linux build and clippy gates |
| #1014 | `9d1c2999324f` | not yet reviewed | â€” |  | victory-shader: lower v_permlane16_b32 and v_permlanex16_b32 |
| #1013 | `cbbb17ea6962` | not yet reviewed | â€” |  | speed up shader structurisation and prioritize live pipeline compiles |
| #1012 | `0ba5b998e6df` | integrated | `8a37c05c` | in the branch | Shaders: descriptors joined by nested control flow |
| #1010 | `caa937ca0d52` | not yet reviewed | ? | Head changed since `79c9b93fb77d`; prior not yet reviewed evidence needs revalidation | GPU: copy title memory into host memory without a system call |
| #1009 | `48a40d2c0110` | not yet reviewed | ? | Head changed since `7ea91124f956`; prior not yet reviewed evidence needs revalidation | xmem: keep the space below 4 GiB for the title's 32-bit memory |
| #1005 | `72b9709675e5` | not yet reviewed | â€” |  | Fix 12 verified regressions in shader translation, runtime, caching and texture handling |
| #1004 | `67891e2341aa` | not yet reviewed | ? | Head changed since `cbc8c33dc679`; prior not yet reviewed evidence needs revalidation | Shaders: lower image_get_lod on cube textures |
| #1002 | `b6db98554243` | not yet reviewed | â€” |  | victory-shader: hash the lowering tables with Fx instead of SipHash (lowering -16%) |
| #999 | `3ca94561506c` | not yet reviewed | â€” |  | Shaders: search lane conditions once and redo a lowering once |
| #997 | `7d1a0a585b18` | rejected | â€” | closes the caller WinHTTP handle where the API keeps it | HLE: prevent automatic WinHTTP redirects |
| #988 | `089bd768d5f1` | integrated | â€” |  | GPU: say when a fault lies past the end of its title memory copy |
| #984 | `501164509d22` | not yet reviewed | â€” |  | Shaders: split flat byte-offset accesses to structured buffers |
| #983 | `1edc06c7f288` | not yet reviewed | ? | Head changed since `486b54eda05f`; prior not yet reviewed evidence needs revalidation | GPU: a full early pipeline list drops the oldest runs' keys |
| #982 | `b4abe5db60f1` | not yet reviewed | â€” |  | GPU: pool upload buffers across command list resets |
| #970 | `60222b335719` | not yet reviewed | â€” |  | GPU: aspect-aware concrete format resolution for typeless views (fixes #882, #876) |
| #969 | `f3457f8350e9` | not yet reviewed | â€” |  | GPU: load the Agility SDK through ID3D12SDKConfiguration1 (#749) |
| #967 | `66d97eea7ea3` | integrated | `ee173bad` | in the branch | GPU: choose an adapter that can run the title, and name the ones that cannot |
| #961 | `31568ced3e44` | not yet reviewed | â€” |  | Shaders, GPU: draw mesh pipelines without mesh shaders through a compute fallback (#3) |
| #960 | `8c12a98be191` | not yet reviewed | â€” |  | Game picker |
| #959 | `6614cc26b7e4` | held | â€” | names the GPU from adapter 0 but reads capabilities from the default device | victory: a menu when opened from Explorer, instead of help that closes at once |
| #958 | `7d0302b2d85e` | not yet reviewed | â€” |  | GPU: unit tests for the OPTIONS, ARCHITECTURE and MULTISAMPLE feature queries |
| #956 | `db86e1e387a6` | not yet reviewed | â€” |  | GPU: support multi-argument ExecuteIndirect signatures with root constants |
| #952 | `faccc4ef5645` | not yet reviewed | â€” |  | GPU: log the host's video-memory use and its budget |
| #946 | `c1596049299b` | not yet reviewed | â€” |  | victory-shader: preserve helper-lane values on pixel waveReadLaneFirst |
| #943 | `12cfd38273db` | not yet reviewed | â€” |  | GPU: use VirtualQuery region fallback in copy_mapped and write_mapped |
| #939 | `d4306f4753fd` | integrated | `4a8b49ec` | in the branch | HLE: XSystemGetXboxLiveSandboxId refuses a buffer under 16 bytes |
| #933 | `9914ea4be88a` | not yet reviewed | â€” |  | GPU: report allocated mips from GetDesc |
| #929 | `1f23c491ce25` | not yet reviewed | ? | Head changed since `2b8756d301c4`; prior not yet reviewed evidence needs revalidation | Audio: a silent playback endpoint when the host has no render device |
| #927 | `bf0f22a1867f` | not yet reviewed | ? | Head changed since `4ce99c162413`; prior not yet reviewed evidence needs revalidation | GameInput: held keys are reported in the order they were pressed, as documented |
| #926 | `ae9158a36562` | not yet reviewed | ? | Head changed since `3cdbc0cabbe9`; prior not yet reviewed evidence needs revalidation | Shaders: 16-bit compares read the low halves whatever op_sel says, as RDNA2 does |
| #708 | `a9698e7488ab` | integrated | `79381585` | in the branch | fix(config): keep the data directory out of the cache root |
| #707 | `7ca777e55543` | not yet reviewed | â€” |  | Shaders: lower tbuffer_store_format_* that need no conversion |
| #703 | `712f89096fd7` | deferred | â€” | 1,000-line BCPack restructure competing with the merged bucket lookup | Formats: eliminate BCPack per-call allocations and optimize streaming decode |
| #697 | `fa8d0845f7a4` | not yet reviewed | â€” |  | Shader: backward dead-code elimination in DXIL bitcode emission |
| #695 | `b91ff3c4829d` | integrated | `958693cf` | in the branch | gpu: VICTORY_D3D12_WARP=0 opts out of the software adapter |
| #693 | `1c94839f9a15` | deferred | â€” | reviewed version left shadows in COPY_SOURCE with no transition back | gpu: move shadow buffers to DEFAULT VRAM with pooled staging transfers |
| #690 | `a35440366602` | not yet reviewed | â€” |  | gpu: expand console primitive topologies into host list index buffers |
| #688 | `2e42e8bf492d` | not yet reviewed | â€” |  | GPU: take back the dynamic descriptor slots of lists that ran when a heap runs short |
| #687 | `73eba29bfaea` | not yet reviewed | â€” |  | hle: track title working-set memory, bound xasync threads, and map XUser devices |
| #686 | `741d675c9028` | integrated | `aa0401c2` | in the branch | docs: check archive integrity after an early boot breakpoint |
| #684 | `9c2a364dde5d` | integrated | `2e672a6d` | in the branch | Shaders: find the PC test compiler independently of the validator |
| #682 | `e1bd18f0e6d6` | not yet reviewed | â€” |  | Shaders: v_mul_legacy_f32 treats denormal operands as zero |
| #679 | `b96c85d47d7c` | not yet reviewed | â€” |  | run: refuse capture and dump paths inside the repo |
| #677 | `0284037e045c` | integrated | `de6e76fc` | in the branch | Build: thin LTO for release and diag builds |
| #674 | `b9e5f2e9ff3d` | integrated | `d28b71de` | in the branch | HLE (xg): validate null tiling output, slice bounds, and checked copy spans |
| #673 | `3ba3f79c1952` | integrated | `9a6bc8e0` | in the branch | Make translate failure examples stable across workers |
| #668 | `5e9d60d134e2` | integrated | `3ad7f8ef` | in the branch | Shaders: support f16 omod and saturate/apply source modifiers in v_cvt_{u16,i16}_f16 |
| #663 | `d4dcd0b63ad1` | integrated | `3c1aca6c` | in the branch | media: apply the local-file rule to the source reader, sink writer and MFCreateFile |
| #662 | `37d52cb78ddc` | not yet reviewed | â€” |  | DSP: refuse XDspDisconnect with active streams; XDSP_E_NO_DEVICE_RESOURCES past the stream limit |
| #660 | `19b02ece91ef` | not yet reviewed | â€” |  | Shaders: drop a wave64 mask whose low dword is seen with two high dwords |
| #659 | `98fe8fe840ba` | integrated | `2ece2f86` | in the branch | Shaders: refuse per-lane reads of mesh v5 in v5 writes and after joins |
| #656 | `3b001ed80c97` | integrated | `39a46807` | in the branch | Input: report the character each held key types |
| #652 | `e3448dc4e976` | integrated | `6a836d63` | in the branch | GPU: use selected mip extents for MSAA resolves |
| #651 | `d615ee0fe0f9` | integrated | `ce478cfb` | in the branch | GPU: select the readable format and plane for depth/stencil shader views |
| #644 | `4eb145be7089` | integrated | `b33b0fed` | in the branch | GPU: spill oversized host root signatures' user data to root CBVs |
| #642 | `4d4da75f64c2` | not yet reviewed | ? | Head changed since `bdef690c7054`; prior integrated evidence needs revalidation | GPU: translate enhanced texture barriers in ID3D12GraphicsCommandList7::Barrier |
| #638 | `16d1aad4d1a9` | not yet reviewed | â€” |  | GPU diagnostics: find what the world hang waits for, and look under a black frame |
| #633 | `90bc07545837` | not yet reviewed | â€” |  | HLE: GetModuleFileNameA answers in the ANSI code page |
| #628 | `717e732f68c5` | not yet reviewed | â€” |  | Shaders: lower ds_min_f32 / ds_max_f32 (and _rtn) with RDNA2's NaN rules |
| #626 | `df9ab3c69172` | not yet reviewed | â€” |  | GPU: fall back to standard root signature flags when host runtime rejects 0xC01/0xC00 |
| #616 | `e930351bebd6` | integrated | `1af23c07` | in the branch | GPU: rectangle draws check their bounds without 32-bit overflow |
| #613 | `7da55befe681` | not yet reviewed | â€” |  | HLE: serialize device callbacks across delivery threads |
| #609 | `6d8c72bc04ee` | not yet reviewed | â€” |  | GPU: retain and retire released host textures, staging, shadow buffers, and descriptor resources across submissions |
| #606 | `05babc08e7b7` | not yet reviewed | ? | Head changed since `95b58c1351de`; prior not yet reviewed evidence needs revalidation | Shaders: ds_swizzle_b32 offsets from 0xC000 are the rotate and FFT modes |
| #602 | `380dab26ccbc` | not yet reviewed | â€” |  | GPU diagnostics: report retained copy memory |
| #601 | `30b962565ff2` | not yet reviewed | â€” |  | GPU: bound texture upload and capture slices to the allocated buffer |
| #599 | `38234626c7c1` | not yet reviewed | â€” |  | Shaders: lane-indexed ops on a wide host wave use the own console wave |
| #596 | `be92188eee0a` | not yet reviewed | â€” |  | Shaders: lower v_cvt_pknorm_{i16,u16}_{f32,f16} with exact rounding |
| #595 | `dfac3b46f19b` | not yet reviewed | â€” |  | GPU: run submissions asynchronously on a host fence timeline |
| #594 | `c3f1d7443d5f` | not yet reviewed | â€” |  | GPU: track texture subresource states |
| #590 | `52ceb8f7173d` | duplicate | â€” | already local as 4777058a | HLE: reject duplicate blob changes in one save update |
| #589 | `ab2e7cc3f648` | integrated | `005d6219` | in the branch | fix(shader): skip unused image atomic descriptor reconstruction |
| #587 | `1052a90a4072` | not yet reviewed | â€” |  | Shaders: guard mesh output bounds, init all primitives, and omit sync on divergent s_barrier |
| #586 | `9b5e23997216` | not yet reviewed | â€” |  | Shaders: honor the mesh allocation request counts |
| #583 | `586d7da39383` | not yet reviewed | ? | Head changed since `07bae5d505f2`; prior not yet reviewed evidence needs revalidation | fix(gpu): respect indirect count buffer and bind null root views |
| #580 | `7ca84cc13670` | integrated | `2db7897e` | in the branch | Docs: add a contributor onboarding guide |
| #579 | `1658c8452a4a` | integrated | `13008f98` | in the branch | GPU: VICTORY_SKIP_WAVE64 diagnostics switch for wave64 programs |
| #578 | `fb7facde2d44` | not yet reviewed | â€” |  | GPU: refuse wave64 mesh and compute programs on hosts without 64-lane waves |
| #577 | `40816378b598` | integrated | `e532ec98` | in the branch | dstorage: retain queued file references and validate null out pointers |
| #575 | `69e44f59cd9c` | rejected | â€” | does not compile: Mutex<Vec<ID3D12Resource>> is not Send | GPU: release resources the CPU is done with behind the queue |
| #574 | `253b9db4a8f3` | duplicate | â€” | already at gameinput.rs:1121 | HLE: write at-rest gamepad state for non-pad readings |
| #573 | `1d09f19b2102` | not yet reviewed | â€” |  | Shaders: lower ds_bpermute_b32 |
| #562 | `4cd352fd51c0` | not yet reviewed | ? | Head changed since `9ca05334eded`; prior not yet reviewed evidence needs revalidation | GPU: reuse immutable descriptor tables across command lists [draft] |
| #561 | `3c02d8cf8947` | not yet reviewed | â€” |  | xmem: commit physical pages in pieces, and report the real count |
| #559 | `15eafe275313` | not yet reviewed | â€” |  | GPU: track mip states for 2D color targets |
| #551 | `967051859c6a` | not yet reviewed | â€” |  | Shaders: lower compute group info, wave id in workgroup, and sample coverage |
| #545 | `d4c0e846b731` | rejected | â€” | a second divergent lint gate on stable | xtask: add qa command to run policy, rustfmt, and clippy checks |
| #544 | `f861613328da` | not yet reviewed | â€” |  | HLE: report queue termination after running callbacks |
| #542 | `31a4692a1581` | not yet reviewed | â€” |  | Shaders: keep buffer offsets below 2 GiB (NVIDIA + vkd3d-proton world device loss) |
| #541 | `cbd84266d515` | not yet reviewed | â€” |  | GPU: retain D3D12 object names in private data |
| #537 | `2021e0f26970` | integrated | `67f5d656` | in the branch | GPU: name the switch when a removal has no device-removed data |
| #535 | `f6585cc0b392` | duplicate | â€” | already at main.rs:248 with its test | App: enforce asset policy check in default_paths |
| #530 | `951ceb1eb8da` | integrated | `2f0a1740` | in the branch | GPU diagnostics: show one of the title's render targets in the window |
| #529 | `609c74fbd59a` | duplicate | â€” | already local as 3a957618 | PE: preallocate symbol_key in CodeViewInfo |
| #523 | `001226617527` | not yet reviewed | â€” |  | Build: compile and test the workspace on Linux again |
| #522 | `882d4f0f5fbe` | duplicate | â€” | already integrated as b26e3c22 | HLE: validate null package identifier in XPackage methods |
| #521 | `75b1b8ff6550` | not yet reviewed | â€” |  | Shaders: model the compute thread-group info SGPR's wave count |
| #517 | `0749a8ea1b35` | not yet reviewed | ? | Head changed since `bab15db5c7a1`; prior not yet reviewed evidence needs revalidation | CLI: report incomplete shader directory scans |
| #512 | `829f774722a6` | duplicate | â€” | already at kernel.rs:166-176 for ordinals and pseudo-module resolution | HLE: resolve ordinals on pseudo-module handles |
| #511 | `02dc7b25b4fe` | not yet reviewed | ? | Head changed since `ff96ffe9644c`; prior not yet reviewed evidence needs revalidation | Shaders: coalesce aligned constant-buffer loads |
| #509 | `daebb5254b58` | not yet reviewed | ? | Head changed since `bfc60cfab5fb`; prior not yet reviewed evidence needs revalidation | GPU: add an opt-in execution performance census |
| #506 | `df7b7548809c` | not yet reviewed | ? | Head changed since `e80778cffa94`; prior not yet reviewed evidence needs revalidation | CLI: reject zero shader translation workers |
| #505 | `d4c8abf932a1` | integrated | `b9f70206` | in the branch | Tests: add synthetic Xbox runtime conformance under Wine |
| #503 | `43710396895e` | not yet reviewed | â€” |  | HLE: pin a call's task queue at XAsyncBegin |
| #500 | `bdbaac636524` | integrated | `a5ccbffd` | in the branch | GPU diagnostics: pick capture frames by draw count |
| #499 | `5c211d04035c` | not yet reviewed | â€” |  | shader: lower v_sat_pk_u8_i16 |
| #498 | `4f48c59608ca` | not yet reviewed | â€” |  | shader: lower v_movrelsd_2_b32 and v_swaprel_b32 |
| #492 | `bf41b81fb085` | not yet reviewed | ? | Head changed since `c7bb83fd37fa`; prior not yet reviewed evidence needs revalidation | GPU: substitute R9G9B9E5_SHAREDEXP render targets and UAVs with R16G16B16A16_FLOAT |
| #491 | `027926b162ac` | not yet reviewed | â€” |  | GPU: remember pipelines whose creation crashed the process (VICTORY_PIPELINE_JOURNAL) |
| #486 | `7f6cd9e6e7cb` | not yet reviewed | ? | Head changed since `6179b10dedff`; prior rejected evidence needs revalidation | shader: opt the test binaries into an app-local Agility SDK runtime |
| #485 | `d454508f71df` | not yet reviewed | â€” |  | GPU: preserve compatible read states for read-only depth targets |
| #482 | `c91e6b6b20e3` | integrated | `42c80756` | in the branch | App: reject wrapping RVAs in VICTORY_WATCH |
| #478 | `7ac2760d317c` | integrated | `93324593` | in the branch | Audio capture: guard WASAPI timestamp errors/underflow, null 0-frame slice UB, unbounded silence allocation, and WAV header u32 overflow |
| #477 | `79c27a93b105` | not yet reviewed | ? | Head changed since `bd59ba346da7`; prior integrated evidence needs revalidation | Shaders: fix DXIL validator UB on null error blobs, unchecked GetStatus, and COM/DLL handle leaks |
| #472 | `f200a2a101e2` | not yet reviewed | â€” |  | GPU: grow the write snapshot in place, and cap idle pool memory |
| #470 | `e9f870735ad8` | not yet reviewed | â€” |  | shader: lower wave prefixes of min and max |
| #465 | `bdced0f38f92` | not yet reviewed | â€” |  | Shaders: lower v_dot2_f32_f16 and v_dot2c_f32_f16 |
| #463 | `f47a36e83f86` | not yet reviewed | â€” |  | GPU: load an app-local D3D12 Agility SDK runtime when present |
| #458 | `360f38256dea` | not yet reviewed | â€” |  | Fix GPU hangs, memory leak, and mesh shader exports; render intro cre… |
| #457 | `08b681c2df8f` | not yet reviewed | â€” |  | fix(shader): accept overwritten VGPR inputs in counted indexed writes |
| #454 | `85c04d129a0a` | rejected | â€” | patches code with no bounds check | Loader: opt-in VICTORY_NO_WATCHDOG hook for the title's watchdog (build 1.0.2.1) |
| #453 | `88b34a447350` | deferred | - | reviewed 2026-11-25: 3 commits (`21c585aa`, `87e9d414`, `88b34a44`) probed against `merge/up2` at `b2f08b94`. `21c585aa` conflicts in `emit/lower.rs` at 2 sites (resolvable), `87e9d414` conflicts at 5 more in `fn interp` plus 3 in `io.rs`; the two rewrote the same interpolation emission that upstream then rewrote again (3,877 changed lines in `lower.rs` between the PR base `ffe310eb` and the integrated `940d32f6`). Merging by hand would move `v_interp_p1/p2` evaluation into unreviewed shader semantics with no GPU check behind it, and a wrong i/j is a silent wrong pixel. Held: needs a re-port onto `940d32f6` and a value check against the hardware before it is accepted. Cherry-pick aborted, tree left at `b2f08b94`. | Shaders: pixel barycentrics through SV_Barycentrics (values and computed interpolation) |
| #452 | `5bcacf5cfdce` | not yet reviewed | â€” |  | Shaders: spin-wait loops stop after 4096 iterations; loop limit in release builds |
| #450 | `11da59948592` | not yet reviewed | â€” |  | GPU: preserve typed null render target slots |
| #446 | `881caef98fa0` | integrated | `efff47d2` | in the branch | HLE: report required size on string getter buffer errors |
| #443 | `54f4329932b1` | not yet reviewed | â€” |  | Audio: keep XMA frames split across rolling inputs |
| #441 | `157e0ceea175` | not yet reviewed | â€” |  | run: let victory.toml pick the console |
| #439 | `28aca3b718f7` | not yet reviewed | â€” |  | Shaders: one console wave sharing LDS runs in the console flow on 32-lane hosts |
| #435 | `657efd827942` | not yet reviewed | â€” |  | GPU: coalesce identical write declarations within a submission |
| #434 | `48f92e23c152` | not yet reviewed | â€” |  | Shaders: a wave32 program's mask read picks its half of a 64-lane ballot |
| #430 | `129d2897c43f` | integrated | `b69d6856` | in the branch | DirectStorage: Close completes what was submitted |
| #428 | `a312333c28eb` | integrated | `ca8d2253` | in the branch | GameInput: next and previous readings of a disconnected device fail |
| #427 | `16d395ccc097` | not yet reviewed | â€” |  | GPU: say in device-removed reports which user data is a table |
| #425 | `012ce71c5a8c` | integrated | `2c9c3325` | in the branch | Game: validate DXBC containers in SGD5 records |
| #421 | `83b2037e40ea` | integrated-local | `5cb63e52` | Extended virtual-path cleanup; fixed source drive-root separator loss; retained local parent clamp; dynamically allocated test letters; full integration gate passed; no-ff merge | HLE: clean dot parts in extended virtual-drive paths |
| #420 | `b4b9498abc00` | not yet reviewed | â€” |  | GPU: include root-selected entries in rebuilt descriptor tables |
| #416 | `16d21661cf55` | not yet reviewed | â€” |  | shader: implement v_cvt_f32_f64 instruction lowering |
| #415 | `6924b095e3bf` | integrated | `e9aecbd8` | in the branch | GPU: use native root type codes in the shader cache |
| #412 | `d2c7d4a329ab` | integrated | `1cbf4b9c` | in the branch | HLE: validate XPackage chunk selectors |
| #411 | `16a5893ea7fb` | not yet reviewed | â€” |  | GPU: require only the descriptor heaps used by the pipeline |
| #409 | `340499c44c7f` | not yet reviewed | â€” |  | GPU: match D3D12 copyable texture footprints |
| #402 | `022e9c82f0ee` | not yet reviewed | â€” |  | GPU: find a buffer holding an address past many smaller ones; reuse loose windows that hold a range |
| #398 | `22cc74ef4cad` | duplicate | â€” | already local as d0462736 | HLE: report the single supported offline user |
| #395 | `8c3e0d4ca82b` | not yet reviewed | â€” |  | shader: unpack compressed f16 pixel exports |
| #391 | `a3f219ecbdb7` | not yet reviewed | â€” |  | Shaders: refuse LDS past the host's group limit by name [draft] |
| #390 | `e125477cbd6d` | integrated | `589ea43d` | in the branch | Crash log: a panic carries our Rust stack [draft] |
| #388 | `e54b6756f5b2` | not yet reviewed | â€” |  | GPU: refresh unwatched textures without a region query; detile into cached memory |
| #387 | `0899b7d7fd97` | not yet reviewed | â€” |  | GPU: preserve execution ordering for buffer copies [draft] |
| #385 | `14c46b2e9944` | not yet reviewed | ? | Head changed since `ec086dca9069`; prior not yet reviewed evidence needs revalidation | Shaders: track every incoming edge of a lane-mask phi |
| #379 | `5ecb6abe769d` | not yet reviewed | â€” |  | Shaders: lower v_div_fixup_f16 |
| #378 | `2392dfd79a19` | integrated | `9a68bad9` | in the branch | CLI: include runtime shader hashes in the translation listing |
| #374 | `9d6bc0c7c681` | not yet reviewed | â€” |  | XAsync: a wait on a failed call during its cleanup waits for the status |
| #369 | `341782191c55` | not yet reviewed | â€” |  | Shaders: eliminate heap allocations during CFG traversal |
| #364 | `e503756597ca` | integrated | `6de0fa33` | in the branch | XMA: accelerate VLC decoding with an eight-bit prefix lookup |
| #347 | `197c7acf1fb0` | not yet reviewed | â€” |  | GPU: ask for an empty pipeline library with a null blob |
| #343 | `9459c12d2e3a` | integrated | `fd228fb5` | in the branch | Shaders: reject overlapping DXBC container parts |
| #342 | `dd1d0bca4ca2` | not yet reviewed | ? | Head changed since `4950d6171111`; prior not yet reviewed evidence needs revalidation | Shaders: lower partial-channel typed buffer stores |
| #334 | `c99e4290b628` | not yet reviewed | â€” |  | GPU: transition host copies of title memory between shader writes and reads |
| #332 | `5a5c155f69c3` | not yet reviewed | ? | Head changed since `5048d2d13f81`; prior not yet reviewed evidence needs revalidation | Shaders: lower partial-channel image_store |
| #329 | `02564c5d9561` | not yet reviewed | â€” |  | Shaders: recover record counts for descriptor-only structured buffers |
| #328 | `62baee73d04d` | not yet reviewed | ? | Head changed since `91e7742f9852`; prior integrated evidence needs revalidation | Shaders: lower MSAA image_load and image_get_resinfo |
| #324 | `c37793a059e3` | integrated | `f9eb3bf0` | in the branch | fix(pe): detect incomplete TLS callback arrays |
| #321 | `d674d9e2bbaa` | not yet reviewed | â€” |  | GPU: honour scheduled frame-event offsets |
| #320 | `41369c90250d` | not yet reviewed | â€” |  | Shaders: lane reads and writes where lanes branch differently run in the console flow |
| #319 | `b36bd4c1d655` | integrated | `f9800117` | in the branch | GPU: end a late flip wait on the tick it missed, not the next |
| #317 | `5a7ef383149e` | not yet reviewed | ? | Head changed since `8133754ea5af`; prior integrated evidence needs revalidation | Shaders: support cube dimension in descriptor_data |
| #312 | `191e335d7848` | deferred | â€” | held as deferred with concrete review findings | HLE: SDL3 gamepad backend for the gameinput shim |
| #310 | `1ea6427f0507` | integrated | `6d897c25` | in the branch | Policy: resolve cache paths inside Windows AppContainer |
| #308 | `6b303847ed87` | not yet reviewed | â€” |  | Shaders: follow lane writes and scalar steps in whole-wave sections |
| #301 | `1af462451537` | not yet reviewed | â€” |  | GPU: non-stream graphics PSOs, stream subobjects 25..=28, texture-to-buffer readbacks, and sRGB view preservation |
| #292 | `e065026b6ed8` | not yet reviewed | â€” |  | Shader: lower WQM QuadAny, clamp LDS accesses, and guard loop budgets & barriers |
| #290 | `ae6ebd9ec270` | not yet reviewed | â€” |  | HLE tests: cover Windows path rules on every host |
| #286 | `6afdb58eeb87` | integrated | `869cb862` | in the branch | Shaders: accept known D3D_MIN_PRECISION signature elements |
| #284 | `fa5189da66a9` | not yet reviewed | â€” |  | Shaders: RDNA2's legacy multiply-adds are the fused ones |
| #280 | `7ecc6209b8b3` | integrated | `25c43bc2` | in the branch | README: a hard drive keeps the first screen black for ~90 s |
| #279 | `02f0e29f8031` | not yet reviewed | â€” |  | GPU: name the commands at a DRED stop, and dump translated shaders |
| #278 | `d063cee543a2` | not yet reviewed | â€” |  | GPU: answer format-info and virtual-address feature queries |
| #275 | `96b282274c7a` | not yet reviewed | â€” |  | GPU: atomic adaptive PSO wait budget and non-blocking pipeline library saves |
| #269 | `9356beeae173` | not yet reviewed | â€” |  | Shaders: scalars leave divergent loops and branches as the wave's last path left them |
| #265 | `964e3adc6a62` | not yet reviewed | â€” |  | Shaders: private (scratch) memory as a per-lane array |
| #261 | `a2243adc60d8` | integrated | `ee4a9ae6` | in the branch | GPU: copy-back puts back only what shaders wrote, in two more cases |
| #259 | `e232d3d7a210` | not yet reviewed | â€” |  | Shaders: query a descriptor's level count only when word 3 is read |
| #250 | `6c2c14146c9e` | not yet reviewed | â€” |  | GPU & CLI: configurable/uncapped FPS limit, high-res pacing, high-refresh modes, and tearing swapchain |
| #244 | `7f95b87ea945` | integrated | `45a4455e` | in the branch | Shaders: share and serialize WARP host operations across threads |
| #242 | `6cb8fbe6b4d4` | not yet reviewed | â€” |  | GPU: make ExecuteIndirect clamping countBuffer-aware and non-destructive |
| #240 | `723f4a296a00` | not yet reviewed | â€” |  | DSP: convolve reverb streams on the host |
| #237 | `203d9f38407e` | not yet reviewed | â€” |  | HLE: offload async saves, reuse audio buffers, pin APU heap, and decouple input rumble/dialogs |
| #236 | `38bf927807fd` | not yet reviewed | â€” |  | XAsync: a canceled pending call's work touches neither freed state nor the next call |
| #234 | `e39fc9cbe8b9` | not yet reviewed | â€” |  | HLE: overlapped DirectStorage I/O and persistent xmem section views |
| #233 | `ededb1babb4d` | not yet reviewed | â€” |  | Shaders: wave64 compute on 32-lane hosts, in halves and in the console flow |
| #230 | `067ece6b1194` | not yet reviewed | â€” |  | GPU: skip unchanged texture uploads via prelude copy recording |
| #228 | `09ff84b6a7cb` | integrated | `f53da6cc` | in the branch | XGameSave: a failed create leaves a null handle, as documented |
| #224 | `b87a9682e346` | integrated | `65c02b0e` | in the branch | HLE: report the host's commit headroom before the title starts |
| #223 | `714b81805f98` | not yet reviewed | â€” |  | Shaders: translate for the host's wave sizes |
| #222 | `9a77707b01c0` | not yet reviewed | â€” |  | apu: warn when the host has no audio output device |
| #216 | `58e267e23054` | not yet reviewed | â€” |  | Loader & HLE: harden VEH crash handler, PE prep/exports/TLS, and allow oleaut32 |
| #212 | `368a1bb2a933` | not yet reviewed | ? | Head changed since `738b8b8ae7cb`; prior not yet reviewed evidence needs revalidation | GPU, HLE: report host display HDR luminance and configure swap chain color space |
| #202 | `dcf666486a3c` | integrated | `ae526377` | in the branch | GPU: skip unmapped regions after a failed bulk read into host memory |
| #200 | `8201c0ebbb34` | not yet reviewed | ? | Head changed since `93f052e23301`; prior not yet reviewed evidence needs revalidation | Shaders: lower v_div_scale_f32, v_div_fmas_f32 and v_div_fixup_f32 |
| #192 | `4a365e3a736a` | not yet reviewed | â€” |  | Shaders: replace single-wave Wave64 LDS lockstep with barriers and lower 32-lane swizzle/DPP at WaveSize(32) |
| #190 | `ffa13108fef5` | integrated | `a4023b67` | in the branch | GPU: build the indirect-argument clamp with our own writer (no shader compiler at run time) |
| #189 | `958057104d97` | not yet reviewed | ? | Head changed since `54b333fc5c27`; prior not yet reviewed evidence needs revalidation | Shaders: emulate Wave64 compute shaders on Wave32 and narrower hosts |
| #188 | `2a6e6a4bcea5` | integrated | `85834d42` | in the branch | Loader: find TLS-array load candidates sixteen offsets at a time (8-9x faster scan) |
| #182 | `18275322e4d3` | integrated | `aa370a59` | in the branch | Files: `..` in a title path stops at the virtual drive's root |
| #177 | `b7300eb5b86c` | integrated | `c68f6e1a` | in the branch | Shaders: key heap handles by element type (atomic on a float-read UAV) |
| #171 | `6b5e0b45073f` | not yet reviewed | ? | Head changed since `8f19f0dc169e`; prior not yet reviewed evidence needs revalidation | Shaders: lower v_mad_u64_u32 and v_mad_i64_i32 |
| #168 | `9483b1a96261` | not yet reviewed | â€” |  | Shaders: SCC of s_min, s_max and s_lshl1-4_add_u32 as the ISA sets it |
| #166 | `634f6f260350` | not yet reviewed | ? | Head changed since `b81071c2fc6d`; prior not yet reviewed evidence needs revalidation | Shaders: lower s_bitset{0,1}_b64, s_brev_b64 and s_bitreplicate_b64_b32 |
| #164 | `825dbd98f714` | not yet reviewed | â€” |  | Shaders: integer memory atomics and sub-dword buffer accesses |
| #160 | `5ce85c5e7087` | not yet reviewed | â€” |  | Shaders: texel offsets, comparison and gather forms of image samples |
| #159 | `f42e7d5e68b2` | not yet reviewed | â€” |  | GPU: occlusion, statistics and timestamp queries run on the host |
| #158 | `520d629ed178` | integrated | `9a6d9e67` | in the branch | GPU: the depth bounds test reaches the host |
| #157 | `13d984adfd77` | not yet reviewed | â€” |  | Shaders: lower 24-bit mul_hi, saturating 16-bit packs, integer dot pr… |
| #151 | `d77408bb71e1` | integrated | `db44521f` | in the branch | Link victory.exe with a 16 MiB default thread stack |
| #150 | `fe014c1b50ed` | not yet reviewed | â€” |  | Input: read DualSense pads over HID when XInput has none |
| #148 | `a0380049aacd` | integrated | `3ff3ce73` | in the branch | Shaders: lower s_bcnt0_i32_b32/_b64, s_ff0_i32_b32/_b64 and s_flbit_i32_b64/_i64 |
| #145 | `ecc4eb668db7` | not yet reviewed | â€” |  | Shaders: read both ballot halves for wave32 host-wave mask ops |
| #139 | `90cf2f6c6b60` | not yet reviewed | ? | Head changed since `697c4bcd1b5e`; prior integrated evidence needs revalidation | Shaders: reuse SSA sums when tracing memory addresses |
| #137 | `31979ad2a749` | not yet reviewed | â€” |  | fix(gpu): preserve source and destination pixels in texture readbacks [draft] |
| #130 | `1a807d28abe1` | not yet reviewed | ? | Head changed since `0ba0851dea0f`; prior not yet reviewed evidence needs revalidation | Profile: opt-in VICTORY_PROFILE (frame times, memory, caches, loads; frames.csv, a Perfetto trace and an on-screen overlay) |
| #123 | `bc747281dcd6` | not yet reviewed | â€” |  | GPU: refuse raw buffer views that do not start on 16 bytes |
| #120 | `cb40cdcc21a7` | integrated | `9df49c0f` | in the branch | Shaders: lower s_absdiff_i32, s_sext_i32_i8/i16 and s_pack_ll/lh/hh_b32_b16 |
| #114 | `0b729de6ee7b` | integrated | `5d8955e0` | in the branch | XAsync: run a failed call's cleanup without the completion lock |
| #113 | `620afd5ec423` | not yet reviewed | â€” |  | GitHub: rebase issue forms and PR template on main |
| #112 | `89be65511431` | not yet reviewed | â€” |  | Build: Linux workspace, contributor setup and CI |
| #105 | `6e6d3fc027d0` | integrated | `4f6dac61` | in the branch | Shaders: a loop that branches back to the entry instruction translates |
| #98 | `f468457a1859` | integrated | `5ceb0e1f` | in the branch | Shaders: model v_permlane16_b32 in whole-wave sections |
| #92 | `4f2d5dbd8f1c` | integrated | `a47f488e` | in the branch | Shaders: lower v_add_nc_i32, v_sub_nc_i32, v_mul_lo_i32 and the signed 16-bit add and sub |
| #91 | `ed5f98ff67e3` | not yet reviewed | â€” |  | Shaders: lower s_cmov_b32, s_cmov_b64 and s_cmovk_i32 |
| #89 | `081afbd5382b` | not yet reviewed | â€” |  | gpu: packed 16-bit render targets are R8G8B8A8 where the GPU cannot render to them |
| #83 | `d8f063b04566` | not yet reviewed | â€” |  | GPU: bound the null sampler staging heap |
| #82 | `df5e928aa0c5` | integrated | `6399ea71` | in the branch | HLE: speak the title's narration with the host's speech synthesizer |
| #78 | `e6faad2c9fc5` | not yet reviewed | â€” |  | Shaders: inline float constants as f16 in v_fma_mix and v_pack_b32_f16 |
| #76 | `dea90ccc7e87` | integrated | `f464bd96` | in the branch | xmem: map a run of physical pages over several placeholders |
| #72 | `521ceccd781e` | not yet reviewed | â€” |  | GPU: retry pipeline cache saves after write failures |
| #67 | `1e5925710425` | not yet reviewed | ? | Head changed since `9ea5068e6075`; prior not yet reviewed evidence needs revalidation | GPU: implement CopyTextureRegion texture-to-buffer readbacks |
| #55 | `56f1d413873d` | not yet reviewed | â€” |  | GPU: create host textures without the uses the host refuses |
| #32 | `9c5328d4ef91` | not yet reviewed | â€” |  | GPU: fence waits, direct memory-write wakes, queue timeline and host list pool |
| #24 | `857897aa580b` | not yet reviewed | ? | Head changed since `d5f41f54c49b`; prior integrated evidence needs revalidation | GPU: run indirect mesh dispatches (DispatchMeshX) through indirect execution |
