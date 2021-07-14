use crate::memory::Memory;

#[derive(Debug)]
pub struct Cpu {
    pc: u16,
    registers: [u8; 16],
    memory: Memory,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0x200,
            registers: [0; 16],
            memory: Memory::new(),
        }
    }

    pub fn execute(&mut self, instruction: u16) {
        self.pc += 2;
        let opargs = OpArgs::new(instruction);
        match opargs.opcode {
            0x01 => self.jp(opargs.address),
            0x02 => self.call(opargs.address),
            0x03 => self.se(opargs.x_reg, opargs.byte),
            0x04 => self.sne(opargs.x_reg, opargs.byte),
            0x05 => self.se(opargs.x_reg, self.reg_val(opargs.y_reg)),
            0x06 => self.ld(opargs.x_reg, opargs.byte),
            0x07 => self.add(opargs.x_reg, opargs.byte),
            0x0E => self.ret(),
            0x80 => self.ld(opargs.x_reg, self.reg_val(opargs.y_reg)),
            0x81 => self.or(opargs.x_reg, opargs.y_reg),
            0x82 => self.and(opargs.x_reg, opargs.y_reg),
            0x83 => self.xor(opargs.x_reg, self.reg_val(opargs.y_reg)),
            0x84 => self.add(opargs.x_reg, self.reg_val(opargs.y_reg)),
            _ => panic!("No matching opcode for {:02x}", opargs.opcode),
        };
    }

    pub fn reg_val(&self, register: usize) -> u8 {
        self.registers[register]
    }

    fn jp(&mut self, address: u16) {
        self.pc = address;
    }

    fn ret(&mut self) {
        self.pc = self.memory.callstack_pop().unwrap();
    }

    fn call(&mut self, address: u16) {
        self.memory.callstack_push(self.pc);
        self.pc = address;
    }

    fn se(&mut self, dest_reg: usize, constant: u8) {
        if constant == self.reg_val(dest_reg) {
            self.pc += 2;
        }
    }

    fn sne(&mut self, dest_reg: usize, constant: u8) {
        if constant != self.reg_val(dest_reg) {
            self.pc += 2;
        }
    }

    fn ld(&mut self, dest_reg: usize, constant: u8) {
        self.registers[dest_reg] = constant;
    }

    fn or(&mut self, dest_reg: usize, src_reg: usize) {
        self.registers[dest_reg] |= self.reg_val(src_reg);
    }

    fn and(&mut self, dest_reg: usize, src_reg: usize) {
        self.registers[dest_reg] &= self.reg_val(src_reg);
    }

    fn xor(&mut self, dest_reg: usize, constant: u8) {
        self.registers[dest_reg] ^= constant;
    }

    fn add(&mut self, dest_reg: usize, constant: u8) {
        let mut value = self.registers[dest_reg] as u16;
        value += constant as u16;
        self.registers[0xF] = (value > 0xFF) as u8;
        self.registers[dest_reg] = value as u8;
    }
}

#[derive(Debug)]
struct OpArgs {
    opcode: u8,
    x_reg: usize,
    y_reg: usize,
    address: u16,
    byte: u8,
    nibble: u8,
}

impl OpArgs {
    fn new(instruction: u16) -> OpArgs {
        OpArgs {
            opcode: match (instruction & 0xF000) >> 12 {
                0x0 | 0x8 | 0xE | 0xF => ((instruction & 0xF000) >> 8) + (instruction & 0x000F),
                op => op,
            } as u8,
            x_reg: ((instruction & 0x0F00) >> 8) as usize,
            y_reg: ((instruction & 0x00F0) >> 4) as usize,
            address: (instruction & 0x0FFF),
            byte: (instruction & 0x00FF) as u8,
            nibble: (instruction & 0x000F) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn pc_starts_at_0x200() {
        let cpu = Cpu::new();
        assert_eq!(0x200, cpu.pc);
    }

    #[test]
    fn execute_normally_increments_pc() {
        let mut cpu = Cpu::new();
        cpu.execute(0x6000);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn ld_constant_to_register() {
        let mut cpu = Cpu::new();
        cpu.execute(0x6075);
        assert_eq!(0x75, cpu.reg_val(0x0));
    }

    #[test]
    fn add_constant_to_register_normal() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 1);
        cpu.execute(0x7001);
        assert_eq!(2, cpu.reg_val(0x0));
        assert_eq!(0, cpu.reg_val(0xF));
    }

    #[test]
    fn add_constant_to_register_overflow() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 0xFF);
        cpu.execute(0x7001);
        assert_eq!(0, cpu.reg_val(0x0));
        assert_eq!(1, cpu.reg_val(0xF));
    }

    #[test]
    fn add_register_to_register_normal() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 1);
        cpu.ld(0x1, 2);
        cpu.execute(0x8014);
        assert_eq!(3, cpu.reg_val(0x0));
        assert_eq!(0, cpu.reg_val(0xF));
    }

    #[test]
    fn add_register_to_register_overflow() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 0xFF);
        cpu.ld(0x1, 1);
        cpu.execute(0x8014);
        assert_eq!(0, cpu.reg_val(0x0));
        assert_eq!(1, cpu.reg_val(0xF));
    }

    #[test]
    fn jp_sets_pc() {
        let mut cpu = Cpu::new();
        cpu.execute(0x1ABC);
        assert_eq!(0xABC, cpu.pc);
    }

    #[test]
    fn call_sets_pc() {
        let mut cpu = Cpu::new();
        cpu.execute(0x2ABC);
        assert_eq!(0xABC, cpu.pc);
    }

    #[test]
    fn ret_pops_pc() {
        let mut cpu = Cpu::new();
        cpu.execute(0x2ABC);
        cpu.execute(0x00EE);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn call_ret_nested() {
        let mut cpu = Cpu::new();
        cpu.execute(0x2678);
        cpu.execute(0x2ABC);
        assert_eq!(0xABC, cpu.pc);

        cpu.execute(0x00EE);
        assert_eq!(0x67A, cpu.pc);

        cpu.execute(0x00EE);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn se_constant_skip() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 32);
        cpu.execute(0x3020);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn se_constant_no_skip() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 32);
        cpu.execute(0x3021);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn sne_constant_skip() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 32);
        cpu.execute(0x4021);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn sne_constant_no_skip() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 32);
        cpu.execute(0x4020);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn se_register_skip() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 32);
        cpu.ld(0x1, 32);
        cpu.execute(0x5010);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn se_register_no_skip() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 32);
        cpu.ld(0x1, 33);
        cpu.execute(0x5010);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn ld_register_to_register() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 32);
        cpu.execute(0x8100);
        assert_eq!(32, cpu.reg_val(0x1));
    }

    #[test]
    fn or_register_to_register() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 0x55);
        cpu.ld(0x1, 0x3C);
        cpu.execute(0x8011);
        assert_eq!(0x7D, cpu.reg_val(0x0));
    }

    #[test]
    fn and_register_to_register() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 0x55);
        cpu.ld(0x1, 0x3C);
        cpu.execute(0x8012);
        assert_eq!(0x14, cpu.reg_val(0x0));
    }

    #[test]
    fn xor_register_to_register() {
        let mut cpu = Cpu::new();
        cpu.ld(0x0, 0x55);
        cpu.ld(0x1, 0x3C);
        cpu.execute(0x8013);
        assert_eq!(0x69, cpu.reg_val(0x0));
    }
}
