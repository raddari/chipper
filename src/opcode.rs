#[allow(non_camel_case_types)]
pub enum Opcode {
    OP_00E0 {},
    OP_00EE {},
    OP_0nnn { nnn: usize },
    OP_1nnn { nnn: usize },
    OP_2nnn { nnn: usize },
    OP_3xkk { x: usize, kk: u8 },
    OP_4xkk { x: usize, kk: u8 },
    OP_5xy0 { x: usize, y: usize },
    OP_6xkk { x: usize, kk: u8 },
    OP_7xkk { x: usize, kk: u8 },
    OP_8xy0 { x: usize, y: usize },
    OP_8xy1 { x: usize, y: usize },
    OP_8xy2 { x: usize, y: usize },
    OP_8xy3 { x: usize, y: usize },
    OP_8xy4 { x: usize, y: usize },
    OP_8xy5 { x: usize, y: usize },
    OP_8xy6 { x: usize, y: usize },
    OP_8xy7 { x: usize, y: usize },
    OP_8xyE { x: usize, y: usize },
    OP_9xy0 { x: usize, y: usize },
    OP_Annn { nnn: usize },
    OP_Bnnn { nnn: usize },
    OP_Cxkk { x: usize, kk: u8 },
    OP_Dxyn { x: usize, y: usize, n: usize },
    OP_Ex9E { x: usize },
    OP_ExA1 { x: usize },
    OP_Fx07 { x: usize },
    OP_Fx0A { x: usize },
    OP_Fx15 { x: usize },
    OP_Fx18 { x: usize },
    OP_Fx1E { x: usize },
    OP_Fx29 { x: usize },
    OP_Fx33 { x: usize },
    OP_Fx55 { x: usize },
    OP_Fx65 { x: usize },
}

const ERR_SUPER_48: &str = "super Chip-48 instructions are not implemented";

impl Opcode {
    pub fn decode(instruction: u16) -> Result<Opcode, &'static str> {
        let nibbles = Self::unpack_nibbles(instruction);
        let (_, x, y, n) = nibbles;
        let nnn = (instruction & 0x0FFF) as usize;
        let kk = (instruction & 0x00FF) as u8;

        use Opcode::*;
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Ok(OP_00E0 {}),
            (0x0, 0x0, 0xE, 0xE) => Ok(OP_00EE {}),
            (0x0, 0x0, 0xC, _)
            | (0x0, 0x0, 0xF, 0xB)
            | (0x0, 0x0, 0xF, 0xC)
            | (0x0, 0x0, 0xF, 0xD)
            | (0x0, 0x0, 0xF, 0xE)
            | (0x0, 0x0, 0xF, 0xF) => Err(ERR_SUPER_48),
            (0x0, _, _, _) => Ok(OP_0nnn { nnn }),
            (0x1, _, _, _) => Ok(OP_1nnn { nnn }),
            (0x2, _, _, _) => Ok(OP_2nnn { nnn }),
            (0x3, _, _, _) => Ok(OP_3xkk { x, kk }),
            (0x4, _, _, _) => Ok(OP_4xkk { x, kk }),
            (0x5, _, _, 0x0) => Ok(OP_5xy0 { x, y }),
            (0x6, _, _, _) => Ok(OP_6xkk { x, kk }),
            (0x7, _, _, _) => Ok(OP_7xkk { x, kk }),
            (0x8, _, _, 0x0) => Ok(OP_8xy0 { x, y }),
            (0x8, _, _, 0x1) => Ok(OP_8xy1 { x, y }),
            (0x8, _, _, 0x2) => Ok(OP_8xy2 { x, y }),
            (0x8, _, _, 0x3) => Ok(OP_8xy3 { x, y }),
            (0x8, _, _, 0x4) => Ok(OP_8xy4 { x, y }),
            (0x8, _, _, 0x5) => Ok(OP_8xy5 { x, y }),
            (0x8, _, _, 0x6) => Ok(OP_8xy6 { x, y }),
            (0x8, _, _, 0x7) => Ok(OP_8xy7 { x, y }),
            (0x8, _, _, 0xE) => Ok(OP_8xyE { x, y }),
            (0x9, _, _, 0x0) => Ok(OP_9xy0 { x, y }),
            (0xA, _, _, _) => Ok(OP_Annn { nnn }),
            (0xB, _, _, _) => Ok(OP_Bnnn { nnn }),
            (0xC, _, _, _) => Ok(OP_Cxkk { x, kk }),
            (0xD, _, _, 0x0) => Err(ERR_SUPER_48),
            (0xD, _, _, _) => Ok(OP_Dxyn { x, y, n }),
            (0xE, _, 0x9, 0xE) => Ok(OP_Ex9E { x }),
            (0xE, _, 0xA, 0x1) => Ok(OP_ExA1 { x }),
            (0xF, _, 0x0, 0x7) => Ok(OP_Fx07 { x }),
            (0xF, _, 0x0, 0xA) => Ok(OP_Fx0A { x }),
            (0xF, _, 0x1, 0x5) => Ok(OP_Fx15 { x }),
            (0xF, _, 0x1, 0x8) => Ok(OP_Fx18 { x }),
            (0xF, _, 0x1, 0xE) => Ok(OP_Fx1E { x }),
            (0xF, _, 0x2, 0x9) => Ok(OP_Fx29 { x }),
            (0xF, _, 0x3, 0x3) => Ok(OP_Fx33 { x }),
            (0xF, _, 0x5, 0x5) => Ok(OP_Fx55 { x }),
            (0xF, _, 0x6, 0x5) => Ok(OP_Fx65 { x }),
            (0xF, _, 0x3, 0x0) | (0xF, _, 0x7, 0x5) | (0xF, _, 0x8, 0x5) => Err(ERR_SUPER_48),
            _ => Err("unknown opcode"),
        }
    }

    fn unpack_nibbles(instruction: u16) -> (usize, usize, usize, usize) {
        (
            ((instruction & 0xF000) >> 12) as usize,
            ((instruction & 0x0F00) >> 8) as usize,
            ((instruction & 0x00F0) >> 4) as usize,
            (instruction & 0x000F) as usize,
        )
    }
}
