use chip8_core::*;
use std::io::{self, Write};

const TICKS_PER_FRAME: u32 = 10;

fn main() -> io::Result<()> {
    let rom_path = "roms/1-chip8-logo.ch8";
    let mut _chip8 = Chip8::new();

    _chip8.load_rom(&rom_path);
    emulator_loop(&mut _chip8)
}

fn emulator_loop(chip: &mut Chip8) -> io::Result<()> {
    let position = term_cursor::get_pos().unwrap();
    let x = position.0;
    let y = position.1 - 1;
    let mut op_counter = 0;

    while op_counter < chip.op_codes_length {
        clear_screen();
        term_cursor::set_pos(x, y).expect("Position to be set.");

        for _ in 0..TICKS_PER_FRAME {
            chip.tick();
        }

        print_screen(&chip.screen);

        op_counter += 1;

        // Sleep for a short duration to simulate frame delay (optional)
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}

pub fn clear_screen() {
    term_cursor::clear().expect("To clear console.");
    io::stdout().flush().unwrap(); // Ensure the clear command is executed immediately
}

pub fn print_screen(screen: &[bool; 64 * 32]) {
    let mut output = String::new();

    for y in 0..crate::SCREEN_HEIGHT {
        for x in 0..crate::SCREEN_WIDTH {
            let index = y * crate::SCREEN_WIDTH + x;
            if screen[index] {
                output.push(' ');
            } else {
                output.push('█');
            }
        }

        output.push('\n');
    }

    print!("{}", output); // Print the entire screen buffer
    io::stdout().flush().unwrap();
}
