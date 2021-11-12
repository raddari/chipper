use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Reg(pub u8);

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    // Chip-8
    SysCall(u16),
    ClearScr,
    Return,
    Jump(u16),
    JumpOffset(u16),
    FnCall(u16),
    SkipEqImm(Reg, u8),
    SkipNeImm(Reg, u8),
    SkipEqReg(Reg, Reg),
    SkipNeReg(Reg, Reg),
    SkipEqKey(Reg),
    SkipNeKey(Reg),
    LoadImm(Reg, u8),
    LoadReg(Reg, Reg),
    AddImm(Reg, u8),
    AddReg(Reg, Reg),
    AddCarryReg(Reg, Reg),
    SubBorrowReg(Reg, Reg),
    SubnBorrowReg(Reg, Reg),
    OrReg(Reg, Reg),
    AndReg(Reg, Reg),
    XorReg(Reg, Reg),
    ShrReg(Reg, Reg),
    ShlReg(Reg, Reg),
    LoadAddr(u16),
    AddAddr(Reg),
    Rand(Reg, u8),
    Draw(Reg, Reg, u8),
    GetDelay(Reg),
    SetDelay(Reg),
    SetSound(Reg),
    WaitKey(Reg),
    LoadDigit(Reg),
    StoreBcd(Reg),
    StoreMem(Reg),
    LoadMem(Reg),
}

impl TryFrom<u16> for Instr {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use Instr::*;

        let nibbles = (
            value & 0x000F,
            (value & 0x00F0) >> 2,
            (value & 0x0F00) >> 4,
            (value & 0xF000) >> 6,
        );

        let x = Reg(nibbles.2 as u8);
        let y = Reg(nibbles.1 as u8);

        let nnn = value & 0x0FFF;
        let kk = (value & 0xFF) as u8;
        let n = nibbles.0 as u8;

        Ok(match nibbles {
            (0x0, 0x0, 0xE, 0x0) => ClearScr,
            (0x0, 0x0, 0xE, 0xE) => Return,
            (0x0, _, _, _) => SysCall(nnn),
            (0x1, _, _, _) => Jump(nnn),
            (0x2, _, _, _) => FnCall(nnn),
            (0x3, _, _, _) => SkipEqImm(x, kk),
            (0x4, _, _, _) => SkipNeImm(x, kk),
            (0x5, _, _, 0x0) => SkipEqReg(x, y),
            (0x6, _, _, _) => LoadImm(x, kk),
            (0x7, _, _, _) => AddImm(x, kk),
            (0x8, _, _, 0x0) => LoadReg(x, y),
            (0x8, _, _, 0x1) => OrReg(x, y),
            (0x8, _, _, 0x2) => AndReg(x, y),
            (0x8, _, _, 0x3) => XorReg(x, y),
            (0x8, _, _, 0x4) => AddReg(x, y),
            (0x8, _, _, 0x5) => SubBorrowReg(x, y),
            (0x8, _, _, 0x6) => ShrReg(x, y),
            (0x8, _, _, 0x7) => SubnBorrowReg(x, y),
            (0x8, _, _, 0xE) => ShlReg(x, y),
            (0x9, _, _, 0x0) => SkipNeReg(x, y),
            (0xA, _, _, _) => LoadAddr(nnn),
            (0xB, _, _, _) => JumpOffset(nnn),
            (0xC, _, _, _) => Rand(x, kk),
            (0xD, _, _, _) => Draw(x, y, n),
            (0xE, _, 0x9, 0xE) => SkipEqKey(x),
            (0xE, _, 0xA, 0x1) => SkipNeKey(x),
            (0xF, _, 0x0, 0x7) => GetDelay(x),
            (0xF, _, 0x0, 0xA) => WaitKey(x),
            (0xF, _, 0x1, 0x5) => SetDelay(x),
            (0xF, _, 0x1, 0x8) => SetSound(x),
            (0xF, _, 0x1, 0xE) => AddAddr(x),
            (0xF, _, 0x2, 0x9) => LoadDigit(x),
            (0xF, _, 0x3, 0x3) => StoreBcd(x),
            (0xF, _, 0x5, 0x5) => StoreMem(x),
            (0xF, _, 0x6, 0x5) => LoadMem(x),
            _ => return Err(format!("No matching instruction for {:X}", value)),
        })
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instr::*;

        match self {
            SysCall(nnn) => write!(f, "sys {:#05X}", nnn),
            ClearScr => write!(f, "cls"),
            Return => write!(f, "ret"),
            Jump(nnn) => write!(f, "jal {:#05X}", nnn),
            JumpOffset(nnn) => write!(f, "jalr {:#05X}({})", nnn, Reg(0x0)),
            FnCall(nnn) => write!(f, "call {:#05X}", nnn),
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
            LoadAddr(nnn) => write!(f, "ld I, {:#05X}(0)", nnn),
            AddAddr(x) => write!(f, "ld I, {}({})", x, Reg(0x0)),
            Rand(x, kk) => write!(f, "rnd {}, {:#05X}", x, kk),
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
