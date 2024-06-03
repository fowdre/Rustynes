mod registers;

pub mod ppu2c02 {
    use crate::nes::Cartridge;
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

        /// Knowing if we're writing to the low or high byte
        address_latch: u8,
        ppu_data_buffer: u8,
        ppu_address: u16,

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

                reg_status: RegisterStatus::from_bits(0),
                reg_mask: RegisterMask::from_bits(0),
                reg_control: RegisterControl::from_bits(0),

                address_latch: 0,
                ppu_data_buffer: 0,
                ppu_address: 0,

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
                    self.ppu_data_buffer = self.ppu_read(cartridge, self.ppu_address, read_only);

                    if self.ppu_address >= 0x3F00 {
                        ret = self.ppu_data_buffer;
                    }
                }
                _ => {}
            };

            ret
        }

        pub fn cpu_write(&mut self, cartridge: &mut Cartridge, addr: u16, data: u8) {
            match addr {
                0x0000 => { // Control
                    self.reg_control = RegisterControl::from_bits(data);
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
                },
                0x0006 => { // PPU Address
                    if self.address_latch == 0 {
                        self.ppu_address = (self.ppu_address & 0x00FF) | ((data as u16) << 8);
                        self.address_latch = 1;
                    } else {
                        self.ppu_address = (self.ppu_address & 0xFF00) | data as u16;
                        self.address_latch = 0;
                    }
                },
                0x0007 => { // PPU Data
                    self.ppu_write(cartridge, self.ppu_address, data);
                    self.ppu_address = self.ppu_address.wrapping_add(1);
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
                    },
                    _ => {}
                }
            }

            ret
        }

        pub fn ppu_write(&mut self, cartridge: &mut Cartridge, mut addr: u16, data: u8) {
            addr &= 0x3FFF;

            if cartridge.cpu_write(addr, data) {
            } else {
                match addr {
                    0x0000..=0x1FFF => { // Pattern
                        self.table_pattern[((addr & 0x1000) >> 12) as usize][(addr & 0x0FFF) as usize] = data;
                    },
                    0x2000..=0x3EFF => { // Name Table
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
        }

        pub fn get_pattern_table(&mut self, cartridge: &Cartridge, index: u8, palette: u8) -> &[Color] {
            for y in 0_u16..16 {
                for x in 0_u16..16 {
                    let offset = y * 256 + x * 16;
                    
                    for row in 0_u16..8 {

                        let mut tile_lsb = self.ppu_read(cartridge, index as u16 * 0x1000 + offset + row    , false);
                        let mut tile_msb = self.ppu_read(cartridge, index as u16 * 0x1000 + offset + row + 8, false);

                        for col in 0_u16..8 {
                            let pixel = (tile_lsb & 0x01) + (tile_msb & 0x01);
                            tile_lsb >>= 1;
                            tile_msb >>= 1;

                            self.displayable_pattern_table[index as usize][((y * 8 + row) * 128 + (x * 8 + (7 - col))) as usize] = *self.get_palette_from_ram(cartridge, palette, pixel);
                        }
                    }
                }
            }
            
            &self.displayable_pattern_table[index as usize]
        }

        pub fn clock(&mut self) {
            if self.scanline == -1 && self.cycle == 1 {
                self.reg_status.set_vertical_blank(false);
            }

            if self.scanline == 241 && self.cycle == 1 {
                self.reg_status.set_vertical_blank(true);
                if self.reg_control.enable_nmi() {
                    self.nmi = true;
                }
            }

            // randomly set the pixel to black or white
            if self.cycle <= 256 && self.scanline <= 240 {
                let index: i32 = self.scanline as i32 * 256 + self.cycle as i32 - 1;

                if index >= 0 && (index as usize) < 256 * 240 {
                    self.displayable_screen[index as usize] = if rand::random() {
                        Color::WHITE
                    } else {
                        Color::BLANK
                    };
                }
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
