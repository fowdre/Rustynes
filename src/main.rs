mod tests;

mod nes;
use nes::Nes;

pub use raylib::prelude::*;
mod display;
use display::{NesDisplay, FlagsDisplay, InstructionHistoryDisplay, TextBox};

fn main() {
    let mut nes = Nes::new();

    nes.cpu_write(0x8000, 0xA2);
    nes.cpu_write(0x8001, 0x0A);
    nes.cpu_write(0x8002, 0x8E);

    // nes.cpu_tick();

    // println!("{:?}", nes);

    let (mut rl_handle, rl_thread) = raylib::init()
        .size(800, 600)
        .title("Rustyness")
        .build();

    NesDisplay::set_options(&mut rl_handle, 60, 20, true);
    let font = NesDisplay::set_font(&mut rl_handle, &rl_thread, "assets/font/Monocraft.ttf", 25);

    let zero_page = TextBox::new(
        NesDisplay::bytes_to_string(nes.get_ram(0x0000, 0x00F0)),
        Vector2::new(10.0, 10.0),
        Color::WHITE,
        Color::WHITE,
        &font,
    );

    let program_location = TextBox::new(
        NesDisplay::bytes_to_string(nes.get_ram(0x8000, 0x80F0)),
        Vector2::new(10.0, 10.0 + zero_page.get_position().y + zero_page.get_dimensions().y + 5.0),
        Color::WHITE,
        Color::WHITE,
        &font,
    );

    let cpu_info = TextBox::new(
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

    // let history_instruction = InstructionHistoryDisplay::new(
    //     Vector2::new(10.0, 10.0 + zero_page.get_position().y + zero_page.get_dimensions().y + 5.0 + program_location.get_dimensions().y + 5.0),
    //     font,
    // );
    // let mut instruction_display = TextBox::new(
    //     nes.get_next_instruction_string(),
    //     Vector2::new(10.0, 10.0 + zero_page.get_position().y + zero_page.get_dimensions().y + 5.0 + program_location.get_dimensions().y + 5.0),
    //     Color::WHITE,
    //     Color::WHITE,
    //     &font,
    // );

    while !rl_handle.window_should_close() {
        let mut rl_draw_handle = rl_handle.begin_drawing(&rl_thread);

        rl_draw_handle.clear_background(Color::new(50, 50, 50, 255));
        zero_page.draw(&mut rl_draw_handle);
        program_location.draw(&mut rl_draw_handle);
        cpu_info.draw(&mut rl_draw_handle);
        flags_display.draw(&mut rl_draw_handle);
        // instruction_display.draw(&mut rl_draw_handle);
    }
}
