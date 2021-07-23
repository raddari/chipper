use enum_ordinalize::*;

#[derive(Clone, Copy, Ordinalize)]
#[repr(usize)]
pub enum Key {
    ZERO = 0,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    A,
    B,
    C,
    D,
    E,
    F,
}

pub struct Keyboard {
    keystate: [bool; 16],
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard::new()
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keystate: [false; 16],
        }
    }

    pub fn press(&mut self, key: Key) {
        self.keystate[key.ordinal()] = true;
    }

    pub fn release(&mut self, key: Key) {
        self.keystate[key.ordinal()] = false;
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.keystate[key.ordinal()]
    }

    pub fn clear(&mut self) {
        self.keystate.fill(false);
    }
}
