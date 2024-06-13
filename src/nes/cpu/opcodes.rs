#![allow(clippy::cast_lossless, clippy::verbose_bit_mask)]

use crate::nes::{Bus, Component2C02, Component6502, ComponentCartridge, Controller, Flags, ADDRESSING_MODES, STACK_ADDRESS};

#[allow(non_snake_case)]
impl Component6502 {
    /// Illegal opcode
    #[allow(clippy::unused_self)]
    pub fn xxx(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
    }

    /// Add Memory to Accumulator with Carry
    pub fn ADC(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);

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
    pub fn AND(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        self.a &= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    /// Shift Left One Bit (Memory or Accumulator)
    pub fn ASL(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) { 
        self.fetch(controllers, cartridge, ppu, bus);
            
        if self.lookup[self.opcode as usize].addr_mode != ADDRESSING_MODES::ACC {
            self.write(self.addr_abs, self.fetched, controllers, cartridge, ppu, bus);
        }

        let result = self.fetched.wrapping_shl(1);
        
        self.set_flag(Flags::C, (self.fetched & (1 << 7)) > 0);
        self.set_flag(Flags::Z, result == 0x00);
        self.set_flag(Flags::N, result & 0x80 != 0);

        if self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::ACC {
            self.a = result;
        } else {
            self.write(self.addr_abs, result, controllers, cartridge, ppu, bus);
        }
    }

    /// Test Bits in Memory with Accumulator
    pub fn BIT(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        
        let tmp: u16 = (self.a & self.fetched) as u16;
        
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x00);
        self.set_flag(Flags::N, self.fetched & (1 << 7) != 0);
        self.set_flag(Flags::V, self.fetched & (1 << 6) != 0);
    }

    /// Force Break
    pub fn BRK(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = self.pc.wrapping_add(1);
        
        self.write(STACK_ADDRESS + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_sub(1);
        self.write(STACK_ADDRESS + self.sp as u16, (self.pc & 0x00FF) as u8, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_sub(1);
        
        self.set_flag(Flags::B, true);
        self.write(STACK_ADDRESS + self.sp as u16, self.status, controllers, cartridge, ppu, bus);
        self.set_flag(Flags::I, true);
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(Flags::B, false);
        
        let low = self.read(0xFFFE, controllers, cartridge, ppu, bus) as u16;
        let high = self.read(0xFFFF, controllers, cartridge, ppu, bus) as u16;
        self.pc = (high << 8) | low;
    }

    // Generic branch instruction
    pub fn branch(&mut self, controllers: &mut [Controller; 2], cartridge: &ComponentCartridge, ppu: &mut Component2C02, bus: &Bus) {
        self.cycles += 1;
        
        let data = self.pc.wrapping_add(self.addr_rel);
        self.read(self.pc, controllers, cartridge, ppu, bus);

        if data & 0xFF00 != self.pc & 0xFF00 {
            self.cycles += 1;
            self.read(self.pc & 0xFF00 | data & 0x00FF, controllers, cartridge, ppu, bus);
        }

        self.pc = data;
    }
    
    /// Branch on Carry Clear
    pub fn BCC(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        if !self.get_flag(Flags::C) {
            self.branch(controllers, cartridge, ppu, bus);
        }
    }
	/// Branch on Carry Set
    pub fn BCS(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        if self.get_flag(Flags::C) {
            self.branch(controllers, cartridge, ppu, bus);
        }
    }
    /// Branch on Result Zero
    pub fn BEQ(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        if self.get_flag(Flags::Z) {
            self.branch(controllers, cartridge, ppu, bus);
        }
    }
    /// Branch on Result Minus
    pub fn BMI(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        if self.get_flag(Flags::N) {
            self.branch(controllers, cartridge, ppu, bus);
        }
    }
	/// Branch on Result not Zero
    pub fn BNE(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        if !self.get_flag(Flags::Z) {
            self.branch(controllers, cartridge, ppu, bus);
        }
    }
    /// Branch on Result Plus
    pub fn BPL(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        if !self.get_flag(Flags::N) {
            self.branch(controllers, cartridge, ppu, bus);
        }
    }
    /// Branch on Overflow Clear
    pub fn BVC(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        if !self.get_flag(Flags::V) {
            self.branch(controllers, cartridge, ppu, bus);
        }
    }
	/// Branch on Overflow Set
    pub fn BVS(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        if self.get_flag(Flags::V) {
            self.branch(controllers, cartridge, ppu, bus);
        }
    }
    
    /// Clear Carry Flag
    pub fn CLC(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.set_flag(Flags::C, false);
    }
    /// Clear Decimal Mode Flag
    pub fn CLD(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.set_flag(Flags::D, false);
    }
    /// Clear Interrupt Disable Bit Flag
    pub fn CLI(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.set_flag(Flags::I, false);
    }
	/// Clear Overflow Flag
    pub fn CLV(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.set_flag(Flags::V, false);
    }

    /// Compare Memory and Accumulator
    pub fn CMP(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        
        let tmp: u16 = (self.a as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.a >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, (tmp & 0x0080) != 0);
    }
    /// Compare Memory and Index X
    pub fn CPX(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        
        let tmp: u16 = (self.x as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.x >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
    }
    /// Compare Memory and Index Y
    pub fn CPY(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        
        let tmp: u16 = (self.y as u16).wrapping_sub(self.fetched as u16);
        
        self.set_flag(Flags::C, self.y >= self.fetched);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
    }
    
	/// Decrement Memory by One
    pub fn DEC(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);

        self.write(self.addr_abs, self.fetched, controllers, cartridge, ppu, bus);
        let tmp = self.fetched.wrapping_sub(1);

        self.set_flag(Flags::Z, tmp == 0x0000);
        self.set_flag(Flags::N, tmp & 0x80 != 0);

        self.write(self.addr_abs, tmp, controllers, cartridge, ppu, bus);
    }
    /// Decrement Index X by One
    pub fn DEX(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.x = self.x.wrapping_sub(1);
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Decrement Index Y by One
    pub fn DEY(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.y = self.y.wrapping_sub(1);
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
    }
    
    /// "Exclusive-OR" Memory with Accumulator
    pub fn EOR(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        self.a ^= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    
	/// Increment Memory by One
    pub fn INC(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);

        self.write(self.addr_abs, self.fetched, controllers, cartridge, ppu, bus);
        let tmp = self.fetched.wrapping_add(1);
        
        self.set_flag(Flags::Z, tmp == 0x0000);
        self.set_flag(Flags::N, tmp & 0x0080 != 0);
        
        self.write(self.addr_abs, tmp, controllers, cartridge, ppu, bus);
    }
    /// Increment Index X by One
    pub fn INX(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.x = self.x.wrapping_add(1);
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Increment Index Y by One
    pub fn INY(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.y = self.y.wrapping_add(1);
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
    }
    
    /// Jump to New Location
    pub fn JMP(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.pc = self.addr_abs;
    }
	/// Jump to New Location Saving Return Address
    pub fn JSR(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus);
        
        self.write(STACK_ADDRESS + self.sp as u16, (self.pc >> 8) as u8, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_sub(1);
        self.write(STACK_ADDRESS + self.sp as u16, (self.pc & 0x00FF) as u8, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_sub(1);
        
        let high = self.read(self.pc, controllers, cartridge, ppu, bus);
        self.pc = (high as u16) << 8 | self.addr_abs;
    }
    
    /// Load Accumulator with Memory
    pub fn LDA(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        self.a = self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    /// Load Index X with Memory
    pub fn LDX(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        self.x = self.fetched;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Load Index Y with Memory
    pub fn LDY(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        self.y = self.fetched;
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
    }
	/// Shift Right One Bit (Memory or Accumulator)
    pub fn LSR(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);

        if self.lookup[self.opcode as usize].addr_mode != ADDRESSING_MODES::ACC {
            self.read(self.addr_abs, controllers, cartridge, ppu, bus);
        }

        let tmp = self.fetched.wrapping_shr(1);

        self.set_flag(Flags::C, (self.fetched & 0x01) != 0);
        self.set_flag(Flags::Z, tmp == 0x00);
        self.set_flag(Flags::N, tmp & 0x80 != 0);

        if self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::ACC {
            self.a = tmp;
        } else {
            self.write(self.addr_abs, tmp, controllers, cartridge, ppu, bus);
        }
    }
    
    /// No Operation
    #[allow(clippy::unused_self)]
    pub fn NOP(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
    }
    
    /// "OR" Memory with Accumulator
    pub fn ORA(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);
        self.a |= self.fetched;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    
    /// Push Accumulator on Stack
    pub fn PHA(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.write(STACK_ADDRESS + self.sp as u16, self.a, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_sub(1);
    }
	/// Push Processor Status on Stack
    pub fn PHP(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.set_flag(Flags::B, true);
        self.set_flag(Flags::U, true);
        self.write(STACK_ADDRESS + self.sp as u16, self.status, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(Flags::B, false);
    }
    /// Pull Accumulator from Stack
    pub fn PLA(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_add(1);
        self.a = self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus);
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0x80) != 0);
    }
    /// Pull Processor Status from Stack
    pub fn PLP(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_add(1);
        self.status = self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus);
        self.status &= !(Flags::B as u8);
        self.set_flag(Flags::U, true);
    }
    
    /// Rotate One Bit Left (Memory or Accumulator)
    pub fn ROL(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);

        if self.lookup[self.opcode as usize].addr_mode != ADDRESSING_MODES::ACC {
            self.write(self.addr_abs, self.fetched, controllers, cartridge, ppu, bus);
        }

        let tmp = self.fetched.wrapping_shl(1) | self.get_flag(Flags::C) as u8;

        self.set_flag(Flags::C, (self.fetched & 0x80) != 0);
        self.set_flag(Flags::Z, tmp == 0x00);
        self.set_flag(Flags::N, tmp & 0x80 != 0);

        if self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::ACC {
            self.a = tmp;
        } else {
            self.write(self.addr_abs, tmp, controllers, cartridge, ppu, bus);
        }
    }
	/// Rotate One Bit Right (Memory or Accumulator)
    pub fn ROR(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);

        if self.lookup[self.opcode as usize].addr_mode != ADDRESSING_MODES::ACC {
            self.write(self.addr_abs, self.fetched, controllers, cartridge, ppu, bus);
        }

        let tmp = self.fetched.wrapping_shr(1) | (self.get_flag(Flags::C) as u8) << 7;

        self.set_flag(Flags::C, (self.fetched & 0x01) != 0);
        self.set_flag(Flags::Z, tmp == 0x00);
        self.set_flag(Flags::N, tmp & 0x80 != 0);

        if self.lookup[self.opcode as usize].addr_mode == ADDRESSING_MODES::ACC {
            self.a = tmp;
        } else {
            self.write(self.addr_abs, tmp, controllers, cartridge, ppu, bus);
        }
    }
    /// Return from Interrupt
    pub fn RTI(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_add(1);
        self.status = self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus);
        self.status &= !(Flags::B as u8);
        self.set_flag(Flags::U, true);
        
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus) as u16) << 8;
    }
    /// Return from Subroutine
    pub fn RTS(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus);
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (self.read(STACK_ADDRESS + self.sp as u16, controllers, cartridge, ppu, bus) as u16) << 8;
        
        self.pc = self.pc.wrapping_add(1);
        self.read(self.pc, controllers, cartridge, ppu, bus);
    }
    
    /// Subtract Memory from Accumulator with Borrow
    pub fn SBC(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.fetch(controllers, cartridge, ppu, bus);

        let value = (self.fetched as u16) ^ 0x00FF;
        
        let tmp = (self.a as u16).wrapping_add(value).wrapping_add(self.get_flag(Flags::C) as u16);

        self.set_flag(Flags::C, (tmp & 0xFF00) != 0);
        self.set_flag(Flags::Z, (tmp & 0x00FF) == 0);
        self.set_flag(Flags::V, ((tmp ^ (self.a as u16)) & (tmp ^ value) & 0x0080) != 0);
        self.set_flag(Flags::N, (tmp & 0x80) != 0);

        self.a = (tmp & 0x00FF) as u8;
    }
	/// Set Carry Flag
    pub fn SEC(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.set_flag(Flags::C, true);
    }
    /// Set Decimal Mode
    pub fn SED(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.set_flag(Flags::D, true);
    }
    /// Set Interrupt Disable Status
    pub fn SEI(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.set_flag(Flags::I, true);
    }
    /// Store Accumulator in Memory
    pub fn STA(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.write(self.addr_abs, self.a, controllers, cartridge, ppu, bus);
    }
	/// Store Index X in Memory
    pub fn STX(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.write(self.addr_abs, self.x, controllers, cartridge, ppu, bus);
    }
    /// Store Index Y in Memory
    pub fn STY(&mut self, controllers: &mut [Controller; 2], cartridge: &mut ComponentCartridge, ppu: &mut Component2C02, bus: &mut Bus) {
        self.write(self.addr_abs, self.y, controllers, cartridge, ppu, bus);
    }
    
    /// Transfer Accumulator to Index X
    pub fn TAX(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.x = self.a;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Transfer Accumulator to Index Y
    pub fn TAY(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.y = self.a;
        
        self.set_flag(Flags::Z, self.y == 0x00);
        self.set_flag(Flags::N, self.y & 0x80 != 0);
    }
	/// Transfer Stack Pointer to Index X
    pub fn TSX(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.x = self.sp;
        
        self.set_flag(Flags::Z, self.x == 0x00);
        self.set_flag(Flags::N, self.x & 0x80 != 0);
    }
    /// Transfer Index X to Accumulator
    pub fn TXA(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.a = self.x;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
    /// Transfer Index X to Stack Pointer
    pub fn TXS(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.sp = self.x;
    }
    /// Transfer Index Y to Accumulator
    pub fn TYA(&mut self, _controllers: &mut [Controller; 2], _cartridge: &mut ComponentCartridge, _ppu: &mut Component2C02, _bus: &mut Bus) {
        self.a = self.y;
        
        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, self.a & 0x80 != 0);
    }
}
