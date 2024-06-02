use crate::nes::{Cpu6502, Bus, Cartridge};


#[allow(non_snake_case)]
impl Cpu6502 {
    /// Accumulator addressing mode
    pub fn addr_ACC(&mut self, _bus: &Bus, _cartridge: &Cartridge) -> u8 {
        self.fetched = self.a;
        
        0
    }

    /// Immediate addressing mode
    pub fn addr_IMM(&mut self, _bus: &Bus, _cartridge: &Cartridge) -> u8 {
        self.addr_abs = self.pc;
        self.pc = self.pc.wrapping_add(1);
        
        0
    }
    
    /// Absolute addressing mode
    pub fn addr_ABS(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        let lo = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let hi = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        self.addr_abs = (hi << 8) | lo;
        
        0
    }
    
    /// Absolute addressing mode with X offset
    pub fn addr_ABSx(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        let lo = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let hi = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs = self.addr_abs.wrapping_add(self.x as u16);
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        
        0
    }
    
    /// Absolute addressing mode with Y offset
    pub fn addr_ABSy(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        let lo = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let hi = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs = self.addr_abs.wrapping_add(self.y as u16);
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        
        0
    }
    
    /// Zero Page addressing mode
    pub fn addr_ZPG(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        self.addr_abs = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
        
        0
    }
    
    /// Zero Page addressing mode with X offset
    pub fn addr_ZPGx(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        self.addr_abs = self.read(bus, cartridge, self.pc) as u16 + self.x as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
        
        0
    }
    
    /// Zero Page addressing mode with Y offset
    pub fn addr_ZPGy(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        self.addr_abs = self.read(bus, cartridge, self.pc) as u16 + self.y as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
        
        0
    }
    
    /// Implied addressing mode
    pub fn addr_IMP(&mut self, _bus: &Bus, _cartridge: &Cartridge) -> u8 {
        0
    }
    
    /// Relative addressing mode
    pub fn addr_REL(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        self.addr_rel = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        if (self.addr_rel & 0x80) != 0 {
            self.addr_rel |= 0xFF00;
        }
        
        0
    }
    
    /// Indirect addressing mode
    /// (implements a hardware bug)
    pub fn addr_IND(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        let ptr_lo = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let ptr_hi = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let ptr = (ptr_hi << 8) | ptr_lo;
        
        if ptr_lo == 0x00FF {
            self.addr_abs = (self.read(bus, cartridge, ptr & 0xFF00) as u16) << 8 | self.read(bus, cartridge, ptr) as u16;
        } else {
            self.addr_abs = (self.read(bus, cartridge, ptr + 1) as u16) << 8 | self.read(bus, cartridge, ptr) as u16;
        }
        
        0
    }
    
    /// Indirect addressing mode with X offset (zero page)
    pub fn addr_INDx(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        let t = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let lo = self.read(bus, cartridge, (t + self.x as u16) & 0x00FF) as u16;
        let hi = self.read(bus, cartridge, (t + self.x as u16 + 1) & 0x00FF) as u16;
        
        self.addr_abs = (hi << 8) | lo;
        
        0
    }
    
    /// Indirect addressing mode with Y offset (zero page)
    pub fn addr_INDy(&mut self, bus: &Bus, cartridge: &Cartridge) -> u8 {
        let t = self.read(bus, cartridge, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let lo = self.read(bus, cartridge, t & 0x00FF) as u16;
        let hi = self.read(bus, cartridge, (t + 1) & 0x00FF) as u16;
        
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs = self.addr_abs.wrapping_add(self.y as u16);
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        
        0
    }
}
