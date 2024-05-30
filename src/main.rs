mod tests;

mod nes;
use nes::Nes;

mod display;
use display::*;

fn main() {
    let mut nes = Nes::new();

    nes.cpu_tick();

    // println!("{:?}", nes);

    let (mut rl_handle, rl_thread) = raylib::init()
        .size(800, 600)
        .title("Rustyness")
        .build();

    NesDisplay::set_options(&mut rl_handle, 60, 20, true);
    let font = NesDisplay::set_font(&mut rl_handle, &rl_thread, "assets/font/Monocraft.ttf", 25);

    let text_box = TextBox::new(
        "Hello, World!\nHello, World!".to_string(),
        Vector2::new(10.0, 10.0),
        Color::WHITE,
        Color::WHITE,
        &font,
    );

    while !rl_handle.window_should_close() {
        let mut rl_draw_handle = rl_handle.begin_drawing(&rl_thread);

        rl_draw_handle.clear_background(Color::new(50, 50, 50, 255));
        text_box.draw(&mut rl_draw_handle);
    }
}
