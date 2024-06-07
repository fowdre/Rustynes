#![allow(clippy::cast_lossless, clippy::verbose_bit_mask)]

use crate::nes::{Bus, Component6502, Flags, ADDRESSING_MODES, STACK_ADDRESS};

#[allow(non_snake_case)]
impl Component6502 {
    /// Illegal opcode
    #[allow(clippy::unused_self)]
    pub fn xxx(&mut self, _bus: &mut Bus) {
    }

    /// Add Memory to Accumulator with Carry
    pub fn ADC(&mut self, bus: &mut Bus) {
        self.fetch(bus);

        let tmp = (self.a as u16).wrapping_add(self.fetched as u16).wrapping_add(self.get_flag(Flags::C) as u16);

        // Set Carry Flag if overflowed
        self.set_flag(Flags::C, tmp > 255);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0);
        // Set Overflow Flag if any of the additions overflowed
        self.set_flag(Flags::V, (((!((self.a as u16) ^ (self.fetched as u16))) & ((self.a as u16) ^ tmp)) & 0x0080) != 0);
        self.set_flag(Flags::N, (tmp & 0x80) != 0);
        
        self.a = (tmp & 0x00FF) as u8;
    }
    /// "AND" Memory with Accumulator
    pub fn AND(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        self.a &= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    /// Shift Left One Bit (Memory or Accumulator)
    pub fn ASL(&mut self, bus: &mut Bus) { 
        self.fetch(bus);
        
        let tmp: u16 = (self.fetched as u16) << 1;
        
        self.set_flag(Flags::C, (tmp & 0xFF00) > 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, tmp & 0x80 != 0);
        
        if (self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::ACC)
        || (self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::IMP) {
            self.a = (tmp & 0x00FF) as u8;
        } else {
            self.write(bus, self.addr_abs, (tmp & 0x00FF) as u8);
        }
    }

    /// Test Bits in Memory with Accumulator
    pub fn BIT(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        
        let tmp: u16 = (self.a & self.fetched) as u16;
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, self.fetched & (1 << 7) != 0);
        self.set_flag(Flags::V, self.fetched & (1 << 6) != 0);
    }

    /// Force Break
    pub fn BRK(&mut self, bus: &mut Bus) {
        self.pc = self.pc.wrapping_add(1);
        
        self.set_flag(Flags::I, true);
        self.write(bus, STACK_ADDRESS + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(bus, STACK_ADDRESS + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        
        self.set_flag(Flags::B, true);
        self.write(bus, STACK_ADDRESS + self.sp as u16, self.status);
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(Flags::B, false);
        
        self.pc = self.read(bus, 0xFFFE) as u16 | ((self.read(bus, 0xFFFF) as u16) << 8);
    }

    // Generic branch instruction
    pub fn branch(&mut self, _bus: &mut Bus) {
        self.cycles += 1;
        self.addr_abs = self.pc.wrapping_add(self.addr_rel);
        
        if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
            self.cycles += 1;
        }
        
        self.pc = self.addr_abs;
    }
    
    /// Branch on Carry Clear
    pub fn BCC(&mut self, bus: &mut Bus) {
        if !self.get_flag(Flags::C) {
            self.branch(bus);
        }
    }
	/// Branch on Carry Set
    pub fn BCS(&mut self, bus: &mut Bus) {
        if self.get_flag(Flags::C) {
            self.branch(bus);
        }
    }
    /// Branch on Result Zero
    pub fn BEQ(&mut self, bus: &mut Bus) {
        if self.get_flag(Flags::Z) {
            self.branch(bus);
        }
    }
    /// Branch on Result Minus
    pub fn BMI(&mut self, bus: &mut Bus) {
        if self.get_flag(Flags::N) {
            self.branch(bus);
        }
    }
	/// Branch on Result not Zero
    pub fn BNE(&mut self, bus: &mut Bus) {
        if !self.get_flag(Flags::Z) {
            self.branch(bus);
        }
    }
    /// Branch on Result Plus
    pub fn BPL(&mut self, bus: &mut Bus) {
        if !self.get_flag(Flags::N) {
            self.branch(bus);
        }
    }
    /// Branch on Overflow Clear
    pub fn BVC(&mut self, _bus: &mut Bus) {
        if !self.get_flag(Flags::V) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
    }
	/// Branch on Overflow Set
    pub fn BVS(&mut self, _bus: &mut Bus) {
        if self.get_flag(Flags::V) {
            self.cycles = self.cycles.wrapping_add(1);
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            
            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles = self.cycles.wrapping_add(1);
            }
            
            self.pc = self.addr_abs;
        }
    }
    
    /// Clear Carry Flag
    pub fn CLC(&mut self, _bus: &mut Bus) {
        self.set_flag(Flags::C, false);
    }
    /// Clear Decimal Mode Flag
    pub fn CLD(&mut self, _bus: &mut Bus) {
        self.set_flag(Flags::D, false);
    }
    /// Clear Interrupt Disable Bit Flag
    pub fn CLI(&mut self, _bus: &mut Bus) {
        self.set_flag(Flags::I, false);
    }
	/// Clear Overflow Flag
    pub fn CLV(&mut self, _bus: &mut Bus) {
        self.set_flag(Flags::V, false);
    }

    /// Compare Memory and Accumulator
    pub fn CMP(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        
        let tmp: u16 = (self.a as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.a >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, (tmp & 0x0080) != 0);
    }
    /// Compare Memory and Index X
    pub fn CPX(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        
        let tmp: u16 = (self.x as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.x >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
    }
    /// Compare Memory and Index Y
    pub fn CPY(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        
        let tmp: u16 = (self.y as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.y >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
    }
    
	/// Decrement Memory by One
    pub fn DEC(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        let tmp = self.fetched.wrapping_sub(1) as u16;
        
        self.write(bus, self.addr_abs, (tmp & 0x00FF) as u8);
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
    }
    /// Decrement Index X by One
    pub fn DEX(&mut self, _bus: &mut Bus) {
        self.x = self.x.wrapping_sub(1);
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Decrement Index Y by One
    pub fn DEY(&mut self, _bus: &mut Bus) {
        self.y = self.y.wrapping_sub(1);
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
    }
    
    /// "Exclusive-OR" Memory with Accumulator
    pub fn EOR(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        self.a ^= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    
	/// Increment Memory by One
    pub fn INC(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        let tmp: u16 = self.fetched as u16 + 1;
        
        self.write(bus, self.addr_abs, (tmp & 0x00FF) as u8);
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
    }
    /// Increment Index X by One
    pub fn INX(&mut self, _bus: &mut Bus) {
        self.x = self.x.wrapping_add(1);
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Increment Index Y by One
    pub fn INY(&mut self, _bus: &mut Bus) {
        self.y = self.y.wrapping_add(1);
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
    }
    
    /// Jump to New Location
    pub fn JMP(&mut self, _bus: &mut Bus) {
        self.pc = self.addr_abs;
    }
	/// Jump to New Location Saving Return Address
    pub fn JSR(&mut self, bus: &mut Bus) {
        self.pc = self.pc.wrapping_sub(1);
        
        self.write(bus, STACK_ADDRESS + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(bus, STACK_ADDRESS + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        
        self.pc = self.addr_abs;
    }
    
    /// Load Accumulator with Memory
    pub fn LDA(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        self.a = self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    /// Load Index X with Memory
    pub fn LDX(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        self.x = self.fetched;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Load Index Y with Memory
    pub fn LDY(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        self.y = self.fetched;
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
    }
	/// Shift Right One Bit (Memory or Accumulator)
    pub fn LSR(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        
        let tmp: u16 = (self.fetched as u16) >> 1;
        
        self.set_flag(Flags::C, (self.fetched & 0x0001) != 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
        
        if (self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::ACC)
        || (self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::IMP) {
            self.a = (tmp & 0x00FF) as u8;
        } else {
            self.write(bus, self.addr_abs, (tmp & 0x00FF) as u8);
        }
    }
    
    /// No Operation
    #[allow(clippy::unused_self)]
    pub fn NOP(&mut self, _bus: &mut Bus) {
    }
    
    /// "OR" Memory with Accumulator
    pub fn ORA(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        self.a |= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    
    /// Push Accumulator on Stack
    pub fn PHA(&mut self, bus: &mut Bus) {
        self.write(bus, STACK_ADDRESS + self.sp as u16, self.a);
        self.sp = self.sp.wrapping_sub(1);
    }
	/// Push Processor Status on Stack
    pub fn PHP(&mut self, bus: &mut Bus) {
        self.set_flag(Flags::B, true);
        self.set_flag(Flags::U, true);
        self.write(bus, STACK_ADDRESS + self.sp as u16, self.status);
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(Flags::B, false);
    }
    /// Pull Accumulator from Stack
    pub fn PLA(&mut self, bus: &mut Bus) {
        self.sp = self.sp.wrapping_add(1);
        self.a = self.read(bus, STACK_ADDRESS + self.sp as u16);
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0x80) != 0);
    }
    /// Pull Processor Status from Stack
    pub fn PLP(&mut self, bus: &mut Bus) {
        self.sp = self.sp.wrapping_add(1);
        self.status = self.read(bus, STACK_ADDRESS + self.sp as u16);
        self.status &= !(Flags::B as u8);
        self.set_flag(Flags::U, true);
    }
    
    /// Rotate One Bit Left (Memory or Accumulator)
    pub fn ROL(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        
        let tmp: u16 = (self.fetched as u16) << 1 | self.get_flag(Flags::C) as u16;
        
        self.set_flag(Flags::C, (tmp & 0xFF00) != 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, tmp & 0x80 != 0);
        
        if (self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::ACC)
        || (self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::IMP) {
            self.a = (tmp & 0x00FF) as u8;
        } else {
            self.write(bus, self.addr_abs, (tmp & 0x00FF) as u8);
        }
    }
	/// Rotate One Bit Right (Memory or Accumulator)
    pub fn ROR(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        
        let tmp: u16 = (self.get_flag(Flags::C) as u16) << 7 | (self.fetched as u16) >> 1;
        
        self.set_flag(Flags::C, (self.fetched & 0x01) != 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, tmp & 0x80 != 0);
        
        if (self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::ACC)
        || (self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::IMP) {
            self.a = (tmp & 0x00FF) as u8;
        } else {
            self.write(bus, self.addr_abs, (tmp & 0x00FF) as u8);
        }
    }
    /// Return from Interrupt
    pub fn RTI(&mut self, bus: &mut Bus) {
        self.sp = self.sp.wrapping_add(1);
        self.status = self.read(bus, STACK_ADDRESS + self.sp as u16);
        self.status &= !(Flags::B as u8);
        self.set_flag(Flags::U, true);
        
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(bus, STACK_ADDRESS + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (self.read(bus, STACK_ADDRESS + self.sp as u16) as u16) << 8;
    }
    /// Return from Subroutine
    pub fn RTS(&mut self, bus: &mut Bus) {
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(bus, STACK_ADDRESS + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (self.read(bus, STACK_ADDRESS + self.sp as u16) as u16) << 8;
        
        self.pc = self.pc.wrapping_add(1);
    }
    
    /// Subtract Memory from Accumulator with Borrow
    pub fn SBC(&mut self, bus: &mut Bus) {
        self.fetch(bus);

        let value = (self.fetched as u16) ^ 0x00FF;
        
        let tmp = (self.a as u16).wrapping_add(value).wrapping_add(self.get_flag(Flags::C) as u16);

        self.set_flag(Flags::C, (tmp & 0xFF00) != 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0);
        self.set_flag(Flags::V, ((tmp ^ (self.a as u16)) & (tmp ^ value) & 0x0080) != 0);
        self.set_flag(Flags::N, (tmp & 0x80) != 0);

        self.a = (tmp & 0x00FF) as u8;
    }
	/// Set Carry Flag
    pub fn SEC(&mut self, _bus: &mut Bus) {
        self.set_flag(Flags::C, true);
    }
    /// Set Decimal Mode
    pub fn SED(&mut self, _bus: &mut Bus) {
        self.set_flag(Flags::D, true);
    }
    /// Set Interrupt Disable Status
    pub fn SEI(&mut self, _bus: &mut Bus) {
        self.set_flag(Flags::I, true);
    }
    /// Store Accumulator in Memory
    pub fn STA(&mut self, bus: &mut Bus) {
        self.write(bus, self.addr_abs, self.a);
    }
	/// Store Index X in Memory
    pub fn STX(&mut self, bus: &mut Bus) {
        self.write(bus, self.addr_abs, self.x);
    }
    /// Store Index Y in Memory
    pub fn STY(&mut self, bus: &mut Bus) {
        self.write(bus, self.addr_abs, self.y);
    }
    
    /// Transfer Accumulator to Index X
    pub fn TAX(&mut self, _bus: &mut Bus) {
        self.x = self.a;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Transfer Accumulator to Index Y
    pub fn TAY(&mut self, _bus: &mut Bus) {
        self.y = self.a;
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
    }
	/// Transfer Stack Pointer to Index X
    pub fn TSX(&mut self, _bus: &mut Bus) {
        self.x = self.sp;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Transfer Index X to Accumulator
    pub fn TXA(&mut self, _bus: &mut Bus) {
        self.a = self.x;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    /// Transfer Index X to Stack Pointer
    pub fn TXS(&mut self, _bus: &mut Bus) {
        self.sp = self.x;
    }
    /// Transfer Index Y to Accumulator
    pub fn TYA(&mut self, _bus: &mut Bus) {
        self.a = self.y;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
}
