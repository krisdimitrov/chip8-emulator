use chip8_core::*;

const TICKS_PER_FRAME: u32 = 10;
fn main() {
    let rom_path = "roms/test_opcode.ch8";
    let mut _chip8 = Chip8::new();

    _chip8.load_rom(&rom_path);
    _chip8.

    while true {
        _chip8.tick();
    }
}
