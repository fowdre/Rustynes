use raylib::prelude::*;
use crate::nes::cartridge::ComponentCartridge;
use crate::constants::*;

#[derive(Debug)]
pub struct ScreenData {
    displayable_screen: [Color; NES_SCREEN_WIDTH as usize * NES_SCREEN_HEIGHT as usize],
    screen_palette: [Color; 64],
    displayable_name_table: [[Color; 256 * 240]; 2],
    displayable_pattern_table: [[Color; 128 * 128]; 2],
}

impl ScreenData {
    pub fn new() -> Self {
        Self {
            displayable_screen: [Color::BLANK; NES_SCREEN_WIDTH as usize * NES_SCREEN_HEIGHT as usize],
            screen_palette: {
                let mut screen_palette = [Color::BLANK; 64];
                screen_palette[0x00] = Color::new(84, 84, 84, 255);
	            screen_palette[0x01] = Color::new(0, 30, 116, 255);
	            screen_palette[0x02] = Color::new(8, 16, 144, 255);
	            screen_palette[0x03] = Color::new(48, 0, 136, 255);
	            screen_palette[0x04] = Color::new(68, 0, 100, 255);
	            screen_palette[0x05] = Color::new(92, 0, 48, 255);
	            screen_palette[0x06] = Color::new(84, 4, 0, 255);
	            screen_palette[0x07] = Color::new(60, 24, 0, 255);
	            screen_palette[0x08] = Color::new(32, 42, 0, 255);
	            screen_palette[0x09] = Color::new(8, 58, 0, 255);
	            screen_palette[0x0A] = Color::new(0, 64, 0, 255);
	            screen_palette[0x0B] = Color::new(0, 60, 0, 255);
	            screen_palette[0x0C] = Color::new(0, 50, 60, 255);
	            screen_palette[0x0D] = Color::new(0, 0, 0, 255);
	            screen_palette[0x0E] = Color::new(0, 0, 0, 255);
	            screen_palette[0x0F] = Color::new(0, 0, 0, 255);

	            screen_palette[0x10] = Color::new(152, 150, 152, 255);
	            screen_palette[0x11] = Color::new(8, 76, 196, 255);
	            screen_palette[0x12] = Color::new(48, 50, 236, 255);
	            screen_palette[0x13] = Color::new(92, 30, 228, 255);
	            screen_palette[0x14] = Color::new(136, 20, 176, 255);
	            screen_palette[0x15] = Color::new(160, 20, 100, 255);
	            screen_palette[0x16] = Color::new(152, 34, 32, 255);
	            screen_palette[0x17] = Color::new(120, 60, 0, 255);
	            screen_palette[0x18] = Color::new(84, 90, 0, 255);
	            screen_palette[0x19] = Color::new(40, 114, 0, 255);
	            screen_palette[0x1A] = Color::new(8, 124, 0, 255);
	            screen_palette[0x1B] = Color::new(0, 118, 40, 255);
	            screen_palette[0x1C] = Color::new(0, 102, 120, 255);
	            screen_palette[0x1D] = Color::new(0, 0, 0, 255);
	            screen_palette[0x1E] = Color::new(0, 0, 0, 255);
	            screen_palette[0x1F] = Color::new(0, 0, 0, 255);

	            screen_palette[0x20] = Color::new(236, 238, 236, 255);
	            screen_palette[0x21] = Color::new(76, 154, 236, 255);
	            screen_palette[0x22] = Color::new(120, 124, 236, 255);
	            screen_palette[0x23] = Color::new(176, 98, 236, 255);
	            screen_palette[0x24] = Color::new(228, 84, 236, 255);
	            screen_palette[0x25] = Color::new(236, 88, 180, 255);
	            screen_palette[0x26] = Color::new(236, 106, 100, 255);
	            screen_palette[0x27] = Color::new(212, 136, 32, 255);
	            screen_palette[0x28] = Color::new(160, 170, 0, 255);
	            screen_palette[0x29] = Color::new(116, 196, 0, 255);
	            screen_palette[0x2A] = Color::new(76, 208, 32, 255);
	            screen_palette[0x2B] = Color::new(56, 204, 108, 255);
	            screen_palette[0x2C] = Color::new(56, 180, 204, 255);
	            screen_palette[0x2D] = Color::new(60, 60, 60, 255);
	            screen_palette[0x2E] = Color::new(0, 0, 0, 255);
	            screen_palette[0x2F] = Color::new(0, 0, 0, 255);

	            screen_palette[0x30] = Color::new(236, 238, 236, 255);
	            screen_palette[0x31] = Color::new(168, 204, 236, 255);
	            screen_palette[0x32] = Color::new(188, 188, 236, 255);
	            screen_palette[0x33] = Color::new(212, 178, 236, 255);
	            screen_palette[0x34] = Color::new(236, 174, 236, 255);
	            screen_palette[0x35] = Color::new(236, 174, 212, 255);
	            screen_palette[0x36] = Color::new(236, 180, 176, 255);
	            screen_palette[0x37] = Color::new(228, 196, 144, 255);
	            screen_palette[0x38] = Color::new(204, 210, 120, 255);
	            screen_palette[0x39] = Color::new(180, 222, 120, 255);
	            screen_palette[0x3A] = Color::new(168, 226, 144, 255);
	            screen_palette[0x3B] = Color::new(152, 226, 180, 255);
	            screen_palette[0x3C] = Color::new(160, 214, 228, 255);
	            screen_palette[0x3D] = Color::new(160, 162, 160, 255);
	            screen_palette[0x3E] = Color::new(0, 0, 0, 255);
	            screen_palette[0x3F] = Color::new(0, 0, 0, 255);

                screen_palette
            },
            displayable_name_table: [[Color::BLANK; 256 * 240]; 2],
            displayable_pattern_table: [[Color::BLANK; 128 * 128]; 2],
        }
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16, color: Color) {
        if x >= NES_SCREEN_WIDTH || y >= NES_SCREEN_HEIGHT {
            return;
        }

        self.displayable_screen[y as usize * NES_SCREEN_WIDTH as usize + x as usize] = color;
    }

    pub fn get_screen(&self) -> &[Color; NES_SCREEN_WIDTH as usize * NES_SCREEN_HEIGHT as usize] {
        &self.displayable_screen
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Component2C02 {

    /// (or VRAM) Background **_layout_**
    name_table: [[u8; 1024]; 2],
    /// Sprites (background & foreground)
    pattern_table: [[u8; 4 * 1024]; 2],
    /// Colors
    pallete_table: [u8; 32],

    /// Row
    scanline: i16,
    /// Column
    cycle: i16,
    
    pub is_frame_complete: bool,
}

impl Component2C02 {
    pub fn new() -> Self {
        Self {
            name_table: [[0; 1024]; 2],
            pattern_table: [[0; 4 * 1024]; 2],
            pallete_table: [0; 32],

            scanline: 0,
            cycle: 0,
            
            is_frame_complete: false,
        }
    }

    pub fn cpu_read(&self, addr: u16, _read_only: bool) -> u8 {
        match addr {
            // Control
            0x0000 => 0,
            // Mask
            0x0001 => 0,
            // Status
            0x0002 => 0,
            // OAM Address
            0x0003 => 0,
            // OAM Data
            0x0004 => 0,
            // Scroll
            0x0005 => 0,
            // PPU Address
            0x0006 => 0,
            // PPU Data
            0x0007 => 0,

            _ => 0,
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            // Control
            0x0000 => 0,
            // Mask
            0x0001 => 0,
            // Status
            0x0002 => 0,
            // OAM Address
            0x0003 => 0,
            // OAM Data
            0x0004 => 0,
            // Scroll
            0x0005 => 0,
            // PPU Address
            0x0006 => 0,
            // PPU Data
            0x0007 => 0,

            _ => 0,
        };
    }

    pub fn ppu_read(&self, mut addr: u16, _read_only: bool, cartridge: &ComponentCartridge) -> u8 {
        let mut data = 0x0000;
        addr &= 0x3FFF;

        // Cartridge has priority over everything else (mappers)
        if cartridge.ppu_read(addr, &mut data) {
            return data;
        }

        data
    }

    pub fn ppu_write(&mut self, mut addr: u16, data: u8, cartridge: &mut ComponentCartridge) {
        addr &= 0x3FFF;

        // Cartridge has priority over everything else (mappers)
        if cartridge.ppu_write(addr, data) {
            return;
        }
    }

    pub fn tick(&mut self, screen: &mut ScreenData) {
        // Random black or white pixels for testing purposes
        screen.draw_pixel(self.cycle as u16, self.scanline as u16, if rand::random() { Color::WHITE } else { Color::BLANK });

        self.cycle += 1;

        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 261 {
                self.scanline = -1;
                self.is_frame_complete = true;
            }
        }
    }
}
