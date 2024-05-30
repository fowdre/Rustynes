pub use raylib::prelude::*;

const BYTES_PER_LINE: u8 = 40;

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


#[derive(Clone)]
struct BytesLine {
    range_lower: u16,
    range_upper: u16,
    bytes: Vec<u8>,
}

impl std::fmt::Display for BytesLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bytes = self
            .bytes
            .iter()
            .fold(String::new(), |acc, b| acc + &format!("{b:02X} "));
        bytes.pop();
        bytes.push('\n');
        write!(f, "[{:04X}-{:04X}] {}", self.range_lower, self.range_upper, bytes)
    }
}

pub fn bytes_to_string(bytes_range: (u16, u16, &[u8])) -> String {
    // Why `bytes_range` isn't simply a &[u8] is because the range is needed for the display
    // However if the starting point is different from 0, an offset needs to be applied

    let mut bytes_repr: Vec<BytesLine> = Vec::new();

    // `BytesLine` is a struct that represents a single line of bytes with ranges
    // The `Display` impl is what takes care of the string representation
    let mut local_bytes_line = BytesLine {
        range_lower: bytes_range.0,
        range_upper: bytes_range.1,
        bytes: Vec::new(),
    };
    
    // Basically, create a `BytesLine` for each line of length `BYTES_PER_LINE`
    let mut byte_count = 0;
    for b in bytes_range.2 {
        byte_count += 1;
        local_bytes_line.bytes.push(*b);
        if byte_count % BYTES_PER_LINE as i32 == 0 { // Each `BYTES_PER_LINE` bytes, create a new line
            local_bytes_line.range_lower = bytes_range.0 + (byte_count - BYTES_PER_LINE as i32).clamp(0, byte_count) as u16;
            local_bytes_line.range_upper = bytes_range.0 + (byte_count - 1) as u16;
            bytes_repr.push(local_bytes_line.clone());
            local_bytes_line.bytes.clear();
        }
    }
    // This is for when a line has less than `BYTES_PER_LINE` bytes
    if !local_bytes_line.bytes.is_empty() {
        local_bytes_line.range_lower = bytes_range.0 + (byte_count - local_bytes_line.bytes.len() as i32) as u16;
        local_bytes_line.range_upper = bytes_range.0 + (byte_count - 1) as u16;
        bytes_repr.push(local_bytes_line);
    }

    let mut bytes_repr = bytes_repr
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("");
    bytes_repr.pop(); // Remove the leading `\n`
    bytes_repr
}
