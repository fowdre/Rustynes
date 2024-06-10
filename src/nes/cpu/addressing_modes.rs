use crate::nes::{Bus, Component2C02, Component6502, ComponentCartridge, Controller};

const fn is_a_read_instruction(opcode: u8) -> bool {
    matches!(opcode, 0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71)
}

#[allow(non_snake_case)]
impl Component6502 {
    /// Accumulator addressing mode
    pub fn addr_ACC(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &Bus) {
        self.fetched = self.a;
    }

    /// Immediate addressing mode
    pub fn addr_IMM(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &Bus) {
        self.addr_abs = self.pc;
        self.pc = self.pc.wrapping_add(1);
    }
    
    /// Absolute addressing mode
    pub fn addr_ABS(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let lo = self.read(self.pc, controllers, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let hi = self.read(self.pc, controllers, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        self.addr_abs = (hi << 8) | lo;
    }
    
    /// Absolute addressing mode with X offset
    pub fn addr_ABX(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let low = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);
        let high = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);

        let effective_address = ((high as u16) << 8) | low as u16;
        let absolute_address = effective_address.wrapping_add(self.x as u16);
        
        if is_a_read_instruction(self.opcode) {
            if effective_address & 0xFF00 != absolute_address & 0xFF00 {
                self.cycles += 1;
                self.read(effective_address & 0xFF00 | absolute_address & 0x00FF, controllers, cartridge, ppu, bus);
            }
        }

        self.addr_abs = absolute_address;
    }
    
    /// Absolute addressing mode with Y offset
    pub fn addr_ABY(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let low = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);
        let high = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);

        let effective_address = ((high as u16) << 8) | low as u16;
        let absolute_address = effective_address.wrapping_add(self.y as u16);

        if effective_address & 0xFF00 != absolute_address & 0xFF00 {
            self.cycles += 1;
            self.read(effective_address & 0xFF00 | absolute_address & 0x00FF, controllers, cartridge, ppu, bus);
        }

        self.addr_abs = absolute_address;
    }
    
    /// Zero Page addressing mode
    pub fn addr_ZP0(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let effective_address = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);
        
        self.addr_abs = effective_address as u16;
    }
    
    /// Zero Page addressing mode with X offset
    pub fn addr_ZPX(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let address = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);

        self.read(address as u16, controllers, cartridge, ppu, bus);

        let effective_address = address.wrapping_add(self.x);

        self.addr_abs = effective_address as u16;
    }
    
    /// Zero Page addressing mode with Y offset
    pub fn addr_ZPY(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        self.addr_abs = self.read(self.pc, controllers, cartridge, ppu, bus) as u16 + self.y as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;
    }
    
    /// Implied addressing mode
    #[allow(clippy::unused_self)]
    pub fn addr_IMP(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &Bus) {
    }
    
    /// Relative addressing mode
    pub fn addr_REL(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        self.addr_rel = self.read(self.pc, controllers, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        if (self.addr_rel & 0x80) != 0 {
            self.addr_rel |= 0xFF00;
        }
    }
    
    /// Indirect addressing mode
    /// (implements a hardware bug)
    pub fn addr_IND(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let ptr_lo = self.read(self.pc, controllers, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let ptr_hi = self.read(self.pc, controllers, cartridge, ppu, bus) as u16;
        self.pc = self.pc.wrapping_add(1);
        
        let ptr = (ptr_hi << 8) | ptr_lo;
        
        if ptr_lo == 0x00FF {
            self.addr_abs = (self.read(ptr & 0xFF00, controllers, cartridge, ppu, bus) as u16) << 8 | self.read(ptr, controllers, cartridge, ppu, bus) as u16;
        } else {
            self.addr_abs = (self.read(ptr + 1, controllers, cartridge, ppu, bus) as u16) << 8 | self.read(ptr, controllers, cartridge, ppu, bus) as u16;
        }
    }
    
    /// Indirect addressing mode with X offset (zero page)
    pub fn addr_IZX(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let pointer = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);
        
        self.read(pointer as u16, controllers, cartridge, ppu, bus);
        let effective_addr = pointer.wrapping_add(self.x);
        
        let low = self.read(effective_addr as u16, controllers, cartridge, ppu, bus);
        let high = self.read(effective_addr.wrapping_add(1) as u16, controllers, cartridge, ppu, bus);

        self.addr_abs = ((high as u16) << 8) | low as u16;
    }
    
    /// Indirect addressing mode with Y offset (zero page)
    pub fn addr_IZY(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let pointer = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);

        let low = self.read(pointer as u16, controllers, cartridge, ppu, bus);
        let high = self.read(pointer.wrapping_add(1) as u16, controllers, cartridge, ppu, bus);

        let effective_addr = ((high as u16) << 8) | low as u16;
        let indirect_addr = effective_addr.wrapping_add(self.y as u16);
        
        if effective_addr & 0xFF00 != indirect_addr & 0xFF00 {
            self.cycles += 1;

            self.read(effective_addr & 0xFF00 | indirect_addr & 0x00FF, controllers, cartridge, ppu, bus);
        }

        self.addr_abs = indirect_addr;
    }
}
