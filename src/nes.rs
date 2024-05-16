mod devices;
mod bus;

pub use bus::Bus;
pub use devices::cpu6502::Cpu6502;

#[derive(Debug)]
pub struct NES {
    cpu: devices::cpu6502::Cpu6502,
    bus: bus::Bus,
}

impl NES {
    pub fn new() -> NES {
        NES {
            cpu: devices::cpu6502::Cpu6502::new(),
            bus: bus::Bus {
                ram: [0; 64 * 1024],
            },
        }
    }
    
    pub fn cpu_read(&self, addr: u16) -> u8 {
        self.cpu.read(&self.bus, addr)
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        self.cpu.write(&mut self.bus, addr, data);
    }

    pub fn cpu_tick(&mut self) {
        self.cpu.clock(&mut self.bus);
    }
}
