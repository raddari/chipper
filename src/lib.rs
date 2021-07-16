pub mod cpu;
pub mod memory;

pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;
pub const CHIP8_VRAM: usize = CHIP8_WIDTH * CHIP8_HEIGHT;
pub const CHIP8_RAM: usize = 4096;
