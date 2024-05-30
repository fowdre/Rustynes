mod devices;
mod bus;

pub use bus::Bus;
pub use devices::cpu6502::{Cpu6502, Flags};

#[derive(Debug)]
pub struct Nes {
    cpu: devices::cpu6502::Cpu6502,
    bus: bus::Bus,
}

impl Nes {
    pub fn new() -> Self {
        Self {
            cpu: devices::cpu6502::Cpu6502::new(),
            bus: bus::Bus {
                ram: [0; 64 * 1024],
            },
        }
    }
    
    #[allow(dead_code)]
    pub const fn cpu_read(&self, addr: u16) -> u8 {
        self.cpu.read(&self.bus, addr)
    }

    #[allow(dead_code)]
    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        self.cpu.write(&mut self.bus, addr, data);
    }

    pub fn cpu_tick(&mut self) {
        self.cpu.clock(&mut self.bus);
    }

    pub fn get_ram(&self, low: u16, high: u16) -> (u16, u16, &[u8]) {
        if low > high {
            return (low, high, &[]); // Otherwise taking the slice panics
        }
        (low, high, &self.bus.ram[low as usize..high as usize])
    }
}
