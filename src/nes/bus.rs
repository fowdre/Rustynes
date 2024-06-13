use crate::nes::{ComponentCartridge, Component2C02};

use super::Controller;

#[derive(Debug, Copy, Clone)]
pub struct Bus {
    pub ram: [u8; 64 * 1024],

    pub dma_page: u8,
    pub dma_addr: u8,
    pub dma_data: u8,
    pub is_dma_active: bool,
    pub dma_wait_for_sync: bool,
}

impl Bus {
    pub const fn new() -> Bus {
        Bus {
            ram: [0; 64 * 1024],
            dma_page: 0,
            dma_addr: 0,
            dma_data: 0,
            is_dma_active: false,
            dma_wait_for_sync: true,
        }
    }

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
                data = controllers[(addr & 0x0001) as usize].read();
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
            // DMA range
            0x4014 => {
                self.dma_page = data;
                self.dma_addr = 0x00;
                self.is_dma_active = true;
            }
            // Controller range
            0x4016..=0x4017 => {
                controllers[(addr & 0x0001) as usize].write();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Bus {
    pub fn test_read(&self, addr: u16) -> u8 {
        let result = self.ram[addr as usize];
        
        println!("dbg [{}, {}, \"read\"]", addr, result);
        crate::tests::cycles_trace::CYCLES.lock().unwrap().push((addr, result, "read".to_string()));
        
        result
    }

    pub fn test_write(&mut self, addr: u16, data: u8) {
        println!("dbg [{}, {}, \"write\"]", addr, data);
        crate::tests::cycles_trace::CYCLES.lock().unwrap().push((addr, data, "write".to_string()));
        self.ram[addr as usize] = data;
    }
}
