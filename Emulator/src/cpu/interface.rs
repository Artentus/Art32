use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PrivilegeLevel {
    System,
    User,
}

impl TryFrom<u32> for PrivilegeLevel {
    type Error = ();

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value = u8::try_from(value).map_err(|_| ())?;
        Self::try_from(value).map_err(|_| ())
    }
}

impl From<PrivilegeLevel> for u32 {
    fn from(value: PrivilegeLevel) -> Self {
        let value: u8 = value.into();
        value as u32
    }
}

pub trait MemoryInterface {
    fn read_32(&mut self, addr: u32, priv_level: PrivilegeLevel, reserve: bool) -> Result<u32, ()>;

    fn read_16(&mut self, addr: u32, priv_level: PrivilegeLevel, reserve: bool) -> Result<u16, ()>;

    fn read_8(&mut self, addr: u32, priv_level: PrivilegeLevel, reserve: bool) -> Result<u8, ()>;

    fn write_32(
        &mut self,
        addr: u32,
        value: u32,
        priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, ()>;

    fn write_16(
        &mut self,
        addr: u32,
        value: u16,
        priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, ()>;

    fn write_8(
        &mut self,
        addr: u32,
        value: u8,
        priv_level: PrivilegeLevel,
        conditional: bool,
    ) -> Result<bool, ()>;
}

pub trait IoInterface {
    fn read(&self, addr: u32, priv_level: PrivilegeLevel) -> Result<u32, ()>;

    fn write(&mut self, addr: u32, value: u32, priv_level: PrivilegeLevel) -> Result<(), ()>;
}
