use super::super::{Flags, Cpu6502, Bus};

#[allow(non_snake_case)]
impl Cpu6502 {
    /// Unofficial opcode
    pub fn xxx(&mut self, _bus: &mut Bus) -> u8 { todo!("Unofficial opcode") }

    /// Add Memory to Accumulator with Carry
    pub fn ADC(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);

        let (tmp, has_overflowed1): (u16, bool) = (self.a as u16).overflowing_add(self.fetched as u16);
        let (tmp, has_overflowed2): (u16, bool) = tmp.overflowing_add(self.get_flag(Flags::C) as u16);

        // Set Carry Flag if overflowed
        self.set_flag(Flags::C, tmp > 255);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0);
        self.set_flag(Flags::N, tmp & 0x80 == 1);
        // Set Overflow Flag if any of the additions overflowed
        self.set_flag(Flags::V, has_overflowed1 || has_overflowed2);
        
        self.a = (tmp & 0x00FF) as u8;
        
        1
    }
    /// "AND" Memory with Accumulator
    pub fn AND(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus);
        self.a &= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 == 1);
        
        1
    }
    /// Shift Left One Bit (Memory or Accumulator)
    pub fn ASL(&mut self, _bus: &mut Bus) -> u8 { todo!("ASL") }
    
    /// Branch on Carry Clear
    pub fn BCC(&mut self, _bus: &mut Bus) -> u8 { todo!("BCC") }
	/// Branch on Carry Set
    pub fn BCS(&mut self, _bus: &mut Bus) -> u8 { todo!("BCS") }
    /// Branch on Result Zero
    pub fn BEQ(&mut self, _bus: &mut Bus) -> u8 { todo!("BEQ") }
    /// Test Bits in Memory with Accumulator
    pub fn BIT(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        
        let tmp: u16 = (self.a & self.fetched) as u16;
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, self.fetched & (1 << 7) == 1);
        self.set_flag(Flags::V, self.fetched & (1 << 6) == 1);
        
        0
    }
    /// Branch on Result Minus
    pub fn BMI(&mut self, _bus: &mut Bus) -> u8 { todo!("BMI") }
	/// Branch on Result not Zero
    pub fn BNE(&mut self, _bus: &mut Bus) -> u8 { todo!("BNE") }
    /// Branch on Result Plus
    pub fn BPL(&mut self, _bus: &mut Bus) -> u8 { todo!("BPL") }
    /// Force Break
    pub fn BRK(&mut self, _bus: &mut Bus) -> u8 { todo!("BRK") }
    /// Branch on Overflow Clear
    pub fn BVC(&mut self, _bus: &mut Bus) -> u8 { todo!("BVC") }
	/// Branch on Overflow Set
    pub fn BVS(&mut self, _bus: &mut Bus) -> u8 { todo!("BVS") }
    
    /// Clear Carry Flag
    pub fn CLC(&mut self, _bus: &mut Bus) -> u8 { todo!("CLC") }
    /// Clear Decimal Mode
    pub fn CLD(&mut self, _bus: &mut Bus) -> u8 { todo!("CLD") }
    /// Clear Interrupt Disable Bit
    pub fn CLI(&mut self, _bus: &mut Bus) -> u8 { todo!("CLI") }
	/// Clear Overflow Flag
    pub fn CLV(&mut self, _bus: &mut Bus) -> u8 { todo!("CLV") }
    /// Compare Memory and Accumulator
    pub fn CMP(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        
        let tmp: u16 = (self.a as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.a >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 == 1);
        
        1
    }
    /// Compare Memory and Index X
    pub fn CPX(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        
        let tmp: u16 = (self.x as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.x >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 == 1);
        
        1
    }
    /// Compare Memory and Index Y
    pub fn CPY(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        
        let tmp: u16 = (self.y as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.y >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 == 1);
        
        1
    }
    
	/// Decrement Memory by One
    pub fn DEC(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        let tmp: u16 = self.fetched as u16 - 1;
        
        self.write(_bus, self.addr_abs, (tmp & 0x00FF) as u8);
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 == 1);
        
        0
    }
    /// Decrement Index X by One
    pub fn DEX(&mut self, _bus: &mut Bus) -> u8 {
        self.x = self.x.wrapping_sub(1);
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 == 1);
        
        0
    }
    /// Decrement Index Y by One
    pub fn DEY(&mut self, _bus: &mut Bus) -> u8 {
        self.y = self.y.wrapping_sub(1);
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 == 1);
        
        0
    }
    
    /// "Exclusive-OR" Memory with Accumulator
    pub fn EOR(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        self.a ^= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 == 1);
        
        1
    }
    
	/// Increment Memory by One
    pub fn INC(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        let tmp: u16 = self.fetched as u16 + 1;
        
        self.write(_bus, self.addr_abs, (tmp & 0x00FF) as u8);
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 == 1);
        
        0
    }
    /// Increment Index X by One
    pub fn INX(&mut self, _bus: &mut Bus) -> u8 {
        self.x = self.x.wrapping_add(1);
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 == 1);
        
        0
    }
    /// Increment Index Y by One
    pub fn INY(&mut self, _bus: &mut Bus) -> u8 {
        self.y = self.y.wrapping_add(1);
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 == 1);
        
        0
    }
    
    /// Jump to New Location
    pub fn JMP(&mut self, _bus: &mut Bus) -> u8 { todo!("JMP") }
	/// Jump to New Location Saving Return Address
    pub fn JSR(&mut self, _bus: &mut Bus) -> u8 { todo!("JSR") }
    
    /// Load Accumulator with Memory
    pub fn LDA(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        self.a = self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 == 1);
        
        1
    }
    /// Load Index X with Memory
    pub fn LDX(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        self.x = self.fetched;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 == 1);
        
        1
    }
    /// Load Index Y with Memory
    pub fn LDY(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        self.y = self.fetched;
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 == 1);
        
        1
    }
	/// Shift Right One Bit (Memory or Accumulator)
    pub fn LSR(&mut self, _bus: &mut Bus) -> u8 { todo!("LSR") }
    
    /// No Operation
    pub fn NOP(&mut self, _bus: &mut Bus) -> u8 { todo!("NOP") }
    
    /// "OR" Memory with Accumulator
    pub fn ORA(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        self.a |= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 == 1);
        
        1
    }
    
    /// Push Accumulator on Stack
    pub fn PHA(&mut self, _bus: &mut Bus) -> u8 {
        self.write(_bus, 0x0100 + self.sp as u16, self.a);
        self.sp = self.sp.wrapping_sub(1);
        
        0
    }
	/// Push Processor Status on Stack
    pub fn PHP(&mut self, _bus: &mut Bus) -> u8 {
        self.write(_bus, 0x0100 + self.sp as u16, self.status | Flags::B as u8 | Flags::U as u8);
        self.set_flag(Flags::B, false);
        self.set_flag(Flags::U, false);
        
        0
    }
    /// Pull Accumulator from Stack
    pub fn PLA(&mut self, _bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.a = self.read(_bus, 0x0100 + self.sp as u16);
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0x80) == 1);
        
        0
    }
    /// Pull Processor Status from Stack
    pub fn PLP(&mut self, _bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.status = self.read(_bus, 0x0100 + self.sp as u16);
        self.set_flag(Flags::U, true);
        
        0
    }
    
    /// Rotate One Bit Left (Memory or Accumulator)
    pub fn ROL(&mut self, _bus: &mut Bus) -> u8 { todo!("ROL") }
	/// Rotate One Bit Right (Memory or Accumulator)
    pub fn ROR(&mut self, _bus: &mut Bus) -> u8 { todo!("ROR") }
    /// Return from Interrupt
    pub fn RTI(&mut self, _bus: &mut Bus) -> u8 { todo!("RTI") }
    /// Return from Subroutine
    pub fn RTS(&mut self, _bus: &mut Bus) -> u8 { todo!("RTS") }
    
    /// Subtract Memory from Accumulator with Borrow
    pub fn SBC(&mut self, _bus: &mut Bus) -> u8 {
        self.fetch(_bus);
        
        // Use two's complement to treat subtraction as addition
        let value: u16 = (self.fetched ^ 0x00FF) as u16;
        
        let (tmp, has_overflowed1): (u16, bool) = (self.a as u16).overflowing_add(value as u16);
        let (tmp, has_overflowed2): (u16, bool) = tmp.overflowing_add(self.get_flag(Flags::C) as u16);
        
        // Set Carry Flag if overflowed
        self.set_flag(Flags::C, tmp > 255);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0);
        self.set_flag(Flags::N, tmp & 0x80 == 1);
        // Set Overflow Flag if any of the additions overflowed
        self.set_flag(Flags::V, has_overflowed1 || has_overflowed2);
        
        self.a = (tmp & 0x00FF) as u8;
        
        1
    }
	/// Set Carry Flag
    pub fn SEC(&mut self, _bus: &mut Bus) -> u8 { todo!("SEC") }
    /// Set Decimal Mode
    pub fn SED(&mut self, _bus: &mut Bus) -> u8 { todo!("SED") }
    /// Set Interrupt Disable Status
    pub fn SEI(&mut self, _bus: &mut Bus) -> u8 { todo!("SEI") }
    /// Store Accumulator in Memory
    pub fn STA(&mut self, _bus: &mut Bus) -> u8 {
        self.write(_bus, self.addr_abs, self.a);
        
        0
    }
	/// Store Index X in Memory
    pub fn STX(&mut self, _bus: &mut Bus) -> u8 {
        self.write(_bus, self.addr_abs, self.x);
        
        0
    }
    /// Store Index Y in Memory
    pub fn STY(&mut self, _bus: &mut Bus) -> u8 {
        self.write(_bus, self.addr_abs, self.y);
        
        0
    }
    
    /// Transfer Accumulator to Index X
    pub fn TAX(&mut self, _bus: &mut Bus) -> u8 {
        self.x = self.a;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 == 1);
        
        0
    }
    /// Transfer Accumulator to Index Y
    pub fn TAY(&mut self, _bus: &mut Bus) -> u8 {
        self.y = self.a;
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 == 1);
        
        0
    }
	/// Transfer Stack Pointer to Index X
    pub fn TSX(&mut self, _bus: &mut Bus) -> u8 {
        self.x = self.sp;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 == 1);
        
        0
    }
    /// Transfer Index X to Accumulator
    pub fn TXA(&mut self, _bus: &mut Bus) -> u8 {
        self.a = self.x;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 == 1);
        
        0
    }
    /// Transfer Index X to Stack Pointer
    pub fn TXS(&mut self, _bus: &mut Bus) -> u8 {
        self.sp = self.x;
        
        0
    }
    /// Transfer Index Y to Accumulator
    pub fn TYA(&mut self, _bus: &mut Bus) -> u8 {
        self.a = self.y;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 == 1);
        
        0
    }
}
