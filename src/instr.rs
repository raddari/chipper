use std::fmt;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instr::*;

        match self {
            SysCall(nnn) => write!(f, "sys {:#05x}", nnn),
            ClearScr => write!(f, "cls"),
            Return => write!(f, "ret"),
            Jump(nnn) => write!(f, "jal {:#05x}", nnn),
            JumpOffset(nnn) => write!(f, "jalr {:#05x}({})", nnn, V(0x0)),
            FnCall(nnn) => write!(f, "call {:#05x}", nnn),
            SkipEqImm(x, kk) => write!(f, "sei {}, {}", x, kk),
            SkipNeImm(x, kk) => write!(f, "snei {}, {}", x, kk),
            SkipEqReg(x, y) => write!(f, "se {}, {}", x, y),
            SkipNeReg(x, y) => write!(f, "sne {}, {}", x, y),
            SkipEqKey(x) => write!(f, "skp {}", x),
            SkipNeKey(x) => write!(f, "sknp {}", x),
            LoadImm(x, kk) => write!(f, "ld {}, {}", x, kk),
            LoadReg(x, y) => write!(f, "ld {}, {}", x, y),
            AddImm(x, kk) => write!(f, "addi {}, {}", x, kk),
            AddReg(x, y) => write!(f, "add {}, {}", x, y),
            AddCarryReg(x, y) => write!(f, "addc {}, {}", x, y),
            SubBorrowReg(x, y) => write!(f, "sub {}, {}", x, y),
            SubnBorrowReg(x, y) => write!(f, "subn {}, {}, {}", x, y, x),
            OrReg(x, y) => write!(f, "or {}, {}", x, y),
            AndReg(x, y) => write!(f, "and {}, {}", x, y),
            XorReg(x, y) => write!(f, "xor {}, {}", x, y),
            ShrReg(x, y) => write!(f, "srl {}, {}", x, y),
            ShlReg(x, y) => write!(f, "sll {}, {}", x, y),
            LoadAddr(nnn) => write!(f, "ld I, {:#05x}(0)", nnn),
            AddAddr(x) => write!(f, "ld I, {}({})", x, V(0x0)),
            Rand(x, kk) => write!(f, "rnd {}, {:#05x}", x, kk),
            Draw(x, y, n) => write!(f, "drw {}, {}, {}", x, y, n),
            GetDelay(x) => write!(f, "ld {}, DT", x),
            SetDelay(x) => write!(f, "ld DT, {}", x),
            SetSound(x) => write!(f, "ld ST, {}", x),
            WaitKey(x) => write!(f, "ld {}, $K", x),
            LoadDigit(x) => write!(f, "ld I, {}", x),
            StoreBcd(x) => write!(f, "bcd [I], {}", x),
            StoreMem(x) => write!(f, "sd [I], {}", x),
            LoadMem(x) => write!(f, "ld {}, [I]", x),
        }
    }
}
