use crate::config::CHIP_8_TOTAL_DATA_REGISTER;

pub struct Chip8Regsiters {
    pub v: [u8; CHIP_8_TOTAL_DATA_REGISTER], // v0 to vf register
    pub i: u16,  // index register
    pub dt: u8, // delay timer
    pub st: u8, // sound timer
    pub pc: u16,
    pub sp: u16,
}

impl Chip8Regsiters {
    pub fn new() -> Self {
        Chip8Regsiters {
            v: [0; CHIP_8_TOTAL_DATA_REGISTER],
            i: 0u16,
            dt: 0u8,
            st: 0u8,
            pc: 0u16,
            sp: 0u16,
        }
    }
}
