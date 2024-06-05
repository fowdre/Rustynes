mod registers;

pub mod ppu2c02 {
    use crate::nes::{Cartridge, Mirror};
    pub use raylib::prelude::core::color::*;
    use super::registers::*;

    #[derive(Debug)]
    pub struct Ppu2C02 {
        /// (or VRAM) Background **_layout_**
        pub table_name: [[u8; 1024]; 2],
        /// Sprites (background & foreground)
        pub table_pattern: [[u8; 4 * 1024]; 2],
        /// Colors
        pub table_pallete: [u8; 32],

        /// Row
        scanline: i16,
        /// Column
        cycle: i16,
        pub nmi: bool,
        pub is_frame_complete: bool,
        
        reg_status: RegisterStatus,
        reg_mask: RegisterMask,
        reg_control: RegisterControl,

        vram_addr: RegisterLoopy,
        tram_addr: RegisterLoopy,
        fine_x: u8,

        bg_next_tile_id: u8,
        bg_next_tile_attribute: u8,
        bg_next_tile_lsb: u8,
        bg_next_tile_msb: u8,

        bg_shifter_pattern_lo: u16,
        bg_shifter_pattern_hi: u16,
        bg_shifter_attribute_lo: u16,
        bg_shifter_attribute_hi: u16,

        /// Knowing if we're writing to the low or high byte
        address_latch: u8,
        ppu_data_buffer: u8,

        screen_palette: [Color; 64],
        pub displayable_name_table: [[Color; 256 * 240]; 2],
        displayable_pattern_table: [[Color; 128 * 128]; 2],
        displayable_screen: [Color; 256 * 240]
    }

    impl Ppu2C02 {
        pub const fn new() -> Self {
            let mut screen_palette = [Color::BLANK; 64];
            {
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
            }

            Self {
                table_name: [[0; 1024]; 2],
                table_pattern: [[0; 4096]; 2],
                table_pallete: [0; 32],

                scanline: 0,
                cycle: 0,
                nmi: false,
                is_frame_complete: false,

                reg_status: RegisterStatus::new(),
                reg_mask: RegisterMask::new(),
                reg_control: RegisterControl::new(),

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

                address_latch: 0,
                ppu_data_buffer: 0,

                screen_palette,
                displayable_screen: [Color::BLANK; 256 * 240],
                displayable_name_table: [[Color::BLANK; 256 * 240]; 2],
                displayable_pattern_table: [[Color::BLANK; 128 * 128]; 2]
            }
        }
        
        pub fn cpu_read(&mut self, cartridge: &Cartridge, addr: u16, read_only: bool) -> u8 {
            let mut ret = 0x0000;

            match addr {
                0x0000 => { // Control
                }
                0x0001 => { // Mask
                }
                0x0002 => { // Status
                    ret = (self.reg_status.into_bits() & 0xE0) | (self.reg_status.into_bits() & 0x1F);
                    self.reg_status.set_vertical_blank(false);
                    self.address_latch = 0;
                }
                0x0003 => { // OAM Address
                }
                0x0004 => { // OAM Data
                }
                0x0005 => { // Scroll
                }
                0x0006 => { // PPU Address
                }
                0x0007 => { // PPU Data
                    ret = self.ppu_data_buffer;
                    self.ppu_data_buffer = self.ppu_read(cartridge, self.vram_addr.into_bits(), read_only);

                    if self.vram_addr.into_bits() > 0x3F00 {
                        ret = self.ppu_data_buffer;
                    }
                    self.vram_addr = RegisterLoopy::from_bits(self.vram_addr.into_bits().wrapping_add(if self.reg_control.increment_mode() { 32 } else { 1 }));
                }
                _ => {}
            };

            ret
        }

        pub fn cpu_write(&mut self, cartridge: &mut Cartridge, addr: u16, data: u8) {
            match addr {
                0x0000 => { // Control
                    self.reg_control = RegisterControl::from_bits(data);
                    self.tram_addr.set_nametable_x(self.reg_control.nametable_x());
                    self.tram_addr.set_nametable_y(self.reg_control.nametable_y());
                },
                0x0001 => { // Mask
                    self.reg_mask = RegisterMask::from_bits(data);
                },
                0x0002 => { // Status
                },
                0x0003 => { // OAM Address
                },
                0x0004 => { // OAM Data
                },
                0x0005 => { // Scroll
                    if self.address_latch == 0 {
                        self.fine_x = data & 0x07;
                        self.tram_addr.set_coarse_x((data >> 3) as usize);
                        self.address_latch = 1;
                    } else {
                        self.tram_addr.set_fine_y((data & 0x07) as usize);
                        self.tram_addr.set_coarse_y((data >> 3) as usize);
                        self.address_latch = 0;
                    }
                },
                0x0006 => { // PPU Address
                    if self.address_latch == 0 {
                        self.tram_addr = RegisterLoopy::from_bits((self.tram_addr.into_bits() & 0x00FF) | (data as u16) << 8);
                        self.address_latch = 1;
                    } else {
                        self.tram_addr = RegisterLoopy::from_bits((self.tram_addr.into_bits() & 0xFF00) | data as u16);
                        self.vram_addr = self.tram_addr;
                        self.address_latch = 0;
                    }
                },
                0x0007 => { // PPU Data
                    self.ppu_write(cartridge, self.vram_addr.into_bits(), data);
                    self.vram_addr = RegisterLoopy::from_bits(self.vram_addr.into_bits().wrapping_add(if self.reg_control.increment_mode() { 32 } else { 1 }));
                },
                _ => {}
            };
        }
        
        pub fn ppu_read(&self, cartridge: &Cartridge, mut addr: u16, _read_only: bool) -> u8 {
            let mut ret = 0x0000;
            addr &= 0x3FFF;

            if cartridge.ppu_read(addr, &mut ret) {
            } else {
                match addr {
                    0x0000..=0x1FFF => { // Pattern
                        ret = self.table_pattern[((addr & 0x1000) >> 12) as usize][(addr & 0x0FFF) as usize];
                    },
                    0x2000..=0x3EFF => { // Name Table
                        addr &= 0x0FFF;
                        
                        match cartridge.mirror {
                            Mirror::Vertical => {
                                match addr {
                                    0x0000..=0x03FF => ret = self.table_name[0][(addr & 0x03FF) as usize],
                                    0x0400..=0x07FF => ret = self.table_name[1][(addr & 0x03FF) as usize],
                                    0x0800..=0x0BFF => ret = self.table_name[0][(addr & 0x03FF) as usize],
                                    0x0C00..=0x0FFF => ret = self.table_name[1][(addr & 0x03FF) as usize],
                                    _ => {}
                                }
                            },
                            Mirror::Horizontal => {
                                match addr {
                                    0x0000..=0x03FF => ret = self.table_name[0][(addr & 0x03FF) as usize],
                                    0x0400..=0x07FF => ret = self.table_name[0][(addr & 0x03FF) as usize],
                                    0x0800..=0x0BFF => ret = self.table_name[1][(addr & 0x03FF) as usize],
                                    0x0C00..=0x0FFF => ret = self.table_name[1][(addr & 0x03FF) as usize],
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    },
                    0x3F00..=0x3FFF => { // Palette
                        addr &= 0x001F;
                        match addr {
                            0x0010 => addr = 0x0000,
                            0x0014 => addr = 0x0004,
                            0x0018 => addr = 0x0008,
                            0x001C => addr = 0x000C,
                            _ => {}
                        }
                        ret = self.table_pallete[addr as usize];
                        // ret = self.table_pallete[addr as usize] & (if self.reg_mask.into_bits() != 0 { 0x30 } else { 0x3F })
                    },
                    _ => {}
                }
            }

            ret
        }

        pub fn ppu_write(&mut self, cartridge: &mut Cartridge, mut addr: u16, data: u8) {
            addr &= 0x3FFF;

            if cartridge.ppu_write(addr, data) {
            } else {
                match addr {
                    0x0000..=0x1FFF => { // Pattern
                        self.table_pattern[((addr & 0x1000) >> 12) as usize][(addr & 0x0FFF) as usize] = data;
                    },
                    0x2000..=0x3EFF => { // Name Table
                        addr &= 0x0FFF;
                        match cartridge.mirror {
                            Mirror::Vertical => {
                                match addr {
                                    0x0000..=0x03FF => self.table_name[0][(addr & 0x03FF) as usize] = data,
                                    0x0400..=0x07FF => self.table_name[1][(addr & 0x03FF) as usize] = data,
                                    0x0800..=0x0BFF => self.table_name[0][(addr & 0x03FF) as usize] = data,
                                    0x0C00..=0x0FFF => self.table_name[1][(addr & 0x03FF) as usize] = data,
                                    _ => {}
                                }
                            },
                            Mirror::Horizontal => {
                                match addr {
                                    0x0000..=0x03FF => self.table_name[0][(addr & 0x03FF) as usize] = data,
                                    0x0400..=0x07FF => self.table_name[0][(addr & 0x03FF) as usize] = data,
                                    0x0800..=0x0BFF => self.table_name[1][(addr & 0x03FF) as usize] = data,
                                    0x0C00..=0x0FFF => self.table_name[1][(addr & 0x03FF) as usize] = data,
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    },
                    0x3F00..=0x3FFF => { // Palette
                        addr &= 0x001F;
                        match addr {
                            0x0010 => addr = 0x0000,
                            0x0014 => addr = 0x0004,
                            0x0018 => addr = 0x0008,
                            0x001C => addr = 0x000C,
                            _ => {}
                        }
                        self.table_pallete[addr as usize] = data;
                    },
                    _ => {}
                }
            }
        }

        pub const fn get_screen(&self) -> &[Color] {
            &self.displayable_screen
        }

        pub fn get_palette_from_ram(&self, cartridge: &Cartridge, palette: u8, pixel: u8) -> &Color {
            &self.screen_palette[self.ppu_read(cartridge, 0x3F00 + (palette << 2) as u16 + pixel as u16, false) as usize]
            // &self.screen_palette[(self.ppu_read(cartridge, 0x3F00 + (palette << 2) as u16 + pixel as u16, false) & 0x3F) as usize]
        }

        pub fn get_pattern_table(&mut self, cartridge: &Cartridge, index: u8, palette: u8) -> &[Color] {
            for y in 0_u16..16 {
                for x in 0_u16..16 {
                    let offset = y * 256 + x * 16;
                    
                    for row in 0_u16..8 {

                        let mut tile_lsb = self.ppu_read(cartridge, index as u16 * 0x1000 + offset + row    , false);
                        let mut tile_msb = self.ppu_read(cartridge, index as u16 * 0x1000 + offset + row + 8, false);

                        for col in 0_u16..8 {
                            let pixel = (tile_lsb & 0x01) + ((tile_msb & 0x01) << 1);
                            tile_lsb >>= 1;
                            tile_msb >>= 1;

                            self.displayable_pattern_table[index as usize][((y * 8 + row) * 128 + (x * 8 + (7 - col))) as usize] = *self.get_palette_from_ram(cartridge, palette, pixel);
                        }
                    }
                }
            }
            
            &self.displayable_pattern_table[index as usize]
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

        pub fn transfer_address_x(&mut self) {
            if self.reg_mask.render_background() || self.reg_mask.render_sprites() {
                self.vram_addr.set_nametable_x(self.tram_addr.nametable_x());
                self.vram_addr.set_coarse_x(self.tram_addr.coarse_x());
            }
        }

        pub fn transfer_address_y(&mut self) {
            if self.reg_mask.render_background() || self.reg_mask.render_sprites() {
                self.vram_addr.set_fine_y(self.tram_addr.fine_y());
                self.vram_addr.set_nametable_y(self.tram_addr.nametable_y());
                self.vram_addr.set_coarse_y(self.tram_addr.coarse_y());
            }
        }

        pub fn load_background_shifters(&mut self) {
            self.bg_shifter_pattern_lo = (self.bg_shifter_pattern_lo & 0xFF00) | self.bg_next_tile_lsb as u16;
            self.bg_shifter_pattern_hi = (self.bg_shifter_pattern_hi & 0xFF00) | self.bg_next_tile_msb as u16;
        
            self.bg_shifter_attribute_lo = (self.bg_shifter_attribute_lo & 0xFF00) | if self.bg_next_tile_attribute & 0b01 != 0 { 0xFF } else { 0x00 };
            self.bg_shifter_attribute_hi = (self.bg_shifter_attribute_hi & 0xFF00) | if self.bg_next_tile_attribute & 0b10 != 0 { 0xFF } else { 0x00 };
        }
        
        pub fn update_shifters(&mut self) {
            if self.reg_mask.render_background() {
                self.bg_shifter_pattern_lo <<= 1;
                self.bg_shifter_pattern_hi <<= 1;
        
                self.bg_shifter_attribute_lo <<= 1;
                self.bg_shifter_attribute_hi <<= 1;
            }
        }

        pub fn clock(&mut self, cartridge: &Cartridge) {
            if self.scanline >= -1 && self.scanline < 240 {
                if self.scanline == -1 && self.cycle == 1 {
                    self.reg_status.set_vertical_blank(false);
                }

                if (self.cycle >= 2 && self.cycle < 258) || (self.cycle >= 321 && self.cycle < 338) {
                    self.update_shifters();

                    match (self.cycle - 1) % 8 {
                        0 => {
                            self.load_background_shifters();
                            self.bg_next_tile_id = self.ppu_read(cartridge, 0x2000 | (self.vram_addr.into_bits() & 0x0FFF), false)
                        },
                        2 => {
                            self.bg_next_tile_attribute = self.ppu_read(cartridge, 0x23C0
                                | ((self.vram_addr.nametable_y() as u16) << 11)
                                | ((self.vram_addr.nametable_x() as u16) << 10)
                                | (((self.vram_addr.coarse_y() >> 2) as u16) << 3)
                                | ((self.vram_addr.coarse_x() >> 2) as u16),
                                false);
                            if self.vram_addr.coarse_y() & 0x02 != 0 {
                                self.bg_next_tile_attribute >>= 4;
                            }
                            if self.vram_addr.coarse_x() & 0x02 != 0 {
                                self.bg_next_tile_attribute >>= 2;
                            }
                            self.bg_next_tile_attribute &= 0x03;
                        },
                        4 => {
                            self.bg_next_tile_lsb = self.ppu_read(cartridge,
                                ((self.reg_control.pattern_background() as u16) << 12)
                                + ((self.bg_next_tile_id as u16) << 4)
                                + (self.vram_addr.fine_y() as u16),
                                false)
                        },
                        6 => {
                            self.bg_next_tile_msb = self.ppu_read(cartridge,
                                ((self.reg_control.pattern_background() as u16) << 12)
                                + ((self.bg_next_tile_id as u16) << 4)
                                + (self.vram_addr.fine_y() as u16) + 8,
                                false)
                        },
                        7 => self.increment_scroll_x(),
                        _ => {}
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
                }
            }

            if self.scanline == 241 && self.cycle == 1 {
                self.reg_status.set_vertical_blank(true);
                if self.reg_control.enable_nmi() {
                    self.nmi = true;
                }
            }

            let mut bg_pixel = 0x00_u8;
            let mut bg_palette = 0x00_u8;

            if self.reg_mask.render_background() {
                let bit_mux = 0x8000 >> self.fine_x;
                
                let p0_pixel = ((self.bg_shifter_pattern_lo & bit_mux) > 0) as u8;
                let p1_pixel = ((self.bg_shifter_pattern_hi & bit_mux) > 0) as u8;
                bg_pixel = (p1_pixel << 1) | p0_pixel;

                let bg_pal0 = ((self.bg_shifter_attribute_lo & bit_mux) > 0) as u8;
                let bg_pal1 = ((self.bg_shifter_attribute_hi & bit_mux) > 0) as u8;
                bg_palette = (bg_pal1 << 1) | bg_pal0;
            }

            let x = self.cycle - 1;
            let y = self.scanline;
            if (0..256).contains(&x) && (0..240).contains(&y) {
                self.displayable_screen[(y as i32 * 256 + x as i32) as usize] = *self.get_palette_from_ram(cartridge, bg_palette, bg_pixel);
            }

            self.cycle += 1;
            
            if self.cycle >= 341 {
                self.cycle = 0;
                self.scanline += 1;
                // dbg!(self.scanline);
                if self.scanline >= 261 {
                    self.scanline = -1;
                    self.is_frame_complete = true;
                }
            }
        }
    }
}
