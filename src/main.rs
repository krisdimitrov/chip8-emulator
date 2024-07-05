extern crate sdl2;

use std::time::Duration;

use audio::SquareWave;
use chip8_core::*;
use sdl2::audio::AudioSpecDesired;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{event::Event, pixels::Color};
use sdl2::{AudioSubsystem, Sdl};

pub const SCALE: u32 = 23;
pub const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
pub const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

const TICKS_PER_FRAME: u32 = 10;

fn main() {
    // Test paths during development
    let paths = [
        "roms/1-chip8-logo.ch8",
        "roms/2-ibm-logo.ch8",
        "roms/3-corax+.ch8",
        "roms/4-flags.ch8",
        "roms/5-quirks.ch8",
        "roms/6-keypad.ch8",
        "roms/test_opcode.ch8",
        "roms/heart_monitor.ch8",
        "roms/chipquarium.ch8",
        "roms/PONG2",
    ];
    let rom_path = paths[9];

    // Setup SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    // Prepare emulator and load ROM
    let mut chip = Chip8::new();
    chip.load_rom(rom_path);

    // Run emulator loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut pause_emulator = false;

    'emulator_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'emulator_loop;
                }
                Event::KeyDown {
                    keymod,
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    if keymod == Mod::LCTRLMOD || keymod == Mod::RCTRLMOD {
                        println!("Resetting emulator");
                        chip.reset();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    pause_emulator = !pause_emulator;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(button) = key_to_button(key) {
                        chip.key_press(button, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(button) = key_to_button(key) {
                        chip.key_press(button, false);
                    }
                }
                _ => (),
            }
        }

        if pause_emulator {
            continue;
        }

        // Perform some work cycles
        for _ in 0..TICKS_PER_FRAME {
            chip.tick();
        }

        if chip.sound_timer > 0 && chip.sound_timer == 1 {
            play_sound(&audio_subsystem);
        }

        chip.tick_timers();
        render(&chip, &mut canvas);
    }
}

fn play_sound(audio_subsystem: &AudioSubsystem) {
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };

    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })
        .unwrap();

    // Start playback
    device.resume();
    std::thread::sleep(Duration::from_millis(100));
}

fn render(chip: &Chip8, canvas: &mut Canvas<Window>) {
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(6, 138, 41));
    canvas.clear();

    let screen_buffer = chip.screen;
    // Now set draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            // Convert our 1D array's index into a 2D (x,y) position
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}

/*
    Keyboard                    Chip-8
    +---+---+---+---+           +---+---+---+---+
    | 1 | 2 | 3 | 4 |           | 1 | 2 | 3 | C |
    +---+---+---+---+           +---+---+---+---+
    | Q | W | E | R |           | 4 | 5 | 6 | D |
    +---+---+---+---+     =>    +---+---+---+---+
    | A | S | D | F |           | 7 | 8 | 9 | E |
    +---+---+---+---+           +---+---+---+---+
    | Z | X | C | V |           | A | 0 | B | F |
    +---+---+---+---+           +---+---+---+---+
*/
fn key_to_button(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
