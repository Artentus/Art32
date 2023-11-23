use super::super::{BranchCondition, Condition, Cpu, Flags, Register};
use super::{br_cond, cond, cpu, reg16, reg32, TestIo, TestMemory};
use crate::{shuffle_bits, Ashr};
use proptest::prelude::*;
use test_strategy::proptest;

fn simulate_add(lhs: u32, rhs: u32, c_in: bool) -> (u32, Flags) {
    let lhs_sign = (lhs as i32) < 0;
    let rhs_sign = (rhs as i32) < 0;

    let (r1, c1) = lhs.overflowing_add(rhs);
    let (r2, c2) = r1.overflowing_add(c_in as u32);
    let result_sign = (r2 as i32) < 0;

    let mut flags = Flags::empty();
    flags.set(Flags::CARRY, c1 | c2);
    flags.set(Flags::ZERO, r2 == 0);
    flags.set(Flags::SIGN, result_sign);
    flags.set(
        Flags::OVERFLOW,
        (lhs_sign == rhs_sign) & (lhs_sign != result_sign),
    );

    (r2, flags)
}

fn align2(x: &i32) -> bool {
    (x & 0x1) == 0
}

#[proptest]
fn ldi_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] target: Register,
    #[strategy(-512..=511)] value: i32,
) {
    let value = value as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    expected_regs.set(target, value);

    let mut mem = [shuffle_bits!(value {
        [4:0] => [11:7],
        [8:6] => [6:4],
        [9] => [3],
        [5] => [2],
    }) | (u32::from(target) << 12)
        | 0b00];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn addi_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] target: Register,
    #[strategy(-512..=511)] value: i32,
) {
    let value = value as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let (result, expected_flags) = simulate_add(expected_regs.get(target), value, false);
    expected_regs.set(target, result);

    let mut mem = [shuffle_bits!(value {
        [4:0] => [11:7],
        [8:6] => [6:4],
        [9] => [3],
        [5] => [2],
    }) | (u32::from(target) << 12)
        | 0b10];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn j_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] base: Register,
    #[strategy(-512..=511)]
    #[filter(align2)]
    offset: i32,
) {
    let offset = offset as u32;

    let expected_program_counter = cpu.state.regs.get(base).wrapping_add(offset) & !0x1;
    let expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;

    let mut mem = [shuffle_bits!(offset {
        [4:1] => [11:8],
        [5] => [7],
        [8:6] => [6:4],
        [9] => [3],
    }) | (u32::from(base) << 12)
        | 0b001];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn jl_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] base: Register,
    #[strategy(-512..=511)]
    #[filter(align2)]
    offset: i32,
) {
    let offset = offset as u32;

    let expected_program_counter = cpu.state.regs.get(base).wrapping_add(offset) & !0x1;
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    expected_regs.set(Register::Ra, cpu.program_counter.wrapping_add(2));

    let mut mem = [shuffle_bits!(offset {
        [4:1] => [11:8],
        [5] => [7],
        [8:6] => [6:4],
        [9] => [3],
    }) | (u32::from(base) << 12)
        | 0b101];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn br_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(br_cond())] condition: BranchCondition,
    #[strategy(-512..=511)]
    #[filter(align2)]
    offset: i32,
) {
    let offset = offset as u32;

    let mut expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    if condition == BranchCondition::Link {
        expected_regs.set(Register::Ra, expected_program_counter);
    }
    if expected_flags.satisfy_branch(condition) {
        expected_program_counter = expected_program_counter.wrapping_add(offset);
    }

    let mut mem = [shuffle_bits!(offset {
        [5] => [15],
        [4:1] => [11:8],
        [8:6] => [6:4],
        [9] => [3],
    }) | (u32::from(condition) << 12)
        | 0b011];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn ldui_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(-2_097_152..=2_097_151)] value: i32,
) {
    let value = (value as u32) << 10;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    expected_regs.set(target, value);

    let mut mem = [shuffle_bits!(value {
        [31:27] => [31:27],
        [12:10] => [26:24],
        [14:13] => [23:22],
        [19:15] => [21:17],
        [26:23] => [11:8],
        [22:20] => [6:4],
    }) | (u32::from(target) << 12)
        | 0b1_000_0011];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn apcui_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(-2_097_152..=2_097_151)] offset: i32,
) {
    let offset = (offset as u32) << 10;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    expected_regs.set(target, expected_program_counter.wrapping_add(offset));

    let mut mem = [shuffle_bits!(offset {
        [31:27] => [31:27],
        [12:10] => [26:24],
        [14:13] => [23:22],
        [19:15] => [21:17],
        [26:23] => [11:8],
        [22:20] => [6:4],
    }) | (u32::from(target) << 12)
        | 0b1_000_1011];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn add_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(reg16())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let (result, expected_flags) =
        simulate_add(expected_regs.get(lhs), expected_regs.get(rhs), false);
    expected_regs.set(lhs, result);

    let mut mem = [(u32::from(lhs) << 12) | (u32::from(rhs) << 8) | 0b000_00111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn sub_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(reg16())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let (result, expected_flags) =
        simulate_add(expected_regs.get(lhs), !expected_regs.get(rhs), true);
    expected_regs.set(lhs, result);

    let mut mem = [(u32::from(lhs) << 12) | (u32::from(rhs) << 8) | 0b001_00111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn and_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(reg16())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) & expected_regs.get(rhs);
    expected_regs.set(lhs, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [(u32::from(lhs) << 12) | (u32::from(rhs) << 8) | 0b010_00111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn or_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(reg16())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) | expected_regs.get(rhs);
    expected_regs.set(lhs, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [(u32::from(lhs) << 12) | (u32::from(rhs) << 8) | 0b011_00111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn xor_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(reg16())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) ^ expected_regs.get(rhs);
    expected_regs.set(lhs, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [(u32::from(lhs) << 12) | (u32::from(rhs) << 8) | 0b100_00111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn shl_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(reg16())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) << (expected_regs.get(rhs) & 0x1F);
    expected_regs.set(lhs, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [(u32::from(lhs) << 12) | (u32::from(rhs) << 8) | 0b101_00111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn lsr_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(reg16())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) >> (expected_regs.get(rhs) & 0x1F);
    expected_regs.set(lhs, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [(u32::from(lhs) << 12) | (u32::from(rhs) << 8) | 0b110_00111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn asr_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(reg16())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs).ashr(expected_regs.get(rhs) & 0x1F);
    expected_regs.set(lhs, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [(u32::from(lhs) << 12) | (u32::from(rhs) << 8) | 0b111_00111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn mov_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(cond())] condition: Condition,
    #[strategy(reg16())] target: Register,
    #[strategy(reg16())] source: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    if expected_flags.satisfy(condition) {
        expected_regs.set(target, expected_regs.get(source));
    }

    let mut mem = [(u32::from(target) << 12)
        | (u32::from(source) << 8)
        | (u32::from(condition) << 5)
        | 0b01111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn cmp_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(reg16())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let expected_regs = cpu.state.regs.clone();
    let (_, expected_flags) = simulate_add(expected_regs.get(lhs), !expected_regs.get(rhs), true);

    let mut mem = [(u32::from(lhs) << 12) | (u32::from(rhs) << 8) | 0b000_10111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn ret_16(#[strategy(cpu())] mut cpu: Cpu) {
    let expected_program_counter = cpu.state.regs.get(Register::Ra) & !0x1;
    let expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;

    let mut mem = [0b0000_100_10111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

// TODO: sysret

#[proptest]
fn fence_16(#[strategy(cpu())] mut cpu: Cpu, #[strategy(0u32..16u32)] bits: u32) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;

    let mut mem = [(bits << 12) | 0b0010_100_10111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn ifence_16(#[strategy(cpu())] mut cpu: Cpu) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;

    let mut mem = [0b0011_100_10111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn envcall_16(#[strategy(cpu())] mut cpu: Cpu, #[strategy(0u32..16u32)] slot: u32) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;

    let mut mem = [(slot << 12) | 0b1110_100_10111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert_eq!(cpu.step(&mut mem, &mut TestIo), Some(slot as u8));

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

// TODO: syscall

#[proptest]
fn shli_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(0u32..32u32)] rhs: u32,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) << rhs;
    expected_regs.set(lhs, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [(u32::from(lhs) << 12) | (rhs << 7) | 0b01_10111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn lsri_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(0u32..32u32)] rhs: u32,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) >> rhs;
    expected_regs.set(lhs, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [(u32::from(lhs) << 12) | (rhs << 7) | 0b10_10111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn asri_16(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg16())] lhs: Register,
    #[strategy(0u32..32u32)] rhs: u32,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(2);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs).ashr(rhs);
    expected_regs.set(lhs, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [(u32::from(lhs) << 12) | (rhs << 7) | 0b11_10111];
    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

// TODO: ld.32 [sp]
// TODO: st.32 [sp]

#[proptest]
fn jl_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] link: Register,
    #[strategy(reg32())] base: Register,
    #[strategy(-4096..=4095)]
    #[filter(align2)]
    offset: i32,
) {
    let offset = offset as u32;

    let expected_program_counter = cpu.state.regs.get(base).wrapping_add(offset) & !0x1;
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    expected_regs.set(link, cpu.program_counter.wrapping_add(4));

    let mut mem = [shuffle_bits!(offset {
        [13] => [31],
        [8:5] => [30:27],
        [12:10] => [26:24],
        [4:1] => [11:8],
        [9] => [7],
    }) | (u32::from(base) << 17)
        | (u32::from(link) << 12)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn br_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(br_cond())] condition: BranchCondition,
    #[strategy(-1_048_576..=1_048_575)]
    #[filter(align2)]
    offset: i32,
) {
    let offset = offset as u32;

    let mut expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    if condition == BranchCondition::Link {
        expected_regs.set(Register::Ra, expected_program_counter);
    }
    if expected_flags.satisfy_branch(condition) {
        expected_program_counter = expected_program_counter.wrapping_add(offset);
    }

    let mut mem = [shuffle_bits!(offset {
        [20] => [31],
        [8:5] => [30:27],
        [12:10] => [26:24],
        [19:13] => [21:15],
        [4:1] => [11:8],
        [9] => [7],
    }) | (u32::from(condition) << 12)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn addi_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(-512..=511)] rhs: i32,
) {
    let rhs = rhs as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let (result, expected_flags) = simulate_add(expected_regs.get(lhs), rhs, false);
    expected_regs.set(target, result);

    let mut mem = [shuffle_bits!(rhs {
        [9] => [31],
        [8:5] => [30:27],
        [4:0] => [11:7],
    }) | (u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | (0b000_01 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn subi_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(-512..=511)] rhs: i32,
) {
    let rhs = rhs as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let (result, expected_flags) = simulate_add(expected_regs.get(lhs), !rhs, true);
    expected_regs.set(target, result);

    let mut mem = [shuffle_bits!(rhs {
        [9] => [31],
        [8:5] => [30:27],
        [4:0] => [11:7],
    }) | (u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | (0b001_01 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn andi_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(-512..=511)] rhs: i32,
) {
    let rhs = rhs as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) & rhs;
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [shuffle_bits!(rhs {
        [9] => [31],
        [8:5] => [30:27],
        [4:0] => [11:7],
    }) | (u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | (0b010_01 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn ori_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(-512..=511)] rhs: i32,
) {
    let rhs = rhs as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) | rhs;
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [shuffle_bits!(rhs {
        [9] => [31],
        [8:5] => [30:27],
        [4:0] => [11:7],
    }) | (u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | (0b011_01 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn xori_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(-512..=511)] rhs: i32,
) {
    let rhs = rhs as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) ^ rhs;
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [shuffle_bits!(rhs {
        [9] => [31],
        [8:5] => [30:27],
        [4:0] => [11:7],
    }) | (u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | (0b100_01 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn shli_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(-512..=511)] rhs: i32,
) {
    let rhs = rhs as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) << (rhs & 0x1F);
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [shuffle_bits!(rhs {
        [9] => [31],
        [8:5] => [30:27],
        [4:0] => [11:7],
    }) | (u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | (0b101_01 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn lsri_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(-512..=511)] rhs: i32,
) {
    let rhs = rhs as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) >> (rhs & 0x1F);
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [shuffle_bits!(rhs {
        [9] => [31],
        [8:5] => [30:27],
        [4:0] => [11:7],
    }) | (u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | (0b110_01 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn asri_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(-512..=511)] rhs: i32,
) {
    let rhs = rhs as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs).ashr(rhs & 0x1F);
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let mut mem = [shuffle_bits!(rhs {
        [9] => [31],
        [8:5] => [30:27],
        [4:0] => [11:7],
    }) | (u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | (0b111_01 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn movi_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(cond())] condition: Condition,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(-512..=511)] rhs: i32,
) {
    let rhs = rhs as u32;

    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    if expected_flags.satisfy(condition) {
        expected_regs.set(target, rhs);
    } else {
        expected_regs.set(target, expected_regs.get(lhs));
    }

    let mut mem = [shuffle_bits!(rhs {
        [9] => [31],
        [8:5] => [30:27],
        [4:0] => [11:7],
    }) | (u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | (u32::from(condition) << 24)
        | (0b01 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

// TODO: ld
// TODO: st

#[proptest]
fn add_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let (result, expected_flags) =
        simulate_add(expected_regs.get(lhs), expected_regs.get(rhs), false);
    expected_regs.set(target, result);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b000_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn sub_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let (result, expected_flags) =
        simulate_add(expected_regs.get(lhs), !expected_regs.get(rhs), true);
    expected_regs.set(target, result);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b001_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn and_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) & expected_regs.get(rhs);
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b010_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn or_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) | expected_regs.get(rhs);
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b011_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn xor_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) ^ expected_regs.get(rhs);
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b100_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn shl_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) << (expected_regs.get(rhs) & 0x1F);
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b101_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn lsr_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs) >> (expected_regs.get(rhs) & 0x1F);
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b110_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn asr_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = expected_regs.get(lhs).ashr(expected_regs.get(rhs) & 0x1F);
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b111_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn mov_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(cond())] condition: Condition,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    if expected_flags.satisfy(condition) {
        expected_regs.set(target, expected_regs.get(rhs));
    } else {
        expected_regs.set(target, expected_regs.get(lhs));
    }

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (u32::from(condition) << 24)
        | (0b11 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn addc_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let (result, mut expected_flags) = simulate_add(
        expected_regs.get(lhs),
        expected_regs.get(rhs),
        cpu.state.flags.contains(Flags::CARRY),
    );
    expected_regs.set(target, result);
    if !cpu.state.flags.contains(Flags::ZERO) {
        expected_flags.remove(Flags::ZERO);
    }

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_000_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn subc_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let (result, mut expected_flags) = simulate_add(
        expected_regs.get(lhs),
        !expected_regs.get(rhs),
        cpu.state.flags.contains(Flags::CARRY),
    );
    expected_regs.set(target, result);
    if !cpu.state.flags.contains(Flags::ZERO) {
        expected_flags.remove(Flags::ZERO);
    }

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_001_11 << 22)
        | 0b0111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn mul_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = u32::wrapping_mul(expected_regs.get(lhs), expected_regs.get(rhs));
    expected_regs.set(target, result);
    expected_flags.set(Flags::ZERO, result == 0);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_000_11 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn mulhuu_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = (u64::wrapping_mul(expected_regs.get(lhs) as u64, expected_regs.get(rhs) as u64)
        >> 32) as u32;
    expected_regs.set(target, result);
    if result != 0 {
        expected_flags.remove(Flags::ZERO);
    }

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_001_11 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn mulhss_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = (i64::wrapping_mul(
        (expected_regs.get(lhs) as i32) as i64,
        (expected_regs.get(rhs) as i32) as i64,
    ) >> 32) as u32;
    expected_regs.set(target, result);
    if result != 0 {
        expected_flags.remove(Flags::ZERO);
    }

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_010_11 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn mulhus_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let mut expected_flags = cpu.state.flags;
    let result = (i64::wrapping_mul(
        (expected_regs.get(lhs) as u32) as i64,
        (expected_regs.get(rhs) as i32) as i64,
    ) >> 32) as u32;
    expected_regs.set(target, result);
    if result != 0 {
        expected_flags.remove(Flags::ZERO);
    }

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_011_11 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn divu_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    let result = if expected_regs.get(rhs) == 0 {
        u32::MAX
    } else {
        u32::wrapping_div(expected_regs.get(lhs), expected_regs.get(rhs))
    };
    expected_regs.set(target, result);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_100_11 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn divs_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    let result = if expected_regs.get(rhs) == 0 {
        if (expected_regs.get(lhs) as i32) < 0 {
            i32::MIN
        } else {
            i32::MAX
        }
    } else {
        i32::wrapping_div(expected_regs.get(lhs) as i32, expected_regs.get(rhs) as i32)
    } as u32;
    expected_regs.set(target, result);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_101_11 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn remu_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    let result = if expected_regs.get(rhs) == 0 {
        0
    } else {
        u32::wrapping_rem(expected_regs.get(lhs), expected_regs.get(rhs))
    };
    expected_regs.set(target, result);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_110_11 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

#[proptest]
fn rems_32(
    #[strategy(cpu())] mut cpu: Cpu,
    #[strategy(reg32())] target: Register,
    #[strategy(reg32())] lhs: Register,
    #[strategy(reg32())] rhs: Register,
) {
    let expected_program_counter = cpu.program_counter.wrapping_add(4);
    let mut expected_regs = cpu.state.regs.clone();
    let expected_flags = cpu.state.flags;
    let result = if expected_regs.get(rhs) == 0 {
        0
    } else {
        i32::wrapping_rem(expected_regs.get(lhs) as i32, expected_regs.get(rhs) as i32) as u32
    };
    expected_regs.set(target, result);

    let rhs_bits = u32::from(rhs);
    let mut mem = [(u32::from(lhs) << 17)
        | (u32::from(target) << 12)
        | shuffle_bits!(rhs_bits { [3:0] => [11:8], [4] => [7] })
        | (0b1_111_11 << 22)
        | 0b1111111];

    let mut mem = TestMemory::new(&mut mem, false);
    prop_assert!(cpu.step(&mut mem, &mut TestIo).is_none());

    prop_assert_eq!(cpu.program_counter, expected_program_counter);
    prop_assert_eq!(cpu.state.regs, expected_regs);
    prop_assert_eq!(cpu.state.flags, expected_flags);
}

// TODO: float instructions

// TODO: atomic instructions
