use crate::config::CHIP_8_TOTAL_STACK_DEPTH;

pub struct Chip8Stack {
    pub stack: [u16; CHIP_8_TOTAL_STACK_DEPTH],
}

impl Chip8Stack {
    pub fn new()->Self{
        Chip8Stack {
            stack: [0u16; CHIP_8_TOTAL_STACK_DEPTH],
        }
    }
}

