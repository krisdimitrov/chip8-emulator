///
/// OP Code implementations
///
use crate::Chip8;

/// Used to mask the address from the opcode and extract the 12 least significant bits
const ADDRESS_MASK: u16 = 0x0FFF;

pub fn op_00e0(chip: &mut Chip8, _op: u16) {
    chip.screen = [false; 64 * 32];
}

pub fn op_2nnn(chip: &mut Chip8, op: u16) {
    let nnn = op & ADDRESS_MASK;

    chip.stack_push(chip.program_counter);
    chip.program_counter = nnn;
}
