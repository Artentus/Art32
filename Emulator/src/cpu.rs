mod register;
use register::*;

pub mod interface;
use interface::*;
use strum::{EnumCount, IntoEnumIterator};

#[cfg(test)]
mod tests;

use crate::{shuffle_bits, Ashr};

const HARD_INT_SLOTS: usize = 16;
const SOFT_INT_SLOTS: usize = 16;

const ILLEGAL_INSTRUCTION_SLOT: usize = 0;
const ACCESS_VIOLATION_SLOT: usize = 1;
const EXCEPTION_SLOTS: usize = 2;

const HARD_INT_TABLE_START: u32 = 0x000;
const HARD_INT_TABLE_END: u32 = HARD_INT_TABLE_START + (HARD_INT_SLOTS as u32) - 1;
const SOFT_INT_TABLE_START: u32 = 0x010;
const SOFT_INT_TABLE_END: u32 = SOFT_INT_TABLE_START + (SOFT_INT_SLOTS as u32) - 1;
const EXCEPTION_TABLE_START: u32 = 0x020;
const EXCEPTION_TABLE_END: u32 = EXCEPTION_TABLE_START + (EXCEPTION_SLOTS as u32) - 1;
const INT_CONFIG_START: u32 = 0x030;
const INT_MASK_ADDR: u32 = INT_CONFIG_START + 0;
const INT_PENDING_ADDR: u32 = INT_CONFIG_START + 1;
const PRIV_LEVEL_ADDR: u32 = INT_CONFIG_START + 2;
const INT_RET_ADDR: u32 = INT_CONFIG_START + 3;
const ALT_REGS_START: u32 = 0x040;
const ALT_REGS_END: u32 = ALT_REGS_START + (Register::COUNT as u32) - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InterruptState {
    Servicing,
    Listening,
}

const RESET_PROGRAM_COUNTER: u32 = crate::system::KERNEL_RAM_START;
const RESET_INTERRUPT_STATE: InterruptState = InterruptState::Servicing;
const RESET_PRIVILEGE_LEVEL: PrivilegeLevel = PrivilegeLevel::System;

#[derive(Debug, Default)]
struct CpuState {
    regs: RegisterFile,
    flags: Flags,
}

#[inline]
fn carry_add(lhs: u32, rhs: u32, c_in: bool) -> (u32, bool) {
    let (r1, c1) = lhs.overflowing_add(rhs);
    let (r2, c2) = r1.overflowing_add(c_in as u32);
    (r2, c1 | c2)
}

#[derive(Debug)]
pub struct Cpu {
    program_counter: u32,
    interrupt_state: InterruptState,
    privilege_level: PrivilegeLevel,
    state: Box<CpuState>,
    alt_state: Box<CpuState>,
    interrupt_mask: u16,
    pending_interrupts: u16,
    hardware_interrupt_table: [u32; HARD_INT_SLOTS],
    software_interrupt_table: [u32; SOFT_INT_SLOTS],
    exception_table: [u32; EXCEPTION_SLOTS],
    interrupt_return_address: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            program_counter: RESET_PROGRAM_COUNTER,
            interrupt_state: RESET_INTERRUPT_STATE,
            privilege_level: RESET_PRIVILEGE_LEVEL,
            state: Default::default(),
            alt_state: Default::default(),
            interrupt_mask: 0,
            pending_interrupts: 0,
            hardware_interrupt_table: Default::default(),
            software_interrupt_table: Default::default(),
            exception_table: Default::default(),
            interrupt_return_address: 0,
        }
    }

    pub fn reset(&mut self) {
        self.program_counter = RESET_PROGRAM_COUNTER;
        self.interrupt_state = RESET_INTERRUPT_STATE;
        self.privilege_level = RESET_PRIVILEGE_LEVEL;
        self.interrupt_mask = 0;
        self.pending_interrupts = 0;
    }

    pub fn signal_interrupt(&mut self, slot: usize) {
        debug_assert!(slot < HARD_INT_SLOTS);
        self.pending_interrupts |= 1 << slot;
    }

    pub fn draw_debug_info(
        &self,
        wgpu_state: &crate::display::WgpuState,
        render_target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        text_renderer: &mut crate::display::TextRenderer,
    ) {
        let pc_text = format!("pc: 0x{:0>8X}", self.program_counter);
        text_renderer.draw_text(
            wgpu_state,
            render_target,
            encoder,
            &pc_text,
            crate::display::Vec2f::new(10.0, 8.0),
            16.0,
            [255; 4],
        );

        for (i, reg) in Register::iter().skip(1).enumerate() {
            let reg_text = format!("{reg}: 0x{:0>8X}", self.get_reg(reg));
            text_renderer.draw_text(
                wgpu_state,
                render_target,
                encoder,
                &reg_text,
                crate::display::Vec2f::new(10.0, 34.0 + 18.0 * (i as f32)),
                16.0,
                [255; 4],
            );
        }
    }

    #[inline]
    fn get_reg(&self, reg: Register) -> u32 {
        self.state.regs.get(reg)
    }

    #[inline]
    fn set_reg(&mut self, reg: Register, value: u32) {
        self.state.regs.set(reg, value);
    }

    fn effective_privilege_level(&self) -> PrivilegeLevel {
        match self.interrupt_state {
            InterruptState::Servicing => PrivilegeLevel::System,
            InterruptState::Listening => self.privilege_level,
        }
    }

    fn next_interrupt(&mut self) -> Option<usize> {
        if self.interrupt_state == InterruptState::Listening {
            let pending = self.pending_interrupts & self.interrupt_mask;
            if pending != 0 {
                let slot = HARD_INT_SLOTS - (pending.leading_zeros() as usize) - 1;
                self.pending_interrupts &= !(1 << slot);
                return Some(slot);
            }
        }

        None
    }

    fn enter_interrupt(&mut self, new_program_counter: u32) {
        debug_assert_eq!(self.interrupt_state, InterruptState::Listening);
        debug_assert_eq!(new_program_counter & 0x1, 0);

        self.interrupt_return_address = self.program_counter;
        self.program_counter = new_program_counter;
        self.interrupt_state = InterruptState::Servicing;
        std::mem::swap(&mut self.state, &mut self.alt_state);
    }

    fn leave_interrupt(&mut self) {
        debug_assert_eq!(self.interrupt_state, InterruptState::Servicing);
        debug_assert_eq!(self.interrupt_return_address & 0x1, 0);

        self.program_counter = self.interrupt_return_address;
        self.interrupt_state = InterruptState::Listening;
        std::mem::swap(&mut self.state, &mut self.alt_state);
    }

    fn illegal_instruction(&mut self) {
        match self.interrupt_state {
            InterruptState::Servicing => {
                panic!("illegal instruction exception inside interrupt handler")
            }
            InterruptState::Listening => {
                self.enter_interrupt(self.exception_table[ILLEGAL_INSTRUCTION_SLOT]);
            }
        }
    }

    fn access_violation(&mut self) {
        match self.interrupt_state {
            InterruptState::Servicing => {
                panic!("access violation exception inside interrupt handler")
            }
            InterruptState::Listening => {
                self.enter_interrupt(self.exception_table[ACCESS_VIOLATION_SLOT]);
            }
        }
    }

    fn read_io<Io: IoInterface>(
        &mut self,
        io: &mut Io,
        addr: u32,
        priv_level: PrivilegeLevel,
    ) -> Result<u32, ()> {
        if priv_level == PrivilegeLevel::System {
            match addr {
                HARD_INT_TABLE_START..=HARD_INT_TABLE_END => {
                    return Ok(
                        self.hardware_interrupt_table[(addr - HARD_INT_TABLE_START) as usize]
                    );
                }
                SOFT_INT_TABLE_START..=SOFT_INT_TABLE_END => {
                    return Ok(
                        self.software_interrupt_table[(addr - SOFT_INT_TABLE_START) as usize]
                    );
                }
                EXCEPTION_TABLE_START..=EXCEPTION_TABLE_END => {
                    return Ok(self.exception_table[(addr - EXCEPTION_TABLE_START) as usize]);
                }
                INT_MASK_ADDR => {
                    return Ok(self.interrupt_mask as u32);
                }
                INT_PENDING_ADDR => {
                    return Ok(self.pending_interrupts as u32);
                }
                PRIV_LEVEL_ADDR => {
                    return Ok(self.privilege_level.into());
                }
                INT_RET_ADDR => {
                    return Ok(self.interrupt_return_address);
                }
                ALT_REGS_START..=ALT_REGS_END => {
                    let reg = Register::try_from(addr - ALT_REGS_START).unwrap();
                    return Ok(self.alt_state.regs.get(reg));
                }
                _ => {}
            }
        }

        io.read(addr, priv_level)
    }

    fn write_io<Io: IoInterface>(
        &mut self,
        io: &mut Io,
        addr: u32,
        value: u32,
        priv_level: PrivilegeLevel,
    ) -> Result<(), ()> {
        if priv_level == PrivilegeLevel::System {
            match addr {
                HARD_INT_TABLE_START..=HARD_INT_TABLE_END => {
                    self.hardware_interrupt_table[(addr - HARD_INT_TABLE_START) as usize] =
                        value & !0x1;
                    return Ok(());
                }
                SOFT_INT_TABLE_START..=SOFT_INT_TABLE_END => {
                    self.software_interrupt_table[(addr - SOFT_INT_TABLE_START) as usize] =
                        value & !0x1;
                    return Ok(());
                }
                EXCEPTION_TABLE_START..=EXCEPTION_TABLE_END => {
                    self.exception_table[(addr - EXCEPTION_TABLE_START) as usize] = value & !0x1;
                    return Ok(());
                }
                INT_MASK_ADDR => {
                    self.interrupt_mask = value as u16;
                    return Ok(());
                }
                INT_PENDING_ADDR => {
                    self.pending_interrupts = value as u16;
                    return Ok(());
                }
                PRIV_LEVEL_ADDR => {
                    self.privilege_level =
                        (value & 0x1).try_into().expect("invalid priviledge level");
                    return Ok(());
                }
                INT_RET_ADDR => {
                    self.interrupt_return_address = value & !0x1;
                    return Ok(());
                }
                ALT_REGS_START..=ALT_REGS_END => {
                    let reg = Register::try_from(addr - ALT_REGS_START).unwrap();
                    self.alt_state.regs.set(reg, value);
                    return Ok(());
                }
                _ => {}
            }
        }

        io.write(addr, value, priv_level)
    }

    fn execute_add(&mut self, lhs: u32, rhs: u32, c_in: bool) -> u32 {
        let lhs_sign = (lhs as i32) < 0;
        let rhs_sign = (rhs as i32) < 0;

        let (result, c_out) = carry_add(lhs, rhs, c_in);
        let result_sign = (result as i32) < 0;

        self.state.flags.set(Flags::CARRY, c_out);
        self.state.flags.set(Flags::SIGN, result_sign);
        self.state.flags.set(
            Flags::OVERFLOW,
            (lhs_sign == rhs_sign) & (lhs_sign != result_sign),
        );

        result
    }

    pub fn step<Mem: MemoryInterface, Io: IoInterface>(
        &mut self,
        mem: &mut Mem,
        io: &mut Io,
    ) -> Option<u8> {
        if let Some(slot) = self.next_interrupt() {
            self.enter_interrupt(self.hardware_interrupt_table[slot]);
            return None;
        }

        debug_assert_eq!(self.program_counter & 0x1, 0);
        let priv_level = self.effective_privilege_level();
        let Ok(lower_inst) = mem.read_16(self.program_counter, priv_level, false) else {
            self.access_violation();
            return None;
        };

        let instruction = lower_inst as u32;
        self.program_counter = self.program_counter.wrapping_add(2);

        // Instruction set:
        // https://docs.google.com/spreadsheets/d/1VGV9Hp17HtE5oG_ltB0xSQ0j2AvfDYW9LLvOI28e6qM/edit?usp=sharing

        if (instruction & 0x1) == 0 {
            // ldi, addi

            let rd_rs1 =
                Register::try_from(shuffle_bits!(instruction { [15:12] => [3:0] })).unwrap();

            let imm = shuffle_bits!(instruction {
                [11:7] => [4:0],
                [6:4] => [8:6],
                sign [3] => [9],
                [2] => [5],
            });

            if (instruction & 0x2) == 0 {
                self.set_reg(rd_rs1, imm);
            } else {
                let sum = self.execute_add(self.get_reg(rd_rs1), imm, false);
                self.state.flags.set(Flags::ZERO, sum == 0);
                self.set_reg(rd_rs1, sum);
            }
        } else if (instruction & 0x2) == 0 {
            // j, jl

            let rb = Register::try_from(shuffle_bits!(instruction { [15:12] => [3:0] })).unwrap();

            let imm = shuffle_bits!(instruction {
                [11:8] => [4:1],
                [7] => [5],
                [6:4] => [8:6],
                sign [3] => [9],
            });

            let jump_addr = self.get_reg(rb).wrapping_add(imm) & !0x1;
            if (instruction & 0x4) != 0 {
                self.set_reg(Register::Ra, self.program_counter);
            }
            self.program_counter = jump_addr;
        } else if (instruction & 0x4) == 0 {
            if (instruction & 0x80) == 0 {
                // br, jr, jrl

                let cond =
                    BranchCondition::try_from(shuffle_bits!(instruction { [14:12] => [2:0] }))
                        .unwrap();

                let imm = shuffle_bits!(instruction {
                    [15] => [5],
                    [11:8] => [4:1],
                    [6:4] => [8:6],
                    sign [3] => [9],
                });

                if cond == BranchCondition::Link {
                    self.set_reg(Register::Ra, self.program_counter);
                }

                if self.state.flags.satisfy_branch(cond) {
                    self.program_counter = self.program_counter.wrapping_add(imm) & !0x1;
                }
            } else {
                let Ok(upper_inst) = mem.read_16(self.program_counter, priv_level, false) else {
                    self.access_violation();
                    return None;
                };

                let instruction = instruction | ((upper_inst as u32) << 16);
                self.program_counter = self.program_counter.wrapping_add(2);

                // ldui, apcui

                let rd =
                    Register::try_from(shuffle_bits!(instruction { [16:12] => [4:0] })).unwrap();

                let imm = shuffle_bits!(instruction {
                    sign [31] => [31],
                    [30:27] => [30:27],
                    [26:24] => [12:10],
                    [23:22] => [14:13],
                    [21:17] => [19:15],
                    [11:8] => [26:23],
                    [6:4] => [22:20],
                });

                if (instruction & 0x8) == 0 {
                    self.set_reg(rd, imm);
                } else {
                    self.set_reg(rd, self.program_counter.wrapping_add(imm));
                }
            }
        } else {
            match (instruction & 0x18) >> 3 {
                0b00 => {
                    // alu

                    let rd_rs1 =
                        Register::try_from(shuffle_bits!(instruction { [15:12] => [3:0] }))
                            .unwrap();
                    let rs2 =
                        Register::try_from(shuffle_bits!(instruction { [11:8] => [3:0] })).unwrap();

                    let lhs = self.get_reg(rd_rs1);
                    let rhs = self.get_reg(rs2);
                    let result = match (instruction & 0xE0) >> 5 {
                        0b000 /* add */ => {
                            self.execute_add(lhs, rhs, false)
                        }
                        0b001 /* sub */ => {
                            self.execute_add(lhs, !rhs, true)
                        }
                        0b010 /* and */ => {
                            lhs & rhs
                        }
                        0b011 /* or */ => {
                            lhs | rhs
                        }
                        0b100 /* xor */ => {
                            lhs ^ rhs
                        }
                        0b101 /* shl */ => {
                            lhs << (rhs & 0x1F)
                        }
                        0b110 /* lsr */ => {
                            lhs >> (rhs & 0x1F)
                        }
                        0b111 /* asr */ => {
                            lhs.ashr(rhs & 0x1F)
                        }
                        _ => unreachable!(),
                    };

                    self.set_reg(rd_rs1, result);
                    self.state.flags.set(Flags::ZERO, result == 0);
                }
                0b01 => {
                    // mov

                    let rd_rs1 =
                        Register::try_from(shuffle_bits!(instruction { [15:12] => [3:0] }))
                            .unwrap();
                    let rs2 =
                        Register::try_from(shuffle_bits!(instruction { [11:8] => [3:0] })).unwrap();
                    let cond =
                        Condition::try_from(shuffle_bits!(instruction { [7:5] => [2:0] })).unwrap();

                    if self.state.flags.satisfy(cond) {
                        let value = self.get_reg(rs2);
                        self.set_reg(rd_rs1, value);
                    };
                }
                0b10 => {
                    if (instruction & 0x60) == 0 {
                        if (instruction & 0x80) == 0 {
                            // cmp

                            let rs1 =
                                Register::try_from(shuffle_bits!(instruction { [15:12] => [3:0] }))
                                    .unwrap();
                            let rs2 =
                                Register::try_from(shuffle_bits!(instruction { [11:8] => [3:0] }))
                                    .unwrap();

                            let lhs = self.get_reg(rs1);
                            let rhs = self.get_reg(rs2);
                            let result = self.execute_add(lhs, !rhs, true);
                            self.state.flags.set(Flags::ZERO, result == 0);
                        } else {
                            match (instruction & 0xF00) >> 8 {
                                0b0000 => {
                                    // ret

                                    self.program_counter = self.get_reg(Register::Ra) & !0x1;
                                }
                                0b0001 => {
                                    // sysret

                                    match self.interrupt_state {
                                        InterruptState::Servicing => {
                                            self.leave_interrupt();
                                        }
                                        InterruptState::Listening => {
                                            self.illegal_instruction();
                                        }
                                    }
                                }
                                0b0010 => {
                                    // fence
                                }
                                0b0011 => {
                                    // ifence
                                }
                                0b0100..=0b1101 => {
                                    self.illegal_instruction();
                                }
                                0b1110 => {
                                    // envcall

                                    let code = shuffle_bits!(instruction { [15:12] => [3:0] });
                                    return Some(code as u8);
                                }
                                0b1111 => {
                                    // syscall

                                    match self.interrupt_state {
                                        InterruptState::Servicing => {
                                            panic!("software interrupt inside interrupt handler")
                                        }
                                        InterruptState::Listening => {
                                            let slot = shuffle_bits!(instruction { [15:12] => [3:0] })
                                                as usize;
                                            self.enter_interrupt(
                                                self.software_interrupt_table[slot],
                                            );
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                    } else {
                        // shli, lsri, asri

                        let rd_rs1 =
                            Register::try_from(shuffle_bits!(instruction { [15:12] => [3:0] }))
                                .unwrap();

                        let imm = shuffle_bits!(instruction {
                            [11:7] => [4:0],
                        });

                        let lhs = self.get_reg(rd_rs1);
                        let result = match (instruction & 0x60) >> 5 {
                            0b01 /* shli */ => lhs << imm,
                            0b10 /* lsri */ => lhs >> imm,
                            0b11 /* asri */ => lhs.ashr(imm),
                            _ => unreachable!(),
                        };

                        self.set_reg(rd_rs1, result);
                        self.state.flags.set(Flags::ZERO, result == 0);
                    }
                }
                0b11 => {
                    if (instruction & 0x20) == 0 {
                        // ld.32 [sp], st.32 [sp]

                        let rd_rs =
                            Register::try_from(shuffle_bits!(instruction { [15:12] => [3:0] }))
                                .unwrap();

                        let imm = shuffle_bits!(instruction {
                            [11:9] => [4:2],
                            [8:7] => [6:5],
                        });

                        let addr = self.get_reg(Register::Sp).wrapping_add(imm);
                        let result = if (instruction & 0x40) == 0 {
                            mem.read_32(addr, priv_level, false)
                                .map(|value| self.set_reg(rd_rs, value))
                        } else {
                            let value = self.get_reg(rd_rs);
                            mem.write_32(addr, value, priv_level, false).map(|_| ())
                        };

                        if result.is_err() {
                            self.access_violation();
                        }
                    } else {
                        let Ok(upper_inst) = mem.read_16(self.program_counter, priv_level, false)
                        else {
                            self.access_violation();
                            return None;
                        };

                        let instruction = instruction | ((upper_inst as u32) << 16);
                        self.program_counter = self.program_counter.wrapping_add(2);

                        match (instruction & 0xC0_0000) >> 22 {
                            0b00 => {
                                if (instruction & 0x40) == 0 {
                                    // jl

                                    let rd = Register::try_from(
                                        shuffle_bits!(instruction { [16:12] => [4:0] }),
                                    )
                                    .unwrap();
                                    let rb = Register::try_from(
                                        shuffle_bits!(instruction { [21:17] => [4:0] }),
                                    )
                                    .unwrap();

                                    let imm = shuffle_bits!(instruction {
                                        sign [31] => [13],
                                        [30:27] => [8:5],
                                        [26:24] => [12:10],
                                        [11:8] => [4:1],
                                        [7] => [9],
                                    });

                                    let jump_addr = self.get_reg(rb).wrapping_add(imm) & !0x1;
                                    self.set_reg(rd, self.program_counter);
                                    self.program_counter = jump_addr;
                                } else {
                                    // br, jr, jrl

                                    let cond = BranchCondition::try_from(
                                        shuffle_bits!(instruction { [14:12] => [2:0] }),
                                    )
                                    .unwrap();

                                    let imm = shuffle_bits!(instruction {
                                        sign [31] => [20],
                                        [30:27] => [8:5],
                                        [26:24] => [12:10],
                                        [21:15] => [19:13],
                                        [11:8] => [4:1],
                                        [7] => [9],
                                    });

                                    if cond == BranchCondition::Link {
                                        self.set_reg(Register::Ra, self.program_counter);
                                    }

                                    if self.state.flags.satisfy_branch(cond) {
                                        self.program_counter =
                                            self.program_counter.wrapping_add(imm) & !0x1;
                                    }
                                }
                            }
                            0b01 => {
                                let rd = Register::try_from(
                                    shuffle_bits!(instruction { [16:12] => [4:0] }),
                                )
                                .unwrap();
                                let rs1 = Register::try_from(
                                    shuffle_bits!(instruction { [21:17] => [4:0] }),
                                )
                                .unwrap();

                                let imm = shuffle_bits!(instruction {
                                    sign [31] => [9],
                                    [30:27] => [8:5],
                                    [11:7] => [4:0],
                                });

                                if (instruction & 0x40) == 0 {
                                    // alui

                                    let lhs = self.get_reg(rs1);
                                    let result = match (instruction & 0x700_0000) >> 24 {
                                        0b000 /* addi */ => {
                                            self.execute_add(lhs, imm, false)
                                        }
                                        0b001 /* subi */ => {
                                            self.execute_add(lhs, !imm, true)
                                        }
                                        0b010 /* andi */ => {
                                            lhs & imm
                                        }
                                        0b011 /* ori */ => {
                                            lhs | imm
                                        }
                                        0b100 /* xori */ => {
                                            lhs ^ imm
                                        }
                                        0b101 /* shli */ => {
                                            lhs << (imm & 0x1F)
                                        }
                                        0b110 /* lsri */ => {
                                            lhs >> (imm & 0x1F)
                                        }
                                        0b111 /* asri */ => {
                                            lhs.ashr(imm & 0x1F)
                                        }
                                        _ => unreachable!(),
                                    };

                                    self.set_reg(rd, result);
                                    self.state.flags.set(Flags::ZERO, result == 0);
                                } else {
                                    // movi

                                    let cond = Condition::try_from(
                                        shuffle_bits!(instruction { [26:24] => [2:0] }),
                                    )
                                    .unwrap();

                                    let value = if self.state.flags.satisfy(cond) {
                                        imm
                                    } else {
                                        self.get_reg(rs1)
                                    };
                                    self.set_reg(rd, value);
                                }
                            }
                            0b10 => {
                                let rb = Register::try_from(
                                    shuffle_bits!(instruction { [21:17] => [4:0] }),
                                )
                                .unwrap();

                                let imm = shuffle_bits!(instruction {
                                    sign [31] => [9],
                                    [30:27] => [8:5],
                                    [11:7] => [4:0],
                                });

                                let addr = self.get_reg(rb).wrapping_add(imm);
                                let result = if (instruction & 0x40) == 0 {
                                    // ld, in

                                    let rd = Register::try_from(
                                        shuffle_bits!(instruction { [16:12] => [4:0] }),
                                    )
                                    .unwrap();

                                    match (instruction & 0x700_0000) >> 24 {
                                        0b000 | 0b001 /* ld.32 */ => mem
                                            .read_32(addr, priv_level, false)
                                            .map(|value| self.set_reg(rd, value)),
                                        0b010 /* ld.8u */ => mem
                                            .read_8(addr, priv_level, false)
                                            .map(|value| self.set_reg(rd, value as u32)),
                                        0b011 /* ld.8s */ => mem.read_8(addr, priv_level, false).map(|value| {
                                            self.set_reg(rd, ((value as i8) as i32) as u32)
                                        }),
                                        0b100 /* ld.16u */ => mem
                                            .read_16(addr, priv_level, false)
                                            .map(|value| self.set_reg(rd, value as u32)),
                                        0b101 /* ld.16s */ => mem.read_16(addr, priv_level, false).map(|value| {
                                            self.set_reg(rd, ((value as i16) as i32) as u32)
                                        }),
                                        0b110 | 0b111 /* in */ => self
                                            .read_io(io, addr, priv_level)
                                            .map(|value| self.set_reg(rd, value)),
                                        _ => unreachable!(),
                                    }
                                } else {
                                    // st, out

                                    let rs = Register::try_from(
                                        shuffle_bits!(instruction { [11:8] => [3:0], [7] => [4] }),
                                    )
                                    .unwrap();

                                    match (instruction & 0x600_0000) >> 25 {
                                        0b00 /* st.32 */ => {
                                            let value = self.get_reg(rs);
                                            mem.write_32(addr, value, priv_level, false).map(|_| ())
                                        }
                                        0b01 /* st.8 */ => {
                                            let value = self.get_reg(rs) as u8;
                                            mem.write_8(addr, value, priv_level, false).map(|_| ())
                                        }
                                        0b10 /* st.16 */ => {
                                            let value = self.get_reg(rs) as u16;
                                            mem.write_16(addr, value, priv_level, false).map(|_| ())
                                        }
                                        0b11 /* out */ => {
                                            let value = self.get_reg(rs);
                                            self.write_io(io, addr, value, priv_level)
                                        }
                                        _ => unreachable!(),
                                    }
                                };

                                if result.is_err() {
                                    self.access_violation();
                                }
                            }
                            0b11 => {
                                let rs1_rb = Register::try_from(
                                    shuffle_bits!(instruction { [21:17] => [4:0] }),
                                )
                                .unwrap();

                                let rd = Register::try_from(
                                    shuffle_bits!(instruction { [16:12] => [4:0] }),
                                )
                                .unwrap();

                                match shuffle_bits!(instruction { [31:27] => [5:1], [6] => [0] }) {
                                    0b000000 /* alu */ => {
                                        let rs2 = Register::try_from(
                                            shuffle_bits!(instruction { [11:8] => [3:0], [7] => [4] }),
                                        )
                                        .unwrap();

                                        let lhs = self.get_reg(rs1_rb);
                                        let rhs = self.get_reg(rs2);
                                        let result = match (instruction & 0x700_0000) >> 24 {
                                            0b000 /* add */ => {
                                                self.execute_add(lhs, rhs, false)
                                            }
                                            0b001 /* sub */ => {
                                                self.execute_add(lhs, !rhs, true)
                                            }
                                            0b010 /* and */ => {
                                                lhs & rhs
                                            }
                                            0b011 /* or */ => {
                                                lhs | rhs
                                            }
                                            0b100 /* xor */ => {
                                                lhs ^ rhs
                                            }
                                            0b101 /* shl */ => {
                                                lhs << (rhs & 0x1F)
                                            }
                                            0b110 /* lsr */ => {
                                                lhs >> (rhs & 0x1F)
                                            }
                                            0b111 /* asr */ => {
                                                lhs.ashr(rhs & 0x1F)
                                            }
                                            _ => unreachable!(),
                                        };

                                        self.set_reg(rd, result);
                                        self.state.flags.set(Flags::ZERO, result == 0);
                                    }
                                    0b000001 /* mov */ => {
                                        let rs2 = Register::try_from(
                                            shuffle_bits!(instruction { [11:8] => [3:0], [7] => [4] }),
                                        )
                                        .unwrap();

                                        let cond = Condition::try_from(
                                            shuffle_bits!(instruction { [26:24] => [2:0] }),
                                        )
                                        .unwrap();

                                        let value = if self.state.flags.satisfy(cond) {
                                            self.get_reg(rs2)
                                        } else {
                                            self.get_reg(rs1_rb)
                                        };
                                        self.set_reg(rd, value);
                                    }
                                    0b000010 /* carry */ => {
                                        let rs2 = Register::try_from(
                                            shuffle_bits!(instruction { [11:8] => [3:0], [7] => [4] }),
                                        )
                                        .unwrap();

                                        let lhs = self.get_reg(rs1_rb);
                                        let rhs = self.get_reg(rs2);
                                        let c_in = self.state.flags.contains(Flags::CARRY);
                                        let result = match (instruction & 0x700_0000) >> 24 {
                                            0b000 /* addc */ => {
                                                self.execute_add(lhs, rhs, c_in)
                                            }
                                            0b001 /* subc */ => {
                                                self.execute_add(lhs, !rhs, c_in)
                                            }
                                            0b010..=0b111 => {
                                                self.illegal_instruction();
                                                return None;
                                            }
                                            _ => unreachable!(),
                                        };

                                        self.set_reg(rd, result);
                                        if result != 0 {
                                            self.state.flags.remove(Flags::ZERO);
                                        }
                                    }
                                    0b000011 /* mul, div */ => {
                                        let rs2 = Register::try_from(
                                            shuffle_bits!(instruction { [11:8] => [3:0], [7] => [4] }),
                                        )
                                        .unwrap();

                                        let lhs = self.get_reg(rs1_rb);
                                        let rhs = self.get_reg(rs2);
                                        match (instruction & 0x700_0000) >> 24 {
                                            0b000 /* mul */ => {
                                                let result = u32::wrapping_mul(lhs, rhs);
                                                self.set_reg(rd, result);
                                                self.state.flags.set(Flags::ZERO, result == 0);
                                            }
                                            0b001 /* mulhuu */ => {
                                                let result = (u64::wrapping_mul(lhs as u64, rhs as u64) >> 32) as u32;
                                                self.set_reg(rd, result);
                                                if result != 0 {
                                                    self.state.flags.remove(Flags::ZERO);
                                                }
                                            }
                                            0b010 /* mulhss */ => {
                                                let result = (i64::wrapping_mul((lhs as i32) as i64, (rhs as i32) as i64) >> 32) as u32;
                                                self.set_reg(rd, result);
                                                if result != 0 {
                                                    self.state.flags.remove(Flags::ZERO);
                                                }
                                            }
                                            0b011 /* mulhus */ => {
                                                let result = (i64::wrapping_mul((lhs as u64) as i64, (rhs as i32) as i64) >> 32) as u32;
                                                self.set_reg(rd, result);
                                                if result != 0 {
                                                    self.state.flags.remove(Flags::ZERO);
                                                }
                                            }
                                            0b100 /* divu */ => {
                                                let result = if rhs == 0 { u32::MAX } else { u32::wrapping_div(lhs, rhs) };
                                                self.set_reg(rd, result);
                                            }
                                            0b101 /* divs */ => {
                                                let result = if rhs == 0 {
                                                    if (lhs as i32) < 0 { i32::MIN } else { i32::MAX }
                                                } else {
                                                    i32::wrapping_div(lhs as i32, rhs as i32)
                                                } as u32;
                                                self.set_reg(rd, result);
                                            }
                                            0b110 /* remu */ => {
                                                let result = if rhs == 0 { 0 } else { u32::wrapping_rem(lhs, rhs) };
                                                self.set_reg(rd, result);
                                            }
                                            0b111 /* rems */ => {
                                                let result = if rhs == 0 { 0 } else { i32::wrapping_rem(lhs as i32, rhs as i32) as u32 };
                                                self.set_reg(rd, result);
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    0b000100 /* fpu3 */ => {
                                        let rs2 = Register::try_from(
                                            shuffle_bits!(instruction { [11:8] => [3:0], [7] => [4] }),
                                        )
                                        .unwrap();

                                        let lhs = f32::from_bits(self.get_reg(rs1_rb));
                                        let rhs = f32::from_bits(self.get_reg(rs2));
                                        let result = match (instruction & 0x700_0000) >> 24 {
                                            0b000 /* fadd */ => {
                                                lhs + rhs
                                            }
                                            0b001 /* fsub */ => {
                                                lhs - rhs
                                            }
                                            0b010 /* fmul */ => {
                                                lhs * rhs
                                            }
                                            0b011 /* fdiv */ => {
                                                lhs / rhs
                                            }
                                            0b100 /* frem */ => {
                                                lhs % rhs
                                            }
                                            0b101 => {
                                                self.illegal_instruction();
                                                return None;
                                            }
                                            0b110 /* fmin */ => {
                                                lhs.min(rhs)
                                            }
                                            0b111 /* fmax */ => {
                                                lhs.max(rhs)
                                            }
                                            _ => unreachable!(),
                                        };

                                        self.set_reg(rd, result.to_bits());
                                    }
                                    0b000101 /* fpu2 */ => {
                                        let value = f32::from_bits(self.get_reg(rs1_rb));
                                        let result = match (instruction & 0x700_0000) >> 24 {
                                            0b000 /* ffloor */ => {
                                                value.floor()
                                            }
                                            0b001 /* fceil */ => {
                                                value.ceil()
                                            }
                                            0b010 /* fround */ => {
                                                value.round()
                                            }
                                            0b011 /* ffract */ => {
                                                value.fract()
                                            }
                                            0b100 /* fabs */ => {
                                                value.abs()
                                            }
                                            0b101 /* fneg */ => {
                                                -value
                                            }
                                            0b110 /* fsqrt */ => {
                                                value.sqrt()
                                            }
                                            0b111 => {
                                                self.illegal_instruction();
                                                return None;
                                            }
                                            _ => unreachable!(),
                                        };

                                        self.set_reg(rd, result.to_bits());
                                    }
                                    0b000110 /* fcmp */ => {
                                        let rs2 = Register::try_from(
                                            shuffle_bits!(instruction { [11:8] => [3:0], [7] => [4] }),
                                        )
                                        .unwrap();

                                        let lhs = f32::from_bits(self.get_reg(rs1_rb));
                                        let rhs = f32::from_bits(self.get_reg(rs2));
                                        let result = match (instruction & 0x300_0000) >> 24 {
                                            0b00 /* eq */ => {
                                                lhs == rhs
                                            }
                                            0b01 /* ne */ => {
                                                lhs != rhs
                                            }
                                            0b10 /* lt */ => {
                                                lhs < rhs
                                            }
                                            0b11 /* ge */ => {
                                                lhs >= rhs
                                            }
                                            _ => unreachable!(),
                                        };

                                        self.set_reg(rd, result as u32);
                                    }
                                    0b000111 /* cvt */ => {
                                        let value = self.get_reg(rs1_rb);
                                        match (instruction & 0x700_0000) >> 24 {
                                            0b000 /* ftoi */ => {
                                                let result = f32::from_bits(value) as u32;
                                                self.set_reg(rd, result);
                                            }
                                            0b001 /* itof */ => {
                                                let result = value as f32;
                                                self.set_reg(rd, result.to_bits());
                                            }
                                            0b010..=0b111 => {
                                                self.illegal_instruction();
                                                return None;
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    0b001000 | 0b001010 | 0b001100 | 0b001110 /* ldr */ => {
                                        let addr = self.get_reg(rs1_rb);
                                        let result = match (instruction & 0x700_0000) >> 24 {
                                            0b000 | 0b001 /* ldr.32 */ => mem
                                                .read_32(addr, priv_level, true)
                                                .map(|value| self.set_reg(rd, value)),
                                            0b010 /* ldr.8u */ => mem
                                                .read_8(addr, priv_level, true)
                                                .map(|value| self.set_reg(rd, value as u32)),
                                            0b011 /* ldr.8s */ => mem.read_8(addr, priv_level, true).map(|value| {
                                                self.set_reg(rd, ((value as i8) as i32) as u32)
                                            }),
                                            0b100 /* ldr.16u */ => mem
                                                .read_16(addr, priv_level, true)
                                                .map(|value| self.set_reg(rd, value as u32)),
                                            0b101 /* ldr.16s */ => mem.read_16(addr, priv_level, true).map(|value| {
                                                self.set_reg(rd, ((value as i16) as i32) as u32)
                                            }),
                                            0b110 | 0b111 => {
                                                self.illegal_instruction();
                                                return None;
                                            }
                                            _ => unreachable!(),
                                        };

                                        if result.is_err() {
                                            self.access_violation();
                                        }
                                    }
                                    0b001001 | 0b001011 | 0b001101 | 0b001111 /* stc */ => {
                                        let rs = Register::try_from(
                                            shuffle_bits!(instruction { [11:8] => [3:0], [7] => [4] }),
                                        )
                                        .unwrap();

                                        let addr = self.get_reg(rs1_rb);
                                        let result = match (instruction & 0x600_0000) >> 25 {
                                            0b00 /* stc.32 */ => {
                                                let value = self.get_reg(rs);
                                                mem.write_32(addr, value, priv_level, true)
                                            }
                                            0b01 /* stc.8 */ => {
                                                let value = self.get_reg(rs) as u8;
                                                mem.write_8(addr, value, priv_level, true)
                                            }
                                            0b10 /* stc.16 */ => {
                                                let value = self.get_reg(rs) as u16;
                                                mem.write_16(addr, value, priv_level, true)
                                            }
                                            0b11 => {
                                                self.illegal_instruction();
                                                return None;
                                            }
                                            _ => unreachable!(),
                                        };

                                        match result {
                                            Ok(value) => self.set_reg(rd, value as u32),
                                            Err(_) => self.access_violation(),
                                        }
                                    }
                                    _ => self.illegal_instruction(),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        None
    }
}
