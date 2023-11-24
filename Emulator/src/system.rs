use crate::cpu::interface::*;
use crate::cpu::Cpu;
use crate::memory::Memory;
use std::collections::VecDeque;

const KERNEL_RAM_SIZE: u32 = 0x0000_8000; // 32kB
pub const KERNEL_RAM_START: u32 = 0x1000_0000;
const KERNEL_RAM_END: u32 = KERNEL_RAM_START + KERNEL_RAM_SIZE - 1;

const SYSTEM_RAM_SIZE: u32 = 0x0010_0000; // 1MB
const SYSTEM_RAM_START: u32 = 0x2000_0000;
const SYSTEM_RAM_END: u32 = SYSTEM_RAM_START + SYSTEM_RAM_SIZE - 1;

const KERNEL: &'static [u8; KERNEL_RAM_SIZE as usize] = include_bytes!("../kernel/kernel.bin");

#[derive(Debug, Default)]
#[repr(transparent)]
struct Reservation {
    addr: Option<u32>,
}

impl Reservation {
    #[inline]
    fn reset(&mut self) {
        self.addr = None;
    }

    #[inline]
    fn take(&mut self, read_addr: u32) {
        self.addr = Some(read_addr & !0x3);
    }

    #[inline]
    fn check_write(&mut self, write_addr: u32) -> bool {
        if self.addr == Some(write_addr & !0x3) {
            self.addr = None;
            true
        } else {
            false
        }
    }
}

pub struct Mmu<'a> {
    kernel_ram: &'a mut Memory,
    system_ram: &'a mut Memory,
    reservation: &'a mut Reservation,
}

impl MemoryInterface for Mmu<'_> {
    fn read_32(
        &mut self,
        addr: u32,
        priv_level: PrivilegeLevel,
        reserve: bool,
    ) -> Result<u32, MemoryError> {
        if (addr & 0x3) != 0 {
            return Err(MemoryError::UnalignedAccess);
        }

        if reserve {
            self.reservation.take(addr);
        }

        match addr {
            KERNEL_RAM_START..=KERNEL_RAM_END if priv_level == PrivilegeLevel::System => {
                Ok(self.kernel_ram.read_32(addr - KERNEL_RAM_START))
            }
            SYSTEM_RAM_START..=SYSTEM_RAM_END => {
                Ok(self.system_ram.read_32(addr - SYSTEM_RAM_START))
            }
            _ => Err(MemoryError::AccessViolation),
        }
    }

    fn read_16(
        &mut self,
        addr: u32,
        priv_level: PrivilegeLevel,
        reserve: bool,
    ) -> Result<u16, MemoryError> {
        if (addr & 0x1) != 0 {
            return Err(MemoryError::UnalignedAccess);
        }

        if reserve {
            self.reservation.take(addr);
        }

        match addr {
            KERNEL_RAM_START..=KERNEL_RAM_END if priv_level == PrivilegeLevel::System => {
                Ok(self.kernel_ram.read_16(addr - KERNEL_RAM_START))
            }
            SYSTEM_RAM_START..=SYSTEM_RAM_END => {
                Ok(self.system_ram.read_16(addr - SYSTEM_RAM_START))
            }
            _ => Err(MemoryError::AccessViolation),
        }
    }

    fn read_8(
        &mut self,
        addr: u32,
        priv_level: PrivilegeLevel,
        reserve: bool,
    ) -> Result<u8, MemoryError> {
        if reserve {
            self.reservation.take(addr);
        }

        match addr {
            KERNEL_RAM_START..=KERNEL_RAM_END if priv_level == PrivilegeLevel::System => {
                Ok(self.kernel_ram.read_8(addr - KERNEL_RAM_START))
            }
            SYSTEM_RAM_START..=SYSTEM_RAM_END => {
                Ok(self.system_ram.read_8(addr - SYSTEM_RAM_START))
            }
            _ => Err(MemoryError::AccessViolation),
        }
    }

    fn write_32(
        &mut self,
        addr: u32,
        value: u32,
        priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, MemoryError> {
        if (addr & 0x3) != 0 {
            return Err(MemoryError::UnalignedAccess);
        }

        let is_reserved = self.reservation.check_write(addr);
        let do_write = is_reserved | !conditional;

        match addr {
            KERNEL_RAM_START..=KERNEL_RAM_END if priv_level == PrivilegeLevel::System => {
                if do_write {
                    self.kernel_ram.write_32(addr - KERNEL_RAM_START, value);
                }

                Ok(do_write)
            }
            SYSTEM_RAM_START..=SYSTEM_RAM_END => {
                if do_write {
                    self.system_ram.write_32(addr - SYSTEM_RAM_START, value);
                }

                Ok(do_write)
            }
            _ => Err(MemoryError::AccessViolation),
        }
    }

    fn write_16(
        &mut self,
        addr: u32,
        value: u16,
        priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, MemoryError> {
        if (addr & 0x1) != 0 {
            return Err(MemoryError::UnalignedAccess);
        }

        let is_reserved = self.reservation.check_write(addr);
        let do_write = is_reserved | !conditional;

        match addr {
            KERNEL_RAM_START..=KERNEL_RAM_END if priv_level == PrivilegeLevel::System => {
                if do_write {
                    self.kernel_ram.write_16(addr - KERNEL_RAM_START, value);
                }

                Ok(do_write)
            }
            SYSTEM_RAM_START..=SYSTEM_RAM_END => {
                if do_write {
                    self.system_ram.write_16(addr - SYSTEM_RAM_START, value);
                }

                Ok(do_write)
            }
            _ => Err(MemoryError::AccessViolation),
        }
    }

    fn write_8(
        &mut self,
        addr: u32,
        value: u8,
        priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, MemoryError> {
        let is_reserved = self.reservation.check_write(addr);
        let do_write = is_reserved | !conditional;

        match addr {
            KERNEL_RAM_START..=KERNEL_RAM_END if priv_level == PrivilegeLevel::System => {
                if do_write {
                    self.kernel_ram.write_8(addr - KERNEL_RAM_START, value);
                }

                Ok(do_write)
            }
            SYSTEM_RAM_START..=SYSTEM_RAM_END => {
                if do_write {
                    self.system_ram.write_8(addr - SYSTEM_RAM_START, value);
                }

                Ok(do_write)
            }
            _ => Err(MemoryError::AccessViolation),
        }
    }
}

const TIMER_LOW_ADDR: u32 = 0x080;
const TIMER_HIGH_ADDR: u32 = 0x081;
const TIMER_ACCURACY_ADDR: u32 = 0x082;

const SERIAL_OUT_DATA_ADDR: u32 = 0x90;
const SERIAL_OUT_COUNT_ADDR: u32 = 0x91;
const SERIAL_IN_DATA_ADDR: u32 = 0x92;
const SERIAL_IN_COUNT_ADDR: u32 = 0x93;

pub struct IoBus<'a> {
    start_time: &'a std::time::Instant,
    serial_buffer: &'a mut VecDeque<u8>,
}

impl IoInterface for IoBus<'_> {
    fn read(&mut self, addr: u32, priv_level: PrivilegeLevel) -> Result<u32, IoError> {
        match addr {
            TIMER_LOW_ADDR => Ok(self.start_time.elapsed().as_nanos() as u32),
            TIMER_HIGH_ADDR => Ok((self.start_time.elapsed().as_nanos() >> 32) as u32),
            TIMER_ACCURACY_ADDR => Ok(1),

            SERIAL_OUT_DATA_ADDR => Err(IoError::AccessViolation),
            SERIAL_OUT_COUNT_ADDR => Ok(u32::MAX),
            SERIAL_IN_DATA_ADDR => Ok(self.serial_buffer.pop_front().unwrap_or(0) as u32),
            SERIAL_IN_COUNT_ADDR => Ok(self.serial_buffer.len() as u32),

            _ => Err(IoError::AccessViolation),
        }
    }

    fn write(&mut self, addr: u32, value: u32, priv_level: PrivilegeLevel) -> Result<(), IoError> {
        match addr {
            TIMER_LOW_ADDR => Err(IoError::AccessViolation),
            TIMER_HIGH_ADDR => Err(IoError::AccessViolation),
            TIMER_ACCURACY_ADDR => Err(IoError::AccessViolation),

            SERIAL_OUT_DATA_ADDR => {
                let value = value as u8;
                if let Some(c) = char::from_u32(value as u32) {
                    print!("{c}");
                }
                Ok(())
            }
            SERIAL_OUT_COUNT_ADDR => Err(IoError::AccessViolation),
            SERIAL_IN_DATA_ADDR => Err(IoError::AccessViolation),
            SERIAL_IN_COUNT_ADDR => Err(IoError::AccessViolation),

            _ => Err(IoError::AccessViolation),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvAction {
    Break,
    Reset,
    Error,
}

impl EnvAction {
    const fn new(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Break),
            1 => Some(Self::Reset),
            2 => Some(Self::Error),
            _ => None,
        }
    }
}

pub struct Art32 {
    cpu: Cpu,
    kernel_ram: Memory,
    system_ram: Memory,
    start_time: std::time::Instant,
    serial_buffer: VecDeque<u8>,
    reservation: Reservation,
}

impl Art32 {
    pub fn new() -> Self {
        let mut kernel_ram = Memory::new(KERNEL_RAM_SIZE);
        kernel_ram.reset(KERNEL);

        Self {
            cpu: Cpu::new(),
            kernel_ram,
            system_ram: Memory::new(SYSTEM_RAM_SIZE),
            start_time: std::time::Instant::now(),
            serial_buffer: VecDeque::new(),
            reservation: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.kernel_ram.reset(KERNEL);
        self.reservation.reset();
    }

    pub fn draw_debug_info(
        &self,
        wgpu_state: &crate::display::WgpuState,
        render_target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        text_renderer: &mut crate::display::TextRenderer,
    ) {
        self.cpu
            .draw_debug_info(wgpu_state, render_target, encoder, text_renderer);
    }

    pub fn step(&mut self) -> Option<EnvAction> {
        let mut mmu = Mmu {
            kernel_ram: &mut self.kernel_ram,
            system_ram: &mut self.system_ram,
            reservation: &mut self.reservation,
        };

        let mut io_bus = IoBus {
            start_time: &self.start_time,
            serial_buffer: &mut self.serial_buffer,
        };

        self.cpu
            .step(&mut mmu, &mut io_bus)
            .and_then(EnvAction::new)
    }
}
