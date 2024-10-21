use crate::config::{CHIP_8_HEIGHT, CHIP_8_WIDTH};

pub struct Chip8Screen {
    pub pixels: [[bool; CHIP_8_WIDTH]; CHIP_8_HEIGHT],
}

impl Chip8Screen {
    pub fn new() -> Self {
        Chip8Screen {
            pixels: [[false; CHIP_8_WIDTH]; CHIP_8_HEIGHT],
        }
    }

    pub fn set_screen(&mut self, x: usize, y: usize) {
        self.pixels[y][x] ^= true;
    }

    pub fn is_set_screen(&self, x: usize, y: usize ) -> bool {
        self.pixels[y][x]
    }

    pub fn chip8_screen_draw_sprite(&mut self, x: usize, y: usize, sprite_iter: &[u8]) -> u8 {
        let mut pixel_collison: u8 = 0;
        for (ly, sprite_byte) in sprite_iter.iter().enumerate() {
            for lx in 0..8 {
                if (sprite_byte & (0b10000000 >> lx)) != 0 {
                    let px = (x + lx) % CHIP_8_WIDTH;
                    let py = (y + ly) % CHIP_8_HEIGHT;

                    if self.is_set_screen(px, py) {
                        pixel_collison = 1;
                    }

                    self.set_screen(px, py);
                }
            }
        }
        pixel_collison
    }

    pub fn clear_screen(&mut self) {
        self.pixels = [[false; CHIP_8_WIDTH]; CHIP_8_HEIGHT];
    }
}