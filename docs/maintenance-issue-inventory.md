# Open upstream issue inventory

Retrieved 2026-11-24T11:51:11.9051768Z through 2026-11-24T11:51:36.6100229Z.
Query: `GET repos/Marvisak/VIctory/issues?state=open&per_page=100`, using gh api --paginate --slurp; entries with pull_request metadata excluded.
Pagination exhausted successfully: 7 pages (100, 100, 100, 100, 100, 100, 34), 634 endpoint entries; 189 open issues after excluding 445 PR entries. Complete for the retrieval snapshot; refresh after the PR pass.

Inventory does not imply reproduction, review, resolution, acceptance, or integration. Issue work awaits dispositions for the PR queue.

| Issue | Title | Status |
|---|---|---|
| [#832](https://github.com/Marvisak/VIctory/issues/832) | Shaders: v_cmp_f/v_cmp_tru on floats emit fcmp false/true, which removes the WARP device (fix on a fork) | awaiting-PR-pass |
| [#831](https://github.com/Marvisak/VIctory/issues/831) | victory config returns success despite asset-policy violations | awaiting-PR-pass |
| [#830](https://github.com/Marvisak/VIctory/issues/830) | Shaders: v_cmp_class_f32 applies abs/neg via float ops, flushing denormals and quieting sNaN (fix on a fork) | awaiting-PR-pass |
| [#829](https://github.com/Marvisak/VIctory/issues/829) | Linux/Wine (Proton 11.0, NVIDIA): frame 1 presents, then null deref at libvkd3d-1.dll+0x3e7b0 right after device slots 0x1c8/0x218 E_NOTIMPL | awaiting-PR-pass |
| [#828](https://github.com/Marvisak/VIctory/issues/828) | Test report: Windows 10 + NVIDIA RTX 3060 — #175 fixes the #598 crash, world entry then hangs the GPU (#597) | awaiting-PR-pass |
| [#827](https://github.com/Marvisak/VIctory/issues/827) | Linux GE-Proton GTA 1.0.2.1: cutscene stays black and D3D12Core crashes at 264b4c | awaiting-PR-pass |
| [#826](https://github.com/Marvisak/VIctory/issues/826) | [Ready to merge] I have prepared a fix for #773 (Auto-size texture mip levels) | awaiting-PR-pass |
| [#825](https://github.com/Marvisak/VIctory/issues/825) | Speech synthesis via Windows.Media.SpeechSynthesis — would you take it? | awaiting-PR-pass |
| [#823](https://github.com/Marvisak/VIctory/issues/823) | Shaders: 16-bit SDWA WORD_0 with UNUSED_SEXT zero-extends instead of sign-extending (fix on a fork) | awaiting-PR-pass |
| [#822](https://github.com/Marvisak/VIctory/issues/822) | GPU crash 0x887A0004 / 0x887A0006 on Nvidia Ampere (RTX 3050) at 3D cutscene transition | awaiting-PR-pass |
| [#821](https://github.com/Marvisak/VIctory/issues/821) | Test report: NVIDIA RTX 5060 Ti — all host-GPU shader tests pass | awaiting-PR-pass |
| [#820](https://github.com/Marvisak/VIctory/issues/820) | Contribution access: PR creation rejected (fix ready in #806, working on #798) | awaiting-PR-pass |
| [#819](https://github.com/Marvisak/VIctory/issues/819) | Testing: a bench script that gives every build the same five numbers (with memory guards) | awaiting-PR-pass |
| [#818](https://github.com/Marvisak/VIctory/issues/818) | Automated GitHub Actions Build Workflow | awaiting-PR-pass |
| [#817](https://github.com/Marvisak/VIctory/issues/817) | NVIDIA (32-lane): 103 compute pipelines refused at CreateComputeShader, reason only visible with VICTORY_D3D12_DEBUG=1 | awaiting-PR-pass |
| [#816](https://github.com/Marvisak/VIctory/issues/816) | [Crash] 0x887A0004 on world load due to unsupported descriptor-as-data and unknown-stride buffers | awaiting-PR-pass |
| [#815](https://github.com/Marvisak/VIctory/issues/815) | Patch (fork): GTA VI bring-up fixes (descriptors, V#, atomics, caches) and mesh-shader TDR on RTX 3080 Ti | awaiting-PR-pass |
| [#814](https://github.com/Marvisak/VIctory/issues/814) | I have prepared a fix for #65 (Sampler heap limit) | awaiting-PR-pass |
| [#813](https://github.com/Marvisak/VIctory/issues/813) | Possible duplicate open PRs: 13 groups (snapshot 2026-11-24) | awaiting-PR-pass |
| [#812](https://github.com/Marvisak/VIctory/issues/812) | Host GPU hang (0x887A0006) when entering the world on RTX 4070 Ti SUPER, stops at command 2958 of 4295 | awaiting-PR-pass |
| [#811](https://github.com/Marvisak/VIctory/issues/811) | HLE: OLEAUT32 ordinals 4/6 (imported by vccorlib140.dll) have no binding | awaiting-PR-pass |
| [#810](https://github.com/Marvisak/VIctory/issues/810) | Contribution: 10 potential fixes for shaders, runtime and texture handling | awaiting-PR-pass |
| [#808](https://github.com/Marvisak/VIctory/issues/808) | RTX 2060 stuck mesh problem | awaiting-PR-pass |
| [#807](https://github.com/Marvisak/VIctory/issues/807) | I have prepared a fix for #777 (DXBC validation) | awaiting-PR-pass |
| [#806](https://github.com/Marvisak/VIctory/issues/806) | GPU: host pipelines keep blending on integer render targets; vkd3d-proton refuses them (fix on a branch) | awaiting-PR-pass |
| [#805](https://github.com/Marvisak/VIctory/issues/805) | HLE: LOAD_LIBRARY_AS_DATAFILE loads outside HostPolicy fail with ERROR_MOD_NOT_FOUND (fix on a fork) | awaiting-PR-pass |
| [#804](https://github.com/Marvisak/VIctory/issues/804) | fail on victory-image…+0xb5a5f8(read 0x68 | awaiting-PR-pass |
| [#803](https://github.com/Marvisak/VIctory/issues/803) | Shaders: s_bfe_u32 (SOP2 0x27) has no scalar hardware test | awaiting-PR-pass |
| [#802](https://github.com/Marvisak/VIctory/issues/802) | Shaders: v_max_f16/v_min_f16/v_max3_f16/v_min3_f16 have no negative-zero ordering test | awaiting-PR-pass |
| [#801](https://github.com/Marvisak/VIctory/issues/801) | 63 open PRs call resource::track with three arguments and break cargo test once merged (since 6638098) | awaiting-PR-pass |
| [#800](https://github.com/Marvisak/VIctory/issues/800) | Shaders: v_min3/max3/med3 integer variants (i32, u32) have no synthetic hardware tests | awaiting-PR-pass |
| [#799](https://github.com/Marvisak/VIctory/issues/799) | Shaders: v_cmp_class_f32 (VOPC 0x088) has no synthetic hardware test | awaiting-PR-pass |
| [#798](https://github.com/Marvisak/VIctory/issues/798) | Shaders: a descriptor dword needed as another s_load's address stops translation (879 programs on 1.0.2.1) | awaiting-PR-pass |
| [#797](https://github.com/Marvisak/VIctory/issues/797) | Cutscene: audio + subtitles work, the video frame does not — two 2560x1440 textures fail CreateCommittedResource (E_INVALIDARG) | awaiting-PR-pass |
| [#796](https://github.com/Marvisak/VIctory/issues/796) | GPU: OMSetRenderTargets dereferences invalid handles on empty or null single-range calls (fix on a fork) | awaiting-PR-pass |
| [#795](https://github.com/Marvisak/VIctory/issues/795) | i am at 22563 shaders out of 26096.. | awaiting-PR-pass |
| [#794](https://github.com/Marvisak/VIctory/issues/794) | I made it to credits | awaiting-PR-pass |
| [#793](https://github.com/Marvisak/VIctory/issues/793) | Core: policy::resolve fails on trailing .. and skips .. folding under Windows \\\\?\\ verbatim prefixes (fix on a fork) | awaiting-PR-pass |
| [#792](https://github.com/Marvisak/VIctory/issues/792) | GPU diagnostics: VICTORY_CAPTURE saves nothing when its folder doesn't exist yet (fix on a branch) | awaiting-PR-pass |
| [#791](https://github.com/Marvisak/VIctory/issues/791) | Host GPU hang (DXGI_ERROR_DEVICE_HUNG) on NVIDIA RTX 2070 during the intro videos; deterministic, same list/command after skipping shaders | awaiting-PR-pass |
| [#790](https://github.com/Marvisak/VIctory/issues/790) | xmem: host memory fills the space below 4 GiB, so the title's 32-bit placeholder is refused at startup (fix on a branch) | awaiting-PR-pass |
| [#789](https://github.com/Marvisak/VIctory/issues/789) | Crash 0xc0000005 at title+0xb5a5f8 (read 0x68) ~2 s after XUserAddAsync, entering the world (NVIDIA RTX 3060) | awaiting-PR-pass |
| [#788](https://github.com/Marvisak/VIctory/issues/788) | suggestion | awaiting-PR-pass |
| [#787](https://github.com/Marvisak/VIctory/issues/787) | CLI: victory compare, an A/B of two translate --list outputs program by program (branch ready; PRs are collaborator-only) | awaiting-PR-pass |
| [#786](https://github.com/Marvisak/VIctory/issues/786) | [GFX10] Device Removed (0x887A0006) after untranslatable compute shaders — GTA 6 Scarlett-Master | awaiting-PR-pass |
| [#785](https://github.com/Marvisak/VIctory/issues/785) | Placed buffer-to-texture copy reads stale data after a same-list GPU write | awaiting-PR-pass |
| [#784](https://github.com/Marvisak/VIctory/issues/784) | After the start screen: 1841 draws/frame but a frozen frame — captured frames show the title's own dark background, not a black screen | awaiting-PR-pass |
| [#783](https://github.com/Marvisak/VIctory/issues/783) | GPU: a copy that meets an unmapped page retries in 64 KiB kernel calls (2 300-2 500 a world frame); one query per region is 2.3x cheaper | awaiting-PR-pass |
| [#782](https://github.com/Marvisak/VIctory/issues/782) | GPU: GetResourceAllocationInfo(2) inflates sub-64 KiB texture alignments and overflows on large totals (fix on a fork) | awaiting-PR-pass |
| [#781](https://github.com/Marvisak/VIctory/issues/781) | RTX 3060 (8 GB): DXGI_ERROR_DEVICE_HUNG in world load (mesh 89036e16fa081505, pixel 508693cae88bae04) | awaiting-PR-pass |
| [#780](https://github.com/Marvisak/VIctory/issues/780) | Pipeline library: some names are stored with a description that differs on the next run (Load mismatch, then StorePipeline 'already exists') | awaiting-PR-pass |
| [#779](https://github.com/Marvisak/VIctory/issues/779) | GPU: GetDesc methods do not return their output pointer in RAX (fix on a fork) | awaiting-PR-pass |
| [#778](https://github.com/Marvisak/VIctory/issues/778) | [NVIDIA RTX 3070] World TDR traced to wave64 mesh shaders (DRED per run); #578 removes the hang, world then renders black | awaiting-PR-pass |
| [#777](https://github.com/Marvisak/VIctory/issues/777) | DXBC parser accepts part offsets inside the header or part table | awaiting-PR-pass |
| [#776](https://github.com/Marvisak/VIctory/issues/776) | GPU: write-watch askings repeat over nested buffers (120-180 ms a world frame); a fix is on a branch | awaiting-PR-pass |
| [#775](https://github.com/Marvisak/VIctory/issues/775) | Shaders: lower v_swaprel_b32 and v_movrelsd_2_b32 (fix ready on a fork) | awaiting-PR-pass |
| [#774](https://github.com/Marvisak/VIctory/issues/774) | Wine: the world stops ~3 min in while Wine reports RtlReleaseSRWLockExclusive on a lock that is not held (x231, three threads) | awaiting-PR-pass |
| [#773](https://github.com/Marvisak/VIctory/issues/773) | GetDesc returns zero mips for automatically sized textures | awaiting-PR-pass |
| [#772](https://github.com/Marvisak/VIctory/issues/772) | World rendering: device removed with a page fault inside a freed texture and garbage descriptor handles | awaiting-PR-pass |
| [#771](https://github.com/Marvisak/VIctory/issues/771) | Resource-backed texture UAV clears are missing from command lists | awaiting-PR-pass |
| [#770](https://github.com/Marvisak/VIctory/issues/770) | Host GPU hang with display driver timeout and all monitors black when entering the world, AMD RX 7700 XT | awaiting-PR-pass |
| [#769](https://github.com/Marvisak/VIctory/issues/769) | [Report] Shader Translation & Pipeline Compilation — NVIDIA GeForce RTX 2060 6GB (Turing) | awaiting-PR-pass |
| [#768](https://github.com/Marvisak/VIctory/issues/768) | Contribution access for a tested Proton early-pipeline cache fix | awaiting-PR-pass |
| [#767](https://github.com/Marvisak/VIctory/issues/767) | SDL3 Xbox and PlayStation controller support with rumble - implementation available | awaiting-PR-pass |
| [#766](https://github.com/Marvisak/VIctory/issues/766) | 4070ti intel i9-12900k | awaiting-PR-pass |
| [#765](https://github.com/Marvisak/VIctory/issues/765) | HLE: RegOpenKeyW/RegOpenKeyExW with a null or empty subkey fail on the emulated OEM key (fix on a fork) | awaiting-PR-pass |
| [#764](https://github.com/Marvisak/VIctory/issues/764) | HLE: GetProcAddress on our pseudo modules returns NULL without setting the last error (fix on a fork) | awaiting-PR-pass |
| [#763](https://github.com/Marvisak/VIctory/issues/763) | Proposal: share the review load with any%5 (working demo on a fork) | awaiting-PR-pass |
| [#762](https://github.com/Marvisak/VIctory/issues/762) | Loader: a title DLL in a case-sensitive folder is skipped without an error (fix on a fork) | awaiting-PR-pass |
| [#761](https://github.com/Marvisak/VIctory/issues/761) | docs/architecture.md still lists BCPack as planned | awaiting-PR-pass |
| [#760](https://github.com/Marvisak/VIctory/issues/760) | Design docs cited in the code are missing from the repo (decision 0003, decision 0007, TODO.md) | awaiting-PR-pass |
| [#758](https://github.com/Marvisak/VIctory/issues/758) | A texture that once received a GPU copy ignores later writes to its memory | awaiting-PR-pass |
| [#757](https://github.com/Marvisak/VIctory/issues/757) | ExecuteIndirect only runs single-argument command signatures | awaiting-PR-pass |
| [#755](https://github.com/Marvisak/VIctory/issues/755) | Offer to Host Forgejo Instance and Chat Server | awaiting-PR-pass |
| [#753](https://github.com/Marvisak/VIctory/issues/753) | Windows 10: root signatures refused ("Unsupported bit-flag set, flags c01") — fixed by enabling the D3D12 Agility SDK | awaiting-PR-pass |
| [#752](https://github.com/Marvisak/VIctory/issues/752) | Linux on AMD: got in-game on a 9070 XT, here's what each crash was | awaiting-PR-pass |
| [#751](https://github.com/Marvisak/VIctory/issues/751) | D3D12 Shadows - 4060 ti | awaiting-PR-pass |
| [#750](https://github.com/Marvisak/VIctory/issues/750) | Shader resource tracking: redundant masked write after a per-lane descriptor join is refused (synthetic repro on #566) | awaiting-PR-pass |
| [#749](https://github.com/Marvisak/VIctory/issues/749) | Windows 10 + KB5029331: in-box d3d12 still refuses 1.1 heap-indexing flags; explicit Agility core load works (XCL proves it on this box) | awaiting-PR-pass |
| [#748](https://github.com/Marvisak/VIctory/issues/748) | The world: 64 indirect draws/frame dropped, 282 compute / 14 mesh / 17 VS-PS shaders do not translate (RX 9070 XT) | awaiting-PR-pass |
| [#747](https://github.com/Marvisak/VIctory/issues/747) | NVIDIA capped view heap: 2^16 dynamic slots starve world tables; 2^18 removes the indirect/mesh drops | awaiting-PR-pass |
| [#746](https://github.com/Marvisak/VIctory/issues/746) | RX 6750 XT on Windows 10 22H2 | awaiting-PR-pass |
| [#745](https://github.com/Marvisak/VIctory/issues/745) | Shaders: v_med3 with a NaN operand gives the median on NVIDIA (fcmp uno folded); fix on a branch | awaiting-PR-pass |
| [#744](https://github.com/Marvisak/VIctory/issues/744) | GPU: cap shader-visible sampler heaps at 2048 descriptors | awaiting-PR-pass |
| [#743](https://github.com/Marvisak/VIctory/issues/743) | testing on RX 9070 XT | awaiting-PR-pass |
| [#742](https://github.com/Marvisak/VIctory/issues/742) | Windows 10: unwinds_like_the_system_walk fails about half the time (RtlCaptureStackBackTrace returns no frames) | awaiting-PR-pass |
| [#741](https://github.com/Marvisak/VIctory/issues/741) | Crash at Game Init on Windows 10 + NVIDIA: R9G9B9E5 render target refused (E_INVALIDARG), then access violation at image+0xb5a5f8 (read 0x68) | awaiting-PR-pass |
| [#740](https://github.com/Marvisak/VIctory/issues/740) | Patch (fork): indirect draw stream compaction pass and Intel Arc DG2 wave clamping | awaiting-PR-pass |
| [#737](https://github.com/Marvisak/VIctory/issues/737) | Patch (fork): add VICTORY_TEXTURE_MIP_DROP quality switch for low-VRAM budgets (75%–93% texture memory savings on 6GB/8GB GPUs) | awaiting-PR-pass |
| [#736](https://github.com/Marvisak/VIctory/issues/736) | Shaders: #612 stops 323 previously validated programs from translating | awaiting-PR-pass |
| [#735](https://github.com/Marvisak/VIctory/issues/735) | RTX 4060 Laptop: world-load device removal is not resolution/RT-bound; debug layer names invalid B5G5R5A1/B4G4R4A4 resources and a 1.31M descriptor heap | awaiting-PR-pass |
| [#734](https://github.com/Marvisak/VIctory/issues/734) | Three ready branches: v_mad_u64/i64, ldexp/frexp, wave64 on NVIDIA (PRs are collaborator-only) | awaiting-PR-pass |
| [#732](https://github.com/Marvisak/VIctory/issues/732) | What's needed right now to get the game further? | awaiting-PR-pass |
| [#730](https://github.com/Marvisak/VIctory/issues/730) | PC FSR support (FidelityFX on host D3D12): branch ready, can't open a PR | awaiting-PR-pass |
| [#729](https://github.com/Marvisak/VIctory/issues/729) | Windows 10: root signatures rejected, and keyboard input lost with an XInput keyboard (two fixes on a fork) | awaiting-PR-pass |
| [#728](https://github.com/Marvisak/VIctory/issues/728) | Patch (fork): lower 64-bit image atomics, SMEM descriptor size reads, and spilled lane masks (+657 PSOs on RTX 5060 Ti) | awaiting-PR-pass |
| [#727](https://github.com/Marvisak/VIctory/issues/727) | modules::display::tests::sdr_and_display_deferrals fails on this machine | awaiting-PR-pass |
| [#726](https://github.com/Marvisak/VIctory/issues/726) | v_med3_f32 with NaN returns wrong side on NVIDIA (NVIDIA-specific test failure) | awaiting-PR-pass |
| [#725](https://github.com/Marvisak/VIctory/issues/725) | Shaders: glc loads aren't marked globally coherent, so cross-workgroup waits may never see the write | awaiting-PR-pass |
| [#724](https://github.com/Marvisak/VIctory/issues/724) | Shaders: one heap descriptor gets an SRV handle for its loads and a UAV handle for its stores | awaiting-PR-pass |
| [#723](https://github.com/Marvisak/VIctory/issues/723) | GPU: an announced kernel read (kernel_will_write) can be re-guarded before it lands, so ReadFile and DirectStorage fail | awaiting-PR-pass |
| [#722](https://github.com/Marvisak/VIctory/issues/722) | GPU: copy-back can mark a page up to date after the title wrote it, so the GPU keeps reading old data | awaiting-PR-pass |
| [#721](https://github.com/Marvisak/VIctory/issues/721) | GPU: copy-back writes whole 64 KiB pieces back and can undo the title's own writes | awaiting-PR-pass |
| [#720](https://github.com/Marvisak/VIctory/issues/720) | Shaders: cube-map arrays sample layer 0, and the +X face for any layer above 0 | awaiting-PR-pass |
| [#718](https://github.com/Marvisak/VIctory/issues/718) | Format support query advertises BC1 as a render target | awaiting-PR-pass |
| [#717](https://github.com/Marvisak/VIctory/issues/717) | PR permission | awaiting-PR-pass |
| [#714](https://github.com/Marvisak/VIctory/issues/714) | Linux (Proton + vkd3d-proton, Strix Halo RADV): GPU page fault (texture read) 1.5–4 min in, DRED never reports because the title's own hang check exits first | awaiting-PR-pass |
| [#713](https://github.com/Marvisak/VIctory/issues/713) | Offer: GPU-verified FidelityFX CAS golden model for shader validation (ffx-kit) | awaiting-PR-pass |
| [#700](https://github.com/Marvisak/VIctory/issues/700) | GPU: a command-list 64-bit memory wait lets the consumer read before its producer writes | awaiting-PR-pass |
| [#698](https://github.com/Marvisak/VIctory/issues/698) | World load on NVIDIA with no device removal: title breakpoint (0x80000003) after ~6.7 min, INSTANCE_LIST pool full (45000/45000) | awaiting-PR-pass |
| [#678](https://github.com/Marvisak/VIctory/issues/678) | Issue Description black screen | awaiting-PR-pass |
| [#653](https://github.com/Marvisak/VIctory/issues/653) | World load removes the host D3D12 device — on the real GPU (0x887A0006) and on WARP (0x887A0005) | awaiting-PR-pass |
| [#630](https://github.com/Marvisak/VIctory/issues/630) | Cross-project site map: the title's two deliberate self-kills, the 8-GB int3, and the cold-run sentinel family (exe hash e95020b2…) | awaiting-PR-pass |
| [#625](https://github.com/Marvisak/VIctory/issues/625) | [NVIDIA RTX 3060 Ti] GPU hang (0x887A0006) ~50s after entering world / first mission subtitles | awaiting-PR-pass |
| [#622](https://github.com/Marvisak/VIctory/issues/622) | Windows 10 + RTX 4070 SUPER: three verified fixes get the title past Game Init into the render loop | awaiting-PR-pass |
| [#619](https://github.com/Marvisak/VIctory/issues/619) | RTX 3060 (8 GB): DXGI_ERROR_DEVICE_HUNG in world load (mesh 1da1eb9822214321, pixel ef9b757c9c1e4b26, command 1733/2145) | awaiting-PR-pass |
| [#614](https://github.com/Marvisak/VIctory/issues/614) | Ray-tracing builder: none of its compute shaders translate in the title (root UAVs, addresses in root constants) | awaiting-PR-pass |
| [#610](https://github.com/Marvisak/VIctory/issues/610) | Diagnostics: VICTORY_D3D12_WARP=0 still selects the software adapter | awaiting-PR-pass |
| [#598](https://github.com/Marvisak/VIctory/issues/598) | Crash 0xc0000005 on world load due to unsupported root signature flags (c01/c00) | awaiting-PR-pass |
| [#597](https://github.com/Marvisak/VIctory/issues/597) | DXGI_ERROR_DEVICE_HUNG (0x887A0006) at world load on 9070xt; many compute shaders fail to translate | awaiting-PR-pass |
| [#593](https://github.com/Marvisak/VIctory/issues/593) | Shaders: lane-indexed reads/writes on a 64-lane host wave use the wrong console wave or are refused | awaiting-PR-pass |
| [#567](https://github.com/Marvisak/VIctory/issues/567) | Telegram Group for Communication | awaiting-PR-pass |
| [#564](https://github.com/Marvisak/VIctory/issues/564) | Five contributions from the RPF9 side: mesh/GS ABI, wave64, typeless formats, pipeline library, XMem pages | awaiting-PR-pass |
| [#557](https://github.com/Marvisak/VIctory/issues/557) | World entry after #336: device removed reading freed streaming textures (page fault inside freed texture 1024x1024) | awaiting-PR-pass |
| [#548](https://github.com/Marvisak/VIctory/issues/548) | RTX 3070 Ti: DXGI_ERROR_DEVICE_HUNG in the world; debug layer reports a title buffer in UAV state used as vertex buffer | awaiting-PR-pass |
| [#533](https://github.com/Marvisak/VIctory/issues/533) | Series X mode crashes at loading artwork on RX 6700 XT, including with WARP | awaiting-PR-pass |
| [#527](https://github.com/Marvisak/VIctory/issues/527) | i don't know how to run this victory how to? anyone | awaiting-PR-pass |
| [#508](https://github.com/Marvisak/VIctory/issues/508) | GPU: writing unbound UAVs removes the device on an RDNA3 iGPU | awaiting-PR-pass |
| [#507](https://github.com/Marvisak/VIctory/issues/507) | [GPU Hang] AMD RX 7900 XTX – Descriptor heap overflow + recurring mesh/compute hashes | awaiting-PR-pass |
| [#496](https://github.com/Marvisak/VIctory/issues/496) | Black world and host GPU reset (DXGI_ERROR_DEVICE_HUNG) on NVIDIA at the first cutscene; 343 shaders don't translate | awaiting-PR-pass |
| [#473](https://github.com/Marvisak/VIctory/issues/473) | Windows 10: the Agility SDK exposes SM 6.6, and with it all seven `valu_tests` pass on hardware | awaiting-PR-pass |
| [#471](https://github.com/Marvisak/VIctory/issues/471) | World load dies at the commit limit (18.0 GB private, 48.8 of 49.0 GB system commit) | awaiting-PR-pass |
| [#468](https://github.com/Marvisak/VIctory/issues/468) | NVIDIA RTX 4060 Laptop: world is black (114 compute pipelines refused with E_INVALIDARG), then GPU fault after ~2 min | awaiting-PR-pass |
| [#466](https://github.com/Marvisak/VIctory/issues/466) | Fluxer server down - Only form of info rn | awaiting-PR-pass |
| [#445](https://github.com/Marvisak/VIctory/issues/445) | VIctory GPU Crash Investigation — Technical Summary and Diagnostic Findings | awaiting-PR-pass |
| [#444](https://github.com/Marvisak/VIctory/issues/444) | IMPORTANT! Creating a decentralized chat for communication and coordination. Stage 1 | awaiting-PR-pass |
| [#417](https://github.com/Marvisak/VIctory/issues/417) | Boot -> account picker -> Rockstar logo -> start screen at a stable 30 fps (RX 7900 XTX / 9800X3D); 5 WARN lines worth a look | awaiting-PR-pass |
| [#410](https://github.com/Marvisak/VIctory/issues/410) | Suggestion: Move the repo to Gitflic or a self hosted repo | awaiting-PR-pass |
| [#407](https://github.com/Marvisak/VIctory/issues/407) | GPU: one address space for title memory, instead of copying it (arena over a CPU-visible heap) | awaiting-PR-pass |
| [#400](https://github.com/Marvisak/VIctory/issues/400) | World on an RTX 5070 Ti: where the ~0.8 s per frame go, and four open copy PRs measured against main | awaiting-PR-pass |
| [#397](https://github.com/Marvisak/VIctory/issues/397) | sui | awaiting-PR-pass |
| [#394](https://github.com/Marvisak/VIctory/issues/394) | Linux support: is there interest, and can I help test? | awaiting-PR-pass |
| [#392](https://github.com/Marvisak/VIctory/issues/392) | NVIDIA (RTX 3080): device removed on null/unsupported views during world load — two fixes + one open hang | awaiting-PR-pass |
| [#386](https://github.com/Marvisak/VIctory/issues/386) | RTX 5090 test: GetCopyableFootprints returns different row sizes and base offsets than native D3D12 | awaiting-PR-pass |
| [#375](https://github.com/Marvisak/VIctory/issues/375) | when can we see lucia get arrested | awaiting-PR-pass |
| [#350](https://github.com/Marvisak/VIctory/issues/350) | Rtx 6000 pro 96gb vram and 192GB ram available to test builds | awaiting-PR-pass |
| [#340](https://github.com/Marvisak/VIctory/issues/340) | Feature request: experimental Linux/Wine support and compatibility documentation | awaiting-PR-pass |
| [#338](https://github.com/Marvisak/VIctory/issues/338) | WARP diagnostic: severe E_OUTOFMEMORY, access violations in d3d10warp.dll and device removal during world load | awaiting-PR-pass |
| [#325](https://github.com/Marvisak/VIctory/issues/325) | RTX 4090: 1 fps in the first cutscene, then host GPU hang (DXGI_ERROR_DEVICE_HUNG) | awaiting-PR-pass |
| [#314](https://github.com/Marvisak/VIctory/issues/314) | Host GPU device removed when entering the world on NVIDIA (RTX 4050 Laptop) | awaiting-PR-pass |
| [#304](https://github.com/Marvisak/VIctory/issues/304) | [Crash / TDR] Host GPU device removed (0x887A0006) ~55s after Mission 1 title card on RX 9070 XT | awaiting-PR-pass |
| [#293](https://github.com/Marvisak/VIctory/issues/293) | World: which untranslated shaders cost the most draws (ranked from two runs) | awaiting-PR-pass |
| [#289](https://github.com/Marvisak/VIctory/issues/289) | Crash right after "audio DSP connected": title memsets unmapped address 0x217800 (len 0x2173a0) | awaiting-PR-pass |
| [#274](https://github.com/Marvisak/VIctory/issues/274) | Ray tracing: what Leonida measured about XboxBvhBuilder and the console BVH format | awaiting-PR-pass |
| [#267](https://github.com/Marvisak/VIctory/issues/267) | GTX1080, Win10 22H2 | awaiting-PR-pass |
| [#264](https://github.com/Marvisak/VIctory/issues/264) | NVIDIA RTX 5070 Ti Laptop: device hung entering world (vertex buffer left in UAV state) + wave64 shaders refused | awaiting-PR-pass |
| [#255](https://github.com/Marvisak/VIctory/issues/255) | World: ~250 indirect rectangle-list draws per frame are dropped (topology 17), and the world stays black behind the cutscene | awaiting-PR-pass |
| [#225](https://github.com/Marvisak/VIctory/issues/225) | Shaders: v_readlane inside a loop the lanes leave at different iterations reads wrong lanes (wave32 and wave64, every GPU) | awaiting-PR-pass |
| [#210](https://github.com/Marvisak/VIctory/issues/210) | 0xc000001d Illegal instruction (CLWB) crash on AMD Ryzen 7 | awaiting-PR-pass |
| [#203](https://github.com/Marvisak/VIctory/issues/203) | Disable forced 4K rendering | awaiting-PR-pass |
| [#201](https://github.com/Marvisak/VIctory/issues/201) | Corpus census: 185 emitted DXIL modules fail dxil.dll validation | awaiting-PR-pass |
| [#199](https://github.com/Marvisak/VIctory/issues/199) | NVIDIA RTX 4070 SUPER: D3D12 resource state validation failure when binding title buffer as vertex buffer | awaiting-PR-pass |
| [#187](https://github.com/Marvisak/VIctory/issues/187) | Feature Request: SDL3 backend for GameInput shim (DualSense, trigger rumble, gyro & dynamic controller glyphs) | awaiting-PR-pass |
| [#181](https://github.com/Marvisak/VIctory/issues/181) | Tests: victory-shader tests crash (0xC0000005) when several tests use WARP devices in parallel | awaiting-PR-pass |
| [#170](https://github.com/Marvisak/VIctory/issues/170) | Wine/vkd3d: indirect execution is dropped because the argument clamp cannot be compiled there; rewriting its HLSL does not help | awaiting-PR-pass |
| [#155](https://github.com/Marvisak/VIctory/issues/155) | Structured buffers: runtime stride channel (design + anchors) | awaiting-PR-pass |
| [#144](https://github.com/Marvisak/VIctory/issues/144) | Wine + vkd3d-proton on NVIDIA: the driver crashes in vkCreateComputePipelines while the world loads (0xc000001d / 0xc0000005, earlier stack overflows) | awaiting-PR-pass |
| [#131](https://github.com/Marvisak/VIctory/issues/131) | MaskData::Host: the high half of a ballot is dropped, and nothing forces the host wave to 32 | awaiting-PR-pass |
| [#125](https://github.com/Marvisak/VIctory/issues/125) | Shaders: mask ops in a pixel/vertex program never read the high ballot half, so a 64-lane host wave loses lanes 32-63 | awaiting-PR-pass |
| [#108](https://github.com/Marvisak/VIctory/issues/108) | Title crashes on CPUs without CLWB support (STATUS_ILLEGAL_INSTRUCTION 0xc000001d) | awaiting-PR-pass |
| [#106](https://github.com/Marvisak/VIctory/issues/106) | Crash with 0xc000001d (illegal instruction): game uses AMD-only CLZERO, unsupported on Intel CPUs | awaiting-PR-pass |
| [#103](https://github.com/Marvisak/VIctory/issues/103) | Crash: 0xc000001d (STATUS_ILLEGAL_INSTRUCTION) at +0x4ecc84a on Intel Core i3-10100F | awaiting-PR-pass |
| [#102](https://github.com/Marvisak/VIctory/issues/102) | GTA VI crashes with 0xC000001D on Xeon E5-2630 v4 | awaiting-PR-pass |
| [#99](https://github.com/Marvisak/VIctory/issues/99) | Access violation at +0x4e33a92 while the first mission intro prestreams (Windows 10, RTX 4080) | awaiting-PR-pass |
| [#97](https://github.com/Marvisak/VIctory/issues/97) | Host GPU device removed on RTX 4070 (diag build, DRED) - many shaders fail to translate | awaiting-PR-pass |
| [#90](https://github.com/Marvisak/VIctory/issues/90) | Linux: main does not build (E0425 in victory-gpu); with #39 the workspace builds and 156 tests pass | awaiting-PR-pass |
| [#85](https://github.com/Marvisak/VIctory/issues/85) | Intel host support (CLWB/CLZERO/MONITORX/SSE4a emulation) + GPU hang on NVIDIA at the language screen | awaiting-PR-pass |
| [#84](https://github.com/Marvisak/VIctory/issues/84) | Windows 10: the host device stops at shader model 6.5 (needs the D3D12 Agility SDK) | awaiting-PR-pass |
| [#71](https://github.com/Marvisak/VIctory/issues/71) | NVIDIA RTX 5070 Ti: world hang (DEVICE_HUNG) on 97c78d1 is not one program, not loops, not barriers; 77 compute pipelines refused | awaiting-PR-pass |
| [#65](https://github.com/Marvisak/VIctory/issues/65) | GPU: a sampler heap over 2,048 loses the host heap entirely | awaiting-PR-pass |
| [#63](https://github.com/Marvisak/VIctory/issues/63) | Windows 11, es0000 q1hy Intel 13th Gen Raptor Lake ES, AMD RX 6800 XT + versione driver Adrenalin 26.10.1 16gb ddr5 | awaiting-PR-pass |
| [#59](https://github.com/Marvisak/VIctory/issues/59) | RX 9060 XT (RDNA4) \| Host GPU device removed (DXGI_ERROR_DEVICE_HUNG) with page fault at 0x1a00b0000 in world | awaiting-PR-pass |
| [#49](https://github.com/Marvisak/VIctory/issues/49) | Testers post specs and test results here | awaiting-PR-pass |
| [#47](https://github.com/Marvisak/VIctory/issues/47) | Leonida Just got Supoena from Rockstar | awaiting-PR-pass |
| [#29](https://github.com/Marvisak/VIctory/issues/29) | RX 9070 XT (RDNA4, Adrenalin 26.11.2): first-boot menus 30 fps for 4 min, no device loss (NVIDIA hangs do not reproduce) | awaiting-PR-pass |
| [#23](https://github.com/Marvisak/VIctory/issues/23) | Title stops at startup when Windows can't commit a 6.5 GB XMemVirtualAlloc (paging file too small, os error 1455) | awaiting-PR-pass |
| [#22](https://github.com/Marvisak/VIctory/issues/22) | Is there any way to help? | awaiting-PR-pass |
| [#8](https://github.com/Marvisak/VIctory/issues/8) | RTX 3080 Ti: first-chance 0x80000003 at title+0x2237178 before any window — menus never reached here | awaiting-PR-pass |
| [#3](https://github.com/Marvisak/VIctory/issues/3) | compute shaders fallback | awaiting-PR-pass |
