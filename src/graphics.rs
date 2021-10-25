pub const GRAPHICS_WIDTH: usize = 64;
pub const GRAPHICS_HEIGHT: usize = 32;
pub const GRAPHICS_VBUFFER: usize = (GRAPHICS_WIDTH * GRAPHICS_HEIGHT) / 8;

#[derive(Default)]
pub struct Graphics {
    vbuffer: Vec<u8>,
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            vbuffer: vec![0; GRAPHICS_VBUFFER],
        }
    }

    pub fn clear(&mut self) {
        self.vbuffer = vec![0; GRAPHICS_VBUFFER]
    }

    pub fn read_buffer(&self, row: usize, col: usize, len: usize) -> Vec<u8> {
        let index = Self::flatten_index(row, col);
        self.vbuffer[index..index + len].to_vec()
    }

    pub fn draw_with_collision(&mut self, row: usize, col: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        let index = Self::flatten_index(row, col);
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
        y * GRAPHICS_WIDTH + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn draw_with_collision_no_overlap() {
        let mut gfx = Graphics::new();
        let bytes = &[0x9A, 0x3C];
        let collision = gfx.draw_with_collision(2, 2, bytes);
        assert_eq!(bytes, &gfx.vbuffer[130..132]);
        assert_eq!(false, collision);
    }

    #[test]
    fn draw_with_collision_overlap() {
        let mut gfx = Graphics::new();
        let bytes = &[0x9A, 0x3C];
        gfx.draw_with_collision(2, 2, &[bytes[0]]);
        let collision = gfx.draw_with_collision(2, 2, &[bytes[1]]);
        assert_eq!(0xA6, gfx.vbuffer[130]);
        assert_eq!(true, collision);
    }
}
