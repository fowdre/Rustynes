use super::super::{Flags, Cpu6502, Bus};

#[allow(non_snake_case)]
impl Cpu6502 {
    /// Unofficial opcode
    pub fn xxx(&mut self, _bus: &Bus) -> u8 { todo!("Unofficial opcode") }

    /// Add Memory to Accumulator with Carry
    pub fn ADC(&mut self, _bus: &Bus) -> u8 { todo!("ADC") }
    /// "AND" Memory with Accumulator
    pub fn AND(&mut self, bus: &Bus) -> u8 { todo!("AND") }
    /// Shift Left One Bit (Memory or Accumulator)
    pub fn ASL(&mut self, _bus: &Bus) -> u8 { todo!("ASL") }
    
    /// Branch on Carry Clear
    pub fn BCC(&mut self, _bus: &Bus) -> u8 { todo!("BCC") }
	/// Branch on Carry Set
    pub fn BCS(&mut self, _bus: &Bus) -> u8 { todo!("BCS") }
    /// Branch on Result Zero
    pub fn BEQ(&mut self, _bus: &Bus) -> u8 { todo!("BEQ") }
    /// Test Bits in Memory with Accumulator
    pub fn BIT(&mut self, _bus: &Bus) -> u8 { todo!("BIT") }
    /// Branch on Result Minus
    pub fn BMI(&mut self, _bus: &Bus) -> u8 { todo!("BMI") }
	/// Branch on Result not Zero
    pub fn BNE(&mut self, _bus: &Bus) -> u8 { todo!("BNE") }
    /// Branch on Result Plus
    pub fn BPL(&mut self, _bus: &Bus) -> u8 { todo!("BPL") }
    /// Force Break
    pub fn BRK(&mut self, _bus: &Bus) -> u8 { todo!("BRK") }
    /// Branch on Overflow Clear
    pub fn BVC(&mut self, _bus: &Bus) -> u8 { todo!("BVC") }
	/// Branch on Overflow Set
    pub fn BVS(&mut self, _bus: &Bus) -> u8 { todo!("BVS") }
    
    /// Clear Carry Flag
    pub fn CLC(&mut self, _bus: &Bus) -> u8 { todo!("CLC") }
    /// Clear Decimal Mode
    pub fn CLD(&mut self, _bus: &Bus) -> u8 { todo!("CLD") }
    /// Clear Interrupt Disable Bit
    pub fn CLI(&mut self, _bus: &Bus) -> u8 { todo!("CLI") }
	/// Clear Overflow Flag
    pub fn CLV(&mut self, _bus: &Bus) -> u8 { todo!("CLV") }
    /// Compare Memory and Accumulator
    pub fn CMP(&mut self, _bus: &Bus) -> u8 { todo!("CMP") }
    /// Compare Memory and Index X
    pub fn CPX(&mut self, _bus: &Bus) -> u8 { todo!("CPX") }
    /// Compare Memory and Index Y
    pub fn CPY(&mut self, _bus: &Bus) -> u8 { todo!("CPY") }
    
	/// Decrement Memory by One
    pub fn DEC(&mut self, _bus: &Bus) -> u8 { todo!("DEC") }
    /// Decrement Index X by One
    pub fn DEX(&mut self, _bus: &Bus) -> u8 { todo!("DEX") }
    /// Decrement Index Y by One
    pub fn DEY(&mut self, _bus: &Bus) -> u8 { todo!("DEY") }
    
    /// "Exclusive-OR" Memory with Accumulator
    pub fn EOR(&mut self, _bus: &Bus) -> u8 { todo!("EOR") }
    
	/// Increment Memory by One
    pub fn INC(&mut self, _bus: &Bus) -> u8 { todo!("INC") }
    /// Increment Index X by One
    pub fn INX(&mut self, _bus: &Bus) -> u8 { todo!("INX") }
    /// Increment Index Y by One
    pub fn INY(&mut self, _bus: &Bus) -> u8 { todo!("INY") }
    
    /// Jump to New Location
    pub fn JMP(&mut self, _bus: &Bus) -> u8 { todo!("JMP") }
	/// Jump to New Location Saving Return Address
    pub fn JSR(&mut self, _bus: &Bus) -> u8 { todo!("JSR") }
    
    /// Load Accumulator with Memory
    pub fn LDA(&mut self, _bus: &Bus) -> u8 { todo!("LDA") }
    /// Load Index X with Memory
    pub fn LDX(&mut self, _bus: &Bus) -> u8 { todo!("LDX") }
    /// Load Index Y with Memory
    pub fn LDY(&mut self, _bus: &Bus) -> u8 { todo!("LDY") }
	/// Shift Right One Bit (Memory or Accumulator)
    pub fn LSR(&mut self, _bus: &Bus) -> u8 { todo!("LSR") }
    
    /// No Operation
    pub fn NOP(&mut self, _bus: &Bus) -> u8 { todo!("NOP") }
    
    /// "OR" Memory with Accumulator
    pub fn ORA(&mut self, _bus: &Bus) -> u8 { todo!("ORA") }
    
    /// Push Accumulator on Stack
    pub fn PHA(&mut self, _bus: &Bus) -> u8 { todo!("PHA") }
	/// Push Processor Status on Stack
    pub fn PHP(&mut self, _bus: &Bus) -> u8 { todo!("PHP") }
    /// Pull Accumulator from Stack
    pub fn PLA(&mut self, _bus: &Bus) -> u8 { todo!("PLA") }
    /// Pull Processor Status from Stack
    pub fn PLP(&mut self, _bus: &Bus) -> u8 { todo!("PLP") }
    
    /// Rotate One Bit Left (Memory or Accumulator)
    pub fn ROL(&mut self, _bus: &Bus) -> u8 { todo!("ROL") }
	/// Rotate One Bit Right (Memory or Accumulator)
    pub fn ROR(&mut self, _bus: &Bus) -> u8 { todo!("ROR") }
    /// Return from Interrupt
    pub fn RTI(&mut self, _bus: &Bus) -> u8 { todo!("RTI") }
    /// Return from Subroutine
    pub fn RTS(&mut self, _bus: &Bus) -> u8 { todo!("RTS") }
    
    /// Subtract Memory from Accumulator with Borrow
    pub fn SBC(&mut self, _bus: &Bus) -> u8 { todo!("SBC") }
	/// Set Carry Flag
    pub fn SEC(&mut self, _bus: &Bus) -> u8 { todo!("SEC") }
    /// Set Decimal Mode
    pub fn SED(&mut self, _bus: &Bus) -> u8 { todo!("SED") }
    /// Set Interrupt Disable Status
    pub fn SEI(&mut self, _bus: &Bus) -> u8 { todo!("SEI") }
    /// Store Accumulator in Memory
    pub fn STA(&mut self, _bus: &Bus) -> u8 { todo!("STA") }
	/// Store Index X in Memory
    pub fn STX(&mut self, _bus: &Bus) -> u8 { todo!("STX") }
    /// Store Index Y in Memory
    pub fn STY(&mut self, _bus: &Bus) -> u8 { todo!("STY") }
    
    /// Transfer Accumulator to Index X
    pub fn TAX(&mut self, _bus: &Bus) -> u8 { todo!("TAX") }
    /// Transfer Accumulator to Index Y
    pub fn TAY(&mut self, _bus: &Bus) -> u8 { todo!("TAY") }
	/// Transfer Stack Pointer to Index X
    pub fn TSX(&mut self, _bus: &Bus) -> u8 { todo!("TSX") }
    /// Transfer Index X to Accumulator
    pub fn TXA(&mut self, _bus: &Bus) -> u8 { todo!("TXA") }
    /// Transfer Index X to Stack Pointer
    pub fn TXS(&mut self, _bus: &Bus) -> u8 { todo!("TXS") }
    /// Transfer Index Y to Accumulator
    pub fn TYA(&mut self, _bus: &Bus) -> u8 { todo!("TYA") }
}
