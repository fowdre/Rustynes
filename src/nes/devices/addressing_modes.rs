use super::super::{Cpu6502, Bus};

#[allow(non_snake_case)]
impl Cpu6502 {
    /// Accumulator addressing mode
    pub fn addr_ACC(&mut self, _bus: &Bus) -> u8 {
        self.fetched = self.a;
        
        0
    }

    /// Immediate addressing mode
    pub fn addr_IMM(&mut self, _bus: &Bus) -> u8 {
        self.addr_abs = self.pc;
        self.pc += 1;
        
        0
    }
    
    /// Absolute addressing mode
    pub fn addr_ABS(&mut self, _bus: &Bus) -> u8 {
        let lo = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        let hi = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        self.addr_abs = (hi << 8) | lo;
        
        0
    }
    
    /// Absolute addressing mode with X offset
    pub fn addr_ABSx(&mut self, _bus: &Bus) -> u8 {
        let lo = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        let hi = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        
        0
    }
    
    /// Absolute addressing mode with Y offset
    pub fn addr_ABSy(&mut self, _bus: &Bus) -> u8 {
        let lo = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        let hi = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        
        0
    }
    
    /// Zero Page addressing mode
    pub fn addr_ZPG(&mut self, bus: &Bus) -> u8 {
        self.addr_abs = self.read(bus, self.pc as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        
        0
    }
    
    /// Zero Page addressing mode with X offset
    pub fn addr_ZPGx(&mut self, _bus: &Bus) -> u8 {
        self.addr_abs = self.read(_bus, self.pc as u16) as u16 + self.x as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        
        0
    }
    
    /// Zero Page addressing mode with Y offset
    pub fn addr_ZPGy(&mut self, _bus: &Bus) -> u8 {
        self.addr_abs = self.read(_bus, self.pc as u16) as u16 + self.y as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        
        0
    }
    
    /// Implied addressing mode
    pub fn addr_IMP(&mut self, _bus: &Bus) -> u8 {
        0
    }
    
    /// Relative addressing mode
    pub fn addr_REL(&mut self, _bus: &Bus) -> u8 {
        self.addr_rel = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        if (self.addr_rel & 0x80) != 0 {
            self.addr_rel |= 0xFF00;
        }
        
        0
    }
    
    /// Indirect addressing mode
    /// (implements a hardware bug)
    pub fn addr_IND(&mut self, _bus: &Bus) -> u8 {
        let ptr_lo = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        let ptr_hi = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        let ptr = (ptr_hi << 8) | ptr_lo;
        
        if ptr_lo == 0x00FF {
            self.addr_abs = (self.read(_bus, ptr & 0xFF00) as u16) << 8 | self.read(_bus, ptr) as u16;
        } else {
            self.addr_abs = (self.read(_bus, ptr + 1) as u16) << 8 | self.read(_bus, ptr) as u16;
        }
        
        0
    }
    
    /// Indirect addressing mode with X offset (zero page)
    pub fn addr_INDx(&mut self, _bus: &Bus) -> u8 {
        let t = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        let lo = self.read(_bus, (t + self.x as u16) & 0x00FF) as u16;
        let hi = self.read(_bus, (t + self.x as u16 + 1) & 0x00FF) as u16;
        
        self.addr_abs = (hi << 8) | lo;
        
        0
    }
    
    /// Indirect addressing mode with Y offset (zero page)
    pub fn addr_INDy(&mut self, _bus: &Bus) -> u8 {
        let t = self.read(_bus, self.pc as u16) as u16;
        self.pc += 1;
        
        let lo = self.read(_bus, t & 0x00FF) as u16;
        let hi = self.read(_bus, (t + 1) & 0x00FF) as u16;
        
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        
        0
    }
}
