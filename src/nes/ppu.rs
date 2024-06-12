#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]

mod registers;
mod oam;

use raylib::color::Color;
use crate::constants::{NES_SCREEN_HEIGHT, NES_SCREEN_WIDTH};
use crate::nes::cartridge::{ComponentCartridge, Mirror};
use registers::{RegisterControl, RegisterLoopy, RegisterMask, RegisterStatus};
use oam::{EntryOA, OAM};

#[derive(Debug)]
pub struct ScreenData {
    pub displayable_screen: Box<[Color]>,
    screen_palette: Box<[Color; 64]>,
    #[allow(dead_code)]
    displayable_name_table: Box<[Box<[Color]>; 2]>,
    pub displayable_pattern_table: Box<[Box<[Color]>; 2]>,
}

impl ScreenData {
    pub fn new() -> Self {
        Self {
            // displayable_screen: [Color::BLANK; NES_SCREEN_WIDTH as usize * NES_SCREEN_HEIGHT as usize],
            // displayable_screen: Box::new([Color::BLANK; NES_SCREEN_WIDTH as usize * NES_SCREEN_HEIGHT as usize]),
            displayable_screen: vec![Color::BLANK; NES_SCREEN_WIDTH as usize * NES_SCREEN_HEIGHT as usize].into_boxed_slice(),
            screen_palette: {
                // let mut screen_palette = [Color::BLANK; 64];
                let mut screen_palette = Box::new([Color::BLANK; 64]);
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

                // screen_palette
                screen_palette
            },
            displayable_name_table: Box::new([vec![Color::BLANK; 256 * 240].into_boxed_slice(), vec![Color::BLANK; 256 * 240].into_boxed_slice()]),
            displayable_pattern_table: Box::new([vec![Color::BLANK; 128 * 128].into_boxed_slice(), vec![Color::BLANK; 128 * 128].into_boxed_slice()]),
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

    pub nmi_occurred: bool,
    /// Knowing if we're writing to the low or high byte
    address_latch: u8,
    ppu_data_buffer: u8,
    
    vram_addr: RegisterLoopy,
    tram_addr: RegisterLoopy,
    fine_x: u8,

    // Background
    bg_next_tile_id: u8,
    bg_next_tile_attribute: u8,
    bg_next_tile_lsb: u8,
    bg_next_tile_msb: u8,
    bg_shifter_pattern_lo: u16,
    bg_shifter_pattern_hi: u16,
    bg_shifter_attribute_lo: u16,
    bg_shifter_attribute_hi: u16,

    /// (O)bject (A)ttribute (M)emory i.e. Sprites RAM
    pub oam: OAM,
    sprites_scanline: [EntryOA; 8],
    sprite_count: u8,
    sprite_shifter_pattern_lo: [u8; 8],
    sprite_shifter_pattern_hi: [u8; 8],

    is_sprite_zero_hit_possible: bool,
    is_sprite_zero_being_rendered: bool,

    /// Row
    scanline: i16,
    /// Column
    cycle: i16,
    
    pub is_frame_complete: bool,
}

impl Component2C02 {
    pub const fn new() -> Self {
        Self {
            name_table: [[0; 1024]; 2],
            pattern_table: [[0; 4 * 1024]; 2],
            pallete_table: [0; 32],

            reg_status: RegisterStatus::new(),
            reg_mask: RegisterMask::new(),
            reg_control: RegisterControl::new(),

            nmi_occurred: false,
            address_latch: 0,
            ppu_data_buffer: 0,

            vram_addr: RegisterLoopy::new(),
            tram_addr: RegisterLoopy::new(),
            fine_x: 0,

            bg_next_tile_id: 0,
            bg_next_tile_attribute: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,
            bg_shifter_pattern_lo: 0,
            bg_shifter_pattern_hi: 0,
            bg_shifter_attribute_lo: 0,
            bg_shifter_attribute_hi: 0,

            oam: OAM::new(),
            sprites_scanline: [EntryOA::new(); 8],
            sprite_count: 0,
            sprite_shifter_pattern_lo: [0; 8],
            sprite_shifter_pattern_hi: [0; 8],

            is_sprite_zero_hit_possible: false,
            is_sprite_zero_being_rendered: false,

            scanline: 0,
            cycle: 0,
            
            is_frame_complete: false,
        }
    }
    
    #[allow(clippy::match_same_arms)]
    pub fn cpu_read(&mut self, addr: u16, read_only: bool, cartridge: &ComponentCartridge) -> u8 {
        let mut data = 0x00;

        match addr {
            // Control
            0x0000 => {}
            // Mask
            0x0001 => {}
            // Status
            0x0002 => {
                data = (self.reg_status.into_bits()) & 0xE0 | (self.ppu_data_buffer & 0x1F);
                self.reg_status.set_vertical_blank(false);
                self.address_latch = 0;
            }
            // OAM Address
            0x0003 => {}
            // OAM Data
            0x0004 => self.oam.read(&mut data),
            // Scroll
            0x0005 => {}
            // PPU Address
            0x0006 => {}
            // PPU Data
            0x0007 => {
                data = self.ppu_data_buffer;
                self.ppu_data_buffer = self.ppu_read(self.vram_addr.into_bits(), read_only, cartridge);

                if self.vram_addr.into_bits() >= 0x3F00 {
                    data = self.ppu_data_buffer;
                }

                self.vram_addr = RegisterLoopy::from_bits(if self.reg_control.increment_mode() { self.vram_addr.into_bits().wrapping_add(32) } else { self.vram_addr.into_bits().wrapping_add(1) });
            }

            _ => {}
        };

        data
    }

    #[allow(clippy::match_same_arms)]
    pub fn cpu_write(&mut self, addr: u16, data: u8, cartridge: &mut ComponentCartridge) {
        match addr {
            // Control
            0x0000 => {
                self.reg_control = RegisterControl::from_bits(data);
                self.tram_addr.set_nametable_x(self.reg_control.nametable_x());
                self.tram_addr.set_nametable_y(self.reg_control.nametable_y());
            }
            // Mask
            0x0001 => self.reg_mask = RegisterMask::from_bits(data),
            // Status
            0x0002 => {}
            // OAM Address
            0x0003 => self.oam.set_address(data),
            // OAM Data
            0x0004 => self.oam.write(self.oam.get_address(), data),
            // Scroll
            0x0005 => {
                if self.address_latch == 0 {
                    self.fine_x = data & 0x07;
                    self.tram_addr.set_coarse_x((data >> 3) as usize);
                    self.address_latch = 1;
                } else {
                    self.tram_addr.set_fine_y((data & 0x07) as usize);
                    self.tram_addr.set_coarse_y((data >> 3) as usize);
                    self.address_latch = 0;
                }
            }
            // PPU Address
            0x0006 => {
                if self.address_latch == 0 {
                    self.tram_addr = RegisterLoopy::from_bits((((data & 0x3F) as u16) << 8) | (self.tram_addr.into_bits() & 0x00FF));
                    self.address_latch = 1;
                } else {
                    self.tram_addr = RegisterLoopy::from_bits((self.tram_addr.into_bits() & 0xFF00) | data as u16);
                    self.vram_addr = self.tram_addr;
                    self.address_latch = 0;
                }
            }
            // PPU Data
            0x0007 => {
                self.ppu_write(self.vram_addr.into_bits(), data, cartridge);
                self.vram_addr = RegisterLoopy::from_bits(if self.reg_control.increment_mode() { self.vram_addr.into_bits().wrapping_add(32) } else { self.vram_addr.into_bits().wrapping_add(1) });
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
                            0x0000..=0x03FF | 0x0800..=0x0BFF => data = self.name_table[0][(addr & 0x03FF) as usize],
                            0x0400..=0x07FF | 0x0C00..=0x0FFF => data = self.name_table[1][(addr & 0x03FF) as usize],
                            _ => {}
                        }
                    }
                    Mirror::Horizontal => {
                        match addr {
                            0x0000..=0x07FF => data = self.name_table[0][(addr & 0x03FF) as usize],
                            0x0800..=0x0FFF => data = self.name_table[1][(addr & 0x03FF) as usize],
                            _ => {}
                        }
                    }
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
                data = self.pallete_table[addr as usize] & if self.reg_mask.grayscale() { 0x30 } else { 0x3F };
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
                            0x0000..=0x03FF | 0x0800..=0x0BFF => self.name_table[0][(addr & 0x03FF) as usize] = data,
                            0x0400..=0x07FF | 0x0C00..=0x0FFF => self.name_table[1][(addr & 0x03FF) as usize] = data,
                            _ => {}
                        }
                    }
                    Mirror::Horizontal => {
                        match addr {
                            0x0000..=0x07FF => self.name_table[0][(addr & 0x03FF) as usize] = data,
                            0x0800..=0x0FFF => self.name_table[1][(addr & 0x03FF) as usize] = data,
                            _ => {}
                        }
                    }
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

    pub fn increment_scroll_x(&mut self) {
        if self.reg_mask.render_background() || self.reg_mask.render_sprites() {
            if self.vram_addr.coarse_x() == 31 {
                self.vram_addr.set_coarse_x(0);
                self.vram_addr.set_nametable_x(!self.vram_addr.nametable_x());
            } else {
                self.vram_addr.set_coarse_x(self.vram_addr.coarse_x().wrapping_add(1));
            }
        }
    }

    pub fn increment_scroll_y(&mut self) {
        if self.reg_mask.render_background() || self.reg_mask.render_sprites() {
            if self.vram_addr.fine_y() < 7 {
                self.vram_addr.set_fine_y(self.vram_addr.fine_y().wrapping_add(1));
            } else {
                self.vram_addr.set_fine_y(0);
            
                if self.vram_addr.coarse_y() == 29 {
                    self.vram_addr.set_coarse_y(0);
                    self.vram_addr.set_nametable_y(!self.vram_addr.nametable_y());
                } else if self.vram_addr.coarse_y() == 31 {
                    self.vram_addr.set_coarse_y(0);
                } else {
                    self.vram_addr.set_coarse_y(self.vram_addr.coarse_y().wrapping_add(1));
                }
            }
        }
    }

    fn transfer_address_x(&mut self) {
        if self.reg_mask.render_background() || self.reg_mask.render_sprites() {
            self.vram_addr.set_nametable_x(self.tram_addr.nametable_x());
            self.vram_addr.set_coarse_x(self.tram_addr.coarse_x());
        }
    }

    fn transfer_address_y(&mut self) {
        if self.reg_mask.render_background() || self.reg_mask.render_sprites() {
            self.vram_addr.set_fine_y(self.tram_addr.fine_y());
            self.vram_addr.set_nametable_y(self.tram_addr.nametable_y());
            self.vram_addr.set_coarse_y(self.tram_addr.coarse_y());
        }
    }

    fn load_background_shifters(&mut self) {
        self.bg_shifter_pattern_lo = (self.bg_shifter_pattern_lo & 0xFF00) | self.bg_next_tile_lsb as u16;
        self.bg_shifter_pattern_hi = (self.bg_shifter_pattern_hi & 0xFF00) | self.bg_next_tile_msb as u16;
    
        self.bg_shifter_attribute_lo = (self.bg_shifter_attribute_lo & 0xFF00) | if self.bg_next_tile_attribute & 0b01 != 0 { 0xFF } else { 0x00 };
        self.bg_shifter_attribute_hi = (self.bg_shifter_attribute_hi & 0xFF00) | if self.bg_next_tile_attribute & 0b10 != 0 { 0xFF } else { 0x00 };
    }
    
    fn update_shifters(&mut self) {
        if self.reg_mask.render_background() {
            self.bg_shifter_pattern_lo <<= 1;
            self.bg_shifter_pattern_hi <<= 1;
    
            self.bg_shifter_attribute_lo <<= 1;
            self.bg_shifter_attribute_hi <<= 1;
        }

        if self.reg_mask.render_sprites() && self.cycle >= 1 && self.cycle < 258 {
            for i in 0..self.sprite_count {
                if self.sprites_scanline[i as usize].x > 0 {
                    self.sprites_scanline[i as usize].x -= 1;
                } else {
                    self.sprite_shifter_pattern_lo[i as usize] <<= 1;
                    self.sprite_shifter_pattern_hi[i as usize] <<= 1;
                }
            }
        }
    }

    pub fn tick(&mut self, screen: &mut ScreenData, cartridge: &ComponentCartridge) {
        if self.scanline >= -1 && self.scanline < 240 {
            if self.scanline == 0 && self.cycle == 0 {
                self.cycle = 1;
            }
            
            if self.scanline == -1 && self.cycle == 1 {
                self.reg_status.set_vertical_blank(false);
                self.reg_status.set_sprite_zero_hit(false);
                self.reg_status.set_sprite_overflow(false);
                
                for i in 0..8 {
                    self.sprite_shifter_pattern_lo[i] = 0;   
                    self.sprite_shifter_pattern_hi[i] = 0;
                }
            }

            if (self.cycle >= 2 && self.cycle < 258) || (self.cycle >= 321 && self.cycle < 338) {
                self.update_shifters();

                match (self.cycle - 1) % 8 {
                    0 => {
                        self.load_background_shifters();
                        self.bg_next_tile_id = self.ppu_read(0x2000 | (self.vram_addr.into_bits() & 0x0FFF), false, cartridge);
                    },
                    2 => {
                        self.bg_next_tile_attribute = self.ppu_read(0x23C0
                            | ((self.vram_addr.nametable_y() as u16) << 11)
                            | ((self.vram_addr.nametable_x() as u16) << 10)
                            | (((self.vram_addr.coarse_y() as u16) >> 2) << 3)
                            | ((self.vram_addr.coarse_x() >> 2) as u16),
                            false, cartridge);
                        if self.vram_addr.coarse_y() & 0x02 != 0 {
                            self.bg_next_tile_attribute >>= 4;
                        }
                        if self.vram_addr.coarse_x() & 0x02 != 0 {
                            self.bg_next_tile_attribute >>= 2;
                        }
                        self.bg_next_tile_attribute &= 0x03;
                    },
                    4 => {
                        self.bg_next_tile_lsb = self.ppu_read(
                            ((self.reg_control.pattern_background() as u16) << 12)
                            + ((self.bg_next_tile_id as u16) << 4)
                            + (self.vram_addr.fine_y() as u16),
                            false, cartridge);
                    },
                    6 => {
                        self.bg_next_tile_msb = self.ppu_read(
                            ((self.reg_control.pattern_background() as u16) << 12)
                            + ((self.bg_next_tile_id as u16) << 4)
                            + (self.vram_addr.fine_y() as u16) + 8,
                            false, cartridge);
                    },
                    7 => self.increment_scroll_x(),
                    _ => {}
                }
            }

            if self.cycle == 256 {
                self.increment_scroll_y();
            }

            if self.cycle == 257 {
                self.transfer_address_x();
            }

            if self.scanline == -1 && self.cycle >= 280 && self.cycle < 305 {
                self.transfer_address_y();
            }

            if self.cycle == 338 || self.cycle == 340 {
                self.bg_next_tile_id = self.ppu_read(0x2000 | (self.vram_addr.into_bits() & 0x0FFF), false, cartridge);
            }

            // Sprite evaluation phase
            if self.cycle == 257 && self.scanline >= 0 {
                self.sprites_scanline.iter_mut().for_each(|sprite| sprite.set(0xFF));
                self.sprite_count = 0;

                for i in 0..8 {
                    self.sprite_shifter_pattern_lo[i] = 0;
                    self.sprite_shifter_pattern_hi[i] = 0;
                }

                self.is_sprite_zero_hit_possible = false;
                let mut n = 0;
                while n < 64 && self.sprite_count < 9 {
                    let diff = self.scanline.wrapping_sub(self.oam.get_entry(n).y as i16);

                    if diff >= 0 && diff < if self.reg_control.sprite_size() { 16 } else { 8 } {
                        if self.sprite_count < 8 {
                            if n == 0 {
                                self.is_sprite_zero_hit_possible = true;
                            }

                            self.sprites_scanline[self.sprite_count as usize] = *self.oam.get_entry(n);
                            self.sprite_count += 1;
                        }
                    }
                    n += 1;
                }

                self.reg_status.set_sprite_overflow(self.sprite_count > 8);
            }

            if self.cycle == 340 {
                for i in 0..self.sprite_count {
                    let mut sprite_pattern_bits_lo: u8;
                    let mut sprite_pattern_bits_hi: u8;
                    let sprite_pattern_addr_lo: u16;

                    if !self.reg_control.sprite_size() { // 8x8 mode
                        if !self.sprites_scanline[i as usize].attributes & 0x80 != 0 { // Not flipped
                            sprite_pattern_addr_lo =
                                  ((self.reg_control.pattern_sprite() as u16) << 12)
                                | ((self.sprites_scanline[i as usize].tile_index as u16) << 4)
                                | (self.scanline as u16 - self.sprites_scanline[i as usize].y as u16);
                        } else { // Flipped vertically
                            sprite_pattern_addr_lo =
                                  ((self.reg_control.pattern_sprite() as u16) << 12)
                                | ((self.sprites_scanline[i as usize].tile_index as u16) << 4)
                                | (7_u16.wrapping_sub(self.scanline as u16).wrapping_sub(self.sprites_scanline[i as usize].y as u16));
                        }
                    } else { // 8x16 mode
                        if !self.sprites_scanline[i as usize].attributes & 0x80 != 0 { // Not flipped
                            if self.scanline - (self.sprites_scanline[i as usize].y as i16) < 8 {
                                sprite_pattern_addr_lo =
                                      (((self.sprites_scanline[i as usize].tile_index & 0x01) as u16) << 12)
                                    | (((self.sprites_scanline[i as usize].tile_index & 0xFE) as u16) << 4)
                                    | ((self.scanline as u16 - self.sprites_scanline[i as usize].y as u16) & 0x07);
                            } else {
                                sprite_pattern_addr_lo =
                                      (((self.sprites_scanline[i as usize].tile_index & 0x01) as u16) << 12)
                                    | ((((self.sprites_scanline[i as usize].tile_index & 0xFE) as u16) + 1) << 4)
                                    | ((self.scanline as u16 - self.sprites_scanline[i as usize].y as u16) & 0x07);
                            }
                        } else { // Flipped vertically
                            if self.scanline - (self.sprites_scanline[i as usize].y as i16) < 8 {
                                sprite_pattern_addr_lo =
                                      (((self.sprites_scanline[i as usize].tile_index & 0x01) as u16) << 12)
                                    | ((((self.sprites_scanline[i as usize].tile_index & 0xFE) as u16) + 1) << 4)
                                    | (7 - (self.scanline as u16 - self.sprites_scanline[i as usize].y as u16));
                            } else {
                                sprite_pattern_addr_lo =
                                      (((self.sprites_scanline[i as usize].tile_index & 0x01) as u16) << 12)
                                    | (((self.sprites_scanline[i as usize].tile_index & 0xFE) as u16) << 4)
                                    | (7 - (self.scanline as u16 - self.sprites_scanline[i as usize].y as u16));
                            }
                        }
                    }

                    let sprite_pattern_addr_hi = sprite_pattern_addr_lo.wrapping_add(8);
                    sprite_pattern_bits_lo = self.ppu_read(sprite_pattern_addr_lo, false, cartridge);
                    sprite_pattern_bits_hi = self.ppu_read(sprite_pattern_addr_hi, false, cartridge);

                    if self.sprites_scanline[i as usize].attributes & 0x40 != 0 {
                        sprite_pattern_bits_lo = sprite_pattern_bits_lo.reverse_bits();
                        sprite_pattern_bits_hi = sprite_pattern_bits_hi.reverse_bits();
                    }

                    self.sprite_shifter_pattern_lo[i as usize] = sprite_pattern_bits_lo;
                    self.sprite_shifter_pattern_hi[i as usize] = sprite_pattern_bits_hi;
                }
            }
        }

        if self.scanline == 241 && self.scanline < 261 && self.cycle == 1 {
            self.reg_status.set_vertical_blank(true);
            if self.reg_control.enable_nmi() {
                self.nmi_occurred = true;
            }
        }

        let bg_pixel = if self.reg_mask.render_background() {
            let bit_mux = 0x8000 >> self.fine_x;
            
            let p0_pixel = ((self.bg_shifter_pattern_lo & bit_mux) > 0) as u8;
            let p1_pixel = ((self.bg_shifter_pattern_hi & bit_mux) > 0) as u8;
            (p1_pixel << 1) | p0_pixel
        } else {
            0
        };

        let bg_palette = if self.reg_mask.render_background() {
            let bit_mux = 0x8000 >> self.fine_x;
            
            let bg_pal0 = ((self.bg_shifter_attribute_lo & bit_mux) > 0) as u8;
            let bg_pal1 = ((self.bg_shifter_attribute_hi & bit_mux) > 0) as u8;
            (bg_pal1 << 1) | bg_pal0
        } else {
            0
        };

        let mut fg_pixel = 0x00;
        let mut fg_palette = 0x00;
        let mut fg_priority = false;
        if self.reg_mask.render_sprites() {
            self.is_sprite_zero_being_rendered = false;

            for i in 0..self.sprite_count {
                if self.sprites_scanline[i as usize].x == 0 {
                    let fg_pixel_lo = ((self.sprite_shifter_pattern_lo[i as usize] & 0x80) > 0) as u8;
                    let fg_pixel_hi = ((self.sprite_shifter_pattern_hi[i as usize] & 0x80) > 0) as u8;
                    fg_pixel = (fg_pixel_hi << 1) | fg_pixel_lo;

                    fg_palette = (self.sprites_scanline[i as usize].attributes & 0x03) + 0x04;
                    fg_priority = (self.sprites_scanline[i as usize].attributes & 0x20) == 0;

                    if fg_pixel != 0 {
                        if i == 0 {
                            self.is_sprite_zero_being_rendered = true;
                        }

                        break;
                    }
                }
            }
        };

        let (pixel, palette) =  match (bg_pixel, fg_pixel) {
            (0, 0) => (0, 0),
            (0, _) => (fg_pixel, fg_palette),
            (_, 0) => (bg_pixel, bg_palette),
            (_, _) => {
                let res = if fg_priority {
                    (fg_pixel, fg_palette)
                } else {
                    (bg_pixel, bg_palette)
                };

                if self.is_sprite_zero_hit_possible && self.is_sprite_zero_being_rendered {
                    if self.reg_mask.render_background() && self.reg_mask.render_sprites() {
                        if !(self.reg_mask.render_background_left() || self.reg_mask.render_sprites_left()) {
                            if self.cycle >= 9 && self.cycle < 258 {
                                self.reg_status.set_sprite_zero_hit(true);
                            }
                        } else {
                            if self.cycle >= 1 && self.cycle < 258 {
                                self.reg_status.set_sprite_zero_hit(true);
                            }
                        }
                    }
                }

                res
            }
        };

        screen.draw_pixel_screen((self.cycle - 1) as u16, self.scanline as u16, self.get_palette_color(screen, palette, pixel, cartridge));


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
