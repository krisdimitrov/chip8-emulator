///
/// OP Code implementations
///
use crate::Chip8;

/// Used to mask the address from the opcode and extract the 12 least significant bits
const ADDRESS_MASK: u16 = 0x0FFF;

///
/// Clear screen.
pub fn op_00e0(chip: &mut Chip8, _op: u16) {
    chip.screen = [false; crate::SCREEN_WIDTH * crate::SCREEN_HEIGHT];
}

///
/// Return from a subroutine
pub fn op_00ee(chip: &mut Chip8, _op: u16) {
    let return_address = chip.stack_pop();
    chip.program_counter = return_address;
}

///
/// Jump
pub fn op_1nnn(chip: &mut Chip8, op: u16) {
    let nnn = op & ADDRESS_MASK;
    chip.program_counter = nnn;
}

///
/// Call subroutine
pub fn op_2nnn(chip: &mut Chip8, op: u16) {
    let nnn = op & ADDRESS_MASK;

    chip.stack_push(chip.program_counter);
    chip.program_counter = nnn;
}

pub fn op_6xnn(chip: &mut Chip8, op: u16, digit2: u16) {
    let x = digit2 as usize;
    let nn = (op & 0xFF) as u8;
    chip.v_registers[x] = nn;
}

///
/// Set I register to NNN
pub fn op_annn(chip: &mut Chip8, op: u16) {
    let nnn: u16 = op & ADDRESS_MASK;
    chip.i_register = nnn;
}

pub fn op_dxyn(chip: &mut Chip8, digit2: u16, digit3: u16, digit4: u16) {
    // Get the (x, y) coords for our sprite
    let x_coord = chip.v_registers[digit2 as usize] as u16;
    let y_coord = chip.v_registers[digit3 as usize] as u16;
    // The last digit determines how many rows high our sprite is
    let num_rows = digit4;

    // Keep track if any pixels were flipped
    let mut flipped = false;
    // Iterate over each row of our sprite
    for y_line in 0..num_rows {
        // Determine which memory address our row's data is stored
        let addr = chip.i_register + y_line as u16;
        let pixels = chip.ram[addr as usize];
        // Iterate over each column in our row
        for x_line in 0..8 {
            // Use a mask to fetch current pixel's bit. Only flip if a 1
            if (pixels & (0b1000_0000 >> x_line)) != 0 {
                // Sprites should wrap around screen, so apply modulo
                let x = (x_coord + x_line) as usize % crate::SCREEN_WIDTH;
                let y = (y_coord + y_line) as usize % crate::SCREEN_HEIGHT;

                // Get our pixel's index in the 1D screen array
                let idx = x + crate::SCREEN_WIDTH * y;
                // Check if we're about to flip the pixel and set
                flipped |= chip.screen[idx];
                chip.screen[idx] ^= true;
            }
        }
    }
    // Populate VF register
    if flipped {
        chip.v_registers[0xF] = 1;
    } else {
        chip.v_registers[0xF] = 0;
    }
}
