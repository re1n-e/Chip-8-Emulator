use crate::config::{CHIP_8_MEMORY_SIZE, CHIP_8_DEFAULT_CHARACTER_SET, CHIP_8_DEFAULT_CHARACTER_SET_LOAD_ADDRESS};

pub struct Chip8Memory {
    pub memory: [u8; CHIP_8_MEMORY_SIZE],
}

impl Chip8Memory {
    pub fn new() -> Self {
        let mut memory: [u8; CHIP_8_MEMORY_SIZE] = [0u8; CHIP_8_MEMORY_SIZE];
        
        // Load the default character set into memory at the specified load address
        for (i, &byte) in CHIP_8_DEFAULT_CHARACTER_SET.iter().enumerate() {
            memory[CHIP_8_DEFAULT_CHARACTER_SET_LOAD_ADDRESS + i] = byte;
        }

        Chip8Memory { memory }
    }
    
    // Set a specific memory location to a value
    pub fn chip8_memory_set(&mut self, index: u16, val: u8) {
        self.memory[index as usize] = val;
    }

    // Get the value at a specific memory location
    pub fn chip8_memory_get(&self, index: u16) -> u8 {
        self.memory[index as usize]
    }

    // Fetch an opcode (2 bytes) from memory at the given index
    pub fn chip8_memory_get_opcode(&self, index: u16) -> u16 {
        // Use chip8_memory_get to retrieve two bytes and combine them to form an opcode
        let high_byte = self.chip8_memory_get(index);          // Get the high byte
        let low_byte = self.chip8_memory_get(index + 1);       // Get the low byte

        (high_byte as u16) << 8 | (low_byte as u16)
    }
}

