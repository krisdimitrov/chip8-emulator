///
/// # Chip8 Core
///

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const START_ADDR: u16 = 0x200;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Chip8 {
    pc: u16, // Program Counter
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_regs: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16, // Stack Pointer
    stack: [u16; STACK_SIZE],
    dt: u8, // Delay Timer
    st: u8, // Sound Timer
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_regs: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            dt: 0,
            st: 0,
        }
    }

    /// Pushes a value onto the stack
    pub fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        return self.stack[self.sp as usize];
    }

    pub fn load(&self) {
        println!("Loading ROM");
    }
}
