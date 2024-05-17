#[derive(Debug)]
pub struct Bus {
    pub ram: [u8; 64 * 1024],
}

impl Bus {
    pub fn read(&self, addr: u16, _read_only: bool) -> u8 {
        if addr >= (0x0000 as u16) && addr <= (0xFFFF as u16) {
            return self.ram[addr as usize]
        }
        0x0000
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr >= (0x0000 as u16) && addr <= (0xFFFF as u16) {
            self.ram[addr as usize] = data;
        }
    }
}
