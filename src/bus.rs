use crate::cpu::Cpu;
use crate::memory::Memory;

pub type ByteMemory = dyn Memory<u8>;

pub struct Bus {
    cpu: Option<Cpu>,
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
    pub fn new_uninit() -> Box<Self> {
        Box::from(Bus {
            cpu: None,
            memory: None,
        })
    }

    pub fn register_cpu(&mut self, cpu: Cpu) {
        self.cpu = Option::from(cpu);
    }

    pub fn register_memory(&mut self, memory: Box<ByteMemory>) {
        self.memory = Option::from(memory);
    }
}
