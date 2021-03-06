pub const CHIP8_RAM: usize = 4096;

pub struct Memory {
    bytes: [u8; CHIP8_RAM],
    callstack: Vec<usize>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            bytes: [0; CHIP8_RAM],
            callstack: vec![0; 16],
        }
    }

    pub fn load(&self, offset: usize, size: usize) -> Vec<u8> {
        self.bytes[offset..offset + size].to_vec()
    }

    pub fn store(&mut self, offset: usize, data: &[u8]) {
        self.bytes[offset..offset + data.len()].copy_from_slice(data);
    }

    pub fn is_callstack_empty(&self) -> bool {
        self.callstack.is_empty()
    }

    pub fn push(&mut self, address: usize) {
        self.callstack.push(address);
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.callstack.pop()
    }
}
