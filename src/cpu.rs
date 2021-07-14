#[derive(Debug)]
pub struct Cpu {
    pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { pc: 0x200 }
    }

    pub fn execute(&mut self, instruction: u16) {
        self.pc += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pc_starts_at_0x200() {
        let cpu = Cpu::new();
        assert_eq!(0x200, cpu.pc);
    }

    #[test]
    fn execute_normally_increments_pc() {
        let mut cpu = Cpu::new();
        cpu.execute(0x0);
        assert_eq!(0x202, cpu.pc);
    }
}
