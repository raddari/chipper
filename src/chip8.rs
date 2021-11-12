pub struct Chip8 {
    pc: u16,
}

impl Chip8 {
    fn new() -> Self {
        Chip8 { pc: 0x200 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn pc_starts_at_0x200() {
        let chip = Chip8::new();
        assert_eq!(0x200, chip.pc);
    }
}
