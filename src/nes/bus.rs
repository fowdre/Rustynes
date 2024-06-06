#[derive(Debug, Copy, Clone)]
pub struct Bus {
    pub ram: [u8; 64 * 1024],
}

impl Bus {
    pub const fn read(&self, addr: u16, _read_only: bool) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}
