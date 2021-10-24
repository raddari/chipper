#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChipKey {
    CK_0 = 0,
    CK_1,
    CK_2,
    CK_3,
    CK_4,
    CK_5,
    CK_6,
    CK_7,
    CK_8,
    CK_9,
    CK_A,
    CK_B,
    CK_C,
    CK_D,
    CK_E,
    CK_F,
}

impl ChipKey {
    pub fn from_byte(byte: u8) -> Option<Self> {
        use ChipKey::*;
        match byte {
            0x0 => Some(CK_0),
            0x1 => Some(CK_1),
            0x2 => Some(CK_2),
            0x3 => Some(CK_3),
            0x4 => Some(CK_4),
            0x5 => Some(CK_5),
            0x6 => Some(CK_6),
            0x7 => Some(CK_7),
            0x8 => Some(CK_8),
            0x9 => Some(CK_9),
            0xA => Some(CK_A),
            0xB => Some(CK_B),
            0xC => Some(CK_C),
            0xD => Some(CK_D),
            0xE => Some(CK_E),
            0xF => Some(CK_F),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Keypad {
    pressed_key: Option<ChipKey>,
}

impl Keypad {
    pub fn new() -> Self {
        Keypad { pressed_key: None }
    }

    pub fn press(&mut self, key: ChipKey) {
        self.pressed_key = Some(key);
    }

    pub fn release(&mut self) {
        self.pressed_key = None;
    }

    pub fn is_pressed(&self, key: ChipKey) -> bool {
        match self.pressed_key {
            Some(k) => k == key,
            None => false,
        }
    }

    pub fn get_pressed(&self) -> Option<ChipKey> {
        self.pressed_key
    }
}
