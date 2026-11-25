//! Texel-fail status (`tfe`) on image loads, samples and gathers: synthetic
//! GFX10 programs lowered to DXIL and, on Windows, validated by `dxil.dll`
//! when present. A texture that is not
//! tiled is resident everywhere, so the status dword after the data reads 0.

use super::lower::{self, Lowered};
use crate::native::{
    ComputeIo, HwStage, NativeProgram, ResourceType, RootParam, RootType, StageIo, UsedResource,
};
use crate::{ShaderError, io, ir, resource, structure};

const S_ENDPGM: u32 = 0xBF81_0000;

/// The descriptors and each lane's output offset: the texture at entry 0
/// of the table at s0 in s[12:19], the sampler of the table at s1 in
/// s[8:11], the raw UAV at dword 16 of the first table in s[40:43], and
/// v30 = the lane's byte offset of two dwords in the UAV.
const PROLOGUE: [u32; 12] = [
    0xBE84_0300, // s_mov_b32 s4, s0
    0xBE85_0380, // s_mov_b32 s5, 0
    0xBE86_0301, // s_mov_b32 s6, s1
    0xBE87_0380, // s_mov_b32 s7, 0
    0xF40C_0302,
    0xFA00_0000, // s_load_dwordx8 s[12:19], s[4:5], null
    0xF408_0203,
    0xFA00_0000, // s_load_dwordx4 s[8:11], s[6:7], null
    0xF408_0A02,
    0xFA00_0040, // s_load_dwordx4 s[40:43], s[4:5], 0x40
    0x343C_0083, // v_lshlrev_b32 v30, 3, v0
    0xBF8C_C07F, // s_waitcnt lgkmcnt(0)
];

/// Texture coordinates (0.5, 0.5): the middle of a 1x1 texture.
const SAMPLE_AT_MIDDLE: [u32; 2] = [
    0x7E04_02F0, // v_mov_b32 v2, 0.5
    0x7E06_02F0, // v_mov_b32 v3, 0.5
];

/// Texel coordinates (0, 0).
const LOAD_AT_ORIGIN: [u32; 2] = [
    0x7E04_0280, // v_mov_b32 v2, 0
    0x7E06_0280, // v_mov_b32 v3, 0
];

/// Stores `data` and `status` (VGPRs) at the lane's two dwords of the UAV.
const fn store_pair(data: u8, status: u8) -> [u32; 5] {
    [
        0xBF8C_3F70, // s_waitcnt vmcnt(0)
        0xE070_1000,
        0x800A_001E | (data as u32) << 8, // buffer_store_dword v<data>, v30, s[40:43], 0 offen
        0xE070_1004,
        0x800A_001E | (status as u32) << 8, // ... offset:4
    ]
}

/// `PROLOGUE`, the coordinates, `access` and the stores of `data` and
/// `status`.
fn program(coords: [u32; 2], access: [u32; 2], data: u8, status: u8) -> Vec<u32> {
    let mut words = PROLOGUE.to_vec();
    words.extend(coords);
    words.extend(access);
    words.extend(store_pair(data, status));
    words.push(S_ENDPGM);
    words
}

/// `image_sample_lz v[12:13], v[2:3], s[12:19], s[8:11] dim:2D tfe`: the
/// red channel in v12, the status in v13.
fn sample_red() -> Vec<u32> {
    program(SAMPLE_AT_MIDDLE, [0xF09D_0108, 0x0043_0C02], 12, 13)
}

/// `image_load v[20:24], v[2:3], s[12:19] dmask:0xf dim:2D tfe`: the four
/// channels in v[20:23], the status in v24 (red stored).
fn load_rgba() -> Vec<u32> {
    program(LOAD_AT_ORIGIN, [0xF001_0F08, 0x0003_1402], 20, 24)
}

/// `image_gather4_lz v[12:16], v[2:3], s[12:19], s[8:11] dim:2D tfe`: four
/// red texels in v[12:15], the status in v16 (the first texel stored).
fn gather_red() -> Vec<u32> {
    program(SAMPLE_AT_MIDDLE, [0xF11D_0108, 0x0043_0C02], 12, 16)
}

/// `image_sample_lz v[12:13], v[2:3], s[12:19], s[8:11] dmask:0x3 dim:2D
/// d16 tfe`: red and green as halves in v12, the status in v13.
fn sample_red_green_d16() -> Vec<u32> {
    program(SAMPLE_AT_MIDDLE, [0xF09D_0308, 0x8043_0C02], 12, 13)
}

/// Lowers a wave32 compute program of 32 threads with the table of views at
/// s0 (a 2D texture at entry 0, a raw UAV at entry 2) and the table of
/// samplers at s1.
fn translate(words: &[u32]) -> Result<Lowered, ShaderError> {
    let table = |user_data: u8| RootParam {
        root: RootType::Table,
        table_offset_dwords: 0,
        sgprs: 1,
        base_register: 0,
        user_data,
        slot: user_data,
    };
    let range = |kind, table_offset_dwords, dwords, user_data| UsedResource {
        kind,
        root: RootType::Table,
        table_offset_dwords,
        lower_bound: 0,
        stride: 0,
        dwords,
        user_data,
        slot: user_data,
    };
    let native = NativeProgram {
        stage: HwStage::Cs,
        wave64: false,
        vgprs: 32,
        layout: vec![table(0), table(1)],
        used: vec![
            range(ResourceType::Srv, 0, 8, 0),
            range(ResourceType::Uav, 16, 4, 0),
            range(ResourceType::Sampler, 0, 4, 1),
        ],
        io: StageIo::Cs(ComputeIo {
            rsrc2: 2 << 1,
            threads: [32, 1, 1],
        }),
        code: words.iter().flat_map(|w| w.to_le_bytes()).collect(),
    };
    let p = ir::build(&native.code, native.wave64, native.vgprs)?;
    let st = structure::structure(&p)?;
    let res = resource::track(&p, &native.layout, native.stage, Some(&st))?;
    let io = io::raise(&p, &native)?;
    lower::lower(&lower::Parts {
        native: &native,
        p: &p,
        st: &st,
        io: &io,
        res: &res,
        inputs: Vec::new(),
        outputs: Vec::new(),
        prims: Vec::new(),
        mesh_stride: 0,
    })
}

/// The reason `words` cannot be translated.
fn refusal(words: &[u32]) -> &'static str {
    match translate(words) {
        Err(ShaderError::Unsupported { what, .. }) => what,
        Err(e) => panic!("{e}"),
        Ok(_) => panic!("translated"),
    }
}

#[test]
fn loads_samples_and_gathers_with_tfe_check_residency() {
    for (label, words) in [
        ("sample", sample_red()),
        ("load", load_rgba()),
        ("gather", gather_red()),
        ("d16 sample", sample_red_green_d16()),
    ] {
        let l = translate(&words).unwrap_or_else(|e| panic!("{label}: {e}"));
        assert!(l.tiled_resources, "{label}: no residency check");
        assert!(!l.wave_ops, "{label}");
        #[cfg(windows)]
        if let Some(text) = super::tests::dump_dxil(&l.dxil) {
            assert!(text.contains("dx.op.checkAccessFullyMapped.i32(i32 71"), "{label}: {text}");
            assert!(text.contains("select i1"), "{label}: residency becomes status 0 or 1");
        }
        let container = crate::Container::parse(&l.dxil).unwrap();
        let features = container.part(crate::FourCC::SFI0).unwrap().data;
        assert_ne!(u64::from_le_bytes(features.try_into().unwrap()) & 0x100, 0);
        #[cfg(windows)]
        if let Ok(mut v) = super::validator::Validator::load(
            super::validator::Validator::default_path().as_deref(),
        ) {
            v.validate(&mut l.dxil.clone())
                .expect("TFE DXIL validation");
        }
    }
}

#[test]
fn accesses_without_tfe_need_no_residency_check() {
    // The sample of `sample_red` without tfe: v13 is never written, so
    // store v12 twice.
    let words = program(SAMPLE_AT_MIDDLE, [0xF09C_0108, 0x0043_0C02], 12, 12);
    assert!(!translate(&words).unwrap().tiled_resources);
}

#[test]
fn tfe_on_a_store_and_lwe_stay_refused() {
    // image_store v[12:16], v[2:3], s[12:19] dmask:0xf dim:2D tfe
    let store = [
        PROLOGUE.as_slice(),
        &LOAD_AT_ORIGIN,
        &[0xF021_0F08, 0x0003_0C02, S_ENDPGM],
    ]
    .concat();
    assert_eq!(refusal(&store), "tfe on an image store, atomic or query");
    // The newer explicit-level gather path returns before the common
    // status write; keep TFE refused until all its load statuses combine.
    let gather_l = program(SAMPLE_AT_MIDDLE, [0xF111_0108, 0x0043_0C02], 12, 16);
    assert_eq!(refusal(&gather_l), "tfe on an explicit-level gather");
    // `sample_red` with lwe as well as tfe.
    let lwe = program(SAMPLE_AT_MIDDLE, [0xF09F_0108, 0x0043_0C02], 12, 13);
    assert_eq!(refusal(&lwe), "an image access with lwe");
}
#[cfg(windows)]
#[test]
fn one_dimensional_views_use_two_dimensional_backing() {
    for (dim, host_kind) in [(0, 2), (4, 7)] {
        let access = [0xF09C_0100 | dim << 3, 0x0043_0C02];
        let lowered = translate(&program(SAMPLE_AT_MIDDLE, access, 12, 12)).unwrap();
        let text = super::tests::dump_dxil(&lowered.dxil).expect("dxc required for coordinate regression");
        assert!(text.contains(&format!("{{ i32 {host_kind}, i32 1033 }}")), "{text}");
        let sample = text.lines().find(|l| l.contains("call %dx.types.ResRet.f32 @dx.op.sampleLevel.f32")).unwrap();
        assert!(sample.contains(", float 0.000000e+00,"), "host y is zero: {sample}");
        if dim == 4 {
            // The second console coordinate becomes the host array slice.
            assert!(!sample.contains("float undef, float undef"), "{sample}");
        }
        let query = [0xF038_0200 | dim << 3, 0x0003_1402];
        let lowered = translate(&program(LOAD_AT_ORIGIN, query, 20, 20)).unwrap();
        let text = super::tests::dump_dxil(&lowered.dxil).unwrap();
        if dim == 4 {
            assert!(text.lines().any(|l| l.contains("extractvalue %dx.types.Dimensions") && l.ends_with(", 2")), "array layers come from third dimension: {text}");
        }
    }
}

#[cfg(windows)]
mod gpu {
    use windows::Win32::Graphics::Direct3D12::*;

    use super::super::host::{Dispatch, Host, Root, View};
    use super::super::{HostShader, validator};
    use super::*;

    /// The texel of the 1x1 texture the programs read.
    const TEXEL: [f32; 4] = [1.0, 0.25, 0.5, 0.75];

    /// `words` lowered, validated by `dxil.dll` when present.
    fn build(words: &[u32]) -> HostShader {
        let l = translate(words).expect("translate");
        let shader = HostShader {
            stage: HwStage::Cs,
            dxil: l.dxil,
            user_data: l.user_data,
            samplers: l.samplers,
            static_samplers: Vec::new(),
            root_cbvs: l.root_cbvs,
            root_srvs: l.root_srvs,
            root_uavs: l.root_uavs,
            memory: l.memory,
            memory_writes: l.memory_writes,
            view_addresses: l.view_addresses,
            heap: l.heap,
            heap_writes: l.heap_writes,
        };
        if let Ok(mut v) =
            validator::Validator::load(validator::Validator::default_path().as_deref())
        {
            v.validate(&mut shader.dxil.clone())
                .expect("DXIL validation failed");
        } else {
            eprintln!("no dxil.dll; validation skipped");
        }
        shader
    }

    /// Runs `words` over the texel's texture (heap index 0; index 1 holds
    /// a second texture so the UAV is at index 2, dword 16 of the table) on
    /// the hardware GPU and on WARP (each skipped without a device) and
    /// returns, per one that ran, each lane's (data, status) pair.
    fn run(words: &[u32]) -> Vec<(&'static str, Vec<[u32; 2]>)> {
        if std::env::var_os("VICTORY_GPU_TESTS").is_none() {
            eprintln!("TFE device execution requires VICTORY_GPU_TESTS=1; skipped");
            return Vec::new();
        }
        let shader = build(words);
        let texel = [TEXEL];
        let unused = [[0.0f32; 4]];
        let views = [
            View::Texture(1, 1, &texel),
            View::Texture(1, 1, &unused),
            View::RawUav(64),
        ];
        let samplers = [D3D12_SAMPLER_DESC {
            Filter: D3D12_FILTER_MIN_MAG_MIP_POINT,
            AddressU: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
            AddressV: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
            AddressW: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
            MaxAnisotropy: 1,
            MaxLOD: f32::MAX,
            ..Default::default()
        }];
        let mut out = Vec::new();
        // One GPU test at a time, with the VALU tests.
        let _one = super::super::valu_tests::ONE_AT_A_TIME
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        for (label, warp) in [("hardware", false), ("warp", true)] {
            let Ok(host) = Host::new(warp) else {
                eprintln!("no SM 6.6 {label} device; skipped");
                continue;
            };
            let (root, pso) = host
                .compute_pipeline(&shader)
                .unwrap_or_else(|e| panic!("{label}: compute_pipeline failed: {e}"));
            // Both tables start at heap index 0.
            let ud = vec![0u32; shader.user_data_dwords().max(1) as usize];
            for u in &shader.user_data {
                assert_eq!(u.root, RootType::Table, "{u:?}");
            }
            let words = host
                .dispatch(
                    &root,
                    &pso,
                    &Dispatch {
                        roots: vec![(0, Root::Constants(ud))],
                        views: &views,
                        samplers: &samplers,
                        groups: [1, 1, 1],
                    },
                )
                .unwrap_or_else(|e| panic!("{label}: dispatch failed: {e}"));
            let pairs = words[0].chunks(2).map(|p| [p[0], p[1]]).collect();
            out.push((label, pairs));
        }
        out
    }

    /// Every lane got `data` and a zero status.
    fn check(words: &[u32], data: u32) {
        let results = run(words);
        for (label, pairs) in results {
            assert_eq!(pairs, vec![[data, 0]; 32], "{label}");
        }
    }

    #[test]
    fn a_sample_with_tfe_reads_resident() {
        check(&sample_red(), TEXEL[0].to_bits());
    }

    #[test]
    fn a_load_with_tfe_puts_the_status_after_four_channels() {
        check(&load_rgba(), TEXEL[0].to_bits());
    }

    #[test]
    fn a_gather_with_tfe_puts_the_status_after_four_texels() {
        check(&gather_red(), TEXEL[0].to_bits());
    }

    #[test]
    fn a_d16_sample_with_tfe_puts_the_status_after_the_packed_halves() {
        // Red 1.0 (0x3C00) low, green 0.25 (0x3400) high.
        check(&sample_red_green_d16(), 0x3400_3C00);
    }
}
