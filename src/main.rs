extern crate sdl2;
use std::io::Read;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use chip8::config::{
    CHIP_8_HEIGHT, CHIP_8_WIDTH, CHIP_8_WINDOW_MULTIPLIER, 
    EMULATOR_WINDOW_TITLE, key2btn, CHIP8_TICKS_PER_FRAME,
};
use chip8::chip8::*;
use std::env;
use std::fs::File;

// Main function: Initializes the Chip8, SDL, and handles the event loop
pub fn main() -> Result<(), String> {
    let mut chip8: Chip8 = Chip8::new();
    load_file(&mut chip8);
    // Initialize Chip8 system
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(EMULATOR_WINDOW_TITLE, (CHIP_8_WIDTH * CHIP_8_WINDOW_MULTIPLIER) as u32, (CHIP_8_HEIGHT * CHIP_8_WINDOW_MULTIPLIER) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();
    
    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..}=> {
                    break 'gameloop;
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                    if let Some(k) = key2btn(key) {
                        chip8.chip8_keyboard.chip8_keyboard_down(k as u8);
                    }
                },
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = key2btn(key) {
                        chip8.chip8_keyboard.chip8_keyboard_up(k as u8);
                    }
                },
                _ => ()
            }
        }

        for _ in 0..CHIP8_TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timers();
        draw_screen(&chip8, &mut canvas).unwrap();
    }

    Ok(())
}

// file handle
fn load_file(chip8: &mut Chip8) {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide a file to load");
    }

    // Clone the file name from the arguments
    let file_name = args[1].clone();

    // Open the file in read-binary mode
    let mut file = File::open(&file_name).unwrap();

    // Read the file contents into a Vec<u8>
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    chip8.chip8_load(&buffer, buffer.len());
}

fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) -> Result<(), String>{
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Now set draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for x in 0..CHIP_8_WIDTH {
        for y in 0..CHIP_8_HEIGHT {
            if chip8.chip8_screen.is_set_screen(x, y) {
                canvas.fill_rect(Rect::new(
                    (x * CHIP_8_WINDOW_MULTIPLIER) as i32, 
                    (y * CHIP_8_WINDOW_MULTIPLIER) as i32, 
                    CHIP_8_WINDOW_MULTIPLIER as u32, 
                    CHIP_8_WINDOW_MULTIPLIER as u32))?;
            }
        }
    }
    canvas.present();
    Ok(())
}