pub struct Cpu {
    pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { pc: 0x200 }
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
}
