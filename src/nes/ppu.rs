#[derive(Debug, Copy, Clone)]
pub struct Component2C02 {
}

impl Component2C02 {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn cpu_read(&self, addr: u16, _read_only: bool) -> u8 {
        match addr {
            _ => 0
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            _ => {}
        }
    }

    pub fn ppu_read(&self, addr: u16, _read_only: bool) -> u8 {
        match addr {
            _ => 0
        }
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) {
        match addr {
            _ => {}
        }
    }
}
