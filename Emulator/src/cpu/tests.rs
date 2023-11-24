mod instruction;

use super::interface::*;
use super::{BranchCondition, Condition, Cpu, Flags, Register, RESET_PROGRAM_COUNTER};
use bytemuck::{cast_slice, cast_slice_mut};
use proptest::prelude::*;
use strum::IntoEnumIterator;

fn cpu() -> impl Strategy<Value = Cpu> {
    const FLAG_RANGE: std::ops::RangeInclusive<u8> = Flags::empty().bits()..=Flags::all().bits();

    let regs = [any::<u32>(); 32];
    let flags = FLAG_RANGE.prop_filter_map("invalid flags", Flags::from_bits);

    (regs, flags).prop_map(|(reg_values, flags)| {
        let mut cpu = Cpu::new();
        for (reg, value) in Register::iter().zip(reg_values) {
            cpu.state.regs.set(reg, value);
        }
        cpu.state.flags = flags;
        cpu
    })
}

fn reg32() -> impl Strategy<Value = Register> {
    any::<proptest::sample::Selector>().prop_map(|sel| sel.select(Register::iter()))
}

fn reg16() -> impl Strategy<Value = Register> {
    any::<proptest::sample::Selector>().prop_map(|sel| sel.select(Register::iter().take(16)))
}

fn cond() -> impl Strategy<Value = Condition> {
    any::<proptest::sample::Selector>().prop_map(|sel| sel.select(Condition::iter()))
}

fn br_cond() -> impl Strategy<Value = BranchCondition> {
    any::<proptest::sample::Selector>().prop_map(|sel| sel.select(BranchCondition::iter()))
}

struct TestMemory<'a> {
    mem: &'a mut [u32],
    pass_cond: bool,
}

impl<'a> TestMemory<'a> {
    #[inline]
    fn new(mem: &'a mut [u32], pass_cond: bool) -> Self {
        Self { mem, pass_cond }
    }
}

impl MemoryInterface for TestMemory<'_> {
    fn read_32(
        &mut self,
        addr: u32,
        _priv_level: PrivilegeLevel,
        _reserve: bool,
    ) -> Result<u32, MemoryError> {
        if (addr & 0x3) != 0 {
            return Err(MemoryError::UnalignedAccess);
        }

        let mem_end = RESET_PROGRAM_COUNTER + ((self.mem.len() * 4) as u32) - 1;

        if (RESET_PROGRAM_COUNTER..=mem_end).contains(&addr) {
            let mem: &[u32] = cast_slice(&self.mem);
            Ok(u32::from_le(
                mem[((addr - RESET_PROGRAM_COUNTER) >> 2) as usize],
            ))
        } else {
            Err(MemoryError::AccessViolation)
        }
    }

    fn read_16(
        &mut self,
        addr: u32,
        _priv_level: PrivilegeLevel,
        _reserve: bool,
    ) -> Result<u16, MemoryError> {
        if (addr & 0x1) != 0 {
            return Err(MemoryError::UnalignedAccess);
        }

        let mem_end = RESET_PROGRAM_COUNTER + ((self.mem.len() * 4) as u32) - 1;

        if (RESET_PROGRAM_COUNTER..=mem_end).contains(&addr) {
            let mem: &[u16] = cast_slice(&self.mem);
            Ok(u16::from_le(
                mem[((addr - RESET_PROGRAM_COUNTER) >> 1) as usize],
            ))
        } else {
            Err(MemoryError::AccessViolation)
        }
    }

    fn read_8(
        &mut self,
        addr: u32,
        _priv_level: PrivilegeLevel,
        _reserve: bool,
    ) -> Result<u8, MemoryError> {
        let mem_end = RESET_PROGRAM_COUNTER + ((self.mem.len() * 4) as u32) - 1;

        if (RESET_PROGRAM_COUNTER..=mem_end).contains(&addr) {
            let mem: &[u8] = cast_slice(&self.mem);
            Ok(u8::from_le(
                mem[((addr - RESET_PROGRAM_COUNTER) >> 0) as usize],
            ))
        } else {
            Err(MemoryError::AccessViolation)
        }
    }

    fn write_32(
        &mut self,
        addr: u32,
        value: u32,
        _priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, MemoryError> {
        if (addr & 0x3) != 0 {
            return Err(MemoryError::UnalignedAccess);
        }

        let mem_end = RESET_PROGRAM_COUNTER + ((self.mem.len() * 4) as u32) - 1;
        let do_write = self.pass_cond | !conditional;

        if (RESET_PROGRAM_COUNTER..=mem_end).contains(&addr) {
            if do_write {
                let mem: &mut [u32] = cast_slice_mut(&mut self.mem);
                mem[((addr - RESET_PROGRAM_COUNTER) >> 2) as usize] = u32::to_le(value);
            }

            Ok(do_write)
        } else {
            Err(MemoryError::AccessViolation)
        }
    }

    fn write_16(
        &mut self,
        addr: u32,
        value: u16,
        _priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, MemoryError> {
        if (addr & 0x1) != 0 {
            return Err(MemoryError::UnalignedAccess);
        }

        let mem_end = RESET_PROGRAM_COUNTER + ((self.mem.len() * 4) as u32) - 1;
        let do_write = self.pass_cond | !conditional;

        if (RESET_PROGRAM_COUNTER..=mem_end).contains(&addr) {
            if do_write {
                let mem: &mut [u16] = cast_slice_mut(&mut self.mem);
                mem[((addr - RESET_PROGRAM_COUNTER) >> 1) as usize] = u16::to_le(value);
            }

            Ok(do_write)
        } else {
            Err(MemoryError::AccessViolation)
        }
    }

    fn write_8(
        &mut self,
        addr: u32,
        value: u8,
        _priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, MemoryError> {
        let mem_end = RESET_PROGRAM_COUNTER + ((self.mem.len() * 4) as u32) - 1;
        let do_write = self.pass_cond | !conditional;

        if (RESET_PROGRAM_COUNTER..=mem_end).contains(&addr) {
            if do_write {
                let mem: &mut [u8] = cast_slice_mut(&mut self.mem);
                mem[((addr - RESET_PROGRAM_COUNTER) >> 0) as usize] = u8::to_le(value);
            }

            Ok(do_write)
        } else {
            Err(MemoryError::AccessViolation)
        }
    }
}

struct TestIo;

impl IoInterface for TestIo {
    fn read(&mut self, _addr: u32, _priv_level: PrivilegeLevel) -> Result<u32, IoError> {
        Err(IoError::AccessViolation)
    }

    fn write(
        &mut self,
        _addr: u32,
        _value: u32,
        _priv_level: PrivilegeLevel,
    ) -> Result<(), IoError> {
        Err(IoError::AccessViolation)
    }
}
