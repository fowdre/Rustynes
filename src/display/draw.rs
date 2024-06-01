use raylib::prelude::*;

use super::super::nes::{Nes, CpuInfo};

const BYTES_PER_LINE: u8 = 40;

pub struct TextBox<'font> {
    outline_rect: Rectangle,
    outline_color: Color,
    text: String,
    bak_text_color: Color,
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
            bak_text_color: text_color,
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

    pub fn set_text(&mut self, text: String, color: Option<Color>) {
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
        if let Some(color) = color {
            self.text_color = color;
        } else {
            self.text_color = self.bak_text_color;
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_dimensions(&self) -> Vector2 {
        Vector2::new(self.outline_rect.width, self.outline_rect.height)
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

    pub fn cpu_info_to_string(cpu_info: CpuInfo) -> String {
        format!(
            "PC:\t{pc:04X}\nSP:\t{sp:04X}\nA:\t\t({a:03}) {a:02X}\nX:\t\t({x:03}) {x:02X}\nY:\t\t({y:03}) {y:02X}\n",
            pc = cpu_info.program_counter,
            sp = cpu_info.stack_pointer,
            a = cpu_info.reg_a,
            x = cpu_info.reg_x,
            y = cpu_info.reg_y
        )
    }
}

#[derive(Copy, Clone)]
struct FlagBox {
    pub box_color: Color,
    pub character: char,
    pub is_set: bool,
}

pub struct FlagsDisplay<'font> {
    flags: [FlagBox; 8],
    position: Vector2,
    outline_rects: [Rectangle; 8],
    outline_color: Color,
    text_color: Color,
    font: &'font Font,   
}

impl<'font> FlagsDisplay<'font> {
    pub fn new(flags: u8, position: Vector2, size: f32, spacing: f32, font: &'font Font) -> Self {
        let mut flags_array = [FlagBox {
            box_color: Color::BLANK,
            character: ' ',
            is_set: false,
        }; 8];
        let flags_chars = ['C', 'Z', 'I', 'D', 'B', 'U', 'V', 'N'];
        for (i, f) in flags_array.iter_mut().enumerate() {
            f.character = flags_chars[i];
            f.is_set = (flags & (1 << i)) != 0;
            if f.is_set {
                f.box_color = Color::BLANK;
            }
        }

        let mut outline_rects = [Rectangle::default(); 8];

        let mut accumulator = 0.0;
        for (i, rect) in outline_rects.iter_mut().enumerate() {
            *rect = Rectangle {
                x: position.x + size * i as f32 + accumulator,
                y: position.y,
                width: size,
                height: size,
            };
            if i == 3 {
                accumulator = 0.0;
            }
            if i > 3 {
                rect.x = position.x + size * (i - 4) as f32 + (accumulator - spacing);
                rect.y += size + spacing;
            }
            accumulator += spacing;
        }

        Self {
            flags: flags_array,
            position,
            outline_rects,
            outline_color: Color::BLANK,
            text_color: Color::BLANK,
            font,
        }
    }

    pub fn set_colors(&mut self, outline: Color, text: Color, flag_active: Color) {
        self.outline_color = outline;
        self.text_color = text;
        for flag in self.flags.iter_mut() {
            flag.box_color = flag_active;
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        for rect in self.outline_rects.iter() {
            handle.draw_rectangle_lines_ex(*rect, 2.0, self.outline_color);
        }

        for (i, flag) in self.flags.iter().enumerate() {
            if flag.is_set {
                handle.draw_rectangle_rec(self.outline_rects[i], flag.box_color);
            }
            handle.draw_rectangle_lines_ex(self.outline_rects[i], 2.0, self.outline_color);
            handle.draw_text_ex(
                self.font,
                &flag.character.to_string(),
                Vector2::new(self.outline_rects[i].x + 10.0, self.outline_rects[i].y + 5.0),
                self.font.base_size() as f32,
                2.0,
                self.text_color,
            );
        }
    }

    pub fn set_flags(&mut self, flags: u8) {
        for (i, flag) in self.flags.iter_mut().enumerate() {
            flag.is_set = (flags & (1 << i)) != 0;
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_dimensions(&self) -> Vector2 {
        Vector2::new(
            self.outline_rects[3].width + self.outline_rects[3].x - self.outline_rects[0].x,
            self.outline_rects[7].height + self.outline_rects[7].y - self.outline_rects[0].y,
        )
    }
}

pub struct InstructionHistoryDisplay<'font> {
    instructions: Vec<String>,
    count: u8,
    position: Vector2,
    font: &'font Font,
}

impl<'font> InstructionHistoryDisplay<'font> {
    pub fn new(position: Vector2, count: u8, font: &'font Font) -> Self {
        Self {
            instructions: Vec::with_capacity(count as usize),
            count,
            position,
            font,
        }
    }

    pub fn update(&mut self, nes: &Nes, pc: u16) {
        self.instructions.clear();

        let tmp = nes.get_instruction_string_range(pc, pc.wrapping_add(self.count as u16));
        for i in tmp {
            self.instructions.push(i);
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        let mut y_offset = 0.0;
        for (i, instruction) in self.instructions.iter().enumerate() {
            let mut color = if i == 0 {
                Color::LIGHTGREEN
            } else {
                Color::WHITE
            };
            if instruction == "00 (IMP) BRK" {
                color = Color::DARKGRAY;
            }
            handle.draw_text_ex(
                self.font,
                instruction,
                Vector2::new(self.position.x, self.position.y + y_offset),
                self.font.base_size() as f32,
                2.0,
                color,
            );
            y_offset += self.font.base_size() as f32 + 5.0;
        }
    }
}
