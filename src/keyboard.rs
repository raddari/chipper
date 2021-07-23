use enum_ordinalize::*;

#[derive(Clone, Copy, PartialEq, Eq, Ordinalize)]
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
    pressed_key: Option<Key>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard::new()
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { pressed_key: None }
    }

    pub fn press(&mut self, key: Key) {
        self.pressed_key = Some(key);
    }

    pub fn release(&mut self) {
        self.pressed_key = None;
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        match self.pressed_key {
            Some(k) => k == key,
            None => false,
        }
    }
}
