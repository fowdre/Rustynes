pub mod ppu2c02 {
    
    #[derive(Debug)]
    pub struct Ppu2C02 {
        
    }

    impl Ppu2C02 {
        pub fn cpu_read(&self, addr: u16, read_only: bool) -> u8 {
            0
        }

        pub fn cpu_write(&mut self, addr: u16, data: u8) {
            
        }

        pub fn ppu_read(&self, addr: u16, read_only: bool) -> u8 {
            0
        }

        pub fn ppu_write(&mut self, addr: u16, data: u8) {
            
        }
    }
}
