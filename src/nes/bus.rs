use super::Cartridge;
use super::ppu::ppu2c02::Ppu2C02;

#[derive(Debug)]
pub struct Bus {
    pub cpu_ram: [u8; 2 * 1024],
}

impl Bus {
    pub const fn new() -> Self {
        Self {
            cpu_ram: [0; 2 * 1024],
        }
    }

    pub fn cpu_read(&self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, addr: u16, read_only: bool) -> u8 {
        let mut ret = 0x0000;

        if cartridge.cpu_read(addr, &mut ret) { // Cartidge range
            return ret;
        }
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x07FF) as usize], // RAM range
            0x2000..=0x3FFF => ppu.cpu_read(addr & 0x0007, read_only), // PPU range
            _ => ret
        }
    }

    pub fn cpu_write(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, addr: u16, data: u8) {
        if cartridge.cpu_write(addr, data) {
            return;
        }
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x07FF) as usize] = data, // RAM range
            0x2000..=0x3FFF => ppu.cpu_write(addr & 0x0007, data), // PPU range
            _ => {}
        }
    }
}
