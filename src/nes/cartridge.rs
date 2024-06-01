pub struct Cartridge {

}

impl Cartridge {
    pub fn cpu_read(&self, addr: u16, data: &u8) -> u8 {
        0
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {

    }

    pub fn ppu_read(&self, addr: u16, data: &u8) -> u8 {
        0
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) {

    }
}
