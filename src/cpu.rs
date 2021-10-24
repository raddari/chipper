use crate::keyboard::{Key, Keyboard};
use crate::memory::Memory;
use crate::{CHIP8_VBUFFER, CHIP8_WIDTH};
use rand::prelude::*;
use PcResult::*;

const INSTRUCTION_SIZE: usize = 2;

pub struct Cpu {
    pc: usize,
    ri: usize,
    v: [u8; 16],
    dt: u8,
    st: u8,
    memory: Memory,
    keyboard: Keyboard,
    vbuffer: [u8; CHIP8_VBUFFER],
    random: StdRng,
}

enum PcResult {
    Wait,
    Hop,
    Skip,
    Jump(usize),
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
            st: 0,
            memory,
            keyboard,
            vbuffer: [0; CHIP8_VBUFFER],
            random: StdRng::from_entropy(),
        }
    }

    fn tick(&mut self) {
        let bytes = self.memory.load(self.pc, INSTRUCTION_SIZE);
        let instruction = ((bytes[0] as u16) << 8) | (bytes[1] as u16);
        self.decode_execute(instruction);
    }

    fn decode_execute(&mut self, instruction: u16) {
        let nibbles = Self::unpack_nibbles(instruction);
        let nnn = (instruction & 0x0FFF) as usize;
        let kk = (instruction & 0x00FF) as u8;
        let x = nibbles.1;
        let y = nibbles.2;
        let n = nibbles.3;

        let result = match nibbles {
            (0x0, 0x0, 0xC, _)
            | (0x0, 0x0, 0xF, 0xB)
            | (0x0, 0x0, 0xF, 0xC)
            | (0x0, 0x0, 0xF, 0xD)
            | (0x0, 0x0, 0xF, 0xE)
            | (0x0, 0x0, 0xF, 0xF)
            | (0xD, _, _, 0x0)
            | (0xF, _, 0x3, 0x0)
            | (0xF, _, 0x7, 0x5)
            | (0xF, _, 0x8, 0x5) => panic!(
                "Super CHIP-48 instruction not implemented: {:#06x}",
                instruction
            ),
            (0x0, 0x0, 0xE, 0x0) => self.op_00E0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00EE(),
            (0x0, _, _, _) => self.op_0nnn(nnn),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            (0x4, _, _, _) => self.op_4xkk(x, kk),
            (0x5, _, _, 0x0) => self.op_5xy0(x, y),
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
            (0xE, _, 0x9, 0xE) => self.op_Ex9E(x),
            (0xE, _, 0xA, 0x1) => self.op_ExA1(x),
            (0xF, _, 0x0, 0x7) => self.op_Fx07(x),
            (0xF, _, 0x0, 0xA) => self.op_Fx0A(x),
            (0xF, _, 0x1, 0x5) => self.op_Fx15(x),
            (0xF, _, 0x1, 0x8) => self.op_Fx18(x),
            (0xF, _, 0x1, 0xE) => self.op_Fx1E(x),
            (0xF, _, 0x3, 0x3) => self.op_Fx33(x),
            (0xF, _, 0x5, 0x5) => self.op_Fx55(x),
            (0xF, _, 0x6, 0x5) => self.op_Fx65(x),
            _ => panic!("Unknown opcode: {:#06x}", instruction),
        };

        match result {
            Wait => (),
            Hop => self.pc += INSTRUCTION_SIZE,
            Skip => self.pc += 2 * INSTRUCTION_SIZE,
            Jump(dest) => self.pc = dest,
        }
    }

    fn unpack_nibbles(instruction: u16) -> (usize, usize, usize, usize) {
        (
            ((instruction & 0xF000) >> 12) as usize,
            ((instruction & 0x0F00) >> 8) as usize,
            ((instruction & 0x00F0) >> 4) as usize,
            (instruction & 0x000F) as usize,
        )
    }

    fn op_00E0(&mut self) -> PcResult {
        self.vbuffer.fill(0);
        Hop
    }

    fn op_00EE(&mut self) -> PcResult {
        match self.memory.callstack_pop() {
            Some(n) => Jump(n),
            None => Hop,
        }
    }

    fn op_0nnn(&self, _address: usize) -> PcResult {
        // No implementation yet.
        // Possible: use to talk to debugger from the ROM?
        Hop
    }

    fn op_1nnn(&mut self, address: usize) -> PcResult {
        Jump(address)
    }

    fn op_2nnn(&mut self, address: usize) -> PcResult {
        self.memory.callstack_push(self.pc + INSTRUCTION_SIZE);
        Jump(address)
    }

    fn op_3xkk(&mut self, x: usize, kk: u8) -> PcResult {
        self.skip_with_condition(kk == self.v[x])
    }

    fn op_4xkk(&mut self, x: usize, kk: u8) -> PcResult {
        self.skip_with_condition(kk != self.v[x])
    }

    fn op_5xy0(&mut self, x: usize, y: usize) -> PcResult {
        self.skip_with_condition(self.v[x] == self.v[y])
    }

    fn op_6xkk(&mut self, x: usize, kk: u8) -> PcResult {
        self.v[x] = kk;
        Hop
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) -> PcResult {
        self.add_with_overflow(x, kk);
        Hop
    }

    fn op_8xy0(&mut self, x: usize, y: usize) -> PcResult {
        self.v[x] = self.v[y];
        Hop
    }

    fn op_8xy1(&mut self, x: usize, y: usize) -> PcResult {
        self.v[x] |= self.v[y];
        Hop
    }

    fn op_8xy2(&mut self, x: usize, y: usize) -> PcResult {
        self.v[x] &= self.v[y];
        Hop
    }

    fn op_8xy3(&mut self, x: usize, y: usize) -> PcResult {
        self.v[x] ^= self.v[y];
        Hop
    }

    fn op_8xy4(&mut self, x: usize, y: usize) -> PcResult {
        self.add_with_overflow(x, self.v[y]);
        Hop
    }

    fn op_8xy5(&mut self, x: usize, y: usize) -> PcResult {
        self.sub_with_underflow(x, x, self.v[y]);
        Hop
    }

    fn op_8xy6(&mut self, x: usize, _y: usize) -> PcResult {
        self.overflow_flag((self.v[x] & 0x1) == 1);
        self.v[x] >>= 1;
        Hop
    }

    fn op_8xy7(&mut self, x: usize, y: usize) -> PcResult {
        self.sub_with_underflow(x, y, self.v[x]);
        Hop
    }

    fn op_8xyE(&mut self, x: usize, _y: usize) -> PcResult {
        self.overflow_flag((self.v[x] & 0x80) == 0x80);
        self.v[x] <<= 1;
        Hop
    }

    fn op_9xy0(&mut self, x: usize, y: usize) -> PcResult {
        self.skip_with_condition(self.v[x] != self.v[y])
    }

    fn op_Annn(&mut self, address: usize) -> PcResult {
        self.ri = address;
        Hop
    }

    fn op_Bnnn(&mut self, address: usize) -> PcResult {
        Jump(address + (self.v[0x0] as usize))
    }

    fn op_Cxkk(&mut self, x: usize, kk: u8) -> PcResult {
        let value = (self.random.next_u32() as u8) & kk;
        self.v[x] = value;
        Hop
    }

    fn op_Dxyn(&mut self, x: usize, y: usize, n: usize) -> PcResult {
        let sprite = self.memory.load(self.ri as usize, n);
        let flat = Self::flatten_index(self.v[x] as usize, self.v[y] as usize);
        let collision = self.draw_and_check_collision(flat, &sprite);
        self.overflow_flag(collision);
        Hop
    }

    fn op_Ex9E(&mut self, x: usize) -> PcResult {
        self.skip_with_condition(self.check_key(x))
    }

    fn op_ExA1(&mut self, x: usize) -> PcResult {
        self.skip_with_condition(!self.check_key(x))
    }

    fn op_Fx07(&mut self, x: usize) -> PcResult {
        self.v[x] = self.dt;
        Hop
    }

    fn op_Fx0A(&mut self, x: usize) -> PcResult {
        match self.keyboard.get_pressed() {
            Some(k) => {
                self.v[x] = k as u8;
                Hop
            }
            None => Wait,
        }
    }

    fn op_Fx15(&mut self, x: usize) -> PcResult {
        self.dt = self.v[x];
        Hop
    }

    fn op_Fx18(&mut self, x: usize) -> PcResult {
        self.st = self.v[x];
        Hop
    }

    fn op_Fx1E(&mut self, x: usize) -> PcResult {
        self.ri += self.v[x] as usize;
        Hop
    }

    fn op_Fx33(&mut self, x: usize) -> PcResult {
        let vx = self.v[x];
        let bcd = &[(vx / 100) % 10, (vx / 10) % 10, vx % 10];
        self.memory.store(self.ri, bcd);
        Hop
    }

    fn op_Fx55(&mut self, x: usize) -> PcResult {
        self.memory.store(self.ri, &self.v[0..x + 1]);
        Hop
    }

    fn op_Fx65(&mut self, x: usize) -> PcResult {
        let regs = self.memory.load(self.ri, x + 1);
        self.v.copy_from_slice(&regs);
        Hop
    }

    fn check_key(&self, src: usize) -> bool {
        match Key::from_ordinal(self.v[src]) {
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

    fn skip_with_condition(&self, condition: bool) -> PcResult {
        if condition {
            return PcResult::Skip;
        }
        Hop
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

    #[test]
    fn ld_register_to_dt() {
        uses!(cpu);
        cpu.v[0x0] = 45;
        cpu.decode_execute(0xF015);
        assert_eq!(45, cpu.dt);
    }

    #[test]
    fn ld_register_to_st() {
        uses!(cpu);
        cpu.v[0x0] = 45;
        cpu.decode_execute(0xF018);
        assert_eq!(45, cpu.st);
    }

    #[test]
    fn add_register_to_address() {
        uses!(cpu);
        cpu.ri = 24;
        cpu.v[0x0] = 32;
        cpu.decode_execute(0xF01E);
        assert_eq!(56, cpu.ri);
    }

    #[test]
    fn ld_bcd_register() {
        uses!(cpu);
        cpu.v[0x0] = 123;
        cpu.ri = 0x300;
        cpu.decode_execute(0xF033);
        let bcd = cpu.memory.load(0x300, 3);
        assert_eq!(1, bcd[0]);
        assert_eq!(2, bcd[1]);
        assert_eq!(3, bcd[2]);
    }

    #[test]
    fn sd_registers() {
        uses!(cpu);
        cpu.ri = 0x300;
        cpu.v[0x0] = 23;
        cpu.v[0x9] = 2;
        cpu.v[0xF] = 1;
        cpu.decode_execute(0xFF55);
        let mem = cpu.memory.load(0x300, 16);
        assert_eq!(vec![23, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 1], mem);
    }

    #[test]
    fn ld_registers() {
        uses!(cpu);
        cpu.ri = 0x300;
        cpu.memory.store(
            cpu.ri,
            &[23, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 1],
        );
        cpu.decode_execute(0xFF65);
        assert_eq!(23, cpu.v[0x0]);
        assert_eq!(2, cpu.v[0x9]);
        assert_eq!(1, cpu.v[0xF]);
    }
}
