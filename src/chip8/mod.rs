// use crate::config;
use crate::memory::Chip8Memory;
use crate::registers::Chip8Regsiters;
use crate::stack::Chip8Stack;
use crate::keyboard::Chip8Keyboard;
use crate::screen::Chip8Screen;
use crate::sound::beep;
use crate::config::{CHIP8_PROGRAM_LOAD_ADDRESS, CHIP_8_MEMORY_SIZE, CHIP_8_TOTAL_KEYS, CHIP8_DEFAULT_SPRITE_HEIGHT};
use rand::Rng;

pub struct Chip8 {
    pub chip8_memory: Chip8Memory,
    pub registers: Chip8Regsiters,
    pub chip8_stack: Chip8Stack,
    pub chip8_keyboard: Chip8Keyboard,
    pub chip8_screen: Chip8Screen,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            chip8_memory: Chip8Memory::new(),
            registers: Chip8Regsiters::new(),
            chip8_stack: Chip8Stack::new(),
            chip8_keyboard: Chip8Keyboard::new(),
            chip8_screen: Chip8Screen::new(),     
        }        
    }

    fn chip8_stack_push(&mut self, val: u16) {
        self.chip8_stack.stack[self.registers.sp as usize] = val;
        self.registers.sp += 1;
    }

    fn chip8_stack_pop(&mut self) -> u16 {
        self.registers.sp -= 1;
        self.chip8_stack.stack[self.registers.sp as usize]
    }

    pub fn get_display(&self) -> &[[bool; 64]; 32] {
        &self.chip8_screen.pixels
    }

    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode & execute
        self.execute(op);
    }

    pub fn tick_timers(&mut self) {
        if self.registers.dt > 0 {
            self.registers.dt -= 1;
        }

        if self.registers.st > 0 {
            beep(15000, 10).unwrap();
            self.registers.st -= 1;
        }
    }

    // Load a program into memory, starting at the program load address
    pub fn chip8_load(&mut self, buffer: &[u8], size: usize) {
        assert!(size + CHIP8_PROGRAM_LOAD_ADDRESS < CHIP_8_MEMORY_SIZE);
        for (i, &data) in buffer.iter().enumerate() {
            self.chip8_memory.memory[CHIP8_PROGRAM_LOAD_ADDRESS + i] = data;
        }
        self.registers.pc = CHIP8_PROGRAM_LOAD_ADDRESS as u16;
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.chip8_memory.memory[self.registers.pc as usize] as u16;
        let lower_byte = self.chip8_memory.memory[(self.registers.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.registers.pc += 2;
        op
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // NOP
            (0, 0, 0, 0) => return,
            // CLS
            (0, 0, 0xE, 0) => {
                self.chip8_screen.clear_screen();
            },
            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.chip8_stack_pop();
                self.registers.pc = ret_addr;
            },
            // JMP NNN
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.registers.pc = nnn;
            },
            // CALL NNN
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.chip8_stack_push(self.registers.pc);
                self.registers.pc = nnn;
            },
            // SKIP VX == NN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.registers.v[x] == nn {
                    self.registers.pc += 2;
                }
            },
            // SKIP VX != NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.registers.v[x] != nn {
                    self.registers.pc += 2;
                }
            },
            // SKIP VX == VY
            (5, _, _, _) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.registers.v[x] == self.registers.v[y] {
                    self.registers.pc += 2;
                }
            },
            // VX = NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.registers.v[x] = nn;
            },
            // VX += NN
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.registers.v[x] = self.registers.v[x].wrapping_add(nn);
            },
            // VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers.v[x] = self.registers.v[y];
            },
            // VX |= VY
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers.v[x] |= self.registers.v[y];
            },
            // VX &= VY
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers.v[x] &= self.registers.v[y];
            },
            // VX ^= VY
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers.v[x] ^= self.registers.v[y];
            },
            // VX += VY
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.registers.v[x].overflowing_add(self.registers.v[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.registers.v[x] = new_vx;
                self.registers.v[0xF] = new_vf;
            },
            // VX -= VY
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.registers.v[x].overflowing_sub(self.registers.v[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.registers.v[x] = new_vx;
                self.registers.v[0xF] = new_vf;
            },
            // VX >>= 1
            (8, _, _, 6) => {
                let x = digit2 as usize;
                let lsb = self.registers.v[x] & 1;
                self.registers.v[x] >>= 1;
                self.registers.v[0xF] = lsb;
            },
            // VX = VY - VX
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.registers.v[y].overflowing_sub(self.registers.v[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.registers.v[x] = new_vx;
                self.registers.v[0xF] = new_vf;
            },
            // VX <<= 1
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                let msb = (self.registers.v[x] >> 7) & 1;
                self.registers.v[x] <<= 1;
                self.registers.v[0xF] = msb;
            },
            // SKIP VX != VY
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.registers.v[x] != self.registers.v[y] {
                    self.registers.pc += 2;
                }
            },
            // I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.registers.i = nnn;
            },
            // JMP V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.registers.pc = (self.registers.v[0] as u16) + nnn;
            },
            // VX = rand() & NN
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = rand::thread_rng().gen();
                self.registers.v[x] = rng & nn;
            },
            // DRAW
            (0xD, _, _, _) => {
                // Get the (x, y) coords for our sprite
                let x = self.registers.v[digit2 as usize] as usize;
                let y = self.registers.v[digit3 as usize] as usize;
                // The last digit determines how many rows high our sprite is
                let num = digit4;

                let start = self.registers.i as usize;
                let end = start + num as usize;
                let sprite_iter = &self.chip8_memory.memory[start..end];
                self.registers.v[0x0f] = self.chip8_screen.chip8_screen_draw_sprite(x , y, sprite_iter);
            },
            // SKIP KEY PRESS
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                let vx = self.registers.v[x];
                let key = self.chip8_keyboard.keyboard[vx as usize];
                if key {
                    self.registers.pc += 2;
                }
            },
            // SKIP KEY RELEASE
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                let vx = self.registers.v[x];
                let key = self.chip8_keyboard.keyboard[vx as usize];
                if !key {
                    self.registers.pc += 2;
                }
            },
            // VX = DT
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.registers.v[x] = self.registers.dt;
            },
            // WAIT KEY
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;
                for i in 0..CHIP_8_TOTAL_KEYS {
                    if self.chip8_keyboard.keyboard[i] {
                        self.registers.v[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    // Redo opcode
                    self.registers.pc -= 2;
                }
            },
            // DT = VX
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.registers.dt = self.registers.v[x];
            },
            // ST = VX
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.registers.st = self.registers.v[x];
            },
            // I += VX
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                let vx = self.registers.v[x] as u16;
                self.registers.i = self.registers.i.wrapping_add(vx);
            },
            // I = FONT
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let c = self.registers.v[x] as u16;
                self.registers.i = c * CHIP8_DEFAULT_SPRITE_HEIGHT;
            },
            // BCD
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let vx = self.registers.v[x] as f32;

                // Fetch the hundreds digit by dividing by 100 and tossing the decimal
                let hundreds = (vx / 100.0).floor() as u8;
                // Fetch the tens digit by dividing by 10, tossing the ones digit and the decimal
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                // Fetch the ones digit by tossing the hundreds and the tens
                let ones = (vx % 10.0) as u8;

                self.chip8_memory.memory[self.registers.i as usize] = hundreds;
                self.chip8_memory.memory[(self.registers.i + 1) as usize] = tens;
                self.chip8_memory.memory[(self.registers.i + 2) as usize] = ones;
            },
            // STORE V0 - VX
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let i = self.registers.i as usize;
                for idx in 0..=x {
                    self.chip8_memory.memory[i + idx] = self.registers.v[idx];
                }
            },
            // LOAD V0 - VX
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.registers.i as usize;
                for idx in 0..=x {
                    self.registers.v[idx] = self.chip8_memory.memory[i + idx];
                }
            },
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {:#04x}", op),
        }
    }
}



