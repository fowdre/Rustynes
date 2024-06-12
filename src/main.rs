// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#![allow(clippy::cast_lossless, clippy::similar_names, clippy::module_name_repetitions)]
#![warn(missing_debug_implementations, rust_2018_idioms)]

mod tests;
mod constants;
mod nes;
mod display;

use raylib::prelude::*;
use nes::Nes;
use display::draw::{FlagsDisplay, InstructionHistoryDisplay, NesDisplay, ScreenDisplay, TextBox};

#[allow(clippy::too_many_lines)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let mut nes = Nes::new();
    if args.len() == 1 {
        nes.load_cartridge("./ROMS/nestest.nes");
    } else {
        nes.load_cartridge(&args[1]);
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
        Vector2::new(cycles_left_display.get_position().x - 250.0, 10.0 + zero_page.get_position().y + zero_page.get_dimensions().y + 5.0 + program_location.get_dimensions().y + 5.0),
        28,
        &font,
    );

    let mut screen_display = ScreenDisplay::new(
        Vector2::new(10.0 * 60.0, 10.0 + program_location.get_dimensions().y + program_location.get_position().y),
        Vector2::new(256.0, 240.0),
        3.0,
    );

    let mut pattern_table_display_1 = ScreenDisplay::new(
        Vector2::new(program_location.get_position().x, program_location.get_position().y + program_location.get_dimensions().y + 10.0),
        Vector2::new(128.0, 128.0),
        3.0,
    );

    let mut pattern_table_display_2 = ScreenDisplay::new(
        Vector2::new(pattern_table_display_1.get_position().x, pattern_table_display_1.get_position().y + pattern_table_display_1.get_dimensions().y + 10.0),
        Vector2::new(128.0, 128.0),
        3.0,
    );

    while !rl_handle.window_should_close() {
        let frame_time = rl_handle.get_frame_time();

        // Controls
        nes.controllers[0].controller = 0x00;
        nes.controllers[0].controller |= if rl_handle.is_key_down(KeyboardKey::KEY_Z) { 0x80 } else { 0x00 };
        nes.controllers[0].controller |= if rl_handle.is_key_down(KeyboardKey::KEY_X) { 0x40 } else { 0x00 };
        nes.controllers[0].controller |= if rl_handle.is_key_down(KeyboardKey::KEY_A) { 0x20 } else { 0x00 };
        nes.controllers[0].controller |= if rl_handle.is_key_down(KeyboardKey::KEY_S) { 0x10 } else { 0x00 };
        nes.controllers[0].controller |= if rl_handle.is_key_down(KeyboardKey::KEY_UP) { 0x08 } else { 0x00 };
        nes.controllers[0].controller |= if rl_handle.is_key_down(KeyboardKey::KEY_DOWN) { 0x04 } else { 0x00 };
        nes.controllers[0].controller |= if rl_handle.is_key_down(KeyboardKey::KEY_LEFT) { 0x02 } else { 0x00 };
        nes.controllers[0].controller |= if rl_handle.is_key_down(KeyboardKey::KEY_RIGHT) { 0x01 } else { 0x00 };
        
        // Resume / Pause
        if rl_handle.is_key_pressed(KeyboardKey::KEY_SPACE) {
            nes.pause = !nes.pause;
        }
        // Reset
        if rl_handle.is_key_pressed(KeyboardKey::KEY_R) {
            nes.reset();
        }

        // Palette cycling
        if rl_handle.is_key_pressed(KeyboardKey::KEY_P) {
            nes.cycle_palette();
        }

        if !nes.pause {
            if nes.timer > 0.0 {
                nes.timer -= frame_time;
            } else {
                nes.timer += (1.0 / constants::FPS) - frame_time;

                loop {
                    nes.tick();
                    if nes.is_ppu_frame_complete() {
                        break;
                    }
                }
                nes.set_ppu_frame_complete(false);
            }
        } else if let Some(key) = rl_handle.get_key_pressed() {
            match key {
                // Step into next CPU clock cycle
                KeyboardKey::KEY_C => {
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
                // Step into next PPU frame
                KeyboardKey::KEY_F => {
                    loop {
                        nes.tick();
                        if nes.is_ppu_frame_complete() {
                            break;
                        }
                    }
                    loop {
                        nes.tick();
                        if nes.is_cpu_instruction_complete() {
                            break;
                        }
                    }
                    nes.set_ppu_frame_complete(false);
                    
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
        // history_instruction_display.update(&mut nes);
        cycles_left_display.set_text(format!("Next in\n[{cycle}] cycles"), cycle_text_color);
        screen_display.update(&mut rl_handle, &rl_thread, nes.get_screen());
        pattern_table_display_1.update(&mut rl_handle, &rl_thread, nes.get_pattern_table(0));
        pattern_table_display_2.update(&mut rl_handle, &rl_thread, nes.get_pattern_table(1));
        
        let mut rl_draw_handle = rl_handle.begin_drawing(&rl_thread);

        rl_draw_handle.clear_background(Color::new(50, 50, 50, 255));
        
        zero_page.draw(&mut rl_draw_handle);
        program_location.draw(&mut rl_draw_handle);
        cpu_info.draw(&mut rl_draw_handle);
        flags_display.draw(&mut rl_draw_handle);
        history_instruction_display.draw(&mut rl_draw_handle);
        cycles_left_display.draw(&mut rl_draw_handle);
        screen_display.draw(&mut rl_draw_handle);
        pattern_table_display_1.draw(&mut rl_draw_handle);
        pattern_table_display_2.draw(&mut rl_draw_handle);
    }
}
