#[derive(Debug)]
pub struct Memory {
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
            callstack: Vec::new(),
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
