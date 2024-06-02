pub mod ppu2c02 {
    use crate::nes::Cartridge;
    pub use raylib::prelude::core::color::*;

    #[allow(dead_code)]
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
        pub is_frame_complete: bool,

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
                is_frame_complete: false,

                screen_palette,
                displayable_screen: [Color::BLANK; 256 * 240],
                displayable_name_table: [[Color::BLANK; 256 * 240]; 2],
                displayable_pattern_table: [[Color::BLANK; 128 * 128]; 2]
            }
        }
        
        #[allow(dead_code)]
        pub const fn cpu_read(&self, addr: u16, _read_only: bool) -> u8 {
            match addr {
                0x0000 => 0x00, // Control
                0x0001 => 0x00, // Mask
                0x0002 => 0x00, // Status
                0x0003 => 0x00, // OAM Address
                0x0004 => 0x00, // OAM Data
                0x0005 => 0x00, // Scroll
                0x0006 => 0x00, // PPU Address
                0x0007 => 0x00, // PPU Data
                _ => 0x00
            }
        }

        #[allow(dead_code)]
        pub fn cpu_write(&mut self, addr: u16, _data: u8) {
            match addr {
                0x0000 => {}, // Control
                0x0001 => {}, // Mask
                0x0002 => {}, // Status
                0x0003 => {}, // OAM Address
                0x0004 => {}, // OAM Data
                0x0005 => {}, // Scroll
                0x0006 => {}, // PPU Address
                0x0007 => {}, // PPU Data
                _ => {}
            };
        }
        
        #[allow(dead_code)]
        pub fn ppu_read(&self, cartridge: &Cartridge, mut addr: u16, _read_only: bool) -> u8 {
            let mut ret = 0x0000;

            addr &= 0x3FFF;
            if cartridge.cpu_read(addr, &mut ret) {
            }
            match addr {
                0x0000..=0x1FFF => { // Pattern Table
                    ret = self.table_pattern[((addr & 0x1000) >> 12) as usize][(addr & 0x0FFF) as usize];
                },
                0x2000..=0x3EFF => {
                    // Nametable
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
            
            ret
        }

        #[allow(dead_code)]
        pub fn ppu_write(&mut self, cartridge: &mut Cartridge, mut addr: u16, data: u8) {
            addr &= 0x3FFF;

            if cartridge.cpu_write(addr, data) {
            }
            match addr {
                0x0000..=0x1FFF => { // Pattern Table
                    self.table_pattern[((addr & 0x1000) >> 12) as usize][(addr & 0x0FFF) as usize] = data;
                },
                0x2000..=0x3EFF => {
                    // Nametable
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

        pub const fn get_screen(&self) -> &[Color] {
            &self.displayable_screen
        }

        pub fn get_palette_from_ram(&self, cartridge: &Cartridge, palette: u8, pixel: u8) -> Color {
            self.screen_palette[(self.ppu_read(cartridge, 0x3F00 + ((palette << 2) + pixel) as u16, false)) as usize]
        }
        
        pub fn get_pattern_table(&mut self, cartridge: &Cartridge, index: usize, palette: u8) -> &[Color] {
            for y in 0..16 {
                for x in 0..16 {
                    // 256 because 1 tile is 16x16 | A row has 16 tiles | 16 * 16 = 256
                    let offset = y * 256 + x * 16;
                    
                    for row in 0..8 { // For each tile, 8 rows of 8 pixels
                        let mut lsb = self.ppu_read(cartridge, (index * 0x1000 + offset + row) as u16, false);
                        let mut msb = self.ppu_read(cartridge, (index * 0x1000 + offset + row + 8) as u16, false);
    
                        for col in 0..8 {
                            let pixel = (lsb & 0x01) + (msb & 0x01);
                            lsb >>= 1;
                            msb >>= 1;
    
                            self.displayable_pattern_table[index][(y * 8 + row) * 128 + (x * 8 + (7 - col))] = self.get_palette_from_ram(cartridge, palette, pixel);
                        }
                    }
                }
            }
            
            &self.displayable_pattern_table[index]
        }

        pub fn clock(&mut self) {
            // randomly set the pixel to black or white
            let index: i32 = self.scanline as i32 * 256 + self.cycle as i32 - 1;

            if index >= 0 && (index as usize) < 256 * 240 {
                self.displayable_screen[index as usize % (256 * 240)] = if rand::random() {
                    Color::WHITE
                } else {
                    Color::BLANK
                };
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
