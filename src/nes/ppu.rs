pub mod ppu2c02 {
    
    #[derive(Debug)]
    pub struct Ppu2C02 {
        table_name: [[u8; 1024]; 2],
        table_pallete: [u8; 32],
    }

    impl Ppu2C02 {
        pub fn new() -> Self {
            Self {
                table_name: [[0; 1024]; 2],
                table_pallete: [0; 32],
            }
        }

        pub fn cpu_read(&self, addr: u16, read_only: bool) -> u8 {
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

        pub fn cpu_write(&mut self, addr: u16, data: u8) {
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
        
        #[allow(unused_assignments)]
        pub fn ppu_read(&self, mut addr: u16, read_only: bool) -> u8 {
            addr &= 0x3FFF;
            
            0x000
        }

        #[allow(unused_assignments)]
        pub fn ppu_write(&mut self, mut addr: u16, data: u8) {
            addr &= 0x3FFF;
        }

        pub fn clock(&mut self) {
            
        }
    }
}
