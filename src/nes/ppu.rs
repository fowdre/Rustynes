use crate::nes::cartridge::ComponentCartridge;

#[derive(Debug, Copy, Clone)]
pub struct Component2C02 {
    // screen_palette: [Color; 64],
    // pub displayable_name_table: [[Color; 256 * 240]; 2],
    // displayable_pattern_table: [[Color; 128 * 128]; 2],
    // displayable_screen: [Color; 256 * 240]

    /// (or VRAM) Background **_layout_**
    pub name_table: [[u8; 1024]; 2],
    /// Sprites (background & foreground)
    pub pattern_table: [[u8; 4 * 1024]; 2],
    /// Colors
    pub pallete_table: [u8; 32],
}

impl Component2C02 {
    pub fn new() -> Self {
        Self {
            name_table: [[0; 1024]; 2],
            pattern_table: [[0; 4 * 1024]; 2],
            pallete_table: [0; 32],
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

    pub fn tick(&self) {
    }
}
