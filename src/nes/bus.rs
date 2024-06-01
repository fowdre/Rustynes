use super::Cartridge;

#[derive(Debug)]
pub struct Bus {
    pub cpu_ram: [u8; 2 * 1024],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu_ram: [0; 2 * 1024],
        }
    }

    pub fn cpu_read(&self, cartridge: &Cartridge, addr: u16, _read_only: bool) -> u8 {
        if cartridge.cpu_read(addr, &0x0000) {
            return 0x0000;
        }
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x07FF) as usize],
            0x2000..=0x3FFF => self.cpu_ram[(addr & 0x0007) as usize],
            _ => 0x0000
        }
    }

    pub fn cpu_write(&mut self, cartridge: &mut Cartridge, addr: u16, data: u8) {
        if cartridge.cpu_write(addr, data) {
            return;
        }
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x07FF) as usize] = data,
            0x2000..=0x3FFF => self.cpu_ram[(addr & 0x0007) as usize] = data,
            _ => {}
        }
    }
}
