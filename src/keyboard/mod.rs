use crate::config::CHIP_8_TOTAL_KEYS;

pub struct Chip8Keyboard {
    pub keyboard: [bool; CHIP_8_TOTAL_KEYS],
}

impl Chip8Keyboard {
    pub fn new() -> Self {
        Chip8Keyboard {   
            keyboard: [false; CHIP_8_TOTAL_KEYS],
        }
    }

    pub fn chip8_keyboard_down(&mut self, key: u8) {
        self.keyboard[key as usize] = true;
    }

    pub fn chip8_keyboard_up(&mut self, key: u8) {
        self.keyboard[key as usize] = false;
    }

    pub fn chip8_keyboard_isdown(&mut self, key: u8) -> bool {
        self.keyboard[key as usize]
    }
}
