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
    Add = 0x0,
    Sub = 0x1,
    Mul = 0x2,
    Div = 0x3,
    Rem = 0x4,
    Min = 0x6,
    Max = 0x7,
    Floor = 0x8,
    Ceil = 0x9,
    Round = 0xA,
    Fract = 0xB,
    Abs = 0xC,
    Neg = 0xD,
    Sqrt = 0xE,
    Rsqrt = 0xF,
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
fn golden_abs(lhs: f32, _rhs: f32) -> f32 {
    let value = subnormal_to_zero(lhs);
    let result = value.abs();
    subnormal_to_zero(result)
}

#[inline]
fn golden_neg(lhs: f32, _rhs: f32) -> f32 {
    let value = subnormal_to_zero(lhs);
    let result = -value;
    subnormal_to_zero(result)
}

#[inline]
fn golden_sqrt(lhs: f32, _rhs: f32) -> f32 {
    let value = subnormal_to_zero(lhs);
    let result = value.sqrt();
    subnormal_to_zero(result)
}

#[inline]
fn golden_rsqrt(lhs: f32, _rhs: f32) -> f32 {
    let value = subnormal_to_zero(lhs);
    let result = value.sqrt().recip();
    subnormal_to_zero(result)
}

fn test_impl(
    lhs: f32,
    rhs: f32,
    golden_op: impl Fn(f32, f32) -> f32,
    op: Op,
    eq: impl Fn(f32, f32) -> bool,
    max_cycle_count: u32,
) {
    let expected = golden_op(lhs, rhs);
    let actual_str = FPU.with(|fpu| {
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

        let result = sim.get_wire_state(fpu.result).unwrap();
        result.display_string(NonZeroU8::new(32).unwrap())
    });

    if let Ok(actual) = u32::from_str_radix(&actual_str, 2) {
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
            "\n     lhs: {lhs:+}({})\n     rhs: {rhs:+}({})\nexpected: {expected:+}({})\n  actual: {actual_str}",
            print_float(lhs),
            print_float(rhs),
            print_float(expected),
        );
    }
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn add(lhs: f32, rhs: f32) {
    test_impl(lhs, rhs, golden_add, Op::Add, equals_ignore_rounding, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn sub(lhs: f32, rhs: f32) {
    test_impl(lhs, rhs, golden_sub, Op::Sub, equals_ignore_rounding, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn mul(lhs: f32, rhs: f32) {
    test_impl(lhs, rhs, golden_mul, Op::Mul, equals_ignore_rounding, 1);
}

#[proptest(ProptestConfig { cases : 3000, ..ProptestConfig::default() })]
fn div(lhs: f32, rhs: f32) {
    test_impl(lhs, rhs, golden_div, Op::Div, equals_ignore_rounding, 27);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn abs(value: f32) {
    test_impl(value, 0.0, golden_abs, Op::Abs, equals, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn neg(value: f32) {
    test_impl(value, 0.0, golden_neg, Op::Neg, equals, 1);
}

#[proptest(ProptestConfig { cases : 5000, ..ProptestConfig::default() })]
fn sqrt(value: f32) {
    test_impl(
        value,
        0.0,
        golden_sqrt,
        Op::Sqrt,
        |a, b| equals_allow_imprecision(a, b, 4),
        14,
    );
}

#[proptest(ProptestConfig { cases : 5000, ..ProptestConfig::default() })]
fn rsqrt(value: f32) {
    test_impl(
        value,
        0.0,
        golden_rsqrt,
        Op::Rsqrt,
        |a, b| equals_allow_imprecision(a, b, 4),
        14,
    );
}
