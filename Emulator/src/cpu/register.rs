use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::{Display, EnumCount, EnumIter};

#[derive(
    Debug, Display, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, EnumCount, EnumIter,
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub(super) enum Register {
    Zero,
    Ra,
    Sp,
    Fp,
    Gp,
    Tp,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
}

impl TryFrom<usize> for Register {
    type Error = ();

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = u8::try_from(value).map_err(|_| ())?;
        Self::try_from(value).map_err(|_| ())
    }
}

impl From<Register> for usize {
    #[inline]
    fn from(value: Register) -> Self {
        let value: u8 = value.into();
        value as usize
    }
}

impl TryFrom<u32> for Register {
    type Error = ();

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value = u8::try_from(value).map_err(|_| ())?;
        Self::try_from(value).map_err(|_| ())
    }
}

impl From<Register> for u32 {
    #[inline]
    fn from(value: Register) -> Self {
        let value: u8 = value.into();
        value as u32
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub(super) struct RegisterFile([u32; Register::COUNT]);

impl RegisterFile {
    #[inline]
    pub(super) fn get(&self, reg: Register) -> u32 {
        let index: usize = reg.into();
        self.0[index]
    }

    #[inline]
    pub(super) fn set(&mut self, reg: Register, value: u32) {
        if reg != Register::Zero {
            let index: usize = reg.into();
            self.0[index] = value;
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, EnumIter)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub(super) enum Condition {
    Eq,
    Ne,
    Lt,
    Ge,
    Lts,
    Ges,
    True,
    False,
}

impl TryFrom<u32> for Condition {
    type Error = ();

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value = u8::try_from(value).map_err(|_| ())?;
        Self::try_from(value).map_err(|_| ())
    }
}

impl From<Condition> for u32 {
    fn from(value: Condition) -> Self {
        let value: u8 = value.into();
        value as u32
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, EnumIter)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub(super) enum BranchCondition {
    Eq,
    Ne,
    Lt,
    Ge,
    Lts,
    Ges,
    True,
    Link,
}

impl TryFrom<u32> for BranchCondition {
    type Error = ();

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value = u8::try_from(value).map_err(|_| ())?;
        Self::try_from(value).map_err(|_| ())
    }
}

impl From<BranchCondition> for u32 {
    fn from(value: BranchCondition) -> Self {
        let value: u8 = value.into();
        value as u32
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub(super) struct Flags : u8 {
        const CARRY = 0x1;
        const ZERO = 0x2;
        const SIGN = 0x4;
        const OVERFLOW = 0x8;
    }
}

impl std::fmt::Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = [b'_'; 4];

        if self.contains(Self::CARRY) {
            buffer[3] = b'C';
        }

        if self.contains(Self::ZERO) {
            buffer[2] = b'Z';
        }

        if self.contains(Self::SIGN) {
            buffer[1] = b'S';
        }

        if self.contains(Self::OVERFLOW) {
            buffer[0] = b'O';
        }

        f.write_str(std::str::from_utf8(&buffer).unwrap())
    }
}

impl Flags {
    pub(super) fn satisfy(self, condition: Condition) -> bool {
        match condition {
            Condition::Eq => self.contains(Self::ZERO),
            Condition::Ne => !self.contains(Self::ZERO),
            Condition::Lt => !self.contains(Self::CARRY),
            Condition::Ge => self.contains(Self::CARRY),
            Condition::Lts => self.contains(Self::SIGN) != self.contains(Self::OVERFLOW),
            Condition::Ges => self.contains(Self::SIGN) == self.contains(Self::OVERFLOW),
            Condition::True => true,
            Condition::False => false,
        }
    }

    pub(super) fn satisfy_branch(self, condition: BranchCondition) -> bool {
        match condition {
            BranchCondition::Eq => self.contains(Self::ZERO),
            BranchCondition::Ne => !self.contains(Self::ZERO),
            BranchCondition::Lt => !self.contains(Self::CARRY),
            BranchCondition::Ge => self.contains(Self::CARRY),
            BranchCondition::Lts => self.contains(Self::SIGN) != self.contains(Self::OVERFLOW),
            BranchCondition::Ges => self.contains(Self::SIGN) == self.contains(Self::OVERFLOW),
            BranchCondition::True => true,
            BranchCondition::Link => true,
        }
    }
}
