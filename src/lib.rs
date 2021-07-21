pub mod cpu;
pub mod keyboard;
pub mod memory;
pub mod opcode;

pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;
pub const CHIP8_VBUFFER: usize = 256;
pub const CHIP8_RAM: usize = 4096;
