use crate::memory::Memory;

pub type ByteMemory = dyn Memory<u8>;

pub struct Bus {
    memory: Option<Box<ByteMemory>>,
}

impl Memory<u8> for Bus {
    fn load(&self, offset: usize, size: usize) -> Vec<u8> {
        self.memory.as_deref().unwrap().load(offset, size)
    }

    fn store(&mut self, offset: usize, data: &[u8]) {
        self.memory.as_deref_mut().unwrap().store(offset, data);
    }
}

impl Bus {
    pub fn new(memory: Box<ByteMemory>) -> Self {
        Bus {
            memory: Option::from(memory),
        }
    }
}
