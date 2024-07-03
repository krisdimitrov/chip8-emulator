use rand::random;

///
/// OP Code implementations for CHIP-8 emulator core.
///
use crate::Chip8;

/// Used to mask the address from the opcode and extract the 12 least significant bits
const ADDRESS_MASK: u16 = 0x0FFF;

/// Used to mask the value from the opcode and extract the 8 least significant bits
const VALUE_MASK: u16 = 0xFF;

const FLAG_REGISTER_INDEX: usize = 0xF;

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

///
/// Skip next instruction if Vx != NN
pub fn op_3xnn(chip: &mut Chip8, op: u16, digit2: u16) {
    let x = digit2 as usize;
    let nn = (op & VALUE_MASK) as u8;

    if chip.v_registers[x] == nn {
        chip.program_counter += 2;
    }
}

///
/// Skip next instruction if Vx == NN
pub fn op_4xnn(chip: &mut Chip8, op: u16, digit2: u16) {
    let x = digit2 as usize;
    let nn = (op & VALUE_MASK) as u8;

    if chip.v_registers[x] != nn {
        chip.program_counter += 2;
    }
}

///
/// Skip next instruction if Vx != Vy
pub fn op_5xy0(chip: &mut Chip8, op: u16, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    if chip.v_registers[x] == chip.v_registers[y] {
        chip.program_counter += 2;
    }
}

///
/// Vx = NN
pub fn op_6xnn(chip: &mut Chip8, op: u16, digit2: u16) {
    let x = digit2 as usize;
    let nn = (op & 0xFF) as u8;

    chip.v_registers[x] = nn;
}

///
/// Vx += NN
pub fn op_7xnn(chip: &mut Chip8, op: u16, digit2: u16) {
    let x = digit2;
    let nn = (op & 0xFF) as u8;

    chip.v_registers[x as usize] = chip.v_registers[x as usize].wrapping_add(nn);
}

///
/// Set Vx = Vy
pub fn op_8xy0(chip: &mut Chip8, op: u16, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    chip.v_registers[x] = chip.v_registers[y];
}

///
///
pub fn op_8xy1(chip: &mut Chip8, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    chip.v_registers[x] |= chip.v_registers[y];
}

pub fn op_8xy2(chip: &mut Chip8, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    chip.v_registers[x] &= chip.v_registers[y];
}

pub fn op_8xy3(chip: &mut Chip8, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    chip.v_registers[x] ^= chip.v_registers[y];
}

pub fn op_8xy4(chip: &mut Chip8, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    let (new_value, overflowed) = chip.v_registers[x].overflowing_add(chip.v_registers[y]);
    let flag_value = if overflowed { 1 } else { 0 };

    chip.v_registers[x] = new_value;
    chip.v_registers[FLAG_REGISTER_INDEX] = flag_value;
}

pub fn op_8xy5(chip: &mut Chip8, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    let (new_value, overflowed) = chip.v_registers[x].overflowing_sub(chip.v_registers[y]);
    let flag_value = if overflowed { 0 } else { 1 };

    chip.v_registers[x] = new_value;
    chip.v_registers[FLAG_REGISTER_INDEX] = flag_value;
}

pub fn op_8xy6(chip: &mut Chip8, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    let vy_value = chip.v_registers[y];
    let least_bit = vy_value & 1;

    chip.v_registers[x] = (vy_value >> 1) as u8;
    chip.v_registers[FLAG_REGISTER_INDEX] = least_bit;
}

pub fn op_8xy7(chip: &mut Chip8, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    let (new_value, overflow) = chip.v_registers[y].overflowing_sub(chip.v_registers[x]);
    let flag_value = if overflow { 0 } else { 1 };

    chip.v_registers[x] = new_value;
    chip.v_registers[FLAG_REGISTER_INDEX] = flag_value;
}

pub fn op_8xye(chip: &mut Chip8, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    let vy_value = chip.v_registers[y];
    let least_bit = vy_value & 1;

    chip.v_registers[x] = (vy_value << 1) as u8;
    chip.v_registers[FLAG_REGISTER_INDEX] = least_bit;
}

pub fn op_9xy0(chip: &mut Chip8, digit2: u16, digit3: u16) {
    let x = digit2 as usize;
    let y = digit3 as usize;

    if chip.v_registers[x] != chip.v_registers[y] {
        chip.program_counter += 2;
    }
}

///
/// Set I register to NNN
pub fn op_annn(chip: &mut Chip8, op: u16) {
    let nnn: u16 = op & ADDRESS_MASK;
    chip.i_register = nnn;
}

pub fn op_cxnn(chip: &mut Chip8, op: u16, digit2: u16) {
    let x = digit2 as usize;
    let nn = (op & VALUE_MASK) as u8;
    let random_value: u8 = random();

    chip.v_registers[x] = random_value & nn;
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

///
/// Skip next instruction if key with the value of Vx is pressed
pub fn op_ex9e(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    let vx = chip.v_registers[x];
    let key = chip.keyboard[vx as usize];

    if key {
        chip.program_counter += 2;
    }
}

///
/// Skip next instruction if key with the value of Vx is not pressed
pub fn op_exa1(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    let vx = chip.v_registers[x];
    let key = chip.keyboard[vx as usize];

    if !key {
        chip.program_counter += 2;
    }
}

pub fn op_fx0a(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    let mut key_pressed = false;

    for i in 0..chip.keyboard.len() {
        if (chip.keyboard[i]) {
            key_pressed = true;
            chip.v_registers[x] = i as u8;
            break;
        }
    }

    if !key_pressed {
        chip.program_counter -= 2;
    }
}

pub fn op_fx18(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    chip.sound_timer = chip.v_registers[x];
}

pub fn op_fx29(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    let c = chip.v_registers[x] as u16;

    chip.i_register = c * 5;
}

///
///
pub fn op_fx1e(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    let vx = chip.v_registers[x] as u16;

    chip.i_register = chip.i_register.wrapping_add(vx);
}

///
/// Set Vx = delay timer value
pub fn op_fx07(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    chip.v_registers[x] = chip.delay_timer;
}

///
/// Set Delay Timer = Vx
pub fn op_fx15(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    chip.delay_timer = chip.v_registers[x];
}

pub fn op_fx33(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    let mut vx = chip.v_registers[x] as f32;

    let ones = vx % 10.0;
    vx /= 10.0;

    let tens = vx % 10.0;
    vx /= 10.0;

    let hundreds = vx % 10.0;

    chip.ram[chip.i_register as usize] = hundreds as u8;
    chip.ram[(chip.i_register + 1) as usize] = tens as u8;
    chip.ram[(chip.i_register + 2) as usize] = ones as u8;
}

pub fn op_fx55(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    let i = chip.i_register as usize;

    for idx in 0..=x {
        chip.ram[i + idx] = chip.v_registers[idx];
    }
}

pub fn op_fx65(chip: &mut Chip8, digit2: u16) {
    let x = digit2 as usize;
    let i = chip.i_register as usize;

    for idx in 0..=x {
        chip.v_registers[idx] = chip.ram[i + idx];
    }
}
