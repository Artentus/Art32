use super::MAX_SIM_STEPS;
use crate::module;
use gsim::{LogicBitState, LogicState};
use proptest::prelude::ProptestConfig;
use std::num::NonZeroU8;
use test_strategy::*;

module! {
    FPU: Fpu = "fpu" {
        in lhs,
        in rhs,
        out result,

        in op,
        in start,
        out ready,

        in enable,
        in reset,
        in clk,
    }
}

#[derive(Debug, Clone, Copy, Arbitrary)]
#[repr(u8)]
enum Op {
    Add = 0x00,
    Sub = 0x01,
    Mul = 0x02,
    Div = 0x03,
    Rem = 0x04,
    Min = 0x06,
    Max = 0x07,
    Floor = 0x08,
    Ceil = 0x09,
    Round = 0x0A,
    Trunc = 0x0B,
    Abs = 0x0C,
    Neg = 0x0D,
    Sqrt = 0x0E,
    Rsqrt = 0x0F,
    CmpEq = 0x10,
    CmpNe = 0x11,
    CmpLt = 0x12,
    CmpGe = 0x13,
    FtoI = 0x18,
    ItoF = 0x19,
}

fn equals_allow_imprecision(lhs: f32, rhs: f32, imprecision_range: u32) -> bool {
    use std::num::FpCategory;

    let mut lhs_class = lhs.classify();
    let mut rhs_class = rhs.classify();

    if lhs_class == FpCategory::Subnormal {
        lhs_class = FpCategory::Zero;
    }

    if rhs_class == FpCategory::Subnormal {
        rhs_class = FpCategory::Zero;
    }

    match (lhs_class, rhs_class) {
        (FpCategory::Nan, FpCategory::Nan) => true,
        (FpCategory::Zero, FpCategory::Zero) => true,
        (FpCategory::Infinite, FpCategory::Infinite) => lhs.signum() == rhs.signum(),
        (FpCategory::Subnormal, _) | (_, FpCategory::Subnormal) => {
            unreachable!("subnormals not supported")
        }
        (FpCategory::Normal, FpCategory::Normal) => {
            let lhs = lhs.to_bits();
            let rhs = rhs.to_bits();
            lhs.abs_diff(rhs) <= imprecision_range
        }
        _ => false,
    }
}

#[inline]
fn equals(lhs: f32, rhs: f32) -> bool {
    equals_allow_imprecision(lhs, rhs, 0)
}

#[inline]
fn equals_ignore_rounding(lhs: f32, rhs: f32) -> bool {
    equals_allow_imprecision(lhs, rhs, 1)
}

fn print_float(value: f32) -> String {
    let raw = value.to_bits();

    let s = raw >> 31;
    let e = (raw >> 23) as u8;
    let m = raw & 0x7FFFFF;

    format!("s: {s}, e: {e}, m: 1.{m:0>23b}")
}

#[inline]
fn subnormal_to_zero(value: f32) -> f32 {
    if value.is_subnormal() {
        f32::copysign(0.0, value)
    } else {
        value
    }
}

#[inline]
fn golden_add(lhs: f32, rhs: f32) -> f32 {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    let result = lhs + rhs;
    subnormal_to_zero(result)
}

#[inline]
fn golden_sub(lhs: f32, rhs: f32) -> f32 {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    let result = lhs - rhs;
    subnormal_to_zero(result)
}

#[inline]
fn golden_mul(lhs: f32, rhs: f32) -> f32 {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    let result = lhs * rhs;
    subnormal_to_zero(result)
}

#[inline]
fn golden_div(lhs: f32, rhs: f32) -> f32 {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    let result = lhs / rhs;
    subnormal_to_zero(result)
}

#[inline]
fn golden_min(lhs: f32, rhs: f32) -> f32 {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    let result = lhs.min(rhs);
    subnormal_to_zero(result)
}

#[inline]
fn golden_max(lhs: f32, rhs: f32) -> f32 {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    let result = lhs.max(rhs);
    subnormal_to_zero(result)
}

#[inline]
fn golden_floor(value: f32) -> f32 {
    let value = subnormal_to_zero(value);
    let result = value.floor();
    subnormal_to_zero(result)
}

#[inline]
fn golden_ceil(value: f32) -> f32 {
    let value = subnormal_to_zero(value);
    let result = value.ceil();
    subnormal_to_zero(result)
}

#[inline]
fn golden_round(value: f32) -> f32 {
    let value = subnormal_to_zero(value);
    let result = value.round();
    subnormal_to_zero(result)
}

#[inline]
fn golden_trunc(value: f32) -> f32 {
    let value = subnormal_to_zero(value);
    let result = value.trunc();
    subnormal_to_zero(result)
}

#[inline]
fn golden_abs(value: f32) -> f32 {
    let value = subnormal_to_zero(value);
    let result = value.abs();
    subnormal_to_zero(result)
}

#[inline]
fn golden_neg(value: f32) -> f32 {
    let value = subnormal_to_zero(value);
    let result = -value;
    subnormal_to_zero(result)
}

#[inline]
fn golden_sqrt(value: f32) -> f32 {
    let value = subnormal_to_zero(value);
    let result = value.sqrt();
    subnormal_to_zero(result)
}

#[inline]
fn golden_rsqrt(value: f32) -> f32 {
    let value = subnormal_to_zero(value);
    let result = value.sqrt().recip();
    subnormal_to_zero(result)
}

#[inline]
fn golden_cmpeq(lhs: f32, rhs: f32) -> bool {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    lhs == rhs
}

#[inline]
fn golden_cmpne(lhs: f32, rhs: f32) -> bool {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    lhs != rhs
}

#[inline]
fn golden_cmplt(lhs: f32, rhs: f32) -> bool {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    lhs < rhs
}

#[inline]
fn golden_cmpge(lhs: f32, rhs: f32) -> bool {
    let lhs = subnormal_to_zero(lhs);
    let rhs = subnormal_to_zero(rhs);
    lhs >= rhs
}

#[inline]
fn golden_ftoi(value: f32) -> i32 {
    value as i32
}

#[inline]
fn golden_itof(value: i32) -> f32 {
    value as f32
}

fn test_impl(lhs: f32, rhs: f32, op: Op, max_cycle_count: u32) -> LogicState {
    FPU.with(|fpu| {
        let lhs = LogicState::from_int(lhs.to_bits());
        let rhs = LogicState::from_int(rhs.to_bits());
        let op = LogicState::from_int(op as u32);

        let mut sim = fpu.sim.borrow_mut();
        sim.reset();

        sim.set_wire_drive(fpu.lhs, &lhs).unwrap();
        sim.set_wire_drive(fpu.rhs, &rhs).unwrap();
        sim.set_wire_drive(fpu.op, &op).unwrap();
        sim.set_wire_drive(fpu.enable, &LogicState::LOGIC_1)
            .unwrap();
        sim.set_wire_drive(fpu.reset, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(fpu.start, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(fpu.clk, &LogicState::LOGIC_0).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        sim.set_wire_drive(fpu.reset, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(fpu.clk, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(fpu.clk, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(fpu.reset, &LogicState::LOGIC_0).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        sim.set_wire_drive(fpu.start, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(fpu.clk, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(fpu.clk, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(fpu.start, &LogicState::LOGIC_0).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        let mut cycle_count = 1;
        loop {
            let ready = sim.get_wire_state(fpu.ready).unwrap().get_bit_state(0);
            match ready {
                LogicBitState::HighZ | LogicBitState::Undefined => {
                    panic!("invalid ready output: {ready}")
                }
                LogicBitState::Logic0 => (),
                LogicBitState::Logic1 => break,
            }

            sim.set_wire_drive(fpu.clk, &LogicState::LOGIC_1).unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();
            sim.set_wire_drive(fpu.clk, &LogicState::LOGIC_0).unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();

            cycle_count += 1;

            if cycle_count > max_cycle_count {
                panic!("failed to produce result in time")
            }
        }

        //let dot_file = std::fs::File::create("C:/Users/Mathis/Desktop/fpu.dot").unwrap();
        //let dot_writer = std::io::BufWriter::new(dot_file);
        //sim.write_dot(dot_writer, true).unwrap();

        sim.get_wire_state(fpu.result).unwrap()
    })
}

const WIDTH_32: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(32) };

fn test_binary(
    lhs: f32,
    rhs: f32,
    golden_op: impl Fn(f32, f32) -> f32,
    op: Op,
    eq: impl Fn(f32, f32) -> bool,
    max_cycle_count: u32,
) {
    let expected = golden_op(lhs, rhs);
    let actual_state = test_impl(lhs, rhs, op, max_cycle_count);

    if let Ok(actual) = actual_state.to_int(WIDTH_32) {
        let actual = f32::from_bits(actual);

        if !eq(expected, actual) {
            panic!(
                "\n     lhs: {lhs:+}({})\n     rhs: {rhs:+}({})\nexpected: {expected:+}({})\n  actual: {actual:+}({})",
                print_float(lhs),
                print_float(rhs),
                print_float(expected),
                print_float(actual),
            );
        }
    } else {
        panic!(
            "\n     lhs: {lhs:+}({})\n     rhs: {rhs:+}({})\nexpected: {expected:+}({})\n  actual: {}",
            print_float(lhs),
            print_float(rhs),
            print_float(expected),
            actual_state.display_string(WIDTH_32),
        );
    }
}

fn test_unary(
    value: f32,
    golden_op: impl Fn(f32) -> f32,
    op: Op,
    eq: impl Fn(f32, f32) -> bool,
    max_cycle_count: u32,
) {
    let expected = golden_op(value);
    let actual_state = test_impl(value, 0.0, op, max_cycle_count);

    if let Ok(actual) = actual_state.to_int(WIDTH_32) {
        let actual = f32::from_bits(actual);

        if !eq(expected, actual) {
            panic!(
                "\n   value: {value:+}({})\nexpected: {expected:+}({})\n  actual: {actual:+}({})",
                print_float(value),
                print_float(expected),
                print_float(actual),
            );
        }
    } else {
        panic!(
            "\n   value: {value:+}({})\nexpected: {expected:+}({})\n  actual: {}",
            print_float(value),
            print_float(expected),
            actual_state.display_string(WIDTH_32),
        );
    }
}

fn test_cmp(
    lhs: f32,
    rhs: f32,
    golden_op: impl Fn(f32, f32) -> bool,
    op: Op,
    max_cycle_count: u32,
) {
    let expected = golden_op(lhs, rhs);
    let actual_state = test_impl(lhs, rhs, op, max_cycle_count);

    if let Ok(actual) = actual_state.to_bool() {
        if actual != expected {
            panic!(
                "\n     lhs: {lhs:+}({})\n     rhs: {rhs:+}({})\nexpected: {expected}\n  actual: {actual}",
                print_float(lhs),
                print_float(rhs),
            );
        }
    } else {
        panic!(
            "\n     lhs: {lhs:+}({})\n     rhs: {rhs:+}({})\nexpected: {expected}\n  actual: {}",
            print_float(lhs),
            print_float(rhs),
            actual_state.display_string(NonZeroU8::MIN),
        );
    }
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn add(lhs: f32, rhs: f32) {
    test_binary(lhs, rhs, golden_add, Op::Add, equals_ignore_rounding, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn sub(lhs: f32, rhs: f32) {
    test_binary(lhs, rhs, golden_sub, Op::Sub, equals_ignore_rounding, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn mul(lhs: f32, rhs: f32) {
    test_binary(lhs, rhs, golden_mul, Op::Mul, equals_ignore_rounding, 1);
}

#[proptest(ProptestConfig { cases : 3000, ..ProptestConfig::default() })]
fn div(lhs: f32, rhs: f32) {
    test_binary(lhs, rhs, golden_div, Op::Div, equals_ignore_rounding, 27);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn min(lhs: f32, rhs: f32) {
    test_binary(lhs, rhs, golden_min, Op::Min, equals, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn max(lhs: f32, rhs: f32) {
    test_binary(lhs, rhs, golden_max, Op::Max, equals, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn floor(value: f32) {
    test_unary(value, golden_floor, Op::Floor, equals, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn ceil(value: f32) {
    test_unary(value, golden_ceil, Op::Ceil, equals, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn round(value: f32) {
    test_unary(value, golden_round, Op::Round, equals, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn trunc(value: f32) {
    test_unary(value, golden_trunc, Op::Trunc, equals, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn abs(value: f32) {
    test_unary(value, golden_abs, Op::Abs, equals, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn neg(value: f32) {
    test_unary(value, golden_neg, Op::Neg, equals, 1);
}

#[proptest(ProptestConfig { cases : 5000, ..ProptestConfig::default() })]
fn sqrt(value: f32) {
    test_unary(
        value,
        golden_sqrt,
        Op::Sqrt,
        |a, b| equals_allow_imprecision(a, b, 4),
        14,
    );
}

#[proptest(ProptestConfig { cases : 5000, ..ProptestConfig::default() })]
fn rsqrt(value: f32) {
    test_unary(
        value,
        golden_rsqrt,
        Op::Rsqrt,
        |a, b| equals_allow_imprecision(a, b, 4),
        14,
    );
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn cmpeq(lhs: f32, rhs: f32) {
    test_cmp(lhs, rhs, golden_cmpeq, Op::CmpEq, 1)
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn cmpne(lhs: f32, rhs: f32) {
    test_cmp(lhs, rhs, golden_cmpne, Op::CmpNe, 1)
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn cmplt(lhs: f32, rhs: f32) {
    test_cmp(lhs, rhs, golden_cmplt, Op::CmpLt, 1)
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn cmpge(lhs: f32, rhs: f32) {
    test_cmp(lhs, rhs, golden_cmpge, Op::CmpGe, 1)
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn itof(value: i32) {
    let expected = golden_itof(value);
    let actual_state = test_impl(f32::from_bits(value as u32), 0.0, Op::ItoF, 1);

    if let Ok(actual) = actual_state.to_int(WIDTH_32) {
        let actual = f32::from_bits(actual);

        if !equals_ignore_rounding(expected, actual) {
            panic!(
                "\n   value: {value:+}\nexpected: {expected:+}({})\n  actual: {actual:+}({})",
                print_float(expected),
                print_float(actual),
            );
        }
    } else {
        panic!(
            "\n   value: {value:+}\nexpected: {expected:+}({})\n  actual: {}",
            print_float(expected),
            actual_state.display_string(WIDTH_32),
        );
    }
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn ftoi(value: f32) {
    let expected = golden_ftoi(value);
    let actual_state = test_impl(value, 0.0, Op::FtoI, 1);

    if let Ok(actual) = actual_state.to_int(WIDTH_32) {
        let actual = actual as i32;

        if actual != expected {
            panic!(
                "\n   value: {value:+}({})\nexpected: {expected:+}\n  actual: {actual:+}",
                print_float(value),
            );
        }
    } else {
        panic!(
            "\n   value: {value:+}({})\nexpected: {expected:+}\n  actual: {}",
            print_float(value),
            actual_state.display_string(WIDTH_32),
        );
    }
}
