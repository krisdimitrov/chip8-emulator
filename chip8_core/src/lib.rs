///
///
///
pub mod opcodes;
use std::{fs::File, io::Read};

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const START_ADDRESS: u16 = 0x200;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const FONTSET_SIZE: usize = 80;
pub const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    program_counter: u16,
    ram: [u8; RAM_SIZE],
    v_registers: [u8; NUM_REGS],
    i_register: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],
    delay_timer: u8,
    sound_timer: u8,
    keyboard: [bool; 16],

    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut instance = Self {
            program_counter: START_ADDRESS,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_registers: [0; NUM_REGS],
            i_register: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            keyboard: [false; 16],
        };

        instance.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        return instance;
    }

    pub fn stack_push(&mut self, val: u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    pub fn stack_pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        return self.stack[self.stack_pointer as usize];
    }

    pub fn key_press(&mut self, index: usize, pressed: bool) {
        self.keyboard[index] = pressed;
    }

    ///
    /// Loads the provided ROM file into RAM starting at the start address
    ///
    pub fn load_rom(&mut self, file_path: &str) {
        println!("Loading ROM");

        let mut rom = File::open(file_path).expect("Failed to open file");
        let mut buffer = Vec::new();

        rom.read_to_end(&mut buffer).expect("Failed to read file");

        let start = START_ADDRESS as usize;
        let end = (START_ADDRESS as usize) + buffer.len();

        self.ram[start..end].copy_from_slice(&buffer);
    }

    ///
    /// Represents a single clock cycle of the Chip8
    ///
    pub fn tick(&mut self) {
        let op = self.fetch_opcode();
        self.execute(op);
    }

    ///
    /// Fetches the next opcode from RAM and increments
    /// the program counter by 2
    ///
    pub fn fetch_opcode(&mut self) -> u16 {
        let pc = self.program_counter as usize;
        let high_byte = self.ram[pc] as u16;
        let low_byte = self.ram[pc + 1] as u16;
        let op = (high_byte << 8) | low_byte;

        // PC is incremented by 2 after each 16 byte instruction
        self.program_counter += 2;

        return op;
    }

    ///
    /// Executes the provided opcode
    ///
    pub fn execute(&mut self, op: u16) {
        // Extract the digits from the opcode
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0xE, 0) => opcodes::op_00e0(self, op),
            (0, 0, 0xE, 0xE) => opcodes::op_00ee(self, op),
            (1, _, _, _) => opcodes::op_1nnn(self, op),
            (2, _, _, _) => opcodes::op_2nnn(self, op),
            (3, _, _, _) => opcodes::op_3xnn(self, op, digit2),
            (4, _, _, _) => opcodes::op_4xnn(self, op, digit2),
            (5, _, _, 0) => opcodes::op_5xy0(self, op, digit2, digit3),
            (6, _, _, _) => opcodes::op_6xnn(self, op, digit2),
            (7, _, _, _) => opcodes::op_7xnn(self, op, digit2),
            (8, _, _, 0) => opcodes::op_8xy0(self, op, digit2, digit3),
            (8, _, _, 1) => opcodes::op_8xy1(self, op, digit2, digit3),
            (8, _, _, 2) => opcodes::op_8xy2(self, op, digit2, digit3),
            (8, _, _, 3) => opcodes::op_8xy3(self, op, digit2, digit3),
            (0xA, _, _, _) => opcodes::op_annn(self, op),
            (0xD, _, _, _) => opcodes::op_dxyn(self, digit2, digit3, digit4),
            (0xE, _, 9, 0xE) => opcodes::op_ex9e(self, digit2),
            (0xE, _, 0xA, 1) => opcodes::op_exa1(self, digit2),
            (0xF, _, 1, 0xE) => opcodes::op_fx1e(self, digit2),
            (0xF, _, 0, 7) => opcodes::op_fx07(self, digit2),
            (0xF, _, 1, 5) => opcodes::op_fx15(self, digit2),
            (0xF, _, 5, 5) => opcodes::op_fx55(self, digit2),
            (0xF, _, 6, 5) => opcodes::op_fx65(self, digit2),
            (_, _, _, _) => {
                println!("Unimplemented opcode: {:#04x}", op)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_be_defined() {
        let chip = Chip8::new();
        assert_eq!(chip.program_counter, START_ADDRESS);
    }
}
