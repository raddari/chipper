use std::fmt;

pub struct V(pub u8);

impl From<u8> for V {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "V{}", self.0)
    }
}

pub enum Instr {
    SysCall(u16),
    ClearScr,
    Return,
    Jump(u16),
    JumpOffset(u16),
    FnCall(u16),
    SkipEqImm(V, u8),
    SkipNeImm(V, u8),
    SkipEqReg(V, V),
    SkipNeReg(V, V),
    SkipEqKey(V),
    SkipNeKey(V),
    LoadImm(V, u8),
    LoadReg(V, V),
    AddImm(V, u8),
    AddReg(V, V),
    AddCarryReg(V, V),
    SubBorrowReg(V, V),
    SubnBorrowReg(V, V),
    OrReg(V, V),
    AndReg(V, V),
    XorReg(V, V),
    ShrReg(V, V),
    ShlReg(V, V),
    LoadAddr(u16),
    AddAddr(V),
    Rand(V, u8),
    Draw(V, V, u8),
    GetDelay(V),
    SetDelay(V),
    SetSound(V),
    WaitKey(V),
    LoadDigit(V),
    StoreBcd(V),
    StoreMem(V),
    LoadMem(V),
}
