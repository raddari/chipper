use crate::memory::Memory;
use crate::{CHIP8_VBUFFER, CHIP8_WIDTH};
use rand::prelude::{SeedableRng, StdRng};
use rand::RngCore;

#[derive(Debug)]
pub struct Cpu {
    pc: u16,
    ri: u16,
    v: [u8; 16],
    vbuffer: [u8; CHIP8_VBUFFER],
    memory: Memory,
    random: StdRng,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}

#[allow(non_snake_case)]
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0x200,
            ri: 0,
            v: [0; 16],
            vbuffer: [0; CHIP8_VBUFFER],
            memory: Memory::new(),
            random: StdRng::from_entropy(),
        }
    }

    pub fn execute(&mut self, instruction: u16) {
        self.pc += 2;

        let nibbles = Self::unpack_nibbles(instruction);
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;
        let nnn = (instruction & 0x0FFF) as u16;
        let kk = (instruction & 0x00FF) as u8;

        match nibbles {
            (0x0, 0x0, 0xE, 0xE) => self.op_00EE(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            (0x4, _, _, _) => self.op_4xkk(x, kk),
            (0x5, _, _, _) => self.op_5xy0(x, y),
            (0x6, _, _, _) => self.op_6xkk(x, kk),
            (0x7, _, _, _) => self.op_7xkk(x, kk),
            (0x8, _, _, 0x0) => self.op_8xy0(x, y),
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),
            (0x8, _, _, 0x6) => self.op_8xy6(x, y),
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),
            (0x8, _, _, 0xE) => self.op_8xyE(x, y),
            (0x9, _, _, 0x0) => self.op_9xy0(x, y),
            (0xA, _, _, _) => self.op_Annn(nnn),
            (0xB, _, _, _) => self.op_Bnnn(nnn),
            (0xC, _, _, _) => self.op_Cxkk(x, kk),
            (0xD, _, _, _) => self.op_Dxyn(x, y, n),
            _ => (),
        };
    }

    fn unpack_nibbles(instruction: u16) -> (u8, u8, u8, u8) {
        (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8,
        )
    }

    fn op_1nnn(&mut self, address: u16) {
        self.pc = address;
    }

    fn op_00EE(&mut self) {
        self.pc = self.memory.callstack_pop().unwrap();
    }

    fn op_2nnn(&mut self, address: u16) {
        self.memory.callstack_push(self.pc);
        self.pc = address;
    }

    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if kk == self.v[x] {
            self.pc += 2;
        }
    }

    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if kk != self.v[x] {
            self.pc += 2;
        }
    }

    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.add_with_overflow(x, kk);
    }

    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    fn op_8xy4(&mut self, x: usize, y: usize) {
        self.add_with_overflow(x, self.v[y]);
    }

    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.sub_with_underflow(x, x, self.v[y]);
    }

    fn op_8xy6(&mut self, x: usize, _y: usize) {
        self.overflow_flag((self.v[x] & 0x1) == 1);
        self.v[x] >>= 1;
    }

    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.sub_with_underflow(x, y, self.v[x]);
    }

    fn op_8xyE(&mut self, x: usize, _y: usize) {
        self.overflow_flag((self.v[x] & 0x80) == 0x80);
        self.v[x] <<= 1;
    }

    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    fn op_Annn(&mut self, address: u16) {
        self.ri = address;
    }

    fn op_Bnnn(&mut self, address: u16) {
        self.pc = address + self.v[0x0] as u16;
    }

    fn op_Cxkk(&mut self, x: usize, kk: u8) {
        let value = (self.random.next_u32() as u8) & kk;
        self.v[x] = value;
    }

    fn op_Dxyn(&mut self, x: usize, y: usize, n: usize) {
        let sprite = self.memory.load(self.ri as usize, n);
        let flat = Self::flatten_index(self.v[x] as usize, self.v[y] as usize);
        let collision = self.draw_and_check_collision(flat, &sprite);
        self.overflow_flag(collision);
    }

    fn draw_and_check_collision(&mut self, index: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        for (i, byte) in self.vbuffer[index..index + sprite.len()]
            .iter_mut()
            .enumerate()
        {
            *byte ^= sprite[i];
            if *byte != sprite[i] {
                collision = true;
            }
        }
        collision
    }

    fn flatten_index(x: usize, y: usize) -> usize {
        y * CHIP8_WIDTH + x
    }

    fn overflow_flag(&mut self, condition: bool) {
        self.v[0xF] = condition as u8;
    }

    fn sub_with_underflow(&mut self, dest: usize, src: usize, kk: u8) {
        self.add_with_overflow_dest(dest, src, u8::MAX - kk + 1);
    }

    fn add_with_overflow(&mut self, x: usize, kk: u8) {
        self.add_with_overflow_dest(x, x, kk);
    }

    fn add_with_overflow_dest(&mut self, dest: usize, src: usize, kk: u8) {
        let mut value = self.v[src] as u16;
        value += kk as u16;
        self.overflow_flag(value > 0xFF);
        self.v[dest] = value as u8;
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
        assert_eq!(0x75, cpu.v[0x0]);
    }

    #[test]
    fn add_constant_to_register_normal() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 1;
        cpu.execute(0x7001);
        assert_eq!(2, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn add_constant_to_register_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 0xFF;
        cpu.execute(0x7001);
        assert_eq!(0, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn add_register_to_register_normal() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 1;
        cpu.v[0x1] = 2;
        cpu.execute(0x8014);
        assert_eq!(3, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn add_register_to_register_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 0xFF;
        cpu.v[0x1] = 1;
        cpu.execute(0x8014);
        assert_eq!(0, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
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
        cpu.v[0x0] = 32;
        cpu.execute(0x3020);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn se_constant_no_skip() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 32;
        cpu.execute(0x3021);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn sne_constant_skip() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 32;
        cpu.execute(0x4021);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn sne_constant_no_skip() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 32;
        cpu.execute(0x4020);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn se_register_skip() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 32;
        cpu.v[0x1] = 32;
        cpu.execute(0x5010);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn se_register_no_skip() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 32;
        cpu.v[0x1] = 33;
        cpu.execute(0x5010);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn ld_register_to_register() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 32;
        cpu.execute(0x8100);
        assert_eq!(32, cpu.v[0x1]);
    }

    #[test]
    fn or_register_to_register() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 0x55;
        cpu.v[0x1] = 0x3C;
        cpu.execute(0x8011);
        assert_eq!(0x7D, cpu.v[0x0]);
    }

    #[test]
    fn and_register_to_register() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 0x55;
        cpu.v[0x1] = 0x3C;
        cpu.execute(0x8012);
        assert_eq!(0x14, cpu.v[0x0]);
    }

    #[test]
    fn xor_register_to_register() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 0x55;
        cpu.v[0x1] = 0x3C;
        cpu.execute(0x8013);
        assert_eq!(0x69, cpu.v[0x0]);
    }

    #[test]
    fn sub_register_to_register_no_borrow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 21;
        cpu.v[0x1] = 7;
        cpu.execute(0x8015);
        assert_eq!(14, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn sub_register_to_register_borrow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 7;
        cpu.v[0x1] = 21;
        cpu.execute(0x8015);
        assert_eq!(242, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn srl_no_underflow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 32;
        cpu.execute(0x8006);
        assert_eq!(16, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn srl_underflow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 31;
        cpu.execute(0x8006);
        assert_eq!(15, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn subn_no_borrow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 7;
        cpu.v[0x1] = 21;
        cpu.execute(0x8017);
        assert_eq!(14, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn subn_borrow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 21;
        cpu.v[0x1] = 7;
        cpu.execute(0x8017);
        assert_eq!(242, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn sll_no_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 0x7F;
        cpu.execute(0x800E);
        assert_eq!(0xFE, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn sll_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 0xFF;
        cpu.execute(0x800E);
        assert_eq!(0xFE, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn sne_register_skip() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 1;
        cpu.v[0x1] = 2;
        cpu.execute(0x9010);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn sne_register_no_skip() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 1;
        cpu.v[0x1] = 1;
        cpu.execute(0x9010);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn ld_address_register() {
        let mut cpu = Cpu::new();
        cpu.execute(0xAABC);
        assert_eq!(0xABC, cpu.ri);
    }

    #[test]
    fn jp_address_offset() {
        let mut cpu = Cpu::new();
        cpu.v[0x0] = 2;
        cpu.execute(0xBABC);
        assert_eq!(0xABE, cpu.pc);
    }

    #[test]
    fn rnd_supplied_full_mask() {
        let mut cpu = Cpu::new();
        cpu.random = StdRng::seed_from_u64(0x13375EED);
        cpu.execute(0xC0FF);
        assert_eq!(173, cpu.v[0x0]);
    }

    #[test]
    fn rnd_supplied_partial_mask() {
        let mut cpu = Cpu::new();
        cpu.random = StdRng::seed_from_u64(0x13375EED);
        cpu.execute(0xC07E);
        assert_eq!(44, cpu.v[0x0]);
    }

    #[test]
    fn rnd_supplied_no_mask() {
        let mut cpu = Cpu::new();
        cpu.random = StdRng::seed_from_u64(0x13375EED);
        cpu.execute(0xC000);
        assert_eq!(0, cpu.v[0x0]);
    }

    #[test]
    fn drw_two_byte_sprite_no_overlap() {
        let mut cpu = Cpu::new();
        let bytes = &[0x9A, 0x3C];
        cpu.memory.store(0x100, bytes);
        cpu.ri = 0x100;
        cpu.v[0x0] = 2;
        cpu.execute(0xD002);
        assert_eq!(bytes, &cpu.vbuffer[130..132]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn drw_two_byte_sprite_overlap() {
        let mut cpu = Cpu::new();
        let bytes = &[0x9A, 0x3C];
        cpu.memory.store(0x100, bytes);
        cpu.ri = 0x100;
        cpu.v[0x0] = 2;
        cpu.execute(0xD001);
        cpu.ri = 0x101;
        cpu.execute(0xD001);
        assert_eq!(&[0xA6], &cpu.vbuffer[130..131]);
        assert_eq!(1, cpu.v[0xF]);
    }
}
