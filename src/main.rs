#![allow(clippy::cast_lossless)]

mod nes;
use nes::Nes;

mod display;
use display::draw::{NesDisplay, FlagsDisplay, InstructionHistoryDisplay, TextBox};

use std::io::Read;
use raylib::prelude::*;

const FPS: f32 = 60.0;

fn load_test_rom() -> Vec<u8> {
    let mut rom = Vec::new();
    let mut file = std::fs::File::open("ROMS/nestest.nes").unwrap();
    file.read_to_end(&mut rom).unwrap();
    rom
}

fn main() {
    let mut nes = Nes::new();

    let test_bytes = &load_test_rom()[0x0010..0x0010 + 0x4000];
    #[allow(clippy::cast_possible_truncation)]
    for (i, byte) in test_bytes.iter().enumerate() {
        nes.cpu_write(0x8000 + i as u16, *byte);
        nes.cpu_write(0xC000 + i as u16, *byte);
    }
    nes.reset();

    let (mut rl_handle, rl_thread) = raylib::init()
        .size(800, 600)
        .title("Rustyness")
        .build();

    NesDisplay::set_options(&mut rl_handle, None, 20, true);
    let font = NesDisplay::set_font(&mut rl_handle, &rl_thread, "assets/font/Monocraft.ttf", 25);

    let mut zero_page = TextBox::new(
        NesDisplay::bytes_to_string(nes.get_ram(0x0000, 0x00F0)),
        Vector2::new(10.0, 10.0),
        Color::WHITE,
        Color::WHITE,
        &font,
    );

    let mut program_location = TextBox::new(
        NesDisplay::bytes_to_string(nes.get_ram(0x8000, 0x80F0)),
        Vector2::new(10.0, 10.0 + zero_page.get_position().y + zero_page.get_dimensions().y + 5.0),
        Color::WHITE,
        Color::WHITE,
        &font,
    );

    let mut cpu_info = TextBox::new(
        NesDisplay::cpu_info_to_string(&nes.get_cpu_info()),
        Vector2::new(10.0 + 10.0 + zero_page.get_dimensions().x, 10.0),
        Color::WHITE,
        Color::WHITE,
        &font,
    );

    let mut flags_display = FlagsDisplay::new(
        nes.get_cpu_flags(),
        Vector2::new(10.0 + 10.0 + zero_page.get_dimensions().x, program_location.get_position().y),
        33.0,
        5.0,
        &font,
    );
    flags_display.set_colors(Color::WHITE, Color::WHITE, Color::GREEN);

    let mut cycles_left_display = TextBox::new(
        "Next in\n[0] cycles".to_string(),
        Vector2::new(flags_display.get_position().x, flags_display.get_position().y + flags_display.get_dimensions().y + 9.0),
        Color::WHITE,
        Color::WHITE,
        &font,
    );

    let mut history_instruction_display = InstructionHistoryDisplay::new(
        Vector2::new(cycles_left_display.get_position().x - 150.0, 10.0 + zero_page.get_position().y + zero_page.get_dimensions().y + 5.0 + program_location.get_dimensions().y + 5.0),
        28,
        &font,
    );
    history_instruction_display.update(&nes, nes.get_cpu_info().program_counter);

    while !rl_handle.window_should_close() {
        let frame_time = rl_handle.get_frame_time();
        
        // Resume / Pause
        if rl_handle.is_key_pressed(KeyboardKey::KEY_SPACE) {
            nes.pause = !nes.pause;
        }
        // Reset
        if rl_handle.is_key_pressed(KeyboardKey::KEY_R) {
            nes.reset();
        }

        if !nes.pause {
            if nes.timer > 0.0 {
                nes.timer -= frame_time;
            } else {
                nes.timer += (1.0 / FPS) - frame_time;

                loop {
                    nes.tick();
                    if !nes.is_cpu_instruction_complete() {
                        break;
                    }
                }
            }
        } else if let Some(key) = rl_handle.get_key_pressed() {
            match key {
                // Step into next CPU clock cycle
                KeyboardKey::KEY_S if nes.is_current_tick_cpu() && rl_handle.get_key_pressed() == Some(KeyboardKey::KEY_LEFT_CONTROL) => {
                    loop {
                        nes.tick();
                        if nes.is_cpu_instruction_complete() {
                            break;
                        }
                    }
                }
                // Step by 1 whole CPU instruction
                KeyboardKey::KEY_S => {
                    loop {
                        nes.tick();
                        if nes.is_cpu_instruction_complete() {
                            break;
                        }
                    }
                    loop {
                        nes.tick();
                        if !nes.is_cpu_instruction_complete() {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        let cycle = nes.get_cpu_info().cycles;
        let cycle_text_color = match cycle {
            1 => Some(Color::ORANGE),
            0 => Some(Color::LIGHTGREEN),
            _ => None,
        };

        zero_page.set_text(NesDisplay::bytes_to_string(nes.get_ram(0x0000, 0x00F0)), None);
        program_location.set_text(NesDisplay::bytes_to_string(nes.get_ram(0x8000, 0x80F0)), None);
        cpu_info.set_text(NesDisplay::cpu_info_to_string(&nes.get_cpu_info()), None);
        flags_display.set_flags(nes.get_cpu_flags());
        history_instruction_display.update(&nes, nes.get_cpu_info().program_counter);
        cycles_left_display.set_text(format!("Next in\n[{cycle}] cycles"), cycle_text_color);
        
        let mut rl_draw_handle = rl_handle.begin_drawing(&rl_thread);

        rl_draw_handle.clear_background(Color::new(50, 50, 50, 255));
        
        zero_page.draw(&mut rl_draw_handle);
        program_location.draw(&mut rl_draw_handle);
        cpu_info.draw(&mut rl_draw_handle);
        flags_display.draw(&mut rl_draw_handle);
        history_instruction_display.draw(&mut rl_draw_handle);
        cycles_left_display.draw(&mut rl_draw_handle);
    }
}
