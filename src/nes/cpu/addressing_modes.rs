use crate::nes::{Bus, Component2C02, Component6502, ComponentCartridge, Controller};

const fn is_a_read_instruction(opcode: u8) -> bool {
    matches!(opcode,
        0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 | // ADC
        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 | // AND
        0x24 | 0x2C |                                           // BIT
        0xCD | 0xDD | 0xD9 | 0xD1 |                             // CMP
        0xEC |                                                  // CPX
        0xCC |                                                  // CPY
        0x4D | 0x5D | 0x59 | 0x51 |                             // EOR
        0xAD | 0xBD | 0xB9 | 0xB1 |                             // LDA
        0xAE | 0xBE |                                           // LDX
        0xAC | 0xBC |                                           // LDY
        0x0D | 0x1D | 0x19 | 0x11 |                             // ORA
        0xED | 0xFD | 0xF9 | 0xF1                               // SBC
    )
}

const fn is_a_read_modify_write_instruction(opcode: u8) -> bool {
    matches!(opcode,
        0x0A | 0x06 | 0x16 | 0x0E | 0x1E |              // ASL
        0xC6 | 0xD6 | 0xCE | 0xDE |                     // DEC
        0xEE | 0xFE |                                   // INC
        0x4E | 0x5E |                                   // LSR
        0x2E | 0x3E |                                   // ROL
        0x6E | 0x7E                                     // ROR
    )
}

const fn is_a_write_instruction(opcode: u8) -> bool {
    matches!(opcode,
        0x8D | 0x9D | 0x99 | 0x91 | // STA
        0x8E |                      // STX
        0x8C                        // STY
    )
}

const fn is_a_stack_instruction(opcode: u8) -> bool {
    matches!(opcode, 0x00 | 0x40 | 0x60 | 0x48 | 0x08 | 0x68 | 0x28 | 0x20)
}

#[allow(non_snake_case)]
impl Component6502 {
    /// Accumulator addressing mode
    pub fn addr_ACC(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &Bus) {
        self.read(self.pc, _controllers, _cartridge, _ppu, _bus);
        self.fetched = self.a;
    }

    /// Immediate addressing mode
    pub fn addr_IMM(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &Bus) {
        self.addr_abs = self.pc;
        self.pc = self.pc.wrapping_add(1);
    }
    
    /// Absolute addressing mode
    pub fn addr_ABS(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let low = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);

        let high = if self.opcode == 0x20 {
            0
        } else {
            let res = self.read(self.pc, controllers, cartridge, ppu, bus);
            self.pc = self.pc.wrapping_add(1);
            res
        };

        let effective_address = ((high as u16) << 8) | low as u16;
        
        if self.opcode == 0x4C || is_a_read_instruction(self.opcode) || is_a_read_modify_write_instruction(self.opcode) || is_a_write_instruction(self.opcode) || is_a_stack_instruction(self.opcode) {
            self.addr_abs = effective_address;
        }
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
        } else if is_a_read_modify_write_instruction(self.opcode) || is_a_write_instruction(self.opcode) {
            self.read(effective_address & 0xFF00 | absolute_address & 0x00FF, controllers, cartridge, ppu, bus);
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

        if is_a_read_instruction(self.opcode) {
            if effective_address & 0xFF00 != absolute_address & 0xFF00 {
                self.cycles += 1;
                self.read(effective_address & 0xFF00 | absolute_address & 0x00FF, controllers, cartridge, ppu, bus);
            }
        } else if is_a_read_modify_write_instruction(self.opcode) || is_a_write_instruction(self.opcode) {
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
        let address = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);

        self.read(address as u16, controllers, cartridge, ppu, bus);

        let effective_address = address.wrapping_add(self.y);

        self.addr_abs = effective_address as u16;
    }
    
    /// Implied addressing mode
    #[allow(clippy::unused_self)]
    pub fn addr_IMP(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        if self.opcode != 0x00 {
            self.read(self.pc, controllers, cartridge, ppu, bus);
        }
    }
    
    /// Relative addressing mode
    pub fn addr_REL(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        let operand = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);

        self.addr_rel = operand as i8 as u16;
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

        let effective_address = ((high as u16) << 8) | low as u16;
        let indirect_address = effective_address.wrapping_add(self.y as u16);
        
        if is_a_read_instruction(self.opcode) {
            if effective_address & 0xFF00 != indirect_address & 0xFF00 {
                self.cycles += 1;
                self.read(effective_address & 0xFF00 | indirect_address & 0x00FF, controllers, cartridge, ppu, bus);
            }
        } else if is_a_read_modify_write_instruction(self.opcode) || is_a_write_instruction(self.opcode) {
            self.read(effective_address & 0xFF00 | indirect_address & 0x00FF, controllers, cartridge, ppu, bus);
        }

        self.addr_abs = indirect_address;
    }
}
