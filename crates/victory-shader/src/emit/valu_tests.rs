//! End-to-end tests of synthetic GFX10 VALU shaders lowered to DXIL, validated
//! by `dxil.dll` when present, and executed on the host GPU and WARP.
//!
//! Covers RDNA2 hardware semantics that differ from naive DXIL lowering:
//! `v_med3_f32`/`v_med3_f16` with NaN operands, `v_fract_f32`/`v_fract_f16`
//! near `1.0`, turn-range reduction for `v_sin_*`/`v_cos_*`, the exact
//! rounding of `v_cvt_pknorm_*`, and denormal operands of `v_mul_legacy_f32`.

use super::host::{Dispatch, Host, Root, View};
use super::{HostShader, lower, validator};
use crate::native::{
    ComputeIo, HwStage, NativeProgram, ResourceType, RootParam, RootType, StageIo, UsedResource,
};
use crate::{io, ir, resource, structure};

const S_ENDPGM: u32 = 0xBF81_0000;

/// SOP1 `s_mov_b32 sdst, src0`.
const fn s_mov_b32(sdst: u8, src0: u8) -> u32 {
    0xBE80_0000 | (sdst as u32) << 16 | 0x03 << 8 | src0 as u32
}

/// SOP2 `op sdst, src0, src1`.
const fn sop2(op: u8, sdst: u8, src0: u8, src1: u8) -> u32 {
    0x8000_0000
        | (op as u32 & 0x7F) << 23
        | (sdst as u32 & 0x7F) << 16
        | (src1 as u32) << 8
        | src0 as u32
}

/// SMEM `s_load_dwordx4 s[dst:dst+3], s[base:base+1], 0` (soffset = null).
const fn s_load_dwordx4(dst: u8, base: u8) -> [u32; 2] {
    [
        0x3D << 26 | 0x02 << 18 | (dst as u32) << 6 | (base as u32 >> 1),
        125 << 25,
    ]
}

/// VOP1 `op vdst, src0` (`src0`: 0..105 = SGPR, 256..511 = VGPR).
const fn vop1(op: u16, vdst: u8, src0: u16) -> u32 {
    0x7E00_0000 | (vdst as u32) << 17 | (op as u32) << 9 | (src0 as u32 & 0x1FF)
}

/// VOP3 `op vdst, src0, src1, src2` (no modifiers).
const fn vop3(op: u16, vdst: u8, src0: u16, src1: u16, src2: u16) -> [u32; 2] {
    vop3_mod(op, vdst, src0, src1, src2, 0, 0, 0, false)
}

/// VOP3 `op vdst, src0, src1, src2` with explicit `abs`, `neg`, `omod`, `clamp`.
#[allow(clippy::too_many_arguments)]
const fn vop3_mod(
    op: u16,
    vdst: u8,
    src0: u16,
    src1: u16,
    src2: u16,
    abs: u8,
    neg: u8,
    omod: u8,
    clamp: bool,
) -> [u32; 2] {
    [
        0x35 << 26
            | (op as u32 & 0x3FF) << 16
            | (clamp as u32) << 15
            | (abs as u32 & 7) << 8
            | vdst as u32,
        (neg as u32 & 7) << 29
            | (omod as u32 & 3) << 27
            | (src0 as u32 & 0x1FF)
            | (src1 as u32 & 0x1FF) << 9
            | (src2 as u32 & 0x1FF) << 18,
    ]
}

/// MUBUF `buffer_store_dword vdata, off, s[srsrc:srsrc+3], 0 offset:byte_off`.
const fn buffer_store_dword(vdata: u8, srsrc: u8, byte_off: u16) -> [u32; 2] {
    [
        0x38 << 26 | 0x1C << 18 | (byte_off as u32 & 0xFFF),
        0x80 << 24 | ((srsrc as u32 >> 2) & 0x1F) << 16 | (vdata as u32) << 8,
    ]
}

const fn sgpr(n: u8) -> u16 {
    n as u16
}

const fn vgpr(n: u8) -> u16 {
    256 + n as u16
}

/// Builds and lowers a 1-thread synthetic RDNA2 compute shader that receives
/// 4 dwords of root constants in `s0..s3`, runs `body` (leaving its result in
/// `v0`), and stores `v0` to dword 0 of the raw UAV at heap index 0.
fn build_valu_cs(body: &[u32]) -> HostShader {
    lower_valu_cs(body, 5 << 1, [1, 1, 1]).expect("lower")
}

/// `build_valu_cs` with the program's `COMPUTE_PGM_RSRC2` and thread counts
/// given; the lowering error is returned.
fn lower_valu_cs(body: &[u32], rsrc2: u32, threads: [u32; 3]) -> Result<HostShader, String> {
    let mut words = Vec::new();
    // s[4:5] = descriptor table address (heap index 0 -> byte offset 0).
    words.push(s_mov_b32(5, 0x80)); // inline constant 0
    words.extend_from_slice(&s_load_dwordx4(8, 4));
    // Copy root constants s0..s3 into v1..v4 so VALU instructions operate on
    // VGPRs just as they do in real shaders.
    for i in 0..4u8 {
        words.push(vop1(0x001, i + 1, sgpr(i))); // v_mov_b32
    }
    words.extend_from_slice(body);
    words.extend_from_slice(&buffer_store_dword(0, 8, 0));
    words.push(S_ENDPGM);

    let code: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let native = NativeProgram {
        stage: HwStage::Cs,
        wave64: false,
        vgprs: 8,
        layout: vec![
            RootParam {
                root: RootType::Constants,
                table_offset_dwords: 0,
                sgprs: 4,
                base_register: 0,
                user_data: 0,
                slot: 0,
            },
            RootParam {
                root: RootType::Table,
                table_offset_dwords: 0,
                sgprs: 1,
                base_register: 0,
                user_data: 4,
                slot: 1,
            },
        ],
        used: vec![UsedResource {
            kind: ResourceType::Uav,
            root: RootType::Table,
            table_offset_dwords: 0,
            lower_bound: 0,
            stride: 0,
            dwords: 4,
            user_data: 4,
            slot: 1,
        }],
        io: StageIo::Cs(ComputeIo {
            rsrc2, // 5 user SGPRs (s0..s4) unless the caller says otherwise
            threads,
        }),
        code,
    };

    let p = ir::build(&native.code, native.wave64, native.vgprs).expect("ir::build");
    let st = structure::structure(&p).expect("structure");
    let res =
        resource::track(&p, &native.layout, native.stage, Some(&st)).expect("resource::track");
    let io = io::raise(&p, &native).expect("io::raise");
    let lowered = lower::lower(&lower::Parts {
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
    .map_err(|e| format!("{e:?}"))?;

    let shader = HostShader {
        stage: HwStage::Cs,
        dxil: lowered.dxil,
        user_data: lowered.user_data,
        samplers: lowered.samplers,
        static_samplers: Vec::new(),
        root_cbvs: lowered.root_cbvs,
        root_srvs: lowered.root_srvs,
    };

    if let Ok(mut v) = validator::Validator::load(validator::Validator::default_path().as_deref()) {
        let mut copy = shader.dxil.clone();
        let ours = copy[4..20].to_vec();
        v.validate(&mut copy).expect("DXIL validation failed");
        assert_eq!(copy[4..20], ours[..], "container hash mismatch");
    }

    Ok(shader)
}

/// One GPU test at a time. With these tests on the hardware adapter and WARP
/// from several test threads at once, now and then none of their dispatches
/// ever completes and the test binary hangs; and two GPU tests at once on
/// WARP can end the test process with an access violation. Other GPU test
/// modules take this lock too.
pub(super) static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// `Host::new(false)` also selects WARP when the environment forces it.
/// Do not run that backend twice or label software results as hardware.
pub(super) fn backends(force_warp: bool) -> &'static [(&'static str, bool)] {
    if force_warp {
        &[("warp", true)]
    } else {
        &[("hardware", false), ("warp", true)]
    }
}

#[test]
fn forced_warp_runs_only_the_software_backend() {
    assert_eq!(backends(true), &[("warp", true)]);
    assert_eq!(backends(false), &[("hardware", false), ("warp", true)]);
}

/// Runs `shader` on every available SM 6.6 host (hardware GPU and WARP,
/// or WARP alone with `VICTORY_WARP=1`) for
/// each 4-dword input set and invokes `check(backend, case_index, inputs, result_dword)`.
fn run_cases(
    shader: &HostShader,
    cases: &[[u32; 4]],
    mut check: impl FnMut(&str, usize, [u32; 4], u32),
) {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let mut ran = 0;
    let force_warp = std::env::var("VICTORY_WARP").is_ok_and(|v| v == "1");
    for &(label, warp) in backends(force_warp) {
        let Ok(host) = Host::new(warp) else {
            eprintln!("no SM 6.6 {label} device; skipped");
            continue;
        };
        let (root, pso) = host
            .compute_pipeline(shader)
            .unwrap_or_else(|e| panic!("{label}: compute_pipeline failed: {e}"));
        for (i, &inputs) in cases.iter().enumerate() {
            let mut ud = vec![0u32; shader.user_data_dwords().max(1) as usize];
            for u in &shader.user_data {
                ud[u.index as usize] = match (u.root, u.slot) {
                    (RootType::Constants, 0) => inputs[u.dword as usize],
                    (RootType::Table, 1) => 0,
                    other => panic!("unexpected user data {other:?}"),
                };
            }
            let out = host
                .dispatch(
                    &root,
                    &pso,
                    &Dispatch {
                        roots: vec![(0, Root::Constants(ud))],
                        views: &[View::RawUav(1)],
                        samplers: &[],
                        groups: [1, 1, 1],
                    },
                )
                .unwrap_or_else(|e| panic!("{label}: dispatch failed: {e}"));
            check(label, i, inputs, out[0][0]);
        }
        ran += 1;
    }
    assert!(ran > 0, "neither hardware nor WARP SM 6.6 device available");
}

fn is_f16_nan(bits: u16) -> bool {
    (bits & 0x7FFF) > 0x7C00
}

fn f16_to_f32(bits: u16) -> f32 {
    let sign = u32::from(bits >> 15) << 31;
    let exp = (bits >> 10) & 0x1F;
    let mant = bits & 0x3FF;
    let f32_bits = match exp {
        0 => {
            if mant == 0 {
                sign
            } else {
                let mut m = u32::from(mant);
                let mut e = 127 - 14;
                while (m & 0x400) == 0 {
                    m <<= 1;
                    e -= 1;
                }
                sign | (e << 23) | ((m & 0x3FF) << 13)
            }
        }
        0x1F => sign | (0xFF << 23) | (u32::from(mant) << 13),
        _ => sign | ((u32::from(exp) + 112) << 23) | (u32::from(mant) << 13),
    };
    f32::from_bits(f32_bits)
}

/// MTBUF `op vdata, off, s[srsrc:srsrc+3], 0 format:format` (at byte 0).
const fn mtbuf(op: u8, vdata: u8, srsrc: u8, format: u8) -> [u32; 2] {
    [
        0x3A << 26 | (format as u32 & 0x7F) << 19 | (op as u32 & 7) << 16,
        0x80 << 24
            | (op as u32 >> 3 & 1) << 21
            | (srsrc as u32 >> 2 & 0x1F) << 16
            | (vdata as u32) << 8,
    ]
}

/// Stores `v1` to the buffer, then reads it back into `v0` with the tbuffer
/// load `op` in buffer format `format`.
fn tbuffer_load_of_v1(op: u8, format: u8) -> HostShader {
    let mut body = buffer_store_dword(1, 8, 0).to_vec();
    body.extend(mtbuf(op, 0, 8, format));
    build_valu_cs(&body)
}

#[test]
fn tbuffer_d16_loads_pack_two_channels_per_dword() {
    // tbuffer_load_format_d16_x and tbuffer_load_format_d16_xy.
    let (x, xy) = (8, 9);
    // (opcode, format, dword stored, dword loaded)
    let cases: &[(u8, u8, u32, u32)] = &[
        // 16_16_FLOAT: the f16 pair (1.0, 2.0) as it was stored.
        (xy, 29, 0x4000_3C00, 0x4000_3C00),
        // 16_16_UINT: the 16-bit integers as they were stored.
        (xy, 27, 0xBEEF_1234, 0xBEEF_1234),
        // 16_16_UNORM: 0.0 and 0.5, as f16.
        (xy, 23, 0x8000_0000, 0x3800_0000),
        // 16_FLOAT, one channel: the high half is 0.
        (x, 13, 0x4000_3C00, 0x0000_3C00),
    ];
    for &(op, format, stored, want) in cases {
        let shader = tbuffer_load_of_v1(op, format);
        run_cases(&shader, &[[stored, 0, 0, 0]], |backend, _, _, got| {
            assert_eq!(
                got, want,
                "{backend}: d16 opcode {op} format {format} of {stored:#010x} = {got:#010x}, want {want:#010x}",
            );
        });
    }
}

#[test]
fn packed_add_selects_halves_of_inline_constants() {
    // Add the inline constant 1/(2*pi) to v1 (zero) to read back its two halves.
    // The same GFX10 source field (248) has a different bit pattern for a
    // packed floating-point and a packed integer instruction.
    let cases = [
        // v_pk_add_f16 v0, 0x3118, v1
        ([0xcc0f4000, 0x1a0202f8], 0x0000_3118),
        // v_pk_add_f16 v0, 0x3118, v1 op_sel_hi:[0,1] (use low twice)
        ([0xcc0f4000, 0x120202f8], 0x3118_3118),
        // v_pk_add_u16 v0, 0x3e22f983, v1
        ([0xcc0a4000, 0x1a0202f8], 0x3e22_f983),
    ];
    for (words, expected) in cases {
        let shader = build_valu_cs(&words);
        run_cases(&shader, &[[0; 4]], |backend, _, _, got| {
            assert_eq!(got, expected, "{backend}: {words:08x?}");
        });
    }
}

#[test]
fn v_med3_f32_matches_rdna2_with_nan() {
    // v_med3_f32 v0, v1, v2, v3 (VOP3 opcode 0x157)
    let shader = build_valu_cs(&vop3(0x157, 0, vgpr(1), vgpr(2), vgpr(3)));
    let nan = f32::NAN.to_bits();
    let f = f32::to_bits;

    let cases: &[([u32; 4], Option<f32>)] = &[
        // Normal medians (all orderings).
        ([f(1.0), f(2.0), f(3.0), 0], Some(2.0)),
        ([f(3.0), f(1.0), f(2.0), 0], Some(2.0)),
        ([f(-5.0), f(0.0), f(5.0), 0], Some(0.0)),
        ([f(1.0), f(1.0), f(2.0), 0], Some(1.0)),
        // One NaN: RDNA2 returns min3 of the non-NaN operands.
        ([f(1.0), f(2.0), nan, 0], Some(1.0)),
        ([nan, f(1.0), f(0.0), 0], Some(0.0)),
        ([nan, f(0.0), f(1.0), 0], Some(0.0)),
        ([f(2.0), nan, f(1.0), 0], Some(1.0)),
        ([f(-3.0), nan, f(4.0), 0], Some(-3.0)),
        // Two NaNs: returns the sole non-NaN operand.
        ([nan, f(5.0), nan, 0], Some(5.0)),
        ([nan, nan, f(-2.0), 0], Some(-2.0)),
        ([f(7.0), nan, nan, 0], Some(7.0)),
        // Three NaNs: returns NaN.
        ([nan, nan, nan, 0], None),
    ];

    let inputs: Vec<[u32; 4]> = cases.iter().map(|c| c.0).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let expect = cases[i].1;
        let got_f = f32::from_bits(got);
        match expect {
            Some(want) => assert_eq!(
                got_f,
                want,
                "{backend}: v_med3_f32({:?}, {:?}, {:?}) = {got_f} ({got:#010x}), want {want}",
                f32::from_bits(inp[0]),
                f32::from_bits(inp[1]),
                f32::from_bits(inp[2]),
            ),
            None => assert!(
                got_f.is_nan(),
                "{backend}: v_med3_f32(NaN, NaN, NaN) = {got_f} ({got:#010x}), want NaN",
            ),
        }
    });
}

/// RDNA2 orders -0 below +0 in its maximums and minimums; DXIL's FMax and
/// FMin compare and select, which can give either zero.
#[test]
fn max_and_min_order_negative_zero_below_positive_zero() {
    let (pz, nz) = (0_u32, 0x8000_0000_u32);
    // Each case has a -0 and a +0 among its first two operands.
    let cases = [
        [nz, pz, nz, 0],
        [pz, nz, nz, 0],
        [nz, pz, pz, 0],
        [pz, nz, pz, 0],
    ];
    let ops = [
        ("v_max_f32", 0x110, pz),
        ("v_min_f32", 0x10F, nz),
        ("v_max3_f32", 0x154, pz),
        ("v_min3_f32", 0x151, nz),
    ];
    for (name, op, want) in ops {
        let shader = build_valu_cs(&vop3(op, 0, vgpr(1), vgpr(2), vgpr(3)));
        run_cases(&shader, &cases, |backend, _, inp, got| {
            assert_eq!(got, want, "{backend}: {name} {inp:x?}");
        });
    }
}

/// The f16 forms go through the same `max_min` helper, after widening to f32.
#[test]
fn f16_max_and_min_order_negative_zero_below_positive_zero() {
    let (pz, nz) = (0_u32, 0x8000_u32);
    let cases = [
        [nz, pz, nz, 0],
        [pz, nz, nz, 0],
        [nz, pz, pz, 0],
        [pz, nz, pz, 0],
    ];
    let ops = [
        ("v_max_f16", 0x139, pz),
        ("v_min_f16", 0x13A, nz),
        ("v_max3_f16", 0x354, pz),
        ("v_min3_f16", 0x351, nz),
    ];
    for (name, op, want) in ops {
        let shader = build_valu_cs(&vop3(op, 0, vgpr(1), vgpr(2), vgpr(3)));
        run_cases(&shader, &cases, |backend, _, inp, got| {
            // The f16 result is the low half of v0.
            assert_eq!(got & 0xFFFF, want, "{backend}: {name} {inp:x?}");
        });
    }
}

#[test]
fn v_min3_max3_med3_integer_forms_match_rdna2() {
    // v_{min3,max3,med3}_{i32,u32} v0, v1, v2, v3 (VOP3 0x152..0x159).
    type Pick = fn([u32; 3]) -> u32;
    let ops: [(&str, u16, Pick); 6] = [
        ("v_min3_i32", 0x152, |t| {
            t.into_iter().min_by_key(|&x| x as i32).unwrap()
        }),
        ("v_min3_u32", 0x153, |t| t.into_iter().min().unwrap()),
        ("v_max3_i32", 0x155, |t| {
            t.into_iter().max_by_key(|&x| x as i32).unwrap()
        }),
        ("v_max3_u32", 0x156, |t| t.into_iter().max().unwrap()),
        ("v_med3_i32", 0x158, |mut t| {
            t.sort_by_key(|&x| x as i32);
            t[1]
        }),
        ("v_med3_u32", 0x159, |mut t| {
            t.sort_unstable();
            t[1]
        }),
    ];
    // Every ordering of three values, with the sign bit set on some so that
    // the signed and unsigned answers differ.
    let values = [0u32, 1, 0x7FFF_FFFF, 0x8000_0000, 0xFFFF_FFFF];
    let mut cases = Vec::new();
    for a in values {
        for b in values {
            for c in values {
                cases.push([a, b, c, 0]);
            }
        }
    }
    for (name, op, pick) in ops {
        let shader = build_valu_cs(&vop3(op, 0, vgpr(1), vgpr(2), vgpr(3)));
        run_cases(&shader, &cases, |backend, _, inp, got| {
            let want = pick([inp[0], inp[1], inp[2]]);
            assert_eq!(
                got, want,
                "{backend}: {name}({:#010x}, {:#010x}, {:#010x}) = {got:#010x}, want {want:#010x}",
                inp[0], inp[1], inp[2],
            );
        });
    }
}

#[test]
fn v_med3_f16_matches_rdna2_with_nan() {
    // v_med3_f16 v0, v1, v2, v3 (VOP3 opcode 0x357)
    let shader = build_valu_cs(&vop3(0x357, 0, vgpr(1), vgpr(2), vgpr(3)));
    let h0 = 0x0000u32; // 0.0
    let h1 = 0x3C00u32; // 1.0
    let h2 = 0x4000u32; // 2.0
    let h3 = 0x4200u32; // 3.0
    let h5 = 0x4500u32; // 5.0
    let hn2 = 0xC000u32; // -2.0
    let nan = 0x7E00u32;

    let cases: &[([u32; 4], Option<u16>)] = &[
        ([h1, h2, h3, 0], Some(h2 as u16)),
        ([hn2, h0, h3, 0], Some(h0 as u16)),
        ([h1, h2, nan, 0], Some(h1 as u16)),
        ([nan, h1, h0, 0], Some(h0 as u16)),
        ([nan, h0, h1, 0], Some(h0 as u16)),
        ([h2, nan, h1, 0], Some(h1 as u16)),
        ([nan, h5, nan, 0], Some(h5 as u16)),
        ([nan, nan, nan, 0], None),
    ];

    let inputs: Vec<[u32; 4]> = cases.iter().map(|c| c.0).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let expect = cases[i].1;
        let got_h = got as u16;
        match expect {
            Some(want) => assert_eq!(
                got_h, want,
                "{backend}: v_med3_f16({:#06x}, {:#06x}, {:#06x}) = {got_h:#06x}, want {want:#06x}",
                inp[0], inp[1], inp[2],
            ),
            None => assert!(
                is_f16_nan(got_h),
                "{backend}: v_med3_f16(NaN, NaN, NaN) = {got_h:#06x}, want NaN",
            ),
        }
    });
}

/// `v_cmp_lg_*` is "less or greater": ordered and not equal, so false when
/// either operand is a NaN (RDNA2 ISA; the same on gfx1036 hardware).
#[test]
fn v_cmp_lg_is_false_for_nan() {
    // v_cmp_lg_{f32,f16}_e64 s20, v1, v2 (VOPC 0x005 / 0x0cd in VOP3);
    // v_cndmask_b32_e64 v0, 0, 1, s20 (VOP3 0x101)
    let nan = f32::NAN.to_bits();
    let (inf, ninf) = (f32::INFINITY.to_bits(), f32::NEG_INFINITY.to_bits());
    let f = f32::to_bits;
    let f32_cases: &[([u32; 4], u32)] = &[
        ([f(1.0), f(2.0), 0, 0], 1),
        ([f(2.0), f(1.0), 0, 0], 1),
        ([f(1.0), f(1.0), 0, 0], 0),
        ([f(0.0), f(-0.0), 0, 0], 0),
        ([inf, ninf, 0, 0], 1),
        ([inf, inf, 0, 0], 0),
        ([nan, f(1.0), 0, 0], 0),
        ([f(1.0), nan, 0, 0], 0),
        ([nan, nan, 0, 0], 0),
        ([0x7F80_0001, f(0.0), 0, 0], 0), // a signalling NaN
    ];
    let f16_cases: &[([u32; 4], u32)] = &[
        ([0x3C00, 0x4000, 0, 0], 1),
        ([0x4000, 0x3C00, 0, 0], 1),
        ([0x3C00, 0x3C00, 0, 0], 0),
        ([0x0000, 0x8000, 0, 0], 0),
        ([0x7C00, 0xFC00, 0, 0], 1),
        ([0x7E00, 0x3C00, 0, 0], 0),
        ([0x3C00, 0x7E00, 0, 0], 0),
        ([0x7E00, 0x7E00, 0, 0], 0),
        ([0x7C01, 0x0000, 0, 0], 0),           // a signalling NaN
        ([0xABCD_3C00, 0x1234_7E00, 0, 0], 0), // only the low halves are read
    ];
    for (op, what, cases) in [
        (0x005u16, "v_cmp_lg_f32", f32_cases),
        (0x0cd, "v_cmp_lg_f16", f16_cases),
    ] {
        let mut body = vop3(op, 20, vgpr(1), vgpr(2), 0x80).to_vec();
        body.extend_from_slice(&vop3(0x101, 0, 0x80, 0x81, sgpr(20)));
        let shader = build_valu_cs(&body);
        let inputs: Vec<[u32; 4]> = cases.iter().map(|c| c.0).collect();
        run_cases(&shader, &inputs, |backend, i, inp, got| {
            let want = cases[i].1;
            assert_eq!(
                got, want,
                "{backend}: {what}({:#010x}, {:#010x}) = {got}, want {want}",
                inp[0], inp[1],
            );
        });
    }
}

/// `v_div_fixup_f16` as the RDNA2 ISA lists it; `None` for a NaN.
fn div_fixup_f16_reference(q: u16, den: u16, num: u16) -> Option<u16> {
    let nan = |x: u16| is_f16_nan(x);
    let abs = |x: u16| x & 0x7FFF;
    let sign = (den ^ num) & 0x8000;
    if nan(num) || nan(den) {
        return None;
    }
    if (abs(den) == 0 && abs(num) == 0) || (abs(den) == 0x7C00 && abs(num) == 0x7C00) {
        return None;
    }
    if abs(den) == 0 || abs(num) == 0x7C00 {
        return Some(sign | 0x7C00);
    }
    if abs(den) == 0x7C00 || abs(num) == 0 {
        return Some(sign);
    }
    // An all-ones exponent overflows, so a NaN quotient gives infinity
    // (gfx1036 hardware).
    if abs(q) >= 0x7C00 {
        return Some(sign | 0x7C00);
    }
    Some(sign | abs(q))
}

#[test]
fn v_div_fixup_f16_follows_the_isa_special_cases() {
    // v_mov_b32 v0, 0; v_div_fixup_f16 v0, v1, v2, v3 (VOP3 opcode 0x35f):
    // v1 the quotient, v2 the denominator, v3 the numerator.
    let mut body = vec![vop1(0x001, 0, 128)];
    body.extend_from_slice(&vop3(0x35f, 0, vgpr(1), vgpr(2), vgpr(3)));
    let shader = build_valu_cs(&body);
    // 0, -0, 1, -1.5, 2^-24, max finite, inf, -inf, NaN, -NaN.
    let values = [
        0x0000u16, 0x8000, 0x3C00, 0xBE00, 0x0001, 0x7BFF, 0x7C00, 0xFC00, 0x7E00, 0xFE01,
    ];
    let mut cases = Vec::new();
    for &q in &[0x4248u16, 0xC248, 0x7E00, 0x7C01] {
        for &den in &values {
            for &num in &values {
                cases.push([u32::from(q), u32::from(den), u32::from(num), 0]);
            }
        }
    }
    run_cases(&shader, &cases, |backend, _, inp, got| {
        let (q, den, num) = (inp[0] as u16, inp[1] as u16, inp[2] as u16);
        let got_h = got as u16;
        assert_eq!(got >> 16, 0, "{backend}: the high half changed");
        match div_fixup_f16_reference(q, den, num) {
            Some(want) => assert_eq!(
                got_h, want,
                "{backend}: v_div_fixup_f16({q:#06x}, {den:#06x}, {num:#06x}) = {got_h:#06x}, \
                 want {want:#06x}",
            ),
            None => assert!(
                is_f16_nan(got_h),
                "{backend}: v_div_fixup_f16({q:#06x}, {den:#06x}, {num:#06x}) = {got_h:#06x}, \
                 want NaN",
            ),
        }
    });
}

/// The f16 nearest to `x`, ties to even, as the console's conversions
/// round (MODE's default); found by search over the f16 values, so it does
/// not share the lowering's bit arithmetic.
fn f16_nearest_reference(x: f32) -> u16 {
    if x.is_nan() {
        return 0x7E00;
    }
    let sign = if x.is_sign_negative() { 0x8000 } else { 0 };
    let a = f64::from(x.abs());
    if a >= 65520.0 {
        return sign | 0x7C00;
    }
    // The largest finite half at or below `a` (halves order as their bits).
    let (mut lo, mut hi) = (0u16, 0x7BFF);
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if f64::from(f16_to_f32(mid)) <= a {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let below = f64::from(f16_to_f32(lo));
    let above = if lo == 0x7BFF {
        65536.0
    } else {
        f64::from(f16_to_f32(lo + 1))
    };
    let pick = if a - below < above - a || (a - below == above - a && lo % 2 == 0) {
        lo
    } else {
        lo + 1
    };
    sign | pick
}

#[test]
fn f32_to_f16_rounds_to_nearest_even() {
    // Midpoints between neighbouring halves (ties), the f32 next to them, and
    // a spread of f32 values from below the smallest subnormal to past 65504.
    let mut inputs: Vec<f32> = Vec::new();
    for h in (0x0000u16..0x7BFF)
        .step_by(613)
        .chain([0x0000, 0x0001, 0x03FF, 0x0400, 0x3C00, 0x3C01, 0x7BFE])
    {
        let mid = ((f64::from(f16_to_f32(h)) + f64::from(f16_to_f32(h + 1))) / 2.0) as f32;
        inputs.extend([
            mid,
            -mid,
            f32::from_bits(mid.to_bits() + 1),
            f32::from_bits(mid.to_bits() - 1),
        ]);
    }
    let mut bits = 0x3280_0000u32; // 2^-26
    while bits < 0x4790_0000 {
        inputs.push(f32::from_bits(bits));
        bits += 0x0013_3F7D;
    }
    inputs.extend([
        65504.0,
        65519.99,
        65520.0,
        1.0e6,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        -0.0,
        2.0f32.powi(-25),
        f32::MIN_POSITIVE,
    ]);
    // v_mov_b32 v0, 0; v_cvt_f16_f32 v0, v1 (VOP1 0x00a).
    let shader = build_valu_cs(&[vop1(0x001, 0, 128), vop1(0x00A, 0, vgpr(1))]);
    let cases: Vec<[u32; 4]> = inputs.iter().map(|x| [x.to_bits(), 0, 0, 0]).collect();
    run_cases(&shader, &cases, |backend, i, _, got| {
        let (x, want) = (inputs[i], f16_nearest_reference(inputs[i]));
        let got_h = got as u16;
        if is_f16_nan(want) {
            assert!(
                is_f16_nan(got_h),
                "{backend}: v_cvt_f16_f32({x:e}) = {got_h:#06x}, want NaN"
            );
        } else {
            assert_eq!(
                got_h,
                want,
                "{backend}: v_cvt_f16_f32({x:e} = {:#010x}) = {got_h:#06x}, want {want:#06x}",
                x.to_bits()
            );
        }
    });
    // f16 arithmetic narrows the same way: 1 + 2^-10 plus 2^-11 is a tie
    // that goes to the even 1 + 2^-9 (toward zero would give 1 + 2^-10).
    let mut body = vec![vop1(0x001, 0, 128)];
    body.extend_from_slice(&vop3(0x132, 0, vgpr(1), vgpr(2), 0)); // v_add_f16 v0, v1, v2
    let add = build_valu_cs(&body);
    run_cases(
        &add,
        &[[0x3C01, 0x1000, 0, 0], [0x7BFF, 0x5000, 0, 0]],
        |backend, i, _, got| {
            let want = [0x3C02u32, 0x7C00][i];
            assert_eq!(
                got, want,
                "{backend}: v_add_f16 case {i} = {got:#06x}, want {want:#06x}"
            );
        },
    );
    // v_cvt_pkrtz_f16_f32 still rounds toward zero: 1 + 3/2048 to 1 + 2^-10,
    // 1e6 to the largest finite half, -1e-8 to -0.
    let pkrtz = build_valu_cs(&vop3(0x12F, 0, vgpr(1), vgpr(2), 0));
    let (a, b) = ((1.0f32 + 3.0 / 2048.0).to_bits(), 1.0e6f32.to_bits());
    run_cases(
        &pkrtz,
        &[[a, b, 0, 0], [(-1.0e-8f32).to_bits(), a, 0, 0]],
        |backend, i, _, got| {
            let want = [0x7BFF_3C01u32, 0x3C01_8000][i];
            assert_eq!(
                got, want,
                "{backend}: v_cvt_pkrtz_f16_f32 case {i} = {got:#010x}, want {want:#010x}"
            );
        },
    );
}

#[test]
fn v_fract_f32_never_returns_one() {
    // v_fract_f32 v0, v1 (VOP1 opcode 0x020)
    let shader = build_valu_cs(&[vop1(0x020, 0, vgpr(1))]);
    let f = f32::to_bits;
    let below_one = 0x3F7F_FFFFu32;

    let cases: &[(u32, Option<u32>)] = &[
        // Tiny negatives: must clamp strictly below 1.0 (not 0x3f800000).
        (f(-1e-10), Some(below_one)),
        (f(-1e-8), Some(below_one)),
        (f(-2.0f32.powi(-25)), Some(below_one)),
        (f(-2.0f32.powi(-24)), Some(below_one)),
        // Normal values.
        (f(0.0), Some(f(0.0))),
        (f(-0.0), Some(f(0.0))),
        (f(0.25), Some(f(0.25))),
        (f(3.75), Some(f(0.75))),
        (f(-0.25), Some(f(0.75))),
        (f(-3.25), Some(f(0.75))),
        (f(1.0), Some(f(0.0))),
        (f(-1.0), Some(f(0.0))),
        // Non-finite inputs yield NaN.
        (f(f32::NAN), None),
        (f(f32::INFINITY), None),
        (f(f32::NEG_INFINITY), None),
    ];

    let inputs: Vec<[u32; 4]> = cases.iter().map(|&(x, _)| [x, 0, 0, 0]).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let expect = cases[i].1;
        match expect {
            Some(want) => assert_eq!(
                got,
                want,
                "{backend}: v_fract_f32({:?} / {:#010x}) = {got:#010x} ({:?}), want {want:#010x}",
                f32::from_bits(inp[0]),
                inp[0],
                f32::from_bits(got),
            ),
            None => assert!(
                f32::from_bits(got).is_nan(),
                "{backend}: v_fract_f32({:?}) = {got:#010x}, want NaN",
                f32::from_bits(inp[0]),
            ),
        }
    });
}

#[test]
fn v_fract_f16_never_returns_one() {
    // v_fract_f16 v0, v1 (VOP1 opcode 0x05f)
    let shader = build_valu_cs(&[vop1(0x05f, 0, vgpr(1))]);
    let below_one = 0x3BFFu16; // 1.0 - 2^-11

    assert_eq!(f16_to_f32(0x9400), -2.0f32.powi(-10));
    assert_eq!(f16_to_f32(0x9000), -2.0f32.powi(-11));
    assert_eq!(f16_to_f32(0x8C00), -2.0f32.powi(-12));

    let cases: &[(u16, Option<u16>)] = &[
        // Negatives in (-2^-11, 0): must clamp to 0x3BFF, never 0x3C00 (1.0).
        (0x8C00, Some(below_one)), // -2^-12
        (0x8400, Some(below_one)), // -2^-14
        (0x8001, Some(below_one)), // -2^-24 (smallest negative f16 subnormal)
        (0x9000, Some(below_one)), // -2^-11 (exact 1 - 2^-11 = 0x3BFF)
        (0x9400, Some(0x3BFE)),    // -2^-10 (exact 1 - 2^-10 = 0x3BFE)
        // Normal values.
        (0x0000, Some(0x0000)), // 0.0 -> 0.0
        (0x3400, Some(0x3400)), // 0.25 -> 0.25
        (0xB400, Some(0x3A00)), // -0.25 -> 0.75
        (0x4300, Some(0x3800)), // 3.5 -> 0.5
        (0x3C00, Some(0x0000)), // 1.0 -> 0.0
        // Non-finite inputs yield NaN.
        (0x7E00, None), // NaN
        (0x7C00, None), // +inf
        (0xFC00, None), // -inf
    ];

    let inputs: Vec<[u32; 4]> = cases
        .iter()
        .map(|&(x, _)| [u32::from(x), 0, 0, 0])
        .collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let expect = cases[i].1;
        let got_h = got as u16;
        match expect {
            Some(want) => assert_eq!(
                got_h, want,
                "{backend}: v_fract_f16({:#06x}) = {got_h:#06x}, want {want:#06x}",
                inp[0],
            ),
            None => assert!(
                is_f16_nan(got_h),
                "{backend}: v_fract_f16({:#06x}) = {got_h:#06x}, want NaN",
                inp[0],
            ),
        }
    });
}

/// RDNA2's flt16_to_snorm16 / flt16_to_unorm16: clamp, scale by 32767 or
/// 65535, round the exact product to nearest even; NaN gives 0.
fn cvt_norm16_reference(bits: u16, signed: bool) -> u16 {
    let x = f16_to_f32(bits);
    if x.is_nan() {
        return 0;
    }
    let (lo, scale) = if signed {
        (-1.0, 32767.0)
    } else {
        (0.0, 65535.0)
    };
    // f64 holds the product of an f16 and a 16-bit scale exactly.
    let p = f64::from(x.clamp(lo, 1.0)) * scale;
    p.round_ties_even() as i32 as u16
}

#[test]
fn v_cvt_norm_i16_and_u16_f16_round_the_exact_product() {
    // Every 251st f16 pattern, plus the values where an f32 product would
    // round before the rounding to an integer, ties, and the edges.
    let mut bits: Vec<u16> = (0..=u16::MAX).step_by(251).collect();
    bits.extend([
        0x0000, 0x8000, 0x0001, 0x8001, 0x03FF, 0x0400, 0x3800, 0xB800, 0x3801, 0xB801, 0x37FF,
        0x3BFF, 0xBBFF, 0x3C00, 0xBC00, 0x3C01, 0x7BFF, 0x7C00, 0xFC00, 0x7E00, 0xFE00, 0x7C01,
        0x3A9B, 0xBA9B, 0x2E66, 0x1400,
    ]);
    for (op, signed, what) in [
        (0x063u16, true, "v_cvt_norm_i16_f16"),
        (0x064, false, "v_cvt_norm_u16_f16"),
    ] {
        // v_cvt_norm_*16_f16 v0, v1 (VOP1); v0's high half starts at 0.
        let shader = build_valu_cs(&[vop1(0x001, 0, 128), vop1(op, 0, vgpr(1))]);
        let inputs: Vec<[u32; 4]> = bits.iter().map(|&b| [u32::from(b), 0, 0, 0]).collect();
        run_cases(&shader, &inputs, |backend, i, _, got| {
            let want = cvt_norm16_reference(bits[i], signed);
            assert_eq!(
                got,
                u32::from(want),
                "{backend}: {what}({:#06x}) = {got:#010x}, want {want:#06x}",
                bits[i],
            );
        });
    }
}

#[test]
fn v_sin_and_v_cos_f32_reduce_large_turns() {
    let sin_shader = build_valu_cs(&[vop1(0x035, 0, vgpr(1))]);
    let cos_shader = build_valu_cs(&[vop1(0x036, 0, vgpr(1))]);
    let f = f32::to_bits;
    let half_sqrt2 = std::f32::consts::FRAC_1_SQRT_2;

    let sin_cases: &[(f32, Option<f32>)] = &[
        (0.0, Some(0.0)),
        (0.25, Some(1.0)),
        (255.0, Some(0.0)),
        (255.25, Some(1.0)),
        (255.5, Some(0.0)),
        (-255.25, Some(-1.0)),
        (10_000.125, Some(half_sqrt2)),
        (f32::NAN, None),
        (f32::INFINITY, None),
        (f32::NEG_INFINITY, None),
    ];
    let sin_inputs: Vec<[u32; 4]> = sin_cases.iter().map(|&(x, _)| [f(x), 0, 0, 0]).collect();
    run_cases(&sin_shader, &sin_inputs, |backend, i, inp, got| {
        let expect = sin_cases[i].1;
        let got_f = f32::from_bits(got);
        match expect {
            Some(want) => assert!(
                (got_f - want).abs() < 1e-5,
                "{backend}: v_sin_f32({}) = {got_f}, want {want} (err = {})",
                f32::from_bits(inp[0]),
                (got_f - want).abs(),
            ),
            None => assert!(
                got_f.is_nan(),
                "{backend}: v_sin_f32({}) = {got_f}, want NaN",
                f32::from_bits(inp[0]),
            ),
        }
    });

    let cos_cases: &[(f32, Option<f32>)] = &[
        (0.0, Some(1.0)),
        (0.25, Some(0.0)),
        (255.0, Some(1.0)),
        (255.25, Some(0.0)),
        (255.5, Some(-1.0)),
        (-255.5, Some(-1.0)),
        (10_000.125, Some(half_sqrt2)),
        (f32::NAN, None),
        (f32::INFINITY, None),
        (f32::NEG_INFINITY, None),
    ];
    let cos_inputs: Vec<[u32; 4]> = cos_cases.iter().map(|&(x, _)| [f(x), 0, 0, 0]).collect();
    run_cases(&cos_shader, &cos_inputs, |backend, i, inp, got| {
        let expect = cos_cases[i].1;
        let got_f = f32::from_bits(got);
        match expect {
            Some(want) => assert!(
                (got_f - want).abs() < 1e-5,
                "{backend}: v_cos_f32({}) = {got_f}, want {want} (err = {})",
                f32::from_bits(inp[0]),
                (got_f - want).abs(),
            ),
            None => assert!(
                got_f.is_nan(),
                "{backend}: v_cos_f32({}) = {got_f}, want NaN",
                f32::from_bits(inp[0]),
            ),
        }
    });

    // Main's `trig_turns` preserves sin(-0.0) = -0.0 and produces exact +0.0 at
    // half-turns (sin) and quarter-turns (cos).
    let exact_sin: &[(u32, u32)] = &[
        (0x8000_0000, 0x8000_0000), // sin(-0.0) = -0.0
        (f(255.0), 0),              // sin(255.0) = +0.0
        (f(255.5), 0),              // sin(255.5) = +0.0
    ];
    let exact_sin_in: Vec<[u32; 4]> = exact_sin.iter().map(|&(x, _)| [x, 0, 0, 0]).collect();
    run_cases(&sin_shader, &exact_sin_in, |backend, i, inp, got| {
        let want = exact_sin[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_sin_f32({:#010x}) = {got:#010x}, want {want:#010x}",
            inp[0],
        );
    });
    let exact_cos: &[(u32, u32)] = &[
        (f(0.25), 0),   // cos(0.25) = +0.0
        (f(255.25), 0), // cos(255.25) = +0.0
    ];
    let exact_cos_in: Vec<[u32; 4]> = exact_cos.iter().map(|&(x, _)| [x, 0, 0, 0]).collect();
    run_cases(&cos_shader, &exact_cos_in, |backend, i, inp, got| {
        let want = exact_cos[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_cos_f32({:#010x}) = {got:#010x}, want {want:#010x}",
            inp[0],
        );
    });
}

#[test]
fn v_cvt_u16_and_i16_f16_saturate() {
    // v_mov_b32 v0, 0; v_cvt_u16_f16 / v_cvt_i16_f16 v0, v1 (VOP1 0x052 / 0x053).
    let cases: &[(u16, u16, u16)] = &[
        // (f16 input, u16 result, i16 result)
        (0x0000, 0, 0),          // 0
        (0x8000, 0, 0),          // -0
        (0x3E00, 1, 1),          // 1.5 -> 1 (toward zero)
        (0xBE00, 0, 0xFFFF),     // -1.5 -> 0 / -1
        (0x57D0, 125, 125),      // 125.0
        (0x7800, 32768, 32767),  // 32768 saturates i16
        (0x77FF, 32752, 32752),  // 32752, the largest f16 below 32768
        (0xF800, 0, 0x8000),     // -32768 fits i16
        (0xF801, 0, 0x8000),     // -32800 saturates i16
        (0x7BFF, 65504, 32767),  // max finite f16
        (0xFBFF, 0, 0x8000),     // -65504
        (0x7C00, 0xFFFF, 32767), // +inf
        (0xFC00, 0, 0x8000),     // -inf
        (0x7E00, 0, 0),          // NaN
        (0xFE00, 0, 0),          // -NaN
        (0x0001, 0, 0),          // smallest subnormal
    ];
    for (op, what, pick) in [
        (0x052u16, "v_cvt_u16_f16", 0usize),
        (0x053, "v_cvt_i16_f16", 1),
    ] {
        let shader = build_valu_cs(&[vop1(0x001, 0, 128), vop1(op, 0, vgpr(1))]);
        let inputs: Vec<[u32; 4]> = cases.iter().map(|c| [u32::from(c.0), 0, 0, 0]).collect();
        run_cases(&shader, &inputs, |backend, i, _, got| {
            let want = if pick == 0 { cases[i].1 } else { cases[i].2 };
            assert_eq!(
                got,
                u32::from(want),
                "{backend}: {what}({:#06x}) = {got:#010x}, want {want:#06x}",
                cases[i].0,
            );
        });
    }
}

#[test]
fn integer_add_sub_and_mul_lower_as_their_unsigned_twins() {
    // The low 32 bits of a sum, difference or product do not depend on the
    // sign, so the _i32 forms are bit-identical to the _u32 forms; the ISA
    // says the same ("No carry-in or carry-out", D.i = S0.i + S1.i).
    let cases: &[[u32; 4]] = &[
        [0x7FFF_FFFF, 1, 0, 0],
        [0xFFFF_FFFF, 0xFFFF_FFFF, 0, 0],
        [0x8000_0000, 1, 0, 0],
    ];
    for (op, name) in [(0x37Fu16, "v_add_nc_i32"), (0x16B, "v_mul_lo_i32")] {
        let shader = build_valu_cs(&vop3(op, 0, vgpr(1), vgpr(2), vgpr(0)));
        run_cases(&shader, cases, |backend, _, inp, got| {
            let want = if name == "v_mul_lo_i32" {
                inp[0].wrapping_mul(inp[1])
            } else {
                inp[0].wrapping_add(inp[1])
            };
            assert_eq!(got, want, "{backend}: {name}({:#x}, {:#x})", inp[0], inp[1]);
        });
    }
    // Subtraction: the sign matters for the operand order, not the encoding.
    let sub = build_valu_cs(&vop3(0x376, 0, vgpr(1), vgpr(2), vgpr(0)));
    run_cases(
        &sub,
        &[[1, 2, 0, 0], [0x8000_0000, 1, 0, 0]],
        |backend, i, inp, got| {
            assert_eq!(
                got,
                inp[0].wrapping_sub(inp[1]),
                "{backend}: v_sub_nc_i32 case {i}"
            );
        },
    );
}

#[test]
fn signed_16_bit_add_and_sub_wrap_like_the_unsigned_forms() {
    // v_add_nc_i16 / v_sub_nc_i16: the mantissa is 16 bits either way, so the
    // low half is the same as the unsigned form.
    for (op, add) in [(0x30Du16, true), (0x30E, false)] {
        let shader = build_valu_cs(&vop3(op, 0, vgpr(1), vgpr(2), vgpr(0)));
        let cases: &[(u32, u32)] = &[
            (0x0000_7FFF, 0x0000_0001), // 32767 + 1 wraps to -32768
            (0xFFFF_8000, 0xFFFF_0002), // -32768 + 2
            (0x0000_1234, 0x0000_0001),
        ];
        let inputs: Vec<[u32; 4]> = cases.iter().map(|&(a, b)| [a, b, 0, 0]).collect();
        run_cases(&shader, &inputs, |backend, i, inp, got| {
            let (a, b) = (inp[0] as u16, inp[1] as u16);
            let want = if add {
                a.wrapping_add(b)
            } else {
                a.wrapping_sub(b)
            };
            assert_eq!(
                got as u16,
                want,
                "{backend}: {} case {i} ({a:#06x}, {b:#06x})",
                if add { "v_add_nc_i16" } else { "v_sub_nc_i16" }
            );
        });
    }
}

#[test]
fn v_swap_b32_exchanges_two_vgprs_and_copies_from_anything_else() {
    // ISA 101 V_SWAP_B32, "tmp = D.u; D.u = S0.u; S0.u = tmp": both VGPRs
    // change. Integer constant n is operand code 128 + n.
    let swap_then_read = |read: u16| {
        build_valu_cs(&[
            vop1(0x001, 1, 128 + 5), // v1 = 5
            vop1(0x001, 2, 128 + 7), // v2 = 7
            vop1(0x065, 1, vgpr(2)), // v_swap_b32 v1, v2
            vop1(0x001, 0, read),    // v0 = the register under test
        ])
    };
    let first = swap_then_read(vgpr(1));
    run_cases(&first, &[[0; 4]], |backend, _, _, got| {
        assert_eq!(got, 7, "{backend}: v1 after the swap");
    });
    let second = swap_then_read(vgpr(2));
    run_cases(&second, &[[0; 4]], |backend, _, _, got| {
        assert_eq!(got, 5, "{backend}: v2 after the swap");
    });

    // Only a VGPR source is written back: an integer constant cannot be, so
    // the instruction is a move into the destination.
    let from_constant = build_valu_cs(&[vop1(0x065, 1, 128 + 9), vop1(0x001, 0, vgpr(1))]);
    run_cases(&from_constant, &[[0; 4]], |backend, _, _, got| {
        assert_eq!(got, 9, "{backend}: v1 from a constant");
    });
}

#[test]
fn v_cvt_f16_u16_and_i16_widen_the_integer_first() {
    // v_mov_b32 v0, 0; v_cvt_f16_u16 / v_cvt_f16_i16 v0, v1 (VOP1 0x050 / 0x051).
    // Only integers f16 holds exactly: how the others round is the narrowing's
    // business, not this conversion's. `None`: the result depends on rounding.
    let cases: &[(u16, Option<u16>, Option<u16>)] = &[
        // (u16 / i16 input bits, f16 of the unsigned value, f16 of the signed value)
        (0x0000, Some(0x0000), Some(0x0000)), // 0
        (0x0001, Some(0x3C00), Some(0x3C00)), // 1.0
        (0x07FF, Some(0x67FF), Some(0x67FF)), // 2047, the last integer below 2048 that f16 holds
        (0x8000, Some(0x7800), Some(0xF800)), // 32768 / -32768
        (0xFC00, Some(0x7BE0), Some(0xE400)), // 64512 / -1024
        (0xFFFE, None, Some(0xC000)),         // 65534 does not fit f16 / -2
        (0xFFFF, None, Some(0xBC00)),         // 65535 does not fit f16 / -1
    ];
    for (op, what, pick) in [
        (0x050u16, "v_cvt_f16_u16", 0usize),
        (0x051, "v_cvt_f16_i16", 1),
    ] {
        let shader = build_valu_cs(&[vop1(0x001, 0, 128), vop1(op, 0, vgpr(1))]);
        let inputs: Vec<[u32; 4]> = cases.iter().map(|c| [u32::from(c.0), 0, 0, 0]).collect();
        run_cases(&shader, &inputs, |backend, i, _, got| {
            let want = if pick == 0 { cases[i].1 } else { cases[i].2 };
            let Some(want) = want else { return };
            assert_eq!(
                got,
                u32::from(want),
                "{backend}: {what}({:#06x}) = {got:#010x}, want {want:#06x}",
                cases[i].0,
            );
        });
    }
}

#[test]
fn v_sin_and_v_cos_f16_reduce_large_turns() {
    let sin_shader = build_valu_cs(&[vop1(0x060, 0, vgpr(1))]);
    let cos_shader = build_valu_cs(&[vop1(0x061, 0, vgpr(1))]);

    // Sanity-check the f16 bit encodings of large turns.
    assert_eq!(f16_to_f32(0x5BF8), 255.0);
    assert_eq!(f16_to_f32(0x5BFA), 255.25);
    assert_eq!(f16_to_f32(0x5BFC), 255.5);
    assert_eq!(f16_to_f32(0x7BFF), 65504.0);

    let sin_cases: &[(u16, Option<f32>)] = &[
        (0x5BF8, Some(0.0)),  // 255.0
        (0x5BFA, Some(1.0)),  // 255.25
        (0x5BFC, Some(0.0)),  // 255.5
        (0xDBFA, Some(-1.0)), // -255.25
        (0x7BFF, Some(0.0)),  // 65504.0 (largest finite f16, exact integer)
        (0x7E00, None),       // NaN
        (0x7C00, None),       // +inf
        (0xFC00, None),       // -inf
    ];
    let sin_inputs: Vec<[u32; 4]> = sin_cases
        .iter()
        .map(|&(x, _)| [u32::from(x), 0, 0, 0])
        .collect();
    run_cases(&sin_shader, &sin_inputs, |backend, i, inp, got| {
        let expect = sin_cases[i].1;
        let got_h = got as u16;
        match expect {
            Some(want) => {
                let got_f = f16_to_f32(got_h);
                assert!(
                    (got_f - want).abs() < 1e-3,
                    "{backend}: v_sin_f16({:#06x} = {}) = {got_f} ({got_h:#06x}), want {want}",
                    inp[0],
                    f16_to_f32(inp[0] as u16),
                );
            }
            None => assert!(
                is_f16_nan(got_h),
                "{backend}: v_sin_f16({:#06x}) = {got_h:#06x}, want NaN",
                inp[0],
            ),
        }
    });

    let cos_cases: &[(u16, Option<f32>)] = &[
        (0x5BF8, Some(1.0)),  // 255.0
        (0x5BFA, Some(0.0)),  // 255.25
        (0x5BFC, Some(-1.0)), // 255.5
        (0x7BFF, Some(1.0)),  // 65504.0 (largest finite f16, exact integer)
        (0x7E00, None),       // NaN
        (0x7C00, None),       // +inf
        (0xFC00, None),       // -inf
    ];
    let cos_inputs: Vec<[u32; 4]> = cos_cases
        .iter()
        .map(|&(x, _)| [u32::from(x), 0, 0, 0])
        .collect();
    run_cases(&cos_shader, &cos_inputs, |backend, i, inp, got| {
        let expect = cos_cases[i].1;
        let got_h = got as u16;
        match expect {
            Some(want) => {
                let got_f = f16_to_f32(got_h);
                assert!(
                    (got_f - want).abs() < 1e-3,
                    "{backend}: v_cos_f16({:#06x} = {}) = {got_f} ({got_h:#06x}), want {want}",
                    inp[0],
                    f16_to_f32(inp[0] as u16),
                );
            }
            None => assert!(
                is_f16_nan(got_h),
                "{backend}: v_cos_f16({:#06x}) = {got_h:#06x}, want NaN",
                inp[0],
            ),
        }
    });
}

/// A 32-bit literal in a signed 64-bit operand is sign-extended (an
/// unsigned one's is zero-extended): -100 compared with `v[1:2]`, and
/// shifted by `s_ashr_i64`.
#[test]
fn signed_64_bit_operands_sign_extend_their_literal() {
    const MINUS_100: u32 = -100_i32 as u32;
    let check = |name: &str, body: &[u32], cases: &[([u32; 4], u32)]| {
        let shader = build_valu_cs(body);
        let inputs: Vec<[u32; 4]> = cases.iter().map(|c| c.0).collect();
        run_cases(&shader, &inputs, |backend, i, inp, got| {
            let want = cases[i].1;
            assert_eq!(got, want, "{backend}: {name} {inp:x?}");
        });
    };
    // v_cmp_*_i64 s6, v[1:2], -100 (VOP3, literal src1), then
    // v_cndmask_b32 v0, 0, 1, s6.
    let compare = |op: u16| {
        let [a, b] = vop3(op, 6, vgpr(1), 255, 0);
        let [c, d] = vop3(0x101, 0, 0x80, 0x81, sgpr(6));
        [a, b, MINUS_100, c, d]
    };
    // An i64 as the two input dwords of `v[1:2]`.
    let pair = |n: i64| [n as u32, (n >> 32) as u32, 0, 0];
    let eq = [(pair(-100), 1), ([MINUS_100, 0, 0, 0], 0), (pair(0), 0)];
    check("v_cmp_eq_i64", &compare(0x0A2), &eq);
    let lt = [(pair(-200), 1), (pair(-50), 0), (pair(0), 0)];
    check("v_cmp_lt_i64", &compare(0x0A1), &lt);
    // s_ashr_i64 s[6:7], -100, 4; v_mov_b32 v0, s7: the high dword.
    let ashr = [sop2(0x23, 6, 255, 0x84), MINUS_100, vop1(0x001, 0, sgpr(7))];
    check("s_ashr_i64", &ashr, &[([0; 4], u32::MAX)]);
}

#[test]
fn bfe_signed_and_unsigned_match_rdna2() {
    // v_bfe_i32 v0, v1, v2, v3 (VOP3 opcode 0x149)
    let v_signed = build_valu_cs(&vop3(0x149, 0, vgpr(1), vgpr(2), vgpr(3)));
    // v_bfe_u32 v0, v1, v2, v3 (VOP3 opcode 0x148)
    let v_unsigned = build_valu_cs(&vop3(0x148, 0, vgpr(1), vgpr(2), vgpr(3)));
    // s_bfe_i32 s6, s0, s1 (SOP2 opcode 0x28: s1 = off[4:0] | width[22:16]); v_mov_b32 v0, s6
    let s_signed = build_valu_cs(&[sop2(0x28, 6, 0, 1), vop1(0x001, 0, sgpr(6))]);

    let signed_cases: &[([u32; 4], u32)] = &[
        // Field crosses bit 31: extends from bit 31.
        ([0xd2f9_0159, 28, 16, 0], 0xffff_fffd),
        ([0x8000_0000, 31, 4, 0], 0xffff_ffff),
        ([0x7000_0000, 28, 8, 0], 0x0000_0007),
        // Normal fields inside 32 bits.
        ([0x0000_0080, 4, 4, 0], 0xffff_fff8),
        ([0x0000_0070, 4, 4, 0], 0x0000_0007),
        // Edge widths: 0 yields 0; v_bfe masks width to 5 bits (32 -> 0),
        // while 31 extracts all remaining bits from offset 4.
        ([0xffff_ffff, 4, 0, 0], 0),
        ([0x8000_0000, 4, 31, 0], 0xf800_0000),
    ];
    let v_in: Vec<[u32; 4]> = signed_cases.iter().map(|c| c.0).collect();
    run_cases(&v_signed, &v_in, |backend, i, inp, got| {
        let want = signed_cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_bfe_i32({:#010x}, {}, {}) = {got:#010x}, want {want:#010x}",
            inp[0], inp[1], inp[2],
        );
    });
    // v_bfe_i32 width is 5-bit (src2[4:0]), so width=32 wraps to 0.
    run_cases(
        &v_signed,
        &[[0x8000_0000, 4, 32, 0]],
        |backend, _, _, got| {
            assert_eq!(got, 0, "{backend}: v_bfe_i32 width 32 should mask to 0");
        },
    );

    // s_bfe_i32 width is 7-bit (src1[22:16]), so width=32 keeps all 32 bits.
    let mut s_cases = signed_cases.to_vec();
    s_cases.push(([0x8000_0000, 4, 32, 0], 0xf800_0000));
    let s_in: Vec<[u32; 4]> = s_cases
        .iter()
        .map(|&(inp, _)| [inp[0], (inp[1] & 31) | ((inp[2] & 0x7F) << 16), 0, 0])
        .collect();
    run_cases(&s_signed, &s_in, |backend, i, inp, got| {
        let want = s_cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: s_bfe_i32({:#010x}, {:#010x}) = {got:#010x}, want {want:#010x}",
            inp[0], inp[1],
        );
    });

    let unsigned_cases: &[([u32; 4], u32)] = &[
        ([0xd2f9_0159, 28, 16, 0], 0x0000_000d),
        ([0x8000_0000, 31, 4, 0], 0x0000_0001),
        ([0xffff_ffff, 4, 0, 0], 0),
        ([0x8000_0000, 4, 31, 0], 0x0800_0000),
    ];
    let u_in: Vec<[u32; 4]> = unsigned_cases.iter().map(|c| c.0).collect();
    run_cases(&v_unsigned, &u_in, |backend, i, inp, got| {
        let want = unsigned_cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_bfe_u32({:#010x}, {}, {}) = {got:#010x}, want {want:#010x}",
            inp[0], inp[1], inp[2],
        );
    });

    // s_bfe_u32 s6, s0, s1 (SOP2 opcode 0x27): the same 7-bit width as the signed form,
    // so width=32 keeps all 32 bits and nothing is sign-extended.
    let s_unsigned = build_valu_cs(&[sop2(0x27, 6, 0, 1), vop1(0x001, 0, sgpr(6))]);
    let mut su_cases = unsigned_cases.to_vec();
    su_cases.push(([0x8000_0000, 4, 32, 0], 0x0800_0000));
    let su_in: Vec<[u32; 4]> = su_cases
        .iter()
        .map(|&(inp, _)| [inp[0], (inp[1] & 31) | ((inp[2] & 0x7F) << 16), 0, 0])
        .collect();
    run_cases(&s_unsigned, &su_in, |backend, i, inp, got| {
        let want = su_cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: s_bfe_u32({:#010x}, {:#010x}) = {got:#010x}, want {want:#010x}",
            inp[0], inp[1],
        );
    });
}

#[test]
fn v_cmp_class_f32_matches_rdna2() {
    // v_cmp_class_f32_e64 s6, v1, v2 (VOP3 opcode 0x088); v_cndmask_b32_e64 v0, 0, 1, s6
    let cmp = build_valu_cs(&[
        vop3(0x088, 6, vgpr(1), vgpr(2), 0)[0],
        vop3(0x088, 6, vgpr(1), vgpr(2), 0)[1],
        vop3(0x101, 0, 0x80, 0x81, sgpr(6))[0],
        vop3(0x101, 0, 0x80, 0x81, sgpr(6))[1],
    ]);

    // All 10 IEEE-754 binary32 classes: bit k corresponds to class k.
    let classes: &[(u32, u32)] = &[
        (0x7F80_0001, 0), // +sNaN (quiet bit 0x0040_0000 clear)
        (0xFFBF_FFFF, 0), // -sNaN (sign ignored for NaNs)
        (0x7FC0_0000, 1), // +qNaN
        (0xFFC0_0001, 1), // -qNaN
        (0xFF80_0000, 2), // -inf
        (0xBF80_0000, 3), // -normal (-1.0)
        (0x8000_0001, 4), // -denormal
        (0x8000_0000, 5), // -0.0
        (0x0000_0000, 6), // +0.0
        (0x0000_0001, 7), // +denormal
        (0x3F80_0000, 8), // +normal (+1.0)
        (0x7F80_0000, 9), // +inf
    ];
    let mut cases = Vec::new();
    for &(bits, cls) in classes {
        // Exact class bit -> true; complement of class bit across all 10 bits -> false.
        cases.push(([bits, 1u32 << cls, 0, 0], 1u32));
        cases.push(([bits, 0x3FF & !(1u32 << cls), 0, 0], 0u32));
    }
    let inputs: Vec<[u32; 4]> = cases.iter().map(|c| c.0).collect();
    run_cases(&cmp, &inputs, |backend, i, inp, got| {
        let want = cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_cmp_class_f32({:#010x}, {:#05x}) = {got}, want {want}",
            inp[0], inp[1],
        );
    });
}

#[test]
fn v_cmp_class_f16_matches_rdna2() {
    // v_cmp_class_f16_e64 s6, v1, v2 (VOP3 opcode 0x08f); v_cndmask_b32_e64 v0, 0, 1, s6
    let cmp_lo = build_valu_cs(&[
        vop3(0x08f, 6, vgpr(1), vgpr(2), 0)[0],
        vop3(0x08f, 6, vgpr(1), vgpr(2), 0)[1],
        vop3(0x101, 0, 0x80, 0x81, sgpr(6))[0],
        vop3(0x101, 0, 0x80, 0x81, sgpr(6))[1],
    ]);
    // Same with op_sel:[1,0] (high half of v1) + neg on src0 (bit 29 of w1).
    let [mut w0, mut w1] = vop3(0x08f, 6, vgpr(1), vgpr(2), 0);
    w0 |= 1 << 11; // opsel[0] = 1 (high half)
    w1 |= 1 << 29; // neg[0] = 1
    let cmp_hi_neg = build_valu_cs(&[
        w0,
        w1,
        vop3(0x101, 0, 0x80, 0x81, sgpr(6))[0],
        vop3(0x101, 0, 0x80, 0x81, sgpr(6))[1],
    ]);

    // All 10 IEEE-754 binary16 classes: bit k corresponds to class k.
    let classes: &[(u32, u32)] = &[
        (0x7C01, 0), // sNaN (+sNaN, quiet bit 0x0200 clear)
        (0xFDFF, 0), // -sNaN (sign ignored for NaNs)
        (0x7E00, 1), // +qNaN
        (0xFE01, 1), // -qNaN
        (0xFC00, 2), // -inf
        (0xBC00, 3), // -normal (-1.0)
        (0x8001, 4), // -denormal
        (0x8000, 5), // -0.0
        (0x0000, 6), // +0.0
        (0x0001, 7), // +denormal
        (0x3C00, 8), // +normal (+1.0)
        (0x7C00, 9), // +inf
    ];
    let mut cases = Vec::new();
    for &(bits, cls) in classes {
        // Exact class bit -> true; complement of class bit across all 10 bits -> false.
        cases.push(([bits, 1u32 << cls, 0, 0], 1u32));
        cases.push(([bits, 0x3FF & !(1u32 << cls), 0, 0], 0u32));
    }
    let inputs: Vec<[u32; 4]> = cases.iter().map(|c| c.0).collect();
    run_cases(&cmp_lo, &inputs, |backend, i, inp, got| {
        let want = cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_cmp_class_f16({:#06x}, {:#05x}) = {got}, want {want}",
            inp[0], inp[1],
        );
    });

    // High half with neg: high half 0x3C00 (+1.0) negated -> -normal (class 3);
    // high half 0x7C01 (+sNaN) negated -> still sNaN (class 0, not quieted).
    let hi_cases: &[([u32; 4], u32)] = &[
        ([0x3C00_0000, 1 << 3, 0, 0], 1),
        ([0x3C00_0000, 1 << 8, 0, 0], 0),
        ([0x7C01_FFFF, 1 << 0, 0, 0], 1),
        ([0x7C01_FFFF, 1 << 1, 0, 0], 0),
    ];
    let hi_in: Vec<[u32; 4]> = hi_cases.iter().map(|c| c.0).collect();
    run_cases(&cmp_hi_neg, &hi_in, |backend, i, inp, got| {
        let want = hi_cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_cmp_class_f16(-hi({:#010x}), {:#05x}) = {got}, want {want}",
            inp[0], inp[1],
        );
    });
}

/// VOP2 `op vdst, src0, vsrc1` (`src0`: 0..105 = SGPR, 256..511 = VGPR).
const fn vop2(op: u8, vdst: u8, src0: u16, vsrc1: u8) -> u32 {
    (op as u32 & 0x3F) << 25 | (vdst as u32) << 17 | (vsrc1 as u32) << 9 | (src0 as u32 & 0x1FF)
}

/// VOP3P `op vdst, src0, src1, src2` with its packed modifiers: `op_sel` and
/// `op_sel_hi` pick each source's half for the low and high lanes of the
/// operation (bit k for source k), `neg_lo`/`neg_hi` negate them.
#[allow(clippy::too_many_arguments)]
const fn vop3p_mods(
    op: u8,
    vdst: u8,
    [src0, src1, src2]: [u16; 3],
    op_sel: u8,
    op_sel_hi: u8,
    neg_lo: u8,
    neg_hi: u8,
    clamp: bool,
) -> [u32; 2] {
    [
        0x33 << 26
            | (op as u32 & 0x7F) << 16
            | (clamp as u32) << 15
            | (op_sel_hi as u32 >> 2 & 1) << 14
            | (op_sel as u32 & 7) << 11
            | (neg_hi as u32 & 7) << 8
            | vdst as u32,
        (src0 as u32 & 0x1FF)
            | (src1 as u32 & 0x1FF) << 9
            | (src2 as u32 & 0x1FF) << 18
            | (op_sel_hi as u32 & 3) << 27
            | (neg_lo as u32 & 7) << 29,
    ]
}

/// Inputs `[src0, src1, src2, _]` and the f32 bits RDNA2 gives for
/// `src0.f16[0] * src1.f16[0] + src0.f16[1] * src1.f16[1] + src2.f32`: f16
/// products are exact in f32, so 65504 * 65504 does not overflow as it would
/// in f16.
const DOT2_F32_F16_CASES: &[([u32; 4], u32)] = &[
    // (1, 2) . (3, 4) + 0.5 = 11.5
    ([0x4000_3c00, 0x4400_4200, 0x3f00_0000, 0], 0x4138_0000),
    // (-1.5, 0.25) . (2, -8) - 1 = -6
    ([0x3400_be00, 0xc800_4000, 0xbf80_0000, 0], 0xc0c0_0000),
    // (65504, 65504) . (65504, 0) + 0 = 4290774016
    ([0x7bff_7bff, 0x0000_7bff, 0, 0], 0x4f7f_c004),
    // (inf, 0) . (1, 0) + 0 = inf
    ([0x0000_7c00, 0x0000_3c00, 0, 0], 0x7f80_0000),
];

#[test]
fn v_dot2_f32_f16_sums_f16_products_in_f32() {
    // v0 = v_dot2_f32_f16 v1, v2, v3 (no modifiers: op_sel_hi = 0b111)
    let body = vop3p_mods(0x13, 0, [vgpr(1), vgpr(2), vgpr(3)], 0, 0b111, 0, 0, false);
    let shader = build_valu_cs(&body);
    let inputs: Vec<[u32; 4]> = DOT2_F32_F16_CASES.iter().map(|c| c.0).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let want = DOT2_F32_F16_CASES[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_dot2_f32_f16({:#010x}, {:#010x}, {:#010x}) = {got:#010x}, want {want:#010x}",
            inp[0], inp[1], inp[2],
        );
    });
}

#[test]
fn v_dot2_f32_f16_applies_op_sel_and_negation_per_half() {
    // src0's halves swapped (op_sel bit 0 set, op_sel_hi bit 0 clear), src1's
    // low half negated (neg_lo bit 1), src0's high-lane half negated (neg_hi
    // bit 0): (1, 2) . (3, 4) + 0.5 becomes 2 * -3 + -1 * 4 + 0.5 = -9.5.
    let body = vop3p_mods(
        0x13,
        0,
        [vgpr(1), vgpr(2), vgpr(3)],
        0b001,
        0b110,
        0b010,
        0b001,
        false,
    );
    let shader = build_valu_cs(&body);
    let cases: &[([u32; 4], u32)] = &[
        ([0x4000_3c00, 0x4400_4200, 0x3f00_0000, 0], 0xc118_0000),
        // (-1.5, 0.25) . (2, -8) - 1 with the same modifiers:
        // 0.25 * -2 + 1.5 * -8 - 1 = -13.5
        ([0x3400_be00, 0xc800_4000, 0xbf80_0000, 0], 0xc158_0000),
    ];
    let inputs: Vec<[u32; 4]> = cases.iter().map(|c| c.0).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let want = cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_dot2_f32_f16 with modifiers({:#010x}, {:#010x}, {:#010x}) = {got:#010x}, want {want:#010x}",
            inp[0], inp[1], inp[2],
        );
    });
}

#[test]
fn v_dot2_f32_f16_clamps_to_zero_one() {
    let body = vop3p_mods(0x13, 0, [vgpr(1), vgpr(2), vgpr(3)], 0, 0b111, 0, 0, true);
    let shader = build_valu_cs(&body);
    let cases: &[([u32; 4], u32)] = &[
        // 11.5 clamps to 1.0, -6 to 0.0
        ([0x4000_3c00, 0x4400_4200, 0x3f00_0000, 0], 0x3f80_0000),
        ([0x3400_be00, 0xc800_4000, 0xbf80_0000, 0], 0),
    ];
    let inputs: Vec<[u32; 4]> = cases.iter().map(|c| c.0).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let want = cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_dot2_f32_f16 clamp({:#010x}, {:#010x}, {:#010x}) = {got:#010x}, want {want:#010x}",
            inp[0], inp[1], inp[2],
        );
    });
}

#[test]
fn f16_omod_applies_before_clamp() {
    // VOP3 v_add_f16 (0x132 = 0x100 + VOP2 0x32): v0 = (v1 + v2) * omod [clamp]
    let mul2 = build_valu_cs(&vop3_mod(0x132, 0, vgpr(1), vgpr(2), 0, 0, 0, 1, false));
    let mul4 = build_valu_cs(&vop3_mod(0x132, 0, vgpr(1), vgpr(2), 0, 0, 0, 2, false));
    let div2 = build_valu_cs(&vop3_mod(0x132, 0, vgpr(1), vgpr(2), 0, 0, 0, 3, false));
    let div2_clamp = build_valu_cs(&vop3_mod(0x132, 0, vgpr(1), vgpr(2), 0, 0, 0, 3, true));

    // f16 constants: 0.25=0x3400, 0.5=0x3800, 0.75=0x3A00, 1.0=0x3C00, 1.5=0x3E00, 2.0=0x4000, -0.5=0xB800
    run_cases(
        &mul2,
        &[[0x3400, 0x3800, 0, 0]], // (0.25 + 0.5) * 2 = 1.5 (0x3E00)
        |backend, _, _, got| {
            assert_eq!(got as u16, 0x3E00, "{backend}: v_add_f16 mul:2");
        },
    );
    run_cases(
        &mul4,
        &[[0x3400, 0x3400, 0, 0]], // (0.25 + 0.25) * 4 = 2.0 (0x4000)
        |backend, _, _, got| {
            assert_eq!(got as u16, 0x4000, "{backend}: v_add_f16 mul:4");
        },
    );
    run_cases(
        &div2,
        &[[0x3E00, 0x3E00, 0, 0]], // (1.5 + 1.5) / 2 = 1.5 (0x3E00)
        |backend, _, _, got| {
            assert_eq!(got as u16, 0x3E00, "{backend}: v_add_f16 div:2");
        },
    );
    // Ordering check: (0.75 + 0.75) = 1.5; div:2 before clamp gives 0.75 (0x3A00),
    // whereas clamp before div:2 would give 0.5 (0x3800).
    let clamp_cases: &[(u16, u16, u16)] = &[
        (0x3A00, 0x3A00, 0x3A00), // (0.75 + 0.75) / 2 = 0.75 -> 0.75
        (0x3E00, 0x3E00, 0x3C00), // (1.5 + 1.5) / 2 = 1.5 -> clamped to 1.0
        (0xB800, 0xB800, 0x0000), // (-0.5 + -0.5) / 2 = -0.5 -> clamped to 0.0
    ];
    let inputs: Vec<[u32; 4]> = clamp_cases
        .iter()
        .map(|&(a, b, _)| [u32::from(a), u32::from(b), 0, 0])
        .collect();
    run_cases(&div2_clamp, &inputs, |backend, i, _, got| {
        let want = clamp_cases[i].2;
        assert_eq!(
            got as u16, want,
            "{backend}: v_add_f16 div:2 clamp case {i}: got {:#06x}, want {want:#06x}",
            got as u16,
        );
    });
}

#[test]
fn v_dot2c_f32_f16_accumulates_into_its_destination() {
    // v3 = v_dot2c_f32_f16 v1, v2 (v3 += v1 . v2), then v0 = v3.
    let body = [vop2(0x02, 3, vgpr(1), 2), vop1(0x001, 0, vgpr(3))];
    let shader = build_valu_cs(&body);
    let inputs: Vec<[u32; 4]> = DOT2_F32_F16_CASES.iter().map(|c| c.0).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let want = DOT2_F32_F16_CASES[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_dot2c_f32_f16({:#010x}, {:#010x}) + {:#010x} = {got:#010x}, want {want:#010x}",
            inp[0], inp[1], inp[2],
        );
    });
}

/// Inputs where rounding the sum of the two products before adding the
/// addend gives a different f32 than RDNA2, which rounds the exact
/// `p0 + p1 + addend` once (measured on gfx1036 in #465).
const DOT2_ROUNDED_ONCE_CASES: &[([u32; 4], u32)] = &[
    // (1, 2^-12) . (1, 2^-12) - 1 = 2^-24; 1 + 2^-24 rounded first gives 0.
    ([0x0c00_3c00, 0x0c00_3c00, 0xbf80_0000, 0], 0x3380_0000),
    // (1, 2^-14) . (1, 2^-14) + 2^24 = 2^24 + 1 + 2^-28: above the tie
    // between 2^24 and 2^24 + 2, so it rounds up; 1 + 2^-28 rounded first
    // lands on the tie, which rounds to even (2^24).
    ([0x0400_3c00, 0x0400_3c00, 0x4b80_0000, 0], 0x4b80_0001),
    // (-0, -0) . (1, 1) - 0 = -0: the sign of an all-negative-zero sum.
    ([0x8000_8000, 0x3c00_3c00, 0x8000_0000, 0], 0x8000_0000),
];

#[test]
fn v_dot2_f32_f16_rounds_the_exact_sum_once() {
    let body = vop3p_mods(0x13, 0, [vgpr(1), vgpr(2), vgpr(3)], 0, 0b111, 0, 0, false);
    let shader = build_valu_cs(&body);
    let inputs: Vec<[u32; 4]> = DOT2_ROUNDED_ONCE_CASES.iter().map(|c| c.0).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let want = DOT2_ROUNDED_ONCE_CASES[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_dot2_f32_f16({:#010x}, {:#010x}, {:#010x}) = {got:#010x}, want {want:#010x}",
            inp[0], inp[1], inp[2],
        );
    });
}

#[test]
fn v_dot2c_f32_f16_rounds_the_exact_sum_once() {
    let body = [vop2(0x02, 3, vgpr(1), 2), vop1(0x001, 0, vgpr(3))];
    let shader = build_valu_cs(&body);
    let inputs: Vec<[u32; 4]> = DOT2_ROUNDED_ONCE_CASES.iter().map(|c| c.0).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let want = DOT2_ROUNDED_ONCE_CASES[i].1;
        assert_eq!(
            got, want,
            "{backend}: v_dot2c_f32_f16({:#010x}, {:#010x}) + {:#010x} = {got:#010x}, want {want:#010x}",
            inp[0], inp[1], inp[2],
        );
    });
}

/// `s_and_b32 s7, s6, 63; v_mov_b32 v0, s7` on a program with 6 user SGPRs
/// (s0..s5) and `TG_SIZE_EN`, which puts the thread-group info in s6.
const TG_INFO_RSRC2: u32 = 6 << 1 | 1 << 10;

#[test]
fn thread_group_info_wave_count_matches_the_group() {
    // (threads in X, waves of 32): the info SGPR holds the wave count in 5:0.
    for (threads, waves) in [(1, 1), (32, 1), (33, 2), (64, 2), (96, 3), (256, 8)] {
        let body = [sop2(0x0E, 7, 6, 128 + 63), vop1(0x001, 0, sgpr(7))];
        let shader = lower_valu_cs(&body, TG_INFO_RSRC2, [threads, 1, 1]).expect("lower");
        run_cases(&shader, &[[0; 4]], |backend, _, _, got| {
            assert_eq!(got, waves, "{backend}: {threads} threads");
        });
        // s_bfe_u32 s7, s6, (width 6 << 16 | offset 0): the same field.
        let body = [sop2(0x27, 7, 6, 0xFF), 6 << 16, vop1(0x001, 0, sgpr(7))];
        let shader = lower_valu_cs(&body, TG_INFO_RSRC2, [threads, 1, 1]).expect("lower");
        run_cases(&shader, &[[0; 4]], |backend, _, _, got| {
            assert_eq!(got, waves, "{backend}: {threads} threads (bfe)");
        });
    }
}

#[test]
fn thread_group_info_bits_above_the_wave_count_are_refused() {
    // The wave id / ordered-append field is not derived.
    let body = [sop2(0x0E, 7, 6, 128 + 63), vop1(0x001, 0, sgpr(6))];
    assert!(lower_valu_cs(&body, TG_INFO_RSRC2, [64, 1, 1]).is_err());
    // s_and_b32 with a wider mask (a literal: 0xFFFF).
    let body = [sop2(0x0E, 7, 6, 0xFF), 0xFFFF, vop1(0x001, 0, sgpr(7))];
    assert!(lower_valu_cs(&body, TG_INFO_RSRC2, [64, 1, 1]).is_err());
}

#[test]
fn thread_group_info_mask_may_come_first() {
    // s_and_b32 s7, 63, s6 (the order ACO writes it).
    let body = [sop2(0x0E, 7, 128 + 63, 6), vop1(0x001, 0, sgpr(7))];
    let shader = lower_valu_cs(&body, TG_INFO_RSRC2, [64, 1, 1]).expect("lower");
    run_cases(&shader, &[[0; 4]], |backend, _, _, got| {
        assert_eq!(got, 2, "{backend}");
    });
}

/// `s_cmp_eq_u32 s0, s1; s_cbranch_scc1 join; s_mov_b32 s6, 0; join: <tail>`:
/// the thread-group info reaches `tail` through a phi when the branch is taken.
fn tg_info_through_phi(tail: &[u32]) -> Vec<u32> {
    let mut body = vec![0xBF06_0100, 0xBF85_0001, s_mov_b32(6, 128)];
    body.extend_from_slice(tail);
    body
}

#[test]
fn thread_group_info_full_width_read_after_a_phi_is_refused() {
    let body = tg_info_through_phi(&[vop1(0x001, 0, sgpr(6))]);
    assert!(lower_valu_cs(&body, TG_INFO_RSRC2, [64, 1, 1]).is_err());
    // Through a plain copy as well.
    let body = tg_info_through_phi(&[s_mov_b32(7, 6), vop1(0x001, 0, sgpr(7))]);
    assert!(lower_valu_cs(&body, TG_INFO_RSRC2, [64, 1, 1]).is_err());
}

#[test]
fn thread_group_info_masked_after_a_phi_is_accepted() {
    let body = tg_info_through_phi(&[sop2(0x0E, 7, 6, 128 + 63), vop1(0x001, 0, sgpr(7))]);
    let shader = lower_valu_cs(&body, TG_INFO_RSRC2, [64, 1, 1]).expect("lower");
    // s0 == s1: the info reaches the join, masked to the wave count.
    run_cases(&shader, &[[0; 4]], |backend, _, _, got| {
        assert_eq!(got, 2, "{backend}");
    });
    // A copy first, then the mask.
    let body = tg_info_through_phi(&[
        s_mov_b32(7, 6),
        sop2(0x0E, 7, 7, 128 + 63),
        vop1(0x001, 0, sgpr(7)),
    ]);
    assert!(lower_valu_cs(&body, TG_INFO_RSRC2, [64, 1, 1]).is_ok());
}

/// VOPC `op vcc, src0, vsrc1`.
const fn vopc(op: u8, src0: u16, vsrc1: u8) -> u32 {
    0x7C00_0000 | (op as u32) << 17 | (vsrc1 as u32) << 9 | (src0 as u32 & 0x1FF)
}

/// SOP1 `op sdst, src0`.
const fn sop1(op: u8, sdst: u8, src0: u8) -> u32 {
    0xBE80_0000 | (sdst as u32) << 16 | (op as u32) << 8 | src0 as u32
}

/// DS `ds_bpermute_b32 vdst, vaddr, vdata0 offset:off`.
const fn ds_bpermute_b32(vdst: u8, vaddr: u8, vdata0: u8, offset: u16) -> [u32; 2] {
    [
        0xD800_0000 | (0x0B3 << 18) | offset as u32,
        (vdst as u32) << 24 | (vdata0 as u32) << 8 | vaddr as u32,
    ]
}

/// MUBUF `buffer_store_dword vdata, vaddr, s[srsrc:srsrc+3], 0 offen`.
const fn buffer_store_dword_offen(vdata: u8, vaddr: u8, srsrc: u8) -> [u32; 2] {
    [
        0x38 << 26 | 0x1C << 18 | 1 << 12,
        0x80 << 24 | ((srsrc as u32 >> 2) & 0x1F) << 16 | (vdata as u32) << 8 | vaddr as u32,
    ]
}

/// Builds, validates and runs a single-wave compute shader (`32` threads in
/// Wave32, `64` in Wave64, with zero LDS allocated) where `body` leaves each
/// lane's result in `v3`, stored to dword `threadIdInGroup.x` of a raw UAV.
fn run_wave_cs(wave64: bool, body: &[u32], mut check: impl FnMut(&str, &[u32])) {
    let wave = if wave64 { 64 } else { 32 };
    let mut words = Vec::new();
    // s[4:5] = descriptor table address (heap index 0 -> byte offset 0).
    words.push(s_mov_b32(5, 0x80)); // inline constant 0
    words.extend_from_slice(&s_load_dwordx4(8, 4));
    words.extend_from_slice(body);
    // v7 = v0 << 2 (per-lane byte offset in the output UAV).
    words.push(vop2(0x1A, 7, 0x80 + 2, 0)); // v_lshlrev_b32 v7, 2, v0
    words.extend_from_slice(&buffer_store_dword_offen(3, 7, 8));
    words.push(S_ENDPGM);

    let code: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let native = NativeProgram {
        stage: HwStage::Cs,
        wave64,
        vgprs: 12,
        layout: vec![RootParam {
            root: RootType::Table,
            table_offset_dwords: 0,
            sgprs: 1,
            base_register: 0,
            user_data: 4,
            slot: 0,
        }],
        used: vec![UsedResource {
            kind: ResourceType::Uav,
            root: RootType::Table,
            table_offset_dwords: 0,
            lower_bound: 0,
            stride: 0,
            dwords: 4,
            user_data: 4,
            slot: 0,
        }],
        io: StageIo::Cs(ComputeIo {
            rsrc2: 5 << 1, // 5 user SGPRs, lds_bytes = 0
            threads: [wave, 1, 1],
        }),
        code,
    };

    let p = ir::build(&native.code, native.wave64, native.vgprs).expect("ir::build");
    let st = structure::structure(&p).expect("structure");
    let res =
        resource::track(&p, &native.layout, native.stage, Some(&st)).expect("resource::track");
    let io = io::raise(&p, &native).expect("io::raise");
    let lowered = lower::lower(&lower::Parts {
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
    .expect("lower");

    let shader = HostShader {
        stage: HwStage::Cs,
        dxil: lowered.dxil,
        user_data: lowered.user_data,
        samplers: lowered.samplers,
        static_samplers: Vec::new(),
        root_cbvs: lowered.root_cbvs,
        root_srvs: lowered.root_srvs,
    };

    if let Ok(mut v) = validator::Validator::load(validator::Validator::default_path().as_deref()) {
        let mut copy = shader.dxil.clone();
        v.validate(&mut copy).expect("DXIL validation failed");
    }

    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let mut ran = 0;
    for (label, warp) in [("hardware", false), ("warp", true)] {
        let Ok(host) = Host::new(warp) else {
            eprintln!("no SM 6.6 {label} device; skipped");
            continue;
        };
        let (root, pso) = match host.compute_pipeline(&shader) {
            Ok(p) => p,
            // WARP does not support fixed WaveSize, and 32-lane GPUs reject WaveSize(64).
            Err(e) if warp || wave64 => {
                eprintln!("{label}: skipped fixed wave size {wave} ({e})");
                continue;
            }
            Err(e) => panic!("{label}: compute_pipeline failed: {e}"),
        };
        let ud = vec![0u32; shader.user_data_dwords().max(1) as usize];
        let out = host
            .dispatch(
                &root,
                &pso,
                &Dispatch {
                    roots: vec![(0, Root::Constants(ud))],
                    views: &[View::RawUav(wave)],
                    samplers: &[],
                    groups: [1, 1, 1],
                },
            )
            .unwrap_or_else(|e| panic!("{label}: dispatch failed: {e}"));
        check(label, &out[0]);
        ran += 1;
    }
    if ran == 0 {
        eprintln!("no SM 6.6 Wave{wave} device available; DXIL validation only");
    }
}

#[test]
fn ds_bpermute_b32_gathers_with_offset_and_wrap() {
    // Lane t (v0 = 0..31):
    //   v1 = 0x1000 + t          (source payload)
    //   v2 = ((31 - t) << 2) | (t & 3)  (reversed byte addr + low sub-dword bits)
    //   ds_bpermute_b32 v3, v2, v1 offset:12  (adds +3 lanes and wraps mod 32)
    let mut body = vec![
        vop2(0x25, 1, 0xFF, 0),      // v_add_nc_u32 v1, 0x1000, v0
        0x1000,                      // literal
        vop2(0x26, 2, 0x80 + 31, 0), // v_sub_nc_u32 v2, 31, v0
        vop2(0x1A, 2, 0x80 + 2, 2),  // v_lshlrev_b32 v2, 2, v2
        vop2(0x1B, 4, 0x80 + 3, 0),  // v_and_b32 v4, 3, v0
        vop2(0x1C, 2, vgpr(4), 2),   // v_or_b32 v2, v4, v2
    ];
    body.extend_from_slice(&ds_bpermute_b32(3, 2, 1, 12));

    run_wave_cs(false, &body, |backend, got| {
        for (t, &val) in got.iter().enumerate() {
            let src = ((31 - t as u32) + 3) & 31;
            let want = 0x1000 + src;
            assert_eq!(
                val, want,
                "{backend}: lane {t} got {val:#x}, want {want:#x} (from lane {src})",
            );
        }
    });
}

#[test]
fn ds_bpermute_b32_zeroes_inactive_source_and_preserves_inactive_dst() {
    // Lane t (v0 = 0..31):
    //   v1 = 0x2000 + t          (source payload)
    //   v3 = 0xDEAD_0000 + t     (pre-existing destination value)
    //   v2 = (t + 8) << 2        (pulls from lane (t + 8) & 31)
    // Narrow EXEC to t < 16:
    //   - lanes 0..7  pull from active lanes 8..15   -> 0x2000 + t + 8
    //   - lanes 8..15 pull from inactive lanes 16..23 -> 0
    //   - lanes 16..31 are inactive during bpermute  -> preserve 0xDEAD_0000 + t
    let mut body = vec![
        vop2(0x25, 1, 0xFF, 0),     // v_add_nc_u32 v1, 0x2000, v0
        0x2000,                     // literal
        vop2(0x25, 3, 0xFF, 0),     // v_add_nc_u32 v3, 0xDEAD_0000, v0
        0xDEAD_0000,                // literal
        vop2(0x25, 2, 0x80 + 8, 0), // v_add_nc_u32 v2, 8, v0
        vop2(0x1A, 2, 0x80 + 2, 2), // v_lshlrev_b32 v2, 2, v2
        vop1(0x001, 5, 0x80 + 16),  // v_mov_b32 v5, 16
        vopc(0xC1, vgpr(0), 5),     // v_cmp_lt_u32 vcc, v0, v5
        sop1(0x3C, 6, 0x6A),        // s_and_saveexec_b32 s6, vcc_lo
    ];
    body.extend_from_slice(&ds_bpermute_b32(3, 2, 1, 0));
    body.push(sop1(0x03, 0x7E, 6)); // s_mov_b32 exec_lo, s6

    run_wave_cs(false, &body, |backend, got| {
        for (t, &val) in got.iter().enumerate() {
            let t = t as u32;
            let want = if t < 8 {
                0x2000 + t + 8
            } else if t < 16 {
                0
            } else {
                0xDEAD_0000 + t
            };
            assert_eq!(
                val, want,
                "{backend}: lane {t} got {val:#010x}, want {want:#010x}",
            );
        }
    });
}

#[test]
fn ds_bpermute_b32_divergent_branch_zeroes_skipped_lanes() {
    // Same mask as above, but with `s_cbranch_execz` around `ds_bpermute_b32`
    // so structurisation emits a per-lane CFG branch rather than a guard.
    let mut body = vec![
        vop2(0x25, 1, 0xFF, 0),     // v_add_nc_u32 v1, 0x3000, v0
        0x3000,                     // literal
        vop2(0x25, 3, 0xFF, 0),     // v_add_nc_u32 v3, 0xBEEF_0000, v0
        0xBEEF_0000,                // literal
        vop2(0x25, 2, 0x80 + 8, 0), // v_add_nc_u32 v2, 8, v0
        vop2(0x1A, 2, 0x80 + 2, 2), // v_lshlrev_b32 v2, 2, v2
        vop1(0x001, 5, 0x80 + 16),  // v_mov_b32 v5, 16
        vopc(0xC1, vgpr(0), 5),     // v_cmp_lt_u32 vcc, v0, v5
        sop1(0x3C, 6, 0x6A),        // s_and_saveexec_b32 s6, vcc_lo
        0xBF88_0002,                // s_cbranch_execz +2 (skip the 2-dword DS op)
    ];
    body.extend_from_slice(&ds_bpermute_b32(3, 2, 1, 0));
    body.push(sop1(0x03, 0x7E, 6)); // s_mov_b32 exec_lo, s6

    run_wave_cs(false, &body, |backend, got| {
        for (t, &val) in got.iter().enumerate() {
            let t = t as u32;
            let want = if t < 8 {
                0x3000 + t + 8
            } else if t < 16 {
                0
            } else {
                0xBEEF_0000 + t
            };
            assert_eq!(
                val, want,
                "{backend}: lane {t} got {val:#010x}, want {want:#010x}",
            );
        }
    });
}

#[test]
fn ds_bpermute_b32_wave64_stays_within_each_32_lane_half() {
    // RDNA 2 wave64 permutes lanes 0..31 and 32..63 independently (section 10.4):
    //   v1 = 0x4000 + t
    //   v2 = (t & 31) << 2    (lane 32 passes byte addr 0)
    //   ds_bpermute_b32 v3, v2, v1 offset:12
    // Lower half wraps 29..31 -> 0..2; upper half gathers 35..63 and wraps 61..63 -> 32..34.
    let mut body = vec![
        vop2(0x25, 1, 0xFF, 0),      // v_add_nc_u32 v1, 0x4000, v0
        0x4000,                      // literal
        vop2(0x1B, 2, 0x80 + 31, 0), // v_and_b32 v2, 31, v0
        vop2(0x1A, 2, 0x80 + 2, 2),  // v_lshlrev_b32 v2, 2, v2
    ];
    body.extend_from_slice(&ds_bpermute_b32(3, 2, 1, 12));

    run_wave_cs(true, &body, |backend, got| {
        for (t, &val) in got.iter().enumerate() {
            let t = t as u32;
            let src = (t & !31) | ((t + 3) & 31);
            let want = 0x4000 + src;
            assert_eq!(
                val, want,
                "{backend}: wave64 lane {t} got {val:#x}, want {want:#x} (from lane {src})",
            );
        }
    });
}

#[test]
fn ds_bpermute_b32_wave64_exec_mask_per_half() {
    // Active lanes 0..39 in wave64:
    //   v1 = 0x5000 + t, v3 = 0xCAFE_0000 + t, v2 = ((t + 8) & 31) << 2
    //   - lanes 0..31:  all active, gather from lower half (t + 8) & 31
    //   - lanes 32..39: active, pull from upper-half lanes 40..47 (inactive -> 0,
    //                   even though lower-half lanes 8..15 are active)
    //   - lanes 40..63: inactive -> preserve 0xCAFE_0000 + t
    let mut body = vec![
        vop2(0x25, 1, 0xFF, 0),      // v_add_nc_u32 v1, 0x5000, v0
        0x5000,                      // literal
        vop2(0x25, 3, 0xFF, 0),      // v_add_nc_u32 v3, 0xCAFE_0000, v0
        0xCAFE_0000,                 // literal
        vop2(0x25, 2, 0x80 + 8, 0),  // v_add_nc_u32 v2, 8, v0
        vop2(0x1B, 2, 0x80 + 31, 2), // v_and_b32 v2, 31, v2
        vop2(0x1A, 2, 0x80 + 2, 2),  // v_lshlrev_b32 v2, 2, v2
        vop1(0x001, 5, 0x80 + 40),   // v_mov_b32 v5, 40
        vopc(0xC1, vgpr(0), 5),      // v_cmp_lt_u32 vcc, v0, v5
        sop1(0x24, 6, 0x6A),         // s_and_saveexec_b64 s[6:7], vcc
    ];
    body.extend_from_slice(&ds_bpermute_b32(3, 2, 1, 0));
    body.push(sop1(0x04, 0x7E, 6)); // s_mov_b64 exec, s[6:7]

    run_wave_cs(true, &body, |backend, got| {
        for (t, &val) in got.iter().enumerate() {
            let t = t as u32;
            let want = if t < 32 {
                0x5000 + ((t + 8) & 31)
            } else if t < 40 {
                0
            } else {
                0xCAFE_0000 + t
            };
            assert_eq!(
                val, want,
                "{backend}: wave64 lane {t} got {val:#010x}, want {want:#010x}",
            );
        }
    });
}

/// A 16-bit snorm (`signed`) or unorm of `x`, as the RDNA2 ISA reference
/// gives `v_cvt_pknorm_*`: clamped, scaled, rounded to nearest even from the
/// exact product (exact in f64), NaN as 0.
fn norm16_ref(x: f32, signed: bool) -> u16 {
    if x.is_nan() {
        return 0;
    }
    let (lo, k) = if signed {
        (-1.0, 32767.0)
    } else {
        (0.0, 65535.0)
    };
    let n = (f64::from(x.clamp(lo, 1.0)) * k).round_ties_even() as i32;
    n as u16
}

/// Inputs in [0.5, 1) and near 0.001 whose f32 product with the scale lands
/// exactly on a tie although the exact product does not: a translation that
/// multiplies in f32 rounds them the wrong way.
fn f32_tie_traps(signed: bool) -> Vec<f32> {
    let k: f32 = if signed { 32767.0 } else { 65535.0 };
    let mut traps = Vec::new();
    for start in [0.5f32, 0.001] {
        let first = start.to_bits();
        for b in first..first + 4_000_000 {
            let x = f32::from_bits(b);
            let p = x * k;
            if p.fract() == 0.5 && f64::from(x) * f64::from(k) != f64::from(p) {
                traps.push(x);
                if traps.len() % 3 == 0 {
                    break;
                }
            }
        }
    }
    traps
}

fn pknorm_f32_inputs(signed: bool) -> Vec<f32> {
    let mut xs = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        2.0,
        -2.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        -f32::NAN,
        0.5,
        -0.5,
        0.25,
        1.0 / 3.0,
        -0.7,
        0.999_99,
        f32::MIN_POSITIVE,
        1e-6,
        -1e-6,
        f32::from_bits(1), // denormal
        1.0 / 65535.0,
        0.5 / 65535.0,
        1.5 / 32767.0,
        f32::from_bits(0x3F7F_FFFF), // just below 1
    ];
    xs.extend(f32_tie_traps(signed));
    xs
}

#[test]
fn v_cvt_pknorm_f32_rounds_the_exact_product() {
    for (op, signed, what) in [
        (0x368u16, true, "v_cvt_pknorm_i16_f32"),
        (0x369, false, "v_cvt_pknorm_u16_f32"),
    ] {
        // The traps must tell an f32 product apart from the exact one.
        let k: f32 = if signed { 32767.0 } else { 65535.0 };
        let f32_product = |x: f32| (x * k).round_ties_even() as i32 as u16;
        let traps = f32_tie_traps(signed);
        assert!(
            traps
                .iter()
                .any(|&x| f32_product(x) != norm16_ref(x, signed)),
            "{what}: no input separates an f32 product from the exact one"
        );
        let shader = build_valu_cs(&vop3(op, 0, vgpr(1), vgpr(2), 0));
        let xs = pknorm_f32_inputs(signed);
        // Each input once in each half, paired with its neighbour.
        let cases: Vec<[u32; 4]> = (0..xs.len())
            .map(|i| [xs[i].to_bits(), xs[(i + 1) % xs.len()].to_bits(), 0, 0])
            .collect();
        run_cases(&shader, &cases, |backend, _, inp, got| {
            let (a, b) = (f32::from_bits(inp[0]), f32::from_bits(inp[1]));
            let want = u32::from(norm16_ref(a, signed)) | u32::from(norm16_ref(b, signed)) << 16;
            assert_eq!(
                got, want,
                "{backend}: {what}({a:e} [{:#010x}], {b:e} [{:#010x}]) = {got:#010x}, want {want:#010x}",
                inp[0], inp[1],
            );
        });
    }
}

#[test]
fn v_cvt_pknorm_f16_rounds_the_exact_product() {
    let halves: Vec<u16> = vec![
        0x0000, 0x8000, 0x3C00, 0xBC00, 0x4000, 0xC000, 0x7C00, 0xFC00, 0x7E00, 0xFE00,
        0x3800, // 0.5: a tie in both scales
        0xB800, 0x3555, 0x0001, // smallest denormal
        0x8001, 0x0400, // smallest normal
        0x3BFF, // just below 1
        0x1001, 0x2E66, 0xB266, 0x3A00, 0x3401,
        // Large denormals, which scale to 1 or 2: kept, not flushed (the
        // FP16 input-denormal mode RDNA2 measurements matched).
        0x03D6, 0x83D6, 0x823F, 0x03FF,
    ];
    // Measured on RDNA2 (gfx1036) with FP16 input denormals kept.
    assert_eq!(norm16_ref(f16_to_f32(0x83D6), true), 0xFFFE);
    assert_eq!(norm16_ref(f16_to_f32(0x823F), true), 0xFFFF);
    for (op, signed, what) in [
        (0x312u16, true, "v_cvt_pknorm_i16_f16"),
        (0x313, false, "v_cvt_pknorm_u16_f16"),
    ] {
        let shader = build_valu_cs(&vop3(op, 0, vgpr(1), vgpr(2), 0));
        let cases: Vec<[u32; 4]> = (0..halves.len())
            .map(|i| {
                let next = halves[(i + 1) % halves.len()];
                // Junk in the high halves: only the low halves are read.
                [
                    u32::from(halves[i]) | 0xABCD_0000,
                    u32::from(next) | 0x1234_0000,
                    0,
                    0,
                ]
            })
            .collect();
        run_cases(&shader, &cases, |backend, _, inp, got| {
            let (a, b) = (inp[0] as u16, inp[1] as u16);
            let (fa, fb) = (f16_to_f32(a), f16_to_f32(b));
            let want = u32::from(norm16_ref(fa, signed)) | u32::from(norm16_ref(fb, signed)) << 16;
            assert_eq!(
                got, want,
                "{backend}: {what}({a:#06x}, {b:#06x}) = {got:#010x}, want {want:#010x}",
            );
        });
    }
}

#[test]
fn v_mul_legacy_f32_treats_denormals_as_zero() {
    // v_mul_legacy_f32 v0, v1, v2 (VOP2 opcode 0x007 in its VOP3 encoding)
    let shader = build_valu_cs(&vop3(0x107, 0, vgpr(1), vgpr(2), vgpr(0)));
    let f = f32::to_bits;
    let (denorm, neg_denorm, min_normal) = (0x0000_0001u32, 0x8040_0000u32, 0x0080_0000u32);

    let cases: &[(u32, u32, u32)] = &[
        // Zero or denormal times anything, non-finite included, is +0.
        (f(0.0), f(f32::INFINITY), f(0.0)),
        (f(-0.0), f(f32::NAN), f(0.0)),
        (f(f32::NAN), f(0.0), f(0.0)),
        (denorm, f(f32::INFINITY), f(0.0)),
        (f(f32::NAN), neg_denorm, f(0.0)),
        (denorm, f(2.0f32.powi(100)), f(0.0)),
        // The smallest normal is not zero.
        (min_normal, f(2.0), f(f32::MIN_POSITIVE * 2.0)),
        // Ordinary products.
        (f(1.5), f(-4.0), f(-6.0)),
        (f(3.0), f(0.5), f(1.5)),
    ];

    let inputs: Vec<[u32; 4]> = cases.iter().map(|&(a, b, _)| [a, b, 0, 0]).collect();
    run_cases(&shader, &inputs, |backend, i, inp, got| {
        let want = cases[i].2;
        assert_eq!(
            got, want,
            "{backend}: v_mul_legacy_f32({:#010x}, {:#010x}) = {got:#010x}, want {want:#010x}",
            inp[0], inp[1],
        );
    });
}

#[test]
fn cvt_16bit_int_from_f16_saturates_and_honors_modifiers() {
    // VOP3 v_cvt_u16_f16 = 0x1D2 (0x180 + VOP1 0x52)
    // VOP3 v_cvt_i16_f16 = 0x1D3 (0x180 + VOP1 0x53)
    let cvt_u16 = build_valu_cs(&vop3_mod(0x1D2, 0, vgpr(1), 0, 0, 0, 0, 0, false));
    let cvt_u16_abs = build_valu_cs(&vop3_mod(0x1D2, 0, vgpr(1), 0, 0, 1, 0, 0, false));
    let cvt_i16 = build_valu_cs(&vop3_mod(0x1D3, 0, vgpr(1), 0, 0, 0, 0, 0, false));
    let cvt_i16_neg = build_valu_cs(&vop3_mod(0x1D3, 0, vgpr(1), 0, 0, 0, 1, 0, false));

    let u16_cases: &[(u16, u16)] = &[
        (0xBC00, 0),      // -1.0 -> 0
        (0xFC00, 0),      // -inf -> 0
        (0x7E00, 0),      // NaN -> 0
        (0x57BC, 123),    // 123.75 -> 123
        (0x7BFF, 65504),  // 65504.0 (max finite f16) -> 65504
        (0x7C00, 0xFFFF), // +inf -> 65535 (saturated)
    ];
    let u16_in: Vec<[u32; 4]> = u16_cases
        .iter()
        .map(|&(x, _)| [u32::from(x), 0, 0, 0])
        .collect();
    run_cases(&cvt_u16, &u16_in, |backend, i, inp, got| {
        let want = u16_cases[i].1;
        assert_eq!(
            got as u16, want,
            "{backend}: v_cvt_u16_f16({:#06x}) = {:#06x}, want {want:#06x}",
            inp[0], got as u16,
        );
    });
    run_cases(
        &cvt_u16_abs,
        &[[0xD7BC, 0, 0, 0]], // |-123.75| -> 123
        |backend, _, _, got| {
            assert_eq!(got as u16, 123, "{backend}: v_cvt_u16_f16 |src0|");
        },
    );

    let i16_cases: &[(u16, u16)] = &[
        (0xFC00, 0x8000),           // -inf -> -32768
        (0xF8E2, 0x8000),           // -40000.0 -> -32768 (saturated, not wrapped)
        (0xD7BC, (-123i16) as u16), // -123.75 -> -123
        (0x7E00, 0),                // NaN -> 0
        (0x57BC, 123),              // 123.75 -> 123
        (0x78E2, 0x7FFF),           // +40000.0 -> 32767 (saturated, not wrapped)
        (0x7C00, 0x7FFF),           // +inf -> 32767
    ];
    let i16_in: Vec<[u32; 4]> = i16_cases
        .iter()
        .map(|&(x, _)| [u32::from(x), 0, 0, 0])
        .collect();
    run_cases(&cvt_i16, &i16_in, |backend, i, inp, got| {
        let want = i16_cases[i].1;
        assert_eq!(
            got as u16, want,
            "{backend}: v_cvt_i16_f16({:#06x}) = {:#06x}, want {want:#06x}",
            inp[0], got as u16,
        );
    });
    run_cases(
        &cvt_i16_neg,
        &[[0x57BC, 0, 0, 0], [0x78E2, 0, 0, 0]], // -(123.75) -> -123; -(40000.0) -> -32768
        |backend, i, _, got| {
            let want = [(-123i16) as u16, 0x8000][i];
            assert_eq!(got as u16, want, "{backend}: v_cvt_i16_f16 -src0 case {i}");
        },
    );
}

#[test]
fn integer_16bit_ops_read_inline_floats_as_f32_bits() {
    // v_add_nc_u16 v0, 1.0, v1. The inline 1.0 is 0x3F800000 there, whose
    // low half is 0, so 5 + 0 stays 5 (not 0x3C05 from the f16 pattern).
    let [w0, w1] = vop3(0x303, 0, 242, vgpr(1), 0);
    let shader = build_valu_cs(&[w0, w1]);
    run_cases(&shader, &[[5, 0, 0, 0]], |backend, _, _, got| {
        assert_eq!(got, 5, "{backend}: u16 add reads 1.0 as f32 bits");
    });
    // With op_sel the high half is 0x3F80, so 5 + 16256 stays 16261.
    let hi = [w0 | 1 << 11, w1];
    let shader = build_valu_cs(&hi);
    run_cases(&shader, &[[5, 0, 0, 0]], |backend, _, _, got| {
        assert_eq!(got, 16261, "{backend}: u16 add reads the high f32 half");
    });
}

#[test]
fn v_cmp_f_and_t_64bit_match_rdna2() {
    use crate::dxbc::{Container, FourCC};

    // v_cmp_{f,t}_{i64,u64}_e64 s6, v[1:2], v[3:4]; v_cndmask_b32_e64 v0, 0, 1, s6
    for (op, name, want) in [
        (0x0a0u16, "v_cmp_f_i64", 0u32),
        (0x0a7, "v_cmp_t_i64", 1),
        (0x0e0, "v_cmp_f_u64", 0),
        (0x0e7, "v_cmp_t_u64", 1),
    ] {
        let cmp = vop3(op, 6, vgpr(1), vgpr(3), 0);
        let sel = vop3(0x101, 0, 0x80, 0x81, sgpr(6));
        let shader = build_valu_cs(&[cmp[0], cmp[1], sel[0], sel[1]]);
        // Constant 64-bit compares must not require the Int64 feature bit (0x8000).
        let host = Container::parse(&shader.dxil).expect("Container::parse");
        let sfi0 = host.part(FourCC::SFI0).expect("SFI0").data;
        let features = u64::from_le_bytes(sfi0.try_into().unwrap());
        assert_eq!(
            features & 0x8000,
            0,
            "{name} should not set Int64 SFI0 flag: {features:#x}"
        );
        run_cases(
            &shader,
            &[[0, 0, 0, 0], [0xFFFF_FFFF, 0x7FFF_FFFF, 1, 0x8000_0000]],
            |backend, _, inp, got| {
                assert_eq!(
                    got, want,
                    "{backend}: {name}({inp:08x?}) = {got}, want {want}"
                );
            },
        );
    }
}

#[test]
fn ps_sample_coverage_input_reads_coverage_mask() {
    use super::bitcode::{Attr, Term};
    use super::container::{self, SigElement};
    use super::host::Draw;
    use super::shader::Shader;
    use crate::dxbc::{Container, FourCC};

    // SPI_PS_INPUT_ADDR bit 14 puts the sample coverage mask in v0.
    // Convert u32 -> f32 into v1 and export mrt0 = (v1, v1, v1, v1).
    let words = [
        vop1(0x006, 1, vgpr(0)), // v_cvt_f32_u32 v1, v0
        0xF800_180F,
        0x0101_0101, // exp mrt0 v1, v1, v1, v1 done vm
        S_ENDPGM,
    ];
    let native = NativeProgram {
        stage: HwStage::Ps,
        wave64: false,
        vgprs: 2,
        layout: Vec::new(),
        used: Vec::new(),
        io: StageIo::Ps {
            inputs: Vec::new(),
            input_addr: 1 << 14,
            db_shader_control: 0,
            patches: Vec::new(),
        },
        code: words.iter().flat_map(|w| w.to_le_bytes()).collect(),
    };
    // Include SV_Coverage (kind 14) in console ISG1 to verify emit() strips it
    // (NotInSig in DXIL) while keeping SV_Target in OSG1.
    let isg1 = container::signature(
        &[SigElement {
            name: "SV_Coverage".into(),
            index: 0,
            kind: 14,
            comp: 5,
            interp: 0,
            row: None,
            col: 0,
            cols: 1,
            usage: 1,
            half: false,
        }],
        true,
    );
    let osg1 = container::signature(
        &[SigElement {
            name: "SV_Target".into(),
            index: 0,
            kind: 16,
            comp: 9,
            interp: 0,
            row: Some(0),
            col: 0,
            cols: 4,
            usage: 0xF,
            half: false,
        }],
        false,
    );
    let bytes = crate::dxbc::build(&[(FourCC::ISG1, &isg1), (FourCC::OSG1, &osg1)]);
    let c = Container::parse(&bytes).expect("Container::parse");
    let p = ir::build(&native.code, native.wave64, native.vgprs).expect("ir::build");
    let st = structure::structure(&p).expect("structure");
    let res =
        resource::track(&p, &native.layout, native.stage, Some(&st)).expect("resource::track");
    let io = io::raise(&p, &native).expect("io::raise");
    let ps = super::emit(&c, &native, &p, &st, &io, &res).expect("emit");

    if let Ok(mut v) = validator::Validator::load(validator::Validator::default_path().as_deref()) {
        let mut copy = ps.dxil.clone();
        let ours = copy[4..20].to_vec();
        v.validate(&mut copy).expect("DXIL validation failed");
        assert_eq!(copy[4..20], ours[..], "container hash mismatch");
    }

    // Minimal pass-through VS: POSITION -> SV_Position.
    let mut s = Shader::new(false);
    let b = s.entry;
    let undef = s.m.undef(s.t.i32);
    for col in 0..4u8 {
        let args = [s.m.i32(0), s.m.i32(0), s.m.i8(col), undef];
        let v = s.op(b, "loadInput.f32", Attr::ReadNone, s.t.f32, 4, &args);
        let out_args = [s.m.i32(0), s.m.i32(0), s.m.i8(col), v];
        s.op(b, "storeOutput.f32", Attr::NoUnwind, s.t.void, 5, &out_args);
    }
    s.inputs.push(SigElement {
        name: "POSITION".into(),
        index: 0,
        kind: 0,
        comp: 9,
        interp: 0,
        row: Some(0),
        col: 0,
        cols: 4,
        usage: 0xF,
        half: false,
    });
    s.outputs.push(SigElement {
        name: "SV_Position".into(),
        index: 0,
        kind: 3,
        comp: 9,
        interp: 4,
        row: Some(0),
        col: 0,
        cols: 4,
        usage: 0xF,
        half: false,
    });
    s.m.terminate(b, Term::Ret);
    let vs = HostShader {
        stage: HwStage::Vs,
        dxil: s.finish(),
        user_data: Vec::new(),
        samplers: Vec::new(),
        static_samplers: Vec::new(),
        root_cbvs: Vec::new(),
        root_srvs: Vec::new(),
    };

    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let mut ran = 0;
    for (label, warp) in [("hardware", false), ("warp", true)] {
        let Ok(host) = Host::new(warp) else {
            eprintln!("no SM 6.6 {label} device; skipped");
            continue;
        };
        let (root, pso) = host
            .pipeline(&vs, &ps)
            .unwrap_or_else(|e| panic!("{label}: graphics pipeline failed: {e}"));
        let pixels = host
            .draw(
                &root,
                &pso,
                &Draw {
                    roots: vec![(0, Root::Constants(vec![0])), (1, Root::Constants(vec![0]))],
                    textures: &[],
                    samplers: &[],
                    vertices: &[
                        -1.0, -1.0, 0.0, 1.0, 3.0, -1.0, 0.0, 1.0, -1.0, 3.0, 0.0, 1.0,
                    ],
                    stride: 16,
                    size: (2, 2),
                    mesh_groups: 0,
                },
            )
            .unwrap_or_else(|e| panic!("{label}: draw failed: {e}"));
        // 1x sample target: every covered pixel reports coverage mask = 1 (bit 0).
        assert_eq!(pixels, vec![[1.0; 4]; 4], "{label}");
        ran += 1;
    }
    assert!(ran > 0, "neither hardware nor WARP SM 6.6 device available");
}

/// SMEM `s_load_dwordx4` or `s_load_dwordx8` with a constant byte offset.
const fn s_load_dwords(op: u32, dst: u8, base: u8, byte_off: u32) -> [u32; 2] {
    [
        0x3D << 26 | (op & 0xFF) << 18 | (dst as u32) << 6 | (base as u32 >> 1),
        125 << 25 | (byte_off & 0x1F_FFFF),
    ]
}

/// MIMG sequential-address instruction (`dim`: 1 = 2D).
const fn mimg_sample(
    op: u16,
    dmask: u8,
    dim: u8,
    vdata: u8,
    vaddr: u8,
    srsrc: u8,
    ssamp: u8,
) -> [u32; 2] {
    [
        0x3C << 26
            | ((op as u32 & 0x7F) << 18)
            | ((dmask as u32 & 0xF) << 8)
            | ((dim as u32 & 7) << 3)
            | ((op as u32 >> 7) & 1),
        ((ssamp as u32 >> 2) & 0x1F) << 21
            | ((srsrc as u32 >> 2) & 0x1F) << 16
            | (vdata as u32) << 8
            | vaddr as u32,
    ]
}

/// Builds a 1-thread compute shader with 4 root constants (`s0..s3` copied
/// into `v1..v4`), Texture2D SRV at `s[16:23]` (view table index 0), Raw UAV
/// at `s[24:27]` (view table index 1), and sampler at `s[12:15]` (sampler
/// table index 0). `body` writes 4 result dwords into `v8..v11`.
fn build_sample_cs(body: &[u32]) -> HostShader {
    let mut words = Vec::new();
    // s[4:5] = view table, s[6:7] = sampler table.
    words.push(s_mov_b32(5, 0x80));
    words.push(s_mov_b32(7, 0x80));
    words.extend_from_slice(&s_load_dwords(0x03, 16, 4, 0)); // Texture2D SRV
    words.extend_from_slice(&s_load_dwords(0x02, 24, 4, 32)); // Raw UAV
    words.extend_from_slice(&s_load_dwords(0x02, 12, 6, 0)); // Sampler
    words.push(vop1(0x001, 0, 0x80)); // v0 = 0
    for i in 0..4u8 {
        words.push(vop1(0x001, i + 1, sgpr(i)));
    }
    words.extend_from_slice(body);
    for k in 0..4u8 {
        words.extend_from_slice(&buffer_store_dword(8 + k, 24, u16::from(k) * 4));
    }
    words.push(S_ENDPGM);

    let native = NativeProgram {
        stage: HwStage::Cs,
        wave64: false,
        vgprs: 16,
        layout: vec![
            RootParam {
                root: RootType::Constants,
                table_offset_dwords: 0,
                sgprs: 4,
                base_register: 0,
                user_data: 0,
                slot: 0,
            },
            RootParam {
                root: RootType::Table,
                table_offset_dwords: 0,
                sgprs: 1,
                base_register: 0,
                user_data: 4,
                slot: 1,
            },
            RootParam {
                root: RootType::Table,
                table_offset_dwords: 0,
                sgprs: 1,
                base_register: 0,
                user_data: 6,
                slot: 2,
            },
        ],
        used: vec![
            UsedResource {
                kind: ResourceType::Srv,
                root: RootType::Table,
                table_offset_dwords: 0,
                lower_bound: 0,
                stride: 0,
                dwords: 8,
                user_data: 4,
                slot: 1,
            },
            UsedResource {
                kind: ResourceType::Uav,
                root: RootType::Table,
                table_offset_dwords: 8,
                lower_bound: 0,
                stride: 0,
                dwords: 4,
                user_data: 4,
                slot: 1,
            },
            UsedResource {
                kind: ResourceType::Sampler,
                root: RootType::Table,
                table_offset_dwords: 0,
                lower_bound: 0,
                stride: 0,
                dwords: 4,
                user_data: 6,
                slot: 2,
            },
        ],
        io: StageIo::Cs(ComputeIo {
            rsrc2: 7 << 1,
            threads: [1, 1, 1],
        }),
        code: words.iter().flat_map(|w| w.to_le_bytes()).collect(),
    };

    let p = ir::build(&native.code, native.wave64, native.vgprs).expect("ir::build");
    let st = structure::structure(&p).expect("structure");
    let res =
        resource::track(&p, &native.layout, native.stage, Some(&st)).expect("resource::track");
    let io = io::raise(&p, &native).expect("io::raise");
    let lowered = lower::lower(&lower::Parts {
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
    .expect("lower");

    let shader = HostShader {
        stage: HwStage::Cs,
        dxil: lowered.dxil,
        user_data: lowered.user_data,
        samplers: lowered.samplers,
        static_samplers: Vec::new(),
        root_cbvs: lowered.root_cbvs,
        root_srvs: lowered.root_srvs,
    };

    if let Ok(mut v) = validator::Validator::load(validator::Validator::default_path().as_deref()) {
        let mut copy = shader.dxil.clone();
        let ours = copy[4..20].to_vec();
        v.validate(&mut copy).expect("DXIL validation failed");
        assert_eq!(copy[4..20], ours[..], "container hash mismatch");
    }

    shader
}

fn run_sample_cases(
    shader: &HostShader,
    texels: &[[f32; 4]],
    sampler: windows::Win32::Graphics::Direct3D12::D3D12_SAMPLER_DESC,
    cases: &[([u32; 4], [f32; 4])],
) {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let mut ran = 0;
    for (label, warp) in [("hardware", false), ("warp", true)] {
        let Ok(host) = Host::new(warp) else {
            eprintln!("no SM 6.6 {label} device; skipped");
            continue;
        };
        let (root, pso) = host
            .compute_pipeline(shader)
            .unwrap_or_else(|e| panic!("{label}: compute_pipeline failed: {e}"));
        for (i, &(inputs, want)) in cases.iter().enumerate() {
            let mut ud = vec![0u32; shader.user_data_dwords().max(1) as usize];
            for u in &shader.user_data {
                ud[u.index as usize] = match (u.root, u.slot) {
                    (RootType::Constants, 0) => inputs[u.dword as usize],
                    (RootType::Table, 1 | 2) => 0,
                    other => panic!("unexpected user data {other:?}"),
                };
            }
            let out = host
                .dispatch(
                    &root,
                    &pso,
                    &Dispatch {
                        roots: vec![(0, Root::Constants(ud))],
                        views: &[View::Texture(4, 4, texels), View::RawUav(4)],
                        samplers: &[sampler],
                        groups: [1, 1, 1],
                    },
                )
                .unwrap_or_else(|e| panic!("{label}: dispatch failed: {e}"));
            let got = [
                f32::from_bits(out[0][0]),
                f32::from_bits(out[0][1]),
                f32::from_bits(out[0][2]),
                f32::from_bits(out[0][3]),
            ];
            assert_eq!(got, want, "{label}: case {i} got {got:?}, want {want:?}");
        }
        ran += 1;
    }
    if ran == 0 {
        eprintln!("neither hardware nor WARP SM 6.6 device available; skipped");
    }
}

/// Builds a compute pipeline for `shader` on the first device available
/// (WARP first, then the default adapter): opcode and operand legality is
/// checked by the DXIL validator in `build_sample_cs`, and the driver's
/// compiler is the second gate. Without a device there is nothing to check.
fn pipeline_builds(shader: &HostShader) {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let Some(host) = [true, false].iter().find_map(|&warp| Host::new(warp).ok()) else {
        eprintln!("no SM 6.6 device; skipped the pipeline");
        return;
    };
    host.compute_pipeline(shader)
        .unwrap_or_else(|e| panic!("compute_pipeline failed: {e}"));
}

#[test]
fn image_gather4_c_lz_and_offset_match_d3d12() {
    // image_gather4_c_lz v[8:11], v[1:3], s[16:23], s[12:15] dmask:0x1 dim:SQ_RSRC_IMG_2D
    // Address registers: v1 = reference, v2 = u, v3 = v.
    // The comparison's values are not dispatched: the only view the host binds
    // is RGBA32F, which D3D12 does not compare on (AMD answers 0 and can
    // remove the device), so `build_sample_cs` checks the DXIL's opcode and
    // operands instead, and the driver checks them at pipeline creation.
    let gather_r = build_sample_cs(&mimg_sample(0x04f, 0x1, 1, 8, 1, 16, 12));
    pipeline_builds(&gather_r);

    // Non-red channel gather-cmp (dmask:0x2 -> channel 1).
    let gather_g = build_sample_cs(&mimg_sample(0x04f, 0x2, 1, 8, 1, 16, 12));
    pipeline_builds(&gather_g);

    // image_gather4_c_lz_o v[8:11], v[1:4], s[16:23], s[12:15] dmask:0x1 dim:SQ_RSRC_IMG_2D
    // Address registers: v1 = dynamic offset (6-bit signed x | y<<8), v2 = reference, v3 = u, v4 = v.
    let gather_o = build_sample_cs(&mimg_sample(0x05f, 0x1, 1, 8, 1, 16, 12));
    pipeline_builds(&gather_o);
}

#[test]
fn image_sample_offset_variants_match_d3d12() {
    use super::container::SigElement;
    use super::host::stub_vs;
    use windows::Win32::Graphics::Direct3D12::*;

    let mut texels = [[0.0f32; 4]; 16];
    for y in 0..4usize {
        for x in 0..4usize {
            let v = (y * 4 + x) as f32;
            texels[y * 4 + x] = [v, 100.0 + v, 200.0 + v, 300.0 + v];
        }
    }
    let point_sampler = D3D12_SAMPLER_DESC {
        Filter: D3D12_FILTER_MIN_MAG_MIP_POINT,
        AddressU: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
        AddressV: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
        AddressW: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
        MaxAnisotropy: 1,
        MaxLOD: f32::MAX,
        ..Default::default()
    };
    let f = f32::to_bits;

    // image_sample_lz_o (0x037): v5 = imm offset (+2, +1), v6 = u, v7 = v; dmask = 0x3 (R,G).
    let sample_lz_o = build_sample_cs(&[
        vop1(0x001, 5, 0xFF),
        0x0000_0102,
        vop1(0x001, 6, vgpr(1)),
        vop1(0x001, 7, vgpr(2)),
        vop1(0x001, 10, 0x80),
        vop1(0x001, 11, 0x80),
        mimg_sample(0x037, 0x3, 1, 8, 5, 16, 12)[0],
        mimg_sample(0x037, 0x3, 1, 8, 5, 16, 12)[1],
    ]);
    // (0.125, 0.125) is texel (0,0); +(+2,+1) -> texel (2,1) = [6.0, 106.0].
    run_sample_cases(
        &sample_lz_o,
        &texels,
        point_sampler,
        &[([f(0.125), f(0.125), 0, 0], [6.0, 106.0, 0.0, 0.0])],
    );

    // image_sample_l_o (0x034): v5 = imm offset (-1, +2), v6 = u, v7 = v, v8 = lod; dmask = 0x3.
    let sample_l_o = build_sample_cs(&[
        vop1(0x001, 5, 0xFF),
        0x0000_023F,
        vop1(0x001, 6, vgpr(1)),
        vop1(0x001, 7, vgpr(2)),
        vop1(0x001, 8, vgpr(3)),
        vop1(0x001, 10, 0x80),
        vop1(0x001, 11, 0x80),
        mimg_sample(0x034, 0x3, 1, 8, 5, 16, 12)[0],
        mimg_sample(0x034, 0x3, 1, 8, 5, 16, 12)[1],
    ]);
    // (0.375, 0.125) is texel (1,0); +(-1,+2) -> texel (0,2) = [8.0, 108.0].
    run_sample_cases(
        &sample_l_o,
        &texels,
        point_sampler,
        &[([f(0.375), f(0.125), f(0.0), 0], [8.0, 108.0, 0.0, 0.0])],
    );

    // image_sample_c_lz_o (0x03f): v5 = imm offset (+1, +2), v6 = ref, v7 = u, v8 = v; dmask = 0x1.
    let sample_c_lz_o = build_sample_cs(&[
        vop1(0x001, 5, 0xFF),
        0x0000_0201,
        vop1(0x001, 6, vgpr(1)),
        vop1(0x001, 7, vgpr(2)),
        vop1(0x001, 8, vgpr(3)),
        vop1(0x001, 9, 0x80),
        vop1(0x001, 10, 0x80),
        vop1(0x001, 11, 0x80),
        mimg_sample(0x03f, 0x1, 1, 8, 5, 16, 12)[0],
        mimg_sample(0x03f, 0x1, 1, 8, 5, 16, 12)[1],
    ]);
    // (0.125, 0.125) + (+1,+2) -> texel (1,2) R = 9.0. The comparison's value
    // is not dispatched: an RGBA32F view is not comparison-capable (AMD
    // answers 0 and can remove the device), so its DXIL is validated instead.
    pipeline_builds(&sample_c_lz_o);

    // image_sample_d_o (0x032): v5 = imm offset (+1, +1), v6..v9 = zero grads, v10..v11 = u,v.
    let sample_d_o = build_sample_cs(&[
        vop1(0x001, 5, 0xFF),
        0x0000_0101,
        vop1(0x001, 6, 0x80),
        vop1(0x001, 7, 0x80),
        vop1(0x001, 8, 0x80),
        vop1(0x001, 9, 0x80),
        vop1(0x001, 10, vgpr(1)),
        vop1(0x001, 11, vgpr(2)),
        mimg_sample(0x032, 0x1, 1, 12, 5, 16, 12)[0],
        mimg_sample(0x032, 0x1, 1, 12, 5, 16, 12)[1],
        vop1(0x001, 8, vgpr(12)),
        vop1(0x001, 9, 0x80),
        vop1(0x001, 10, 0x80),
        vop1(0x001, 11, 0x80),
    ]);
    // (0.125, 0.125) + (+1,+1) -> texel (1,1) R = 5.0.
    run_sample_cases(
        &sample_d_o,
        &texels,
        point_sampler,
        &[([f(0.125), f(0.125), 0, 0], [5.0, 0.0, 0.0, 0.0])],
    );

    // Pixel-only offset variants: image_sample_b_o (0x035) and image_sample_c_o (0x038).
    // Validate DXIL and build a live graphics PSO on WARP / hardware.
    for op in [0x035u16, 0x038] {
        let mut words = Vec::new();
        words.push(s_mov_b32(1, 0x80)); // s[0:1] = view table
        words.push(s_mov_b32(3, 0x80)); // s[2:3] = sampler table
        words.extend_from_slice(&s_load_dwords(0x03, 16, 0, 0));
        words.extend_from_slice(&s_load_dwords(0x02, 12, 2, 0));
        words.extend_from_slice(&[vop1(0x001, 1, 0xFF), 0x0000_0101]); // offset (+1,+1)
        words.push(vop1(0x001, 2, 0xF0)); // bias or ref = 0.5
        words.push(vop1(0x001, 3, 0xF0)); // u = 0.5
        words.push(vop1(0x001, 4, 0xF0)); // v = 0.5
        words.extend_from_slice(&mimg_sample(op, 0x1, 1, 0, 1, 16, 12));
        words.extend_from_slice(&[0xF800_180F, 0x0000_0000]); // exp mrt0 v0, v0, v0, v0 done vm
        words.push(S_ENDPGM);

        let native = NativeProgram {
            stage: HwStage::Ps,
            wave64: false,
            vgprs: 8,
            layout: vec![
                RootParam {
                    root: RootType::Table,
                    table_offset_dwords: 0,
                    sgprs: 1,
                    base_register: 0,
                    user_data: 0,
                    slot: 0,
                },
                RootParam {
                    root: RootType::Table,
                    table_offset_dwords: 0,
                    sgprs: 1,
                    base_register: 0,
                    user_data: 2,
                    slot: 1,
                },
            ],
            used: vec![
                UsedResource {
                    kind: ResourceType::Srv,
                    root: RootType::Table,
                    table_offset_dwords: 0,
                    lower_bound: 0,
                    stride: 0,
                    dwords: 8,
                    user_data: 0,
                    slot: 0,
                },
                UsedResource {
                    kind: ResourceType::Sampler,
                    root: RootType::Table,
                    table_offset_dwords: 0,
                    lower_bound: 0,
                    stride: 0,
                    dwords: 4,
                    user_data: 2,
                    slot: 1,
                },
            ],
            io: StageIo::Ps {
                inputs: Vec::new(),
                input_addr: 0,
                db_shader_control: 0,
                patches: Vec::new(),
            },
            code: words.iter().flat_map(|w| w.to_le_bytes()).collect(),
        };
        let p = ir::build(&native.code, native.wave64, native.vgprs).expect("ir::build");
        let st = structure::structure(&p).expect("structure");
        let res =
            resource::track(&p, &native.layout, native.stage, Some(&st)).expect("resource::track");
        let io = io::raise(&p, &native).expect("io::raise");
        let lowered = lower::lower(&lower::Parts {
            native: &native,
            p: &p,
            st: &st,
            io: &io,
            res: &res,
            inputs: Vec::new(),
            outputs: vec![SigElement {
                name: "SV_Target".into(),
                index: 0,
                kind: 16,
                comp: 9,
                interp: 0,
                row: Some(0),
                col: 0,
                cols: 4,
                usage: 0xF,
                half: false,
            }],
            prims: Vec::new(),
            mesh_stride: 0,
        })
        .expect("lower");

        let ps = HostShader {
            stage: HwStage::Ps,
            dxil: lowered.dxil,
            user_data: lowered.user_data,
            samplers: lowered.samplers,
            static_samplers: Vec::new(),
            root_cbvs: lowered.root_cbvs,
            root_srvs: lowered.root_srvs,
        };
        if let Ok(mut v) =
            validator::Validator::load(validator::Validator::default_path().as_deref())
        {
            v.validate(&mut ps.dxil.clone())
                .unwrap_or_else(|e| panic!("DXIL validation failed for op {op:#x}: {e}"));
        }
        let vs = stub_vs(&ps).expect("stub_vs");
        let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
        let Some(host) = [true, false].iter().find_map(|&warp| Host::new(warp).ok()) else {
            eprintln!("no SM 6.6 device; skipped the {op:#x} PSO");
            continue;
        };
        host.pipeline(&vs, &ps)
            .unwrap_or_else(|e| panic!("graphics PSO failed for op {op:#x}: {e}"));
    }
}

/// MTBUF `tbuffer_load_format_{x,xy}` with `offen = 1`, byte offset in `vaddr`,
/// destination in `vdata`, descriptor in `s[srsrc:srsrc+3]`, `soffset = 0`.
const fn tbuffer_load_format_offen(
    op: u8,
    format: u8,
    vdata: u8,
    vaddr: u8,
    srsrc: u8,
) -> [u32; 2] {
    [
        0xE800_0000 | (format as u32 & 0x7F) << 19 | (op as u32 & 0x7) << 16 | 1 << 12,
        0x80 << 24
            | ((op as u32 >> 3) & 1) << 21
            | ((srsrc as u32 >> 2) & 0x1F) << 16
            | (vdata as u32) << 8
            | vaddr as u32,
    ]
}

fn run_cases_with_uav_dwords(
    shader: &HostShader,
    uav_dwords: u32,
    cases: &[[u32; 4]],
    mut check: impl FnMut(&str, usize, [u32; 4], u32),
) {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let mut ran = 0;
    for (label, warp) in [("hardware", false), ("warp", true)] {
        let Ok(host) = Host::new(warp) else {
            eprintln!("no SM 6.6 {label} device; skipped");
            continue;
        };
        let (root, pso) = host
            .compute_pipeline(shader)
            .unwrap_or_else(|e| panic!("{label}: compute_pipeline failed: {e}"));
        for (i, &inputs) in cases.iter().enumerate() {
            let mut ud = vec![0u32; shader.user_data_dwords().max(1) as usize];
            for u in &shader.user_data {
                ud[u.index as usize] = match (u.root, u.slot) {
                    (RootType::Constants, 0) => inputs[u.dword as usize],
                    (RootType::Table, 1) => 0,
                    other => panic!("unexpected user data {other:?}"),
                };
            }
            let out = host
                .dispatch(
                    &root,
                    &pso,
                    &Dispatch {
                        roots: vec![(0, Root::Constants(ud))],
                        views: &[View::RawUav(uav_dwords)],
                        samplers: &[],
                        groups: [1, 1, 1],
                    },
                )
                .unwrap_or_else(|e| panic!("{label}: dispatch failed: {e}"));
            check(label, i, inputs, out[0][0]);
        }
        ran += 1;
    }
    assert!(ran > 0, "neither hardware nor WARP SM 6.6 device available");
}

#[test]
fn tbuffer_16bit_format_loads_handle_dword_straddling_and_tail_bounds() {
    // 16_UINT (format 11): write two dwords into an 8-byte raw UAV, then read
    // a 16-bit element at every valid byte offset 0..=6 (offset 3 straddles
    // dword 0 -> dword 1; offset 6 sits at buffer_end - 2).
    let mut body_u16 = Vec::new();
    body_u16.extend_from_slice(&buffer_store_dword(1, 8, 0));
    body_u16.extend_from_slice(&buffer_store_dword(2, 8, 4));
    body_u16.extend_from_slice(&tbuffer_load_format_offen(0, 11, 0, 3, 8));
    let u16_shader = build_valu_cs(&body_u16);

    let d0 = 0x3422_1100u32; // bytes: 00, 11, 22, 34
    let d1 = 0x7766_5512u32; // bytes: 12, 55, 66, 77
    let u16_cases: &[([u32; 4], u32)] = &[
        ([d0, d1, 0, 0], 0x1100),
        ([d0, d1, 1, 0], 0x2211),
        ([d0, d1, 2, 0], 0x3422),
        ([d0, d1, 3, 0], 0x1234), // dword-straddling: byte 3 of d0 + byte 0 of d1
        ([d0, d1, 4, 0], 0x5512),
        ([d0, d1, 5, 0], 0x6655),
        ([d0, d1, 6, 0], 0x7766), // buffer tail (buffer_end - 2)
    ];
    let u16_in: Vec<[u32; 4]> = u16_cases.iter().map(|c| c.0).collect();
    run_cases_with_uav_dwords(&u16_shader, 2, &u16_in, |backend, i, inp, got| {
        let want = u16_cases[i].1;
        assert_eq!(
            got, want,
            "{backend}: 16_UINT off={} = {got:#06x}, want {want:#06x}",
            inp[2],
        );
    });

    // Exact 4-byte buffer tail: off = 2 (buffer_end - 2) must not touch dword 1.
    let mut body_tail = Vec::new();
    body_tail.extend_from_slice(&buffer_store_dword(1, 8, 0));
    body_tail.extend_from_slice(&tbuffer_load_format_offen(0, 11, 0, 3, 8));
    let tail_shader = build_valu_cs(&body_tail);
    run_cases(
        &tail_shader,
        &[[0xabcd_ef01, 0, 2, 0]],
        |backend, _, _, got| {
            assert_eq!(got, 0xabcd, "{backend}: 16_UINT 4-byte buffer tail off=2");
        },
    );

    // 16_SINT (format 12) at straddling offset 3: [0xFE, 0xFF] -> -2 (0xFFFF_FFFE).
    let mut body_i16 = Vec::new();
    body_i16.extend_from_slice(&buffer_store_dword(1, 8, 0));
    body_i16.extend_from_slice(&buffer_store_dword(2, 8, 4));
    body_i16.extend_from_slice(&tbuffer_load_format_offen(0, 12, 0, 3, 8));
    let i16_shader = build_valu_cs(&body_i16);
    run_cases_with_uav_dwords(
        &i16_shader,
        2,
        &[[0xfe00_0000, 0x0000_00ff, 3, 0]],
        |backend, _, _, got| {
            assert_eq!(got, 0xffff_fffe, "{backend}: 16_SINT straddle off=3");
        },
    );

    // 16_FLOAT (format 13) at straddling offset 3: [0x00, 0x3C] = +1.0f16 -> +1.0f32.
    let mut body_f16 = Vec::new();
    body_f16.extend_from_slice(&buffer_store_dword(1, 8, 0));
    body_f16.extend_from_slice(&buffer_store_dword(2, 8, 4));
    body_f16.extend_from_slice(&tbuffer_load_format_offen(0, 13, 0, 3, 8));
    let f16_shader = build_valu_cs(&body_f16);
    run_cases_with_uav_dwords(
        &f16_shader,
        2,
        &[[0x00aa_bbcc, 0x1122_333c, 3, 0]],
        |backend, _, _, got| {
            assert_eq!(
                got,
                1.0f32.to_bits(),
                "{backend}: 16_FLOAT straddle off=3 = {got:#010x}",
            );
        },
    );

    // 8_8_UINT (format 18, tbuffer_load_format_xy op=1) at straddling offset 3:
    // loads v5 = byte 3 of d0 (0x34), v6 = byte 0 of d1 (0x12); pack (v6 << 16) | v5.
    let mut body_8_8 = Vec::new();
    body_8_8.extend_from_slice(&buffer_store_dword(1, 8, 0));
    body_8_8.extend_from_slice(&buffer_store_dword(2, 8, 4));
    body_8_8.extend_from_slice(&tbuffer_load_format_offen(1, 18, 5, 3, 8));
    // v_lshl_or_b32 v0, v6, 16, v5 (VOP3 0x36f)
    body_8_8.extend_from_slice(&vop3(0x36f, 0, vgpr(6), 0x80 + 16, vgpr(5)));
    let u8_8_shader = build_valu_cs(&body_8_8);
    run_cases_with_uav_dwords(
        &u8_8_shader,
        2,
        &[[d0, d1, 3, 0], [d0, d1, 2, 0]],
        |backend, i, inp, got| {
            let want = if i == 0 { 0x0012_0034 } else { 0x0034_0022 };
            assert_eq!(
                got, want,
                "{backend}: 8_8_UINT off={} = {got:#010x}, want {want:#010x}",
                inp[2],
            );
        },
    );
}
