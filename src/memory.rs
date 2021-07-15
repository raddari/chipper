use crate::CHIP8_RAM;

#[derive(Debug)]
pub struct Memory {
    ram: [u8; CHIP8_RAM],
    callstack: Vec<u16>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            ram: [0; CHIP8_RAM],
            callstack: vec![0; 16],
        }
    }

    pub fn load(&self, offset: usize, size: usize) -> &[u8] {
        &self.ram[offset..offset + size]
    }

    pub fn store(&mut self, offset: usize, bytes: &[u8]) {
        self.ram[offset..offset + bytes.len()].copy_from_slice(bytes);
    }

    pub fn callstack_empty(&self) -> bool {
        self.callstack.is_empty()
    }

    pub fn callstack_push(&mut self, address: u16) {
        self.callstack.push(address);
    }

    pub fn callstack_pop(&mut self) -> Option<u16> {
        self.callstack.pop()
    }
}
