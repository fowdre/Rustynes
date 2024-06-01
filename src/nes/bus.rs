#[derive(Debug)]
pub struct Bus {
    pub cpu_ram: [u8; 2 * 1024],
}

impl Bus {
    pub const fn cpu_read(&self, addr: u16, _read_only: bool) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x07FF) as usize],
            0x2000..=0x3FFF => self.cpu_ram[(addr & 0x0007) as usize],
            _ => 0x00
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x07FF) as usize] = data,
            0x2000..=0x3FFF => self.cpu_ram[(addr & 0x0007) as usize] = data,
            _ => {}
        }
    }
}
