pub use raylib::prelude::*;

pub struct NesDisplay;

impl NesDisplay {
    pub fn set_options(handle: &mut RaylibHandle, fps: u32, line_spacing: i32, fullscreen: bool) {
        handle.set_target_fps(fps);
        handle.set_text_line_spacing(line_spacing);
        if fullscreen {
            handle.toggle_fullscreen();
        }
    }
    
    pub fn set_font(handle: &mut RaylibHandle, thread: &RaylibThread, path: &str, size: i32) -> Font {
        handle.load_font_ex(thread, path, size, None).expect("Could not load font")
    }
}

pub struct TextBox<'font> {
    outline_rect: Rectangle,
    outline_color: Color,
    text: String,
    text_color: Color,
    position: Vector2,
    font: &'font Font,
}

impl<'font> TextBox<'font> {
    pub fn new(text: String, position: Vector2, text_color: Color, outline_color: Color, font: &'font Font) -> Self {
        let text_dimensions = text
            .lines()
            .map(|line| font.measure_text(line, font.base_size() as f32, 2.0).x as i32)
            .max()
            .unwrap_or(0);

        let outline_rect = Rectangle {
            x: position.x,
            y: position.y,
            width: (10 + text_dimensions) as f32,
            height: 10.0 + font.measure_text(&text, font.base_size() as f32, 2.0).y,
        };

        Self {
            outline_rect,
            outline_color,
            text,
            text_color,
            position,
            font,
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        handle.draw_rectangle_lines_ex(self.outline_rect, 2.0, self.outline_color);

        handle.draw_text_ex(
            self.font,
            &self.text,
            Vector2::new(self.position.x + 5.0, self.position.y + 5.0),
            self.font.base_size() as f32,
            2.0,
            self.text_color,
        );
    }

    #[allow(dead_code)]
    fn set_text(&mut self, text: String) {
        let text_dimensions = text
            .lines()
            .map(|line| self.font.measure_text(line, self.font.base_size() as f32, 2.0).x as i32)
            .max()
            .unwrap_or(0);

        let outline_rect = Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: (10 + text_dimensions) as f32,
            height: 10.0 + self.font.measure_text(&text, self.font.base_size() as f32, 2.0).y,
        };

        self.outline_rect = outline_rect;
        self.text = text;
    }
}
