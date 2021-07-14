#[derive(Debug)]
pub struct Cpu {
    pc: u16,
    registers: [u8; 16],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0x200,
            registers: [0; 16],
        }
    }

    pub fn execute(&mut self, instruction: u16) {
        self.pc += 2;
        let opargs = OpArgs::new(instruction);
        match opargs.opcode {
            0x01 => self.jp(opargs.address),
            0x06 => self.ld(opargs.x_reg, opargs.byte),
            0x07 => self.add(opargs.x_reg, opargs.byte),
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

    fn ld(&mut self, dest_reg: usize, constant: u8) {
        self.registers[dest_reg] = constant;
    }

    fn add(&mut self, dest_reg: usize, constant: u8) {
        let mut value = self.registers[dest_reg] as u16;
        value += constant as u16;
        self.registers[0xF] = (value > 0xFF) as u8;
        self.registers[dest_reg] = value as u8;
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
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
    fn load_constant_to_register() {
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
    fn jump_sets_pc() {
        let mut cpu = Cpu::new();
        cpu.execute(0x1BCD);
        assert_eq!(0xBCD, cpu.pc);
    }
}
