use bytemuck::{cast_slice, cast_slice_mut};

#[repr(transparent)]
pub struct Memory(Box<[u32]>);

impl Memory {
    #[inline]
    pub fn new(size: u32) -> Self {
        debug_assert_eq!(size % 4, 0);
        Self(vec![0u32; (size / 4) as usize].into_boxed_slice())
    }

    #[inline]
    pub fn reset(&mut self, data: &[u8]) {
        let mem: &mut [u8] = cast_slice_mut(&mut self.0);
        mem.copy_from_slice(data);
    }

    #[inline]
    pub fn read_32(&self, addr: u32) -> u32 {
        let mem: &[u32] = cast_slice(&self.0);
        u32::from_le(mem[(addr >> 2) as usize])
    }

    #[inline]
    pub fn read_16(&self, addr: u32) -> u16 {
        let mem: &[u16] = cast_slice(&self.0);
        u16::from_le(mem[(addr >> 1) as usize])
    }

    #[inline]
    pub fn read_8(&self, addr: u32) -> u8 {
        let mem: &[u8] = cast_slice(&self.0);
        u8::from_le(mem[(addr >> 0) as usize])
    }

    #[inline]
    pub fn write_32(&mut self, addr: u32, value: u32) {
        let mem: &mut [u32] = cast_slice_mut(&mut self.0);
        mem[(addr >> 2) as usize] = u32::to_le(value);
    }

    #[inline]
    pub fn write_16(&mut self, addr: u32, value: u16) {
        let mem: &mut [u16] = cast_slice_mut(&mut self.0);
        mem[(addr >> 1) as usize] = u16::to_le(value);
    }

    #[inline]
    pub fn write_8(&mut self, addr: u32, value: u8) {
        let mem: &mut [u8] = cast_slice_mut(&mut self.0);
        mem[(addr >> 0) as usize] = u8::to_le(value);
    }
}
