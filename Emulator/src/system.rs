use crate::cpu::interface::*;
use crate::cpu::Cpu;
use crate::memory::Memory;

const KERNEL_RAM_SIZE: u32 = 0x0000_8000; // 32kB
pub const KERNEL_RAM_START: u32 = 0x1000_0000;
const KERNEL_RAM_END: u32 = KERNEL_RAM_START + KERNEL_RAM_SIZE - 1;

const SYSTEM_RAM_SIZE: u32 = 0x0010_0000; // 1MB
const SYSTEM_RAM_START: u32 = 0x2000_0000;
const SYSTEM_RAM_END: u32 = SYSTEM_RAM_START + SYSTEM_RAM_SIZE - 1;

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
    fn read_32(&mut self, addr: u32, priv_level: PrivilegeLevel, reserve: bool) -> Result<u32, ()> {
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
            _ => Err(()),
        }
    }

    fn read_16(&mut self, addr: u32, priv_level: PrivilegeLevel, reserve: bool) -> Result<u16, ()> {
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
            _ => Err(()),
        }
    }

    fn read_8(&mut self, addr: u32, priv_level: PrivilegeLevel, reserve: bool) -> Result<u8, ()> {
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
            _ => Err(()),
        }
    }

    fn write_32(
        &mut self,
        addr: u32,
        value: u32,
        priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, ()> {
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
            _ => Err(()),
        }
    }

    fn write_16(
        &mut self,
        addr: u32,
        value: u16,
        priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, ()> {
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
            _ => Err(()),
        }
    }

    fn write_8(
        &mut self,
        addr: u32,
        value: u8,
        priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, ()> {
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
            _ => Err(()),
        }
    }
}

const TIMER_LOW_ADDR: u32 = 0x080;
const TIMER_HIGH_ADDR: u32 = 0x081;
const TIMER_ACCURACY_ADDR: u32 = 0x082;

pub struct IoBus<'a> {
    start_time: &'a std::time::Instant,
}

impl IoInterface for IoBus<'_> {
    fn read(&self, addr: u32, priv_level: PrivilegeLevel) -> Result<u32, ()> {
        match addr {
            TIMER_LOW_ADDR => Ok(self.start_time.elapsed().as_nanos() as u32),
            TIMER_HIGH_ADDR => Ok((self.start_time.elapsed().as_nanos() >> 32) as u32),
            TIMER_ACCURACY_ADDR => Ok(1),
            _ => Err(()),
        }
    }

    fn write(&mut self, addr: u32, value: u32, priv_level: PrivilegeLevel) -> Result<(), ()> {
        match addr {
            _ => Err(()),
        }
    }
}

pub struct Art32 {
    cpu: Cpu,
    kernel_ram: Memory,
    system_ram: Memory,
    start_time: std::time::Instant,
    reservation: Reservation,
}

impl Art32 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            kernel_ram: Memory::new(KERNEL_RAM_SIZE),
            system_ram: Memory::new(SYSTEM_RAM_SIZE),
            start_time: std::time::Instant::now(),
            reservation: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
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

    pub fn step(&mut self) {
        let mut mmu = Mmu {
            kernel_ram: &mut self.kernel_ram,
            system_ram: &mut self.system_ram,
            reservation: &mut self.reservation,
        };

        let mut io_bus = IoBus {
            start_time: &self.start_time,
        };

        if let Some(code) = self.cpu.step(&mut mmu, &mut io_bus) {}
    }
}
