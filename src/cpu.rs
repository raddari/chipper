use crate::keyboard::{Key, Keyboard};
use crate::memory::Memory;
use crate::opcode::Opcode;
use crate::{CHIP8_VBUFFER, CHIP8_WIDTH};
use rand::prelude::{SeedableRng, StdRng};
use rand::RngCore;

pub struct Cpu {
    pc: usize,
    ri: usize,
    v: [u8; 16],
    dt: u8,
    memory: Memory,
    keyboard: Keyboard,
    vbuffer: [u8; CHIP8_VBUFFER],
    random: StdRng,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new(Memory::new(), Keyboard::new())
    }
}

#[allow(non_snake_case)]
impl Cpu {
    pub fn new(memory: Memory, keyboard: Keyboard) -> Self {
        Cpu {
            pc: 0x200,
            ri: 0,
            v: [0; 16],
            dt: 0,
            memory,
            keyboard,
            vbuffer: [0; CHIP8_VBUFFER],
            random: StdRng::from_entropy(),
        }
    }

    fn tick(&mut self) {
        let bytes = self.memory.load(self.pc as usize, 2);
        let instruction = ((bytes[0] as u16) << 8) | (bytes[1] as u16);
        self.decode_execute(instruction);
    }

    fn decode_execute(&mut self, instruction: u16) {
        match Opcode::decode(instruction) {
            Ok(op) => self.execute(op),
            Err(_) => self.pc += 2,
        }
    }

    fn execute(&mut self, opcode: Opcode) {
        self.pc += 2;
        match opcode {
            Opcode::OP_00E0 {} => self.op_00E0(),
            Opcode::OP_00EE {} => self.op_00EE(),
            Opcode::OP_0nnn { nnn } => self.op_0nnn(nnn),
            Opcode::OP_1nnn { nnn } => self.op_1nnn(nnn),
            Opcode::OP_2nnn { nnn } => self.op_2nnn(nnn),
            Opcode::OP_3xkk { x, kk } => self.op_3xkk(x, kk),
            Opcode::OP_4xkk { x, kk } => self.op_4xkk(x, kk),
            Opcode::OP_5xy0 { x, y } => self.op_5xy0(x, y),
            Opcode::OP_6xkk { x, kk } => self.op_6xkk(x, kk),
            Opcode::OP_7xkk { x, kk } => self.op_7xkk(x, kk),
            Opcode::OP_8xy0 { x, y } => self.op_8xy0(x, y),
            Opcode::OP_8xy1 { x, y } => self.op_8xy1(x, y),
            Opcode::OP_8xy2 { x, y } => self.op_8xy2(x, y),
            Opcode::OP_8xy3 { x, y } => self.op_8xy3(x, y),
            Opcode::OP_8xy4 { x, y } => self.op_8xy4(x, y),
            Opcode::OP_8xy5 { x, y } => self.op_8xy5(x, y),
            Opcode::OP_8xy6 { x, y } => self.op_8xy6(x, y),
            Opcode::OP_8xy7 { x, y } => self.op_8xy7(x, y),
            Opcode::OP_8xyE { x, y } => self.op_8xyE(x, y),
            Opcode::OP_9xy0 { x, y } => self.op_9xy0(x, y),
            Opcode::OP_Annn { nnn } => self.op_Annn(nnn),
            Opcode::OP_Bnnn { nnn } => self.op_Bnnn(nnn),
            Opcode::OP_Cxkk { x, kk } => self.op_Cxkk(x, kk),
            Opcode::OP_Dxyn { x, y, n } => self.op_Dxyn(x, y, n),
            Opcode::OP_Ex9E { x } => self.op_Ex9E(x),
            Opcode::OP_ExA1 { x } => self.op_ExA1(x),
            Opcode::OP_Fx07 { x } => self.op_Fx07(x),
            _ => (),
        };
    }

    fn op_00E0(&mut self) {
        self.vbuffer.fill(0);
    }

    fn op_00EE(&mut self) {
        self.pc = match self.memory.callstack_pop() {
            Some(n) => n,
            None => self.pc,
        };
    }

    fn op_0nnn(&self, _address: usize) {
        // No implementation yet.
        // Possible: use to talk to debugger from the ROM?
    }

    fn op_1nnn(&mut self, address: usize) {
        self.pc = address;
    }

    fn op_2nnn(&mut self, address: usize) {
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

    fn op_Annn(&mut self, address: usize) {
        self.ri = address;
    }

    fn op_Bnnn(&mut self, address: usize) {
        self.pc = address + self.v[0x0] as usize;
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

    fn op_Ex9E(&mut self, x: usize) {
        if self.check_key(x) {
            self.pc += 2;
        }
    }

    fn op_ExA1(&mut self, x: usize) {
        if !self.check_key(x) {
            self.pc += 2;
        }
    }

    fn op_Fx07(&mut self, x: usize) {
        self.v[x] = self.dt;
    }

    fn check_key(&self, src: usize) -> bool {
        match Key::from_ordinal(self.v[src] as usize) {
            Some(key) => self.keyboard.is_pressed(key),
            None => false,
        }
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
#[allow(unused_mut)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! uses {
        ($cpu_var:ident) => {
            let mut $cpu_var = Cpu::new(Memory::new(), Keyboard::new());
        };
    }

    #[test]
    fn pc_starts_at_0x200() {
        uses!(cpu);
        assert_eq!(0x200, cpu.pc);
    }

    #[test]
    fn decode_execute_normally_increments_pc() {
        uses!(cpu);
        cpu.decode_execute(0x6000);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn ld_constant_to_register() {
        uses!(cpu);
        cpu.decode_execute(0x6075);
        assert_eq!(0x75, cpu.v[0x0]);
    }

    #[test]
    fn add_constant_to_register_normal() {
        uses!(cpu);
        cpu.v[0x0] = 1;
        cpu.decode_execute(0x7001);
        assert_eq!(2, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn add_constant_to_register_overflow() {
        uses!(cpu);
        cpu.v[0x0] = 0xFF;
        cpu.decode_execute(0x7001);
        assert_eq!(0, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn add_register_to_register_normal() {
        uses!(cpu);
        cpu.v[0x0] = 1;
        cpu.v[0x1] = 2;
        cpu.decode_execute(0x8014);
        assert_eq!(3, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn add_register_to_register_overflow() {
        uses!(cpu);
        cpu.v[0x0] = 0xFF;
        cpu.v[0x1] = 1;
        cpu.decode_execute(0x8014);
        assert_eq!(0, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn jp_sets_pc() {
        uses!(cpu);
        cpu.decode_execute(0x1ABC);
        assert_eq!(0xABC, cpu.pc);
    }

    #[test]
    fn call_sets_pc() {
        uses!(cpu);
        cpu.decode_execute(0x2ABC);
        assert_eq!(0xABC, cpu.pc);
    }

    #[test]
    fn ret_pops_pc() {
        uses!(cpu);
        cpu.decode_execute(0x2ABC);
        cpu.decode_execute(0x00EE);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn call_ret_nested() {
        uses!(cpu);
        cpu.decode_execute(0x2678);
        cpu.decode_execute(0x2ABC);
        assert_eq!(0xABC, cpu.pc);

        cpu.decode_execute(0x00EE);
        assert_eq!(0x67A, cpu.pc);

        cpu.decode_execute(0x00EE);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn se_constant_skip() {
        uses!(cpu);
        cpu.v[0x0] = 32;
        cpu.decode_execute(0x3020);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn se_constant_no_skip() {
        uses!(cpu);
        cpu.v[0x0] = 32;
        cpu.decode_execute(0x3021);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn sne_constant_skip() {
        uses!(cpu);
        cpu.v[0x0] = 32;
        cpu.decode_execute(0x4021);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn sne_constant_no_skip() {
        uses!(cpu);
        cpu.v[0x0] = 32;
        cpu.decode_execute(0x4020);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn se_register_skip() {
        uses!(cpu);
        cpu.v[0x0] = 32;
        cpu.v[0x1] = 32;
        cpu.decode_execute(0x5010);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn se_register_no_skip() {
        uses!(cpu);
        cpu.v[0x0] = 32;
        cpu.v[0x1] = 33;
        cpu.decode_execute(0x5010);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn ld_register_to_register() {
        uses!(cpu);
        cpu.v[0x0] = 32;
        cpu.decode_execute(0x8100);
        assert_eq!(32, cpu.v[0x1]);
    }

    #[test]
    fn or_register_to_register() {
        uses!(cpu);
        cpu.v[0x0] = 0x55;
        cpu.v[0x1] = 0x3C;
        cpu.decode_execute(0x8011);
        assert_eq!(0x7D, cpu.v[0x0]);
    }

    #[test]
    fn and_register_to_register() {
        uses!(cpu);
        cpu.v[0x0] = 0x55;
        cpu.v[0x1] = 0x3C;
        cpu.decode_execute(0x8012);
        assert_eq!(0x14, cpu.v[0x0]);
    }

    #[test]
    fn xor_register_to_register() {
        uses!(cpu);
        cpu.v[0x0] = 0x55;
        cpu.v[0x1] = 0x3C;
        cpu.decode_execute(0x8013);
        assert_eq!(0x69, cpu.v[0x0]);
    }

    #[test]
    fn sub_register_to_register_no_borrow() {
        uses!(cpu);
        cpu.v[0x0] = 21;
        cpu.v[0x1] = 7;
        cpu.decode_execute(0x8015);
        assert_eq!(14, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn sub_register_to_register_borrow() {
        uses!(cpu);
        cpu.v[0x0] = 7;
        cpu.v[0x1] = 21;
        cpu.decode_execute(0x8015);
        assert_eq!(242, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn srl_no_underflow() {
        uses!(cpu);
        cpu.v[0x0] = 32;
        cpu.decode_execute(0x8006);
        assert_eq!(16, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn srl_underflow() {
        uses!(cpu);
        cpu.v[0x0] = 31;
        cpu.decode_execute(0x8006);
        assert_eq!(15, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn subn_no_borrow() {
        uses!(cpu);
        cpu.v[0x0] = 7;
        cpu.v[0x1] = 21;
        cpu.decode_execute(0x8017);
        assert_eq!(14, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn subn_borrow() {
        uses!(cpu);
        cpu.v[0x0] = 21;
        cpu.v[0x1] = 7;
        cpu.decode_execute(0x8017);
        assert_eq!(242, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn sll_no_overflow() {
        uses!(cpu);
        cpu.v[0x0] = 0x7F;
        cpu.decode_execute(0x800E);
        assert_eq!(0xFE, cpu.v[0x0]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn sll_overflow() {
        uses!(cpu);
        cpu.v[0x0] = 0xFF;
        cpu.decode_execute(0x800E);
        assert_eq!(0xFE, cpu.v[0x0]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn sne_register_skip() {
        uses!(cpu);
        cpu.v[0x0] = 1;
        cpu.v[0x1] = 2;
        cpu.decode_execute(0x9010);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn sne_register_no_skip() {
        uses!(cpu);
        cpu.v[0x0] = 1;
        cpu.v[0x1] = 1;
        cpu.decode_execute(0x9010);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn ld_address_register() {
        uses!(cpu);
        cpu.decode_execute(0xAABC);
        assert_eq!(0xABC, cpu.ri);
    }

    #[test]
    fn jp_address_offset() {
        uses!(cpu);
        cpu.v[0x0] = 2;
        cpu.decode_execute(0xBABC);
        assert_eq!(0xABE, cpu.pc);
    }

    #[test]
    fn rnd_supplied_full_mask() {
        uses!(cpu);
        cpu.random = StdRng::seed_from_u64(0x13375EED);
        cpu.decode_execute(0xC0FF);
        assert_eq!(173, cpu.v[0x0]);
    }

    #[test]
    fn rnd_supplied_partial_mask() {
        uses!(cpu);
        cpu.random = StdRng::seed_from_u64(0x13375EED);
        cpu.decode_execute(0xC07E);
        assert_eq!(44, cpu.v[0x0]);
    }

    #[test]
    fn rnd_supplied_no_mask() {
        uses!(cpu);
        cpu.random = StdRng::seed_from_u64(0x13375EED);
        cpu.decode_execute(0xC000);
        assert_eq!(0, cpu.v[0x0]);
    }

    #[test]
    fn drw_two_byte_sprite_no_overlap() {
        uses!(cpu);
        let bytes = &[0x9A, 0x3C];
        cpu.memory.store(0x100, bytes);
        cpu.ri = 0x100;
        cpu.v[0x0] = 2;
        cpu.decode_execute(0xD002);
        assert_eq!(bytes, &cpu.vbuffer[130..132]);
        assert_eq!(0, cpu.v[0xF]);
    }

    #[test]
    fn drw_two_byte_sprite_overlap() {
        uses!(cpu);
        let bytes = &[0x9A, 0x3C];
        cpu.memory.store(0x100, bytes);
        cpu.ri = 0x100;
        cpu.v[0x0] = 2;
        cpu.decode_execute(0xD001);
        cpu.ri = 0x101;
        cpu.decode_execute(0xD001);
        assert_eq!(&[0xA6], &cpu.vbuffer[130..131]);
        assert_eq!(1, cpu.v[0xF]);
    }

    #[test]
    fn cls_empties_vbuffer() {
        uses!(cpu);
        let bytes = &[0x9A, 0x3C];
        cpu.memory.store(0x100, bytes);
        cpu.ri = 0x100;
        cpu.v[0x0] = 2;
        cpu.decode_execute(0xD002);
        cpu.decode_execute(0x00E0);
        assert_eq!(&[0x0, 0x0], &cpu.vbuffer[130..132]);
    }

    #[test]
    fn skp_register_keyboard_skip() {
        uses!(cpu);
        cpu.v[0x0] = 0xB;
        cpu.keyboard.press(Key::B);
        cpu.decode_execute(0xE09E);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn skp_register_keyboard_no_skip() {
        uses!(cpu);
        cpu.v[0x0] = 0xB;
        cpu.keyboard.press(Key::C);
        cpu.decode_execute(0xE09E);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn sknp_register_keyboard_skip() {
        uses!(cpu);
        cpu.v[0x0] = 0xB;
        cpu.keyboard.press(Key::C);
        cpu.decode_execute(0xE0A1);
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn sknp_register_keyboard_no_skip() {
        uses!(cpu);
        cpu.v[0x0] = 0xB;
        cpu.keyboard.press(Key::B);
        cpu.decode_execute(0xE0A1);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn ld_dt_to_register() {
        uses!(cpu);
        cpu.dt = 3;
        cpu.decode_execute(0xF007);
        assert_eq!(3, cpu.v[0x0]);
    }

    #[test]
    fn ld_register_wait_for_key() {
        uses!(cpu);
        cpu.memory.store(0x200, &[0xF0, 0x0A]);
        cpu.tick();
        assert_eq!(0x200, cpu.pc);

        cpu.keyboard.press(Key::B);
        cpu.tick();
        assert_eq!(0x202, cpu.pc);
        assert_eq!(0xB, cpu.v[0x0]);
    }
}
