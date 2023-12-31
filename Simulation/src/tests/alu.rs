use super::MAX_SIM_STEPS;
use crate::module;
use gsim::{LogicBitState, LogicState};
use proptest::prelude::ProptestConfig;
use std::num::NonZeroU8;
use test_strategy::*;

module! {
    ALU: Alu = "alu" {
        in lhs,
        in rhs,

        in op,
        in start,
        in flags,
        in condition,

        out result,
        out next_flags,
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
    And = 0x02,
    Or = 0x03,
    Xor = 0x04,
    Shl = 0x05,
    Lsr = 0x06,
    Asr = 0x07,
    AddC = 0x08,
    SubC = 0x09,
    Mul = 0x10,
    MulHuu = 0x11,
    MulHss = 0x12,
    MulHus = 0x13,
    Divu = 0x14,
    Divs = 0x15,
    Remu = 0x16,
    Rems = 0x17,
    Cond = 0x18,
}

#[derive(Debug, Clone, Copy, Arbitrary)]
#[repr(u8)]
enum Condition {
    Equal = 0x0,
    NotEqual = 0x1,
    UnsignedLessThan = 0x2,
    UnsignedGreaterOrEqual = 0x3,
    SignedLessThan = 0x4,
    SignedGreaterOrEqual = 0x5,
    Always = 0x6,
    Never = 0x7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Arbitrary)]
struct Flags {
    carry: bool,
    zero: bool,
    sign: bool,
    overflow: bool,
}

impl Flags {
    #[inline]
    fn parse(raw: u32) -> Self {
        Self {
            carry: ((raw >> 0) & 0b1) > 0,
            zero: ((raw >> 1) & 0b1) > 0,
            sign: ((raw >> 2) & 0b1) > 0,
            overflow: ((raw >> 3) & 0b1) > 0,
        }
    }

    #[inline]
    fn satisfy(self, cond: Condition) -> bool {
        match cond {
            Condition::Equal => self.zero,
            Condition::NotEqual => !self.zero,
            Condition::UnsignedLessThan => !self.carry,
            Condition::UnsignedGreaterOrEqual => self.carry,
            Condition::SignedLessThan => self.sign != self.overflow,
            Condition::SignedGreaterOrEqual => self.sign == self.overflow,
            Condition::Always => true,
            Condition::Never => false,
        }
    }

    #[inline]
    fn to_bits(&self) -> [LogicBitState; 4] {
        [
            LogicBitState::from_bool(self.overflow),
            LogicBitState::from_bool(self.sign),
            LogicBitState::from_bool(self.zero),
            LogicBitState::from_bool(self.carry),
        ]
    }
}

impl std::fmt::Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = [b'.'; 4];
        if self.carry {
            s[3] = b'C';
        }
        if self.zero {
            s[2] = b'Z';
        }
        if self.sign {
            s[1] = b'S';
        }
        if self.overflow {
            s[0] = b'O';
        }

        let s = unsafe { std::str::from_utf8_unchecked(&s) };
        f.write_str(s)
    }
}

#[inline]
fn carry_add(lhs: u32, rhs: u32, carry: bool) -> (u32, bool) {
    let (r1, c1) = lhs.overflowing_add(rhs);
    let (r2, c2) = r1.overflowing_add(carry as u32);
    (r2, c1 | c2)
}

#[inline]
fn golden_add(lhs: u32, rhs: u32, _flags: Flags, _cond: Condition) -> (u32, Flags) {
    let (result, carry) = carry_add(lhs, rhs, false);
    let lhs_sign = (lhs as i32) < 0;
    let rhs_sign = (rhs as i32) < 0;
    let sign = (result as i32) < 0;
    let flags = Flags {
        carry,
        zero: result == 0,
        sign,
        overflow: (lhs_sign == rhs_sign) & (lhs_sign != sign),
    };
    (result, flags)
}

#[inline]
fn golden_sub(lhs: u32, rhs: u32, _flags: Flags, _cond: Condition) -> (u32, Flags) {
    let rhs = !rhs;
    let (result, carry) = carry_add(lhs, rhs, true);
    let lhs_sign = (lhs as i32) < 0;
    let rhs_sign = (rhs as i32) < 0;
    let sign = (result as i32) < 0;
    let flags = Flags {
        carry,
        zero: result == 0,
        sign,
        overflow: (lhs_sign == rhs_sign) & (lhs_sign != sign),
    };
    (result, flags)
}

#[inline]
fn golden_addc(lhs: u32, rhs: u32, flags: Flags, _cond: Condition) -> (u32, Flags) {
    let (result, carry) = carry_add(lhs, rhs, flags.carry);
    let lhs_sign = (lhs as i32) < 0;
    let rhs_sign = (rhs as i32) < 0;
    let sign = (result as i32) < 0;
    let flags = Flags {
        carry,
        zero: (result == 0) & flags.zero,
        sign,
        overflow: (lhs_sign == rhs_sign) & (lhs_sign != sign),
    };
    (result, flags)
}

#[inline]
fn golden_subc(lhs: u32, rhs: u32, flags: Flags, _cond: Condition) -> (u32, Flags) {
    let rhs = !rhs;
    let (result, carry) = carry_add(lhs, rhs, flags.carry);
    let lhs_sign = (lhs as i32) < 0;
    let rhs_sign = (rhs as i32) < 0;
    let sign = (result as i32) < 0;
    let flags = Flags {
        carry,
        zero: (result == 0) & flags.zero,
        sign,
        overflow: (lhs_sign == rhs_sign) & (lhs_sign != sign),
    };
    (result, flags)
}

#[inline]
fn golden_and(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = lhs & rhs;
    flags.zero = result == 0;
    (result, flags)
}

#[inline]
fn golden_or(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = lhs | rhs;
    flags.zero = result == 0;
    (result, flags)
}

#[inline]
fn golden_xor(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = lhs ^ rhs;
    flags.zero = result == 0;
    (result, flags)
}

#[inline]
fn golden_shl(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = lhs.wrapping_shl(rhs);
    flags.zero = result == 0;
    (result, flags)
}

#[inline]
fn golden_lsr(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = lhs.wrapping_shr(rhs);
    flags.zero = result == 0;
    (result, flags)
}

#[inline]
fn golden_asr(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = (lhs as i32).wrapping_shr(rhs) as u32;
    flags.zero = result == 0;
    (result, flags)
}

#[inline]
fn golden_mul(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = lhs.wrapping_mul(rhs);
    flags.zero = result == 0;
    (result, flags)
}

#[inline]
fn golden_mulhuu(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = ((lhs as u64).wrapping_mul(rhs as u64) >> 32) as u32;
    flags.zero &= result == 0;
    (result, flags)
}

#[inline]
fn golden_mulhss(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = (((lhs as i32) as i64).wrapping_mul((rhs as i32) as i64) >> 32) as u32;
    flags.zero &= result == 0;
    (result, flags)
}

#[inline]
fn golden_mulhus(lhs: u32, rhs: u32, mut flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = ((lhs as i64).wrapping_mul((rhs as i32) as i64) >> 32) as u32;
    flags.zero &= result == 0;
    (result, flags)
}

#[inline]
fn golden_divu(lhs: u32, rhs: u32, flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = lhs.wrapping_div(rhs);
    (result, flags)
}

#[inline]
fn golden_divs(lhs: u32, rhs: u32, flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = (lhs as i32).wrapping_div(rhs as i32) as u32;
    (result, flags)
}

#[inline]
fn golden_remu(lhs: u32, rhs: u32, flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = lhs.wrapping_rem(rhs);
    (result, flags)
}

#[inline]
fn golden_rems(lhs: u32, rhs: u32, flags: Flags, _cond: Condition) -> (u32, Flags) {
    let result = (lhs as i32).wrapping_rem(rhs as i32) as u32;
    (result, flags)
}

#[inline]
fn golden_cond(lhs: u32, rhs: u32, flags: Flags, cond: Condition) -> (u32, Flags) {
    let result = if flags.satisfy(cond) { rhs } else { lhs };
    (result, flags)
}

fn test_impl(
    lhs: u32,
    rhs: u32,
    flags: Flags,
    cond: Condition,
    op: Op,
    max_cycle_count: u32,
) -> (LogicState, LogicState) {
    ALU.with(|alu| {
        let lhs = LogicState::from_int(lhs);
        let rhs = LogicState::from_int(rhs);
        let mut flags = LogicState::from_bits(&flags.to_bits()).unwrap();
        let cond = LogicState::from_int(cond as u32);
        let op = LogicState::from_int(op as u32);

        let mut sim = alu.sim.borrow_mut();
        sim.reset();

        sim.set_wire_drive(alu.lhs, &lhs).unwrap();
        sim.set_wire_drive(alu.rhs, &rhs).unwrap();
        sim.set_wire_drive(alu.flags, &flags).unwrap();
        sim.set_wire_drive(alu.condition, &cond).unwrap();
        sim.set_wire_drive(alu.op, &op).unwrap();
        sim.set_wire_drive(alu.enable, &LogicState::LOGIC_1)
            .unwrap();
        sim.set_wire_drive(alu.reset, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(alu.start, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(alu.clk, &LogicState::LOGIC_0).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        sim.set_wire_drive(alu.reset, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(alu.clk, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(alu.clk, &LogicState::LOGIC_0).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        sim.set_wire_drive(alu.reset, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(alu.start, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(alu.clk, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(alu.clk, &LogicState::LOGIC_0).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        let mut cycle_count = 1;
        loop {
            flags = sim.get_wire_state(alu.next_flags).unwrap();

            let ready = sim.get_wire_state(alu.ready).unwrap().get_bit_state(0);
            match ready {
                LogicBitState::HighZ | LogicBitState::Undefined => {
                    panic!("invalid ready output: {ready}")
                }
                LogicBitState::Logic0 => (),
                LogicBitState::Logic1 => break,
            }

            sim.set_wire_drive(alu.clk, &LogicState::LOGIC_1).unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();
            sim.set_wire_drive(alu.flags, &flags).unwrap();
            sim.set_wire_drive(alu.clk, &LogicState::LOGIC_0).unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();

            cycle_count += 1;

            if cycle_count > max_cycle_count {
                panic!("failed to produce result in time")
            }
        }

        //let dot_file = std::fs::File::create("C:/Users/Mathis/Desktop/fpu.dot").unwrap();
        //let dot_writer = std::io::BufWriter::new(dot_file);
        //sim.write_dot(dot_writer, true).unwrap();

        (sim.get_wire_state(alu.result).unwrap(), flags)
    })
}

const WIDTH_32: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(32) };
const WIDTH_4: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(4) };

fn test(
    lhs: u32,
    rhs: u32,
    flags: Flags,
    cond: Condition,
    golden_op: impl Fn(u32, u32, Flags, Condition) -> (u32, Flags),
    op: Op,
    max_cycle_count: u32,
) {
    let (expected_result, expected_flags) = golden_op(lhs, rhs, flags, cond);
    let (actual_result_state, actual_flags_state) =
        test_impl(lhs, rhs, flags, cond, op, max_cycle_count);

    let actual_result = actual_result_state.to_int(WIDTH_32);
    let actual_flags = actual_flags_state.to_int(WIDTH_4);

    match (actual_result, actual_flags) {
        (Ok(actual_result), Ok(actual_flags)) => {
            let actual_flags = Flags::parse(actual_flags);

            if (actual_result != expected_result) || (actual_flags != expected_flags) {
                panic!("\n     lhs: {lhs}\n     rhs: {rhs}\n   flags: {flags}\nexpected: {expected_result} [{expected_flags}]\n  actual: {actual_result} [{actual_flags}]");
            }
        }
        (Ok(actual_result), Err(_)) => {
            panic!(
                "\n     lhs: {lhs}\n     rhs: {rhs}\n   flags: {flags}\nexpected: {expected_result} [{expected_flags}]\n  actual: {actual_result} [{}]",
                actual_flags_state.display_string(WIDTH_4),
            );
        }
        (Err(_), Ok(actual_flags)) => {
            let actual_flags = Flags::parse(actual_flags);

            panic!(
                "\n     lhs: {lhs}\n     rhs: {rhs}\n   flags: {flags}\nexpected: {expected_result} [{expected_flags}]\n  actual: {} [{actual_flags}]",
                actual_result_state.display_string(WIDTH_32),
            );
        }
        (Err(_), Err(_)) => {
            panic!(
                "\n     lhs: {lhs}\n     rhs: {rhs}\n   flags: {flags}\nexpected: {expected_result} [{expected_flags}]\n  actual: {} [{}]",
                actual_result_state.display_string(WIDTH_32),
                actual_flags_state.display_string(WIDTH_4),
            );
        }
    }
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn add(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_add, Op::Add, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn sub(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_sub, Op::Sub, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn addc(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_addc, Op::AddC, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn subc(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_subc, Op::SubC, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn and(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_and, Op::And, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn or(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_or, Op::Or, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn xor(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_xor, Op::Xor, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn shl(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_shl, Op::Shl, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn lsr(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_lsr, Op::Lsr, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn asr(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_asr, Op::Asr, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn mul(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_mul, Op::Mul, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn mulhuu(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_mulhuu, Op::MulHuu, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn mulhss(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_mulhss, Op::MulHss, 1);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn mulhus(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_mulhus, Op::MulHus, 1);
}

#[proptest(ProptestConfig { cases : 2000, ..ProptestConfig::default() })]
fn divu(lhs: u32, #[strategy(1u32..)] rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_divu, Op::Divu, 34);
}

#[proptest(ProptestConfig { cases : 2000, ..ProptestConfig::default() })]
fn divs(lhs: u32, #[strategy(1u32..)] rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_divs, Op::Divs, 34);
}

#[proptest(ProptestConfig { cases : 2000, ..ProptestConfig::default() })]
fn remu(lhs: u32, #[strategy(1u32..)] rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_remu, Op::Remu, 34);
}

#[proptest(ProptestConfig { cases : 2000, ..ProptestConfig::default() })]
fn rems(lhs: u32, #[strategy(1u32..)] rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_rems, Op::Rems, 34);
}

#[proptest(ProptestConfig { cases : 10000, ..ProptestConfig::default() })]
fn cond(lhs: u32, rhs: u32, flags: Flags, cond: Condition) {
    test(lhs, rhs, flags, cond, golden_cond, Op::Cond, 1);
}
