use crate::nes::{ComponentCartridge, Component2C02};

use super::Controller;

#[derive(Debug, Copy, Clone)]
pub struct Bus {
    pub ram: [u8; 64 * 1024],
}

impl Bus {
    pub fn cpu_read(&self, addr: u16, read_only: bool, controllers: &mut [Controller; 2], cartridge: &ComponentCartridge, ppu: &mut Component2C02) -> u8 {
        let mut data = 0x00;

        // Cartridge has priority over everything else (mappers)
        if cartridge.cpu_read(addr, &mut data) {
            return data;
        }
        match addr {
            // RAM range
            0x0000..=0x1FFF => data = self.ram[(addr & 0x07FF) as usize],
            // PPU range
            0x2000..=0x3FFF => data = ppu.cpu_read(addr & 0x0007, read_only, cartridge),
            // Controller range
            0x4016..=0x4017 => {
                // data = controllers[(addr & 0x0001) as usize].read(),
                data = if controllers[(addr & 0x0001) as usize].controller_state & 0x80 != 0 { 1 } else { 0 };
                controllers[(addr & 0x0001) as usize].controller_state <<= 1;
            }
            _ => {}
        };

        data
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02) {
        // Cartridge has priority over everything else (mappers)
        if cartridge.cpu_write(addr, data) {
            return;
        }
        match addr {
            // RAM range
            0x0000..=0x1FFF => self.ram[(addr & 0x07FF) as usize] = data,
            // PPU range
            0x2000..=0x3FFF => ppu.cpu_write(addr & 0x0007, data, cartridge),
            // Controller range
            0x4016..=0x4017 => {
                // controllers[(addr & 0x0001) as usize].write(),
                controllers[(addr & 0x0001) as usize].controller_state = controllers[(addr & 0x0001) as usize].controller;
            }
            _ => {}
        }
    }
}
