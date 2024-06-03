use crate::nes::{Cartridge, Cpu6502, Flags, Ppu2C02, Bus};

#[allow(non_snake_case)]
impl Cpu6502 {
    /// Illegal opcode
    pub fn xxx(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        0
    }

    /// Add Memory to Accumulator with Carry
    pub fn ADC(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);

        let (tmp, has_overflowed1): (u16, bool) = (self.a as u16).overflowing_add(self.fetched as u16);
        let (tmp, has_overflowed2): (u16, bool) = tmp.overflowing_add(self.get_flag(Flags::C) as u16);

        // Set Carry Flag if overflowed
        self.set_flag(Flags::C, tmp > 255);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0);
        self.set_flag(Flags::N, tmp & 0x80 != 0);
        // Set Overflow Flag if any of the additions overflowed
        self.set_flag(Flags::V, has_overflowed1 || has_overflowed2);
        
        self.a = (tmp & 0x00FF) as u8;
        
        1
    }
    /// "AND" Memory with Accumulator
    pub fn AND(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        self.a &= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
        
        1
    }
    /// Shift Left One Bit (Memory or Accumulator)
    pub fn ASL(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 { 
        self.fetch(cartridge, ppu, bus);
        
        let tmp: u16 = (self.fetched as u16) << 1;
        
        self.set_flag(Flags::C, (tmp & 0xFF00) > 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, tmp & 0x80 != 0);
        
        if (self.lookup[self.opcode as usize].addr_mode as usize == Self::addr_ACC as usize)
        || (self.lookup[self.opcode as usize].addr_mode as usize == Self::addr_IMP as usize) {
            self.a = (tmp & 0x00FF) as u8;
        } else {
            self.write(cartridge, ppu, bus, self.addr_abs, (tmp & 0x00FF) as u8);
        }
        
        0
    }
    
    /// Branch on Carry Clear
    pub fn BCC(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        if !self.get_flag(Flags::C) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
        
        0
    }
	/// Branch on Carry Set
    pub fn BCS(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        if self.get_flag(Flags::C) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
        
        0
    }
    /// Branch on Result Zero
    pub fn BEQ(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        if self.get_flag(Flags::Z) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
        
        0
    }
    /// Test Bits in Memory with Accumulator
    pub fn BIT(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        
        let tmp: u16 = (self.a & self.fetched) as u16;
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, self.fetched & (1 << 7) != 0);
        self.set_flag(Flags::V, self.fetched & (1 << 6) != 0);
        
        0
    }
    /// Branch on Result Minus
    pub fn BMI(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        if self.get_flag(Flags::N) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
        
        0
    }
	/// Branch on Result not Zero
    pub fn BNE(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        if !self.get_flag(Flags::Z) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
        
        0
    }
    /// Branch on Result Plus
    pub fn BPL(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        if !self.get_flag(Flags::N) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
        
        0
    }
    /// Force Break
    pub fn BRK(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        
        self.set_flag(Flags::I, true);
        self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        
        self.set_flag(Flags::B, true);
        self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, self.status);
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(Flags::B, false);
        
        self.pc = self.read(cartridge, ppu, bus, 0xFFFE) as u16 | ((self.read(cartridge, ppu, bus, 0xFFFF) as u16) << 8);
        
        0
    }
    /// Branch on Overflow Clear
    pub fn BVC(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        if !self.get_flag(Flags::V) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
        
        0
    }
	/// Branch on Overflow Set
    pub fn BVS(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        if self.get_flag(Flags::V) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
        
        0
    }
    
    /// Clear Carry Flag
    pub fn CLC(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.set_flag(Flags::C, false);
        
        0
    }
    /// Clear Decimal Mode Flag
    pub fn CLD(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.set_flag(Flags::D, false);
        
        0
    }
    /// Clear Interrupt Disable Bit Flag
    pub fn CLI(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.set_flag(Flags::I, false);
        
        0
    }
	/// Clear Overflow Flag
    pub fn CLV(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.set_flag(Flags::V, false);
        
        0
    }
    /// Compare Memory and Accumulator
    pub fn CMP(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        
        let tmp: u16 = (self.a as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.a >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, (tmp & 0x0080) != 0);
        
        1
    }
    /// Compare Memory and Index X
    pub fn CPX(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        
        let tmp: u16 = (self.x as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.x >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
        
        1
    }
    /// Compare Memory and Index Y
    pub fn CPY(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        
        let tmp: u16 = (self.y as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.y >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
        
        1
    }
    
	/// Decrement Memory by One
    pub fn DEC(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        let tmp: u16 = self.fetched as u16 - 1;
        
        self.write(cartridge, ppu, bus, self.addr_abs, (tmp & 0x00FF) as u8);
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
        
        0
    }
    /// Decrement Index X by One
    pub fn DEX(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.x = self.x.wrapping_sub(1);
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
        
        0
    }
    /// Decrement Index Y by One
    pub fn DEY(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.y = self.y.wrapping_sub(1);
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
        
        0
    }
    
    /// "Exclusive-OR" Memory with Accumulator
    pub fn EOR(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        self.a ^= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
        
        1
    }
    
	/// Increment Memory by One
    pub fn INC(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        let tmp: u16 = self.fetched as u16 + 1;
        
        self.write(cartridge, ppu, bus, self.addr_abs, (tmp & 0x00FF) as u8);
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
        
        0
    }
    /// Increment Index X by One
    pub fn INX(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.x = self.x.wrapping_add(1);
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
        
        0
    }
    /// Increment Index Y by One
    pub fn INY(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.y = self.y.wrapping_add(1);
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
        
        0
    }
    
    /// Jump to New Location
    pub fn JMP(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.pc = self.addr_abs;
        
        0
    }
	/// Jump to New Location Saving Return Address
    pub fn JSR(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.pc = self.pc.wrapping_sub(1);
        
        self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        
        self.pc = self.addr_abs;
        
        0
    }
    
    /// Load Accumulator with Memory
    pub fn LDA(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        self.a = self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
        
        1
    }
    /// Load Index X with Memory
    pub fn LDX(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        self.x = self.fetched;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
        
        1
    }
    /// Load Index Y with Memory
    pub fn LDY(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        self.y = self.fetched;
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
        
        1
    }
	/// Shift Right One Bit (Memory or Accumulator)
    pub fn LSR(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        
        let tmp: u16 = (self.fetched as u16) >> 1;
        
        self.set_flag(Flags::C, (self.fetched & 0x0001) != 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
        
        if (self.lookup[self.opcode as usize].addr_mode as usize == Self::addr_ACC as usize)
        || (self.lookup[self.opcode as usize].addr_mode as usize == Self::addr_IMP as usize) {
            self.a = (tmp & 0x00FF) as u8;
        } else {
            self.write(cartridge, ppu, bus, self.addr_abs, (tmp & 0x00FF) as u8);
        }
        
        0
    }
    
    /// No Operation
    pub fn NOP(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        match self.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1,
            _ => 0,
        }
    }
    
    /// "OR" Memory with Accumulator
    pub fn ORA(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        self.a |= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
        
        1
    }
    
    /// Push Accumulator on Stack
    pub fn PHA(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, self.a);
        self.sp = self.sp.wrapping_sub(1);
        
        0
    }
	/// Push Processor Status on Stack
    pub fn PHP(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, self.status | Flags::B as u8 | Flags::U as u8);
        self.set_flag(Flags::B, false);
        self.set_flag(Flags::U, false);
        
        0
    }
    /// Pull Accumulator from Stack
    pub fn PLA(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.a = self.read(cartridge, ppu, bus, 0x0100 + self.sp as u16);
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0x80) != 0);
        
        0
    }
    /// Pull Processor Status from Stack
    pub fn PLP(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.status = self.read(cartridge, ppu, bus, 0x0100 + self.sp as u16);
        self.set_flag(Flags::U, true);
        
        0
    }
    
    /// Rotate One Bit Left (Memory or Accumulator)
    pub fn ROL(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        
        let tmp: u16 = (self.fetched as u16) << 1 | self.get_flag(Flags::C) as u16;
        
        self.set_flag(Flags::C, (tmp & 0xFF00) != 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, tmp & 0x80 != 0);
        
        if (self.lookup[self.opcode as usize].addr_mode as usize == Self::addr_ACC as usize)
        || (self.lookup[self.opcode as usize].addr_mode as usize == Self::addr_IMP as usize) {
            self.a = (tmp & 0x00FF) as u8;
        } else {
            self.write(cartridge, ppu, bus, self.addr_abs, (tmp & 0x00FF) as u8);
        }
        
        0
    }
	/// Rotate One Bit Right (Memory or Accumulator)
    pub fn ROR(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        
        let tmp: u16 = (self.get_flag(Flags::C) as u16) << 7 | (self.fetched as u16) >> 1;
        
        self.set_flag(Flags::C, (self.fetched & 0x01) != 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, tmp & 0x80 != 0);
        
        if (self.lookup[self.opcode as usize].addr_mode as usize == Self::addr_ACC as usize)
        || (self.lookup[self.opcode as usize].addr_mode as usize == Self::addr_IMP as usize) {
            self.a = (tmp & 0x00FF) as u8;
        } else {
            self.write(cartridge, ppu, bus, self.addr_abs, (tmp & 0x00FF) as u8);
        }
        
        0
    }
    /// Return from Interrupt
    pub fn RTI(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.status = self.read(cartridge, ppu, bus, 0x0100 + self.sp as u16);
        self.status &= !(Flags::B as u8);
        self.status &= !(Flags::U as u8);
        
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(cartridge, ppu, bus, 0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (self.read(cartridge, ppu, bus, 0x0100 + self.sp as u16) as u16) << 8;
        
        0
    }
    /// Return from Subroutine
    pub fn RTS(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(cartridge, ppu, bus, 0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (self.read(cartridge, ppu, bus, 0x0100 + self.sp as u16) as u16) << 8;
        
        self.pc = self.pc.wrapping_add(1);
        
        0
    }
    
    /// Subtract Memory from Accumulator with Borrow
    pub fn SBC(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.fetch(cartridge, ppu, bus);
        
        // Use two's complement to treat subtraction as addition
        let value: u16 = (self.fetched ^ 0x00FF) as u16;
        
        let (tmp, has_overflowed1): (u16, bool) = (self.a as u16).overflowing_add(value);
        let (tmp, has_overflowed2): (u16, bool) = tmp.overflowing_add(self.get_flag(Flags::C) as u16);
        
        // Set Carry Flag if overflowed
        self.set_flag(Flags::C, tmp > 255);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0);
        self.set_flag(Flags::N, tmp & 0x80 != 0);
        // Set Overflow Flag if any of the additions overflowed
        self.set_flag(Flags::V, has_overflowed1 || has_overflowed2);
        
        self.a = (tmp & 0x00FF) as u8;
        
        1
    }
	/// Set Carry Flag
    pub fn SEC(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.set_flag(Flags::C, true);
        
        0
    }
    /// Set Decimal Mode
    pub fn SED(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.set_flag(Flags::D, true);
        
        0
    }
    /// Set Interrupt Disable Status
    pub fn SEI(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.set_flag(Flags::I, true);
        
        0
    }
    /// Store Accumulator in Memory
    pub fn STA(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.write(cartridge, ppu, bus, self.addr_abs, self.a);
        
        0
    }
	/// Store Index X in Memory
    pub fn STX(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.write(cartridge, ppu, bus, self.addr_abs, self.x);
        
        0
    }
    /// Store Index Y in Memory
    pub fn STY(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) -> u8 {
        self.write(cartridge, ppu, bus, self.addr_abs, self.y);
        
        0
    }
    
    /// Transfer Accumulator to Index X
    pub fn TAX(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.x = self.a;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
        
        0
    }
    /// Transfer Accumulator to Index Y
    pub fn TAY(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.y = self.a;
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
        
        0
    }
	/// Transfer Stack Pointer to Index X
    pub fn TSX(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.x = self.sp;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
        
        0
    }
    /// Transfer Index X to Accumulator
    pub fn TXA(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.a = self.x;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
        
        0
    }
    /// Transfer Index X to Stack Pointer
    pub fn TXS(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.sp = self.x;
        
        0
    }
    /// Transfer Index Y to Accumulator
    pub fn TYA(&mut self, _cartridge: &mut Cartridge, _ppu: &mut Ppu2C02, _bus: &mut Bus) -> u8 {
        self.a = self.y;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
        
        0
    }
}
