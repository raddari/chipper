use crate::CHIP8_RAM;

pub trait Memory<T>
where
    T: Copy,
{
    fn load(&self, offset: usize, size: usize) -> Vec<T>;
    fn store(&mut self, offset: usize, data: &[T]);
}

#[derive(Debug)]
pub struct Ram {
    bytes: [u8; CHIP8_RAM],
    callstack: Vec<u16>,
}

impl Default for Ram {
    fn default() -> Self {
        Ram::new()
    }
}

impl Memory<u8> for Ram {
    fn load(&self, offset: usize, size: usize) -> Vec<u8> {
        self.bytes[offset..offset + size].to_vec()
    }

    fn store(&mut self, offset: usize, data: &[u8]) {
        self.bytes[offset..offset + data.len()].copy_from_slice(data);
    }
}

impl Ram {
    pub fn new() -> Self {
        Ram {
            bytes: [0; CHIP8_RAM],
            callstack: vec![0; 16],
        }
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
