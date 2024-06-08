use crate::nes::{Bus, Component2C02, Component6502, ComponentCartridge};

#[allow(non_snake_case)]
impl Component6502 {
    /// Accumulator addressing mode
    pub fn addr_ACC(&mut self, _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &Bus) {
        self.fetched = self.a;
    }

    /// Immediate addressing mode
    pub fn addr_IMM(&mut self, _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &Bus) {
        self.addr_abs = self.pc;
        self.pc = self.pc.wrapping_add(1);
    }
    
    /// Absolute addressing mode
    pub fn addr_ABS(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let lo = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let hi = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        self.addr_abs = (hi << 8) | lo;
    }
    
    /// Absolute addressing mode with X offset
    pub fn addr_ABX(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let lo = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let hi = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs = self.addr_abs.wrapping_add(self.x as u16);
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            self.cycles += 1;
        }
    }
    
    /// Absolute addressing mode with Y offset
    pub fn addr_ABY(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let lo = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let hi = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs = self.addr_abs.wrapping_add(self.y as u16);
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            self.cycles += 1;
        }
    }
    
    /// Zero Page addressing mode
    pub fn addr_ZP0(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        self.addr_abs = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
    }
    
    /// Zero Page addressing mode with X offset
    pub fn addr_ZPX(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        self.addr_abs = self.read(self.pc, cartridge, ppu, bus) as u16 + self.x as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
    }
    
    /// Zero Page addressing mode with Y offset
    pub fn addr_ZPY(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        self.addr_abs = self.read(self.pc, cartridge, ppu, bus) as u16 + self.y as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
    }
    
    /// Implied addressing mode
    #[allow(clippy::unused_self)]
    pub fn addr_IMP(&mut self, _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &Bus) {
    }
    
    /// Relative addressing mode
    pub fn addr_REL(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        self.addr_rel = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        if (self.addr_rel & 0x80) != 0 {
            self.addr_rel |= 0xFF00;
        }
    }
    
    /// Indirect addressing mode
    /// (implements a hardware bug)
    pub fn addr_IND(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let ptr_lo = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let ptr_hi = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let ptr = (ptr_hi << 8) | ptr_lo;
        
        if ptr_lo == 0x00FF {
            self.addr_abs = (self.read(ptr & 0xFF00, cartridge, ppu, bus) as u16) << 8 | self.read(ptr, cartridge, ppu, bus) as u16;
        } else {
            self.addr_abs = (self.read(ptr + 1, cartridge, ppu, bus) as u16) << 8 | self.read(ptr, cartridge, ppu, bus) as u16;
        }
    }
    
    /// Indirect addressing mode with X offset (zero page)
    pub fn addr_IZX(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let t = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let lo = self.read((t + self.x as u16) & 0x00FF, cartridge, ppu, bus) as u16;
        let hi = self.read((t + self.x as u16 + 1) & 0x00FF, cartridge, ppu, bus) as u16;
        
        self.addr_abs = (hi << 8) | lo;
    }
    
    /// Indirect addressing mode with Y offset (zero page)
    pub fn addr_IZY(&mut self, cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let t = self.read(self.pc, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let lo = self.read(t & 0x00FF, cartridge, ppu, bus) as u16;
        let hi = self.read((t + 1) & 0x00FF, cartridge, ppu, bus) as u16;
        
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs = self.addr_abs.wrapping_add(self.y as u16);
        
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            self.cycles += 1;
        }
    }
}
