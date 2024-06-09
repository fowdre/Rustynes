mod registers;

use raylib::color::Color;
use registers::*;
use crate::constants::*;
use crate::nes::cartridge::{ComponentCartridge, Mirror};

#[derive(Debug)]
pub struct ScreenData {
    pub displayable_screen: [Color; NES_SCREEN_WIDTH as usize * NES_SCREEN_HEIGHT as usize],
    screen_palette: [Color; 64],
    displayable_name_table: [[Color; 256 * 240]; 2],
    pub displayable_pattern_table: [[Color; 128 * 128]; 2],
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

    pub fn draw_pixel_screen(&mut self, x: u16, y: u16, color: Color) {
        if x >= NES_SCREEN_WIDTH || y >= NES_SCREEN_HEIGHT {
            return;
        }

        self.displayable_screen[y as usize * NES_SCREEN_WIDTH as usize + x as usize] = color;
    }

    pub fn draw_pixel_pattern_table(&mut self, index: u8, x: u16, y: u16, color: Color) {
        if x >= 128 || y >= 128 {
            return;
        }

        self.displayable_pattern_table[index as usize][y as usize * 128 + x as usize] = color;
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

    reg_status: RegisterStatus,
    reg_mask: RegisterMask,
    reg_control: RegisterControl,

    /// Knowing if we're writing to the low or high byte
    address_latch: u8,
    ppu_data_buffer: u8,
    ppu_address: u16,
    pub nmi_occurred: bool,

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

            reg_status: RegisterStatus::new(),
            reg_mask: RegisterMask::new(),
            reg_control: RegisterControl::new(),

            address_latch: 0,
            ppu_data_buffer: 0,
            ppu_address: 0,
            nmi_occurred: false,

            scanline: 0,
            cycle: 0,
            
            is_frame_complete: false,
        }
    }

    pub fn cpu_read(&mut self, addr: u16, read_only: bool, cartridge: &ComponentCartridge) -> u8 {
        let mut data = 0x00;

        match addr {
            // Control
            0x0000 => {}
            // Mask
            0x0001 => {}
            // Status
            0x0002 => {
                data = (self.reg_status.into_bits()) & 0xE0 | (self.reg_status.into_bits() & 0x1F);
                self.reg_status.set_vertical_blank(false);
                self.address_latch = 0;
            }
            // OAM Address
            0x0003 => {}
            // OAM Data
            0x0004 => {}
            // Scroll
            0x0005 => {}
            // PPU Address
            0x0006 => {}
            // PPU Data
            0x0007 => {
                data = self.ppu_data_buffer;
                self.ppu_data_buffer = self.ppu_read(self.ppu_address, read_only, cartridge);

                if self.ppu_address >= 0x3F00 {
                    data = self.ppu_data_buffer;
                }

                self.ppu_address = if self.reg_control.increment_mode() { self.ppu_address + 32 } else { self.ppu_address + 1 };
            }

            _ => {}
        };

        data
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8, cartridge: &mut ComponentCartridge) {
        match addr {
            // Control
            0x0000 => self.reg_control = RegisterControl::from_bits(data),
            // Mask
            0x0001 => self.reg_mask = RegisterMask::from_bits(data),
            // Status
            0x0002 => {}
            // OAM Address
            0x0003 => {}
            // OAM Data
            0x0004 => {}
            // Scroll
            0x0005 => {}
            // PPU Address
            0x0006 => {
                if self.address_latch == 0 {
                    self.ppu_address = (((data & 0x3F) as u16) << 8) | (self.ppu_address & 0x00FF);
                    self.address_latch = 1;
                } else {
                    self.ppu_address = (self.ppu_address & 0xFF00) | data as u16;
                    self.address_latch = 0;
                }
            }
            // PPU Data
            0x0007 => {
                self.ppu_write(self.ppu_address, data, cartridge);
                self.ppu_address = if self.reg_control.increment_mode() { self.ppu_address + 32 } else { self.ppu_address + 1 };
            }

            _ => {},
        };
    }

    pub fn ppu_read(&self, mut addr: u16, _read_only: bool, cartridge: &ComponentCartridge) -> u8 {
        let mut data = 0x00;
        addr &= 0x3FFF;

        // Cartridge has priority over everything else (mappers)
        if cartridge.ppu_read(addr, &mut data) {
            return data;
        }
        match addr {
            // Pattern Table range
            0x0000..=0x1FFF => data = self.pattern_table[(addr & 0x1000 >> 12) as usize][(addr & 0x0FFF) as usize],
            // Name Table range
            0x2000..=0x3EFF => {
                addr &= 0x0FFF;
                match cartridge.mirror {
                    Mirror::Vertical => {
                        match addr {
                            0x0000..=0x03FF => data = self.name_table[0][(addr & 0x03FF) as usize],
                            0x0400..=0x07FF => data = self.name_table[1][(addr & 0x03FF) as usize],
                            0x0800..=0x0BFF => data = self.name_table[0][(addr & 0x03FF) as usize],
                            0x0C00..=0x0FFF => data = self.name_table[1][(addr & 0x03FF) as usize],
                            _ => {}
                        }
                    }
                    Mirror::Horizontal => {
                        match addr {
                            0x0000..=0x03FF => data = self.name_table[0][(addr & 0x03FF) as usize],
                            0x0400..=0x07FF => data = self.name_table[1][(addr & 0x03FF) as usize],
                            0x0800..=0x0BFF => data = self.name_table[0][(addr & 0x03FF) as usize],
                            0x0C00..=0x0FFF => data = self.name_table[1][(addr & 0x03FF) as usize],
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            // Palette RAM range
            0x3F00..=0x3FFF => {
                addr &= 0x001F;
                match addr {
                    0x0010 => addr = 0x0000,
                    0x0014 => addr = 0x0004,
                    0x0018 => addr = 0x0008,
                    0x001C => addr = 0x000C,
                    _ => {}
                }
                data = self.pallete_table[addr as usize]
            },
            _ => {}
        };

        data
    }

    pub fn ppu_write(&mut self, mut addr: u16, data: u8, cartridge: &mut ComponentCartridge) {
        addr &= 0x3FFF;

        // Cartridge has priority over everything else (mappers)
        if cartridge.ppu_write(addr, data) {
            return;
        }
        match addr {
            // Pattern Table range
            0x0000..=0x1FFF => self.pattern_table[(addr & 0x1000 >> 12) as usize][(addr & 0x0FFF) as usize] = data,
            // Name Table range
            0x2000..=0x3EFF => {
                addr &= 0x0FFF;
                match cartridge.mirror {
                    Mirror::Vertical => {
                        match addr {
                            0x0000..=0x03FF => self.name_table[0][(addr & 0x03FF) as usize] = data,
                            0x0400..=0x07FF => self.name_table[1][(addr & 0x03FF) as usize] = data,
                            0x0800..=0x0BFF => self.name_table[0][(addr & 0x03FF) as usize] = data,
                            0x0C00..=0x0FFF => self.name_table[1][(addr & 0x03FF) as usize] = data,
                            _ => {}
                        }
                    }
                    Mirror::Horizontal => {
                        match addr {
                            0x0000..=0x03FF => self.name_table[0][(addr & 0x03FF) as usize] = data,
                            0x0400..=0x07FF => self.name_table[1][(addr & 0x03FF) as usize] = data,
                            0x0800..=0x0BFF => self.name_table[0][(addr & 0x03FF) as usize] = data,
                            0x0C00..=0x0FFF => self.name_table[1][(addr & 0x03FF) as usize] = data,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            // Palette RAM range
            0x3F00..=0x3FFF => {
                addr &= 0x001F;
                match addr {
                    0x0010 => addr = 0x0000,
                    0x0014 => addr = 0x0004,
                    0x0018 => addr = 0x0008,
                    0x001C => addr = 0x000C,
                    _ => {}
                }
                self.pallete_table[addr as usize] = data;
            },
            _ => {}
        };
    }

    pub fn tick(&mut self, screen: &mut ScreenData) {
        if self.scanline == -1 && self.cycle == 1 {
            self.reg_status.set_vertical_blank(false);
        }

        if self.scanline == 241 && self.cycle == 1 {
            self.reg_status.set_vertical_blank(true);
            if self.reg_control.enable_nmi() {
                self.nmi_occurred = true;
            }
        }

        // Random black or white pixels for testing purposes
        screen.draw_pixel_screen(self.cycle as u16, self.scanline as u16, if rand::random() { Color::WHITE } else { Color::BLANK });

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

    fn get_palette_color(&self, palette: &ScreenData, palette_index: u8, pixel_index: u8, cartridge: &ComponentCartridge) -> Color {
        palette.screen_palette[(self.ppu_read(0x3F00 + (palette_index << 2) as u16 + pixel_index as u16, false, cartridge) & 0x3F) as usize]
    }

    pub fn fill_pattern_table(&mut self, index: u8, palette_index: u8, screen_data: &mut ScreenData, cartridge: &ComponentCartridge) {
        for y in 0_u16..16 {
            for x in 0_u16..16 {
                let offset = y * 256 + x * 16;
                
                for row in 0_u16..8 {

                    let mut tile_lsb = self.ppu_read(index as u16 * 0x1000 + offset + row, false, cartridge);
                    let mut tile_msb = self.ppu_read(index as u16 * 0x1000 + offset + row + 8, false, cartridge);

                    for col in 0_u16..8 {
                        let pixel = (tile_lsb & 0x01) + ((tile_msb & 0x01) << 1);
                        tile_lsb >>= 1;
                        tile_msb >>= 1;

                        screen_data.draw_pixel_pattern_table(index, x * 8 + (7 - col), y * 8 + row, self.get_palette_color(screen_data, palette_index, pixel, cartridge));
                    }
                }
            }
        };
    }
}
