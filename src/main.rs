mod tests;

mod nes;
use nes::Nes;

pub use raylib::prelude::*;
mod display;
use display::draw::{NesDisplay, FlagsDisplay, InstructionHistoryDisplay, TextBox};

fn main() {
    let mut nes = Nes::new();

    let test_bytes = [0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0x00, 0xEA, 0xEA, 0xEA];

    for (i, byte) in test_bytes.iter().enumerate() {
        nes.cpu_write(0x8000 + i as u16, *byte);
    }

    let (mut rl_handle, rl_thread) = raylib::init()
        .size(800, 600)
        .title("Rustyness")
        .build();

    NesDisplay::set_options(&mut rl_handle, 60, 20, true);
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
        NesDisplay::cpu_info_to_string(nes.get_cpu_info()),
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
        if rl_handle.is_key_pressed(KeyboardKey::KEY_SPACE) || rl_handle.is_key_down(KeyboardKey::KEY_SPACE) && rl_handle.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            let cycle = nes.get_cpu_info().cycles;
            let set_text_color = match cycle {
                1 => Some(Color::ORANGE),
                0 => Some(Color::LIGHTGREEN),
                _ => None,
            };
            nes.cpu_tick();
            if cycle == 0 {
                zero_page.set_text(NesDisplay::bytes_to_string(nes.get_ram(0x0000, 0x00F0)), None);
                program_location.set_text(NesDisplay::bytes_to_string(nes.get_ram(0x8000, 0x80F0)), None);
                cpu_info.set_text(NesDisplay::cpu_info_to_string(nes.get_cpu_info()), None);
                flags_display.set_flags(nes.get_cpu_flags());
                history_instruction_display.update(&nes, nes.get_cpu_info().program_counter);
            }
            cycles_left_display.set_text(format!("Next in\n[{}] cycles", cycle), set_text_color);
        }
        
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
