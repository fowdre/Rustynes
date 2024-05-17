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
    pub fn new() -> Nes {
        Nes {
            cpu: devices::cpu6502::Cpu6502::new(),
            bus: bus::Bus {
                ram: [0; 64 * 1024],
            },
        }
    }
    
    #[allow(dead_code)]
    pub fn cpu_read(&self, addr: u16) -> u8 {
        self.cpu.read(&self.bus, addr)
    }

    #[allow(dead_code)]
    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        self.cpu.write(&mut self.bus, addr, data);
    }

    pub fn cpu_tick(&mut self) {
        self.cpu.clock(&mut self.bus);
    }
}
