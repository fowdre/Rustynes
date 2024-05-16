use super::Cpu6502;

#[allow(non_snake_case)]
impl Cpu6502<'_> {
    /// Unofficial opcode
    pub fn XXX(&mut self) -> u8 { todo!("XXX") }

    /// Add Memory to Accumulator with Carry
    fn ADC(&mut self) -> u8 { todo!("ADC") }
    /// "AND" Memory with Accumulator
    pub fn AND(&mut self) -> u8 { todo!("AND") }
    /// Shift Left One Bit (Memory or Accumulator)
    pub fn ASL(&mut self) -> u8 { todo!("ASL") }
    
    /// Branch on Carry Clear
    fn BCC(&mut self) -> u8 { todo!("BCC") }
	/// Branch on Carry Set
    fn BCS(&mut self) -> u8 { todo!("BCS") }
    /// Branch on Result Zero
    fn BEQ(&mut self) -> u8 { todo!("BEQ") }
    /// Test Bits in Memory with Accumulator
    pub fn BIT(&mut self) -> u8 { todo!("BIT") }
    /// Branch on Result Minus
    pub fn BMI(&mut self) -> u8 { todo!("BMI") }
	/// Branch on Result not Zero
    fn BNE(&mut self) -> u8 { todo!("BNE") }
    /// Branch on Result Plus
    pub fn BPL(&mut self) -> u8 { todo!("BPL") }
    /// Force Break
    pub fn BRK(&mut self) -> u8 { todo!("BRK") }
    /// Branch on Overflow Clear
    fn BVC(&mut self) -> u8 { todo!("BVC") }
	/// Branch on Overflow Set
    fn BVS(&mut self) -> u8 { todo!("BVS") }
    
    /// Clear Carry Flag
    pub fn CLC(&mut self) -> u8 { todo!("CLC") }
    /// Clear Decimal Mode
    fn CLD(&mut self) -> u8 { todo!("CLD") }
    /// Clear Interrupt Disable Bit
    fn CLI(&mut self) -> u8 { todo!("CLI") }
	/// Clear Overflow Flag
    fn CLV(&mut self) -> u8 { todo!("CLV") }
    /// Compare Memory and Accumulator
    fn CMP(&mut self) -> u8 { todo!("CMP") }
    /// Compare Memory and Index X
    fn CPX(&mut self) -> u8 { todo!("CPX") }
    /// Compare Memory and Index Y
    fn CPY(&mut self) -> u8 { todo!("CPY") }
    
	/// Decrement Memory by One
    fn DEC(&mut self) -> u8 { todo!("DEC") }
    /// Decrement Index X by One
    fn DEX(&mut self) -> u8 { todo!("DEX") }
    /// Decrement Index Y by One
    fn DEY(&mut self) -> u8 { todo!("DEY") }
    
    /// "Exclusive-OR" Memory with Accumulator
    fn EOR(&mut self) -> u8 { todo!("EOR") }
    
	/// Increment Memory by One
    fn INC(&mut self) -> u8 { todo!("INC") }
    /// Increment Index X by One
    fn INX(&mut self) -> u8 { todo!("INX") }
    /// Increment Index Y by One
    fn INY(&mut self) -> u8 { todo!("INY") }
    
    /// Jump to New Location
    fn JMP(&mut self) -> u8 { todo!("JMP") }
	/// Jump to New Location Saving Return Address
    pub fn JSR(&mut self) -> u8 { todo!("JSR") }
    
    /// Load Accumulator with Memory
    fn LDA(&mut self) -> u8 { todo!("LDA") }
    /// Load Index X with Memory
    fn LDX(&mut self) -> u8 { todo!("LDX") }
    /// Load Index Y with Memory
    fn LDY(&mut self) -> u8 { todo!("LDY") }
	/// Shift Right One Bit (Memory or Accumulator)
    fn LSR(&mut self) -> u8 { todo!("LSR") }
    
    /// No Operation
    pub fn NOP(&mut self) -> u8 { todo!("NOP") }
    
    /// "OR" Memory with Accumulator
    pub fn ORA(&mut self) -> u8 { todo!("ORA") }
    
    /// Push Accumulator on Stack
    fn PHA(&mut self) -> u8 { todo!("PHA") }
	/// Push Processor Status on Stack
    pub fn PHP(&mut self) -> u8 { todo!("PHP") }
    /// Pull Accumulator from Stack
    fn PLA(&mut self) -> u8 { todo!("PLA") }
    /// Pull Processor Status from Stack
    pub fn PLP(&mut self) -> u8 { todo!("PLP") }
    
    /// Rotate One Bit Left (Memory or Accumulator)
    pub fn ROL(&mut self) -> u8 { todo!("ROL") }
	/// Rotate One Bit Right (Memory or Accumulator)
    fn ROR(&mut self) -> u8 { todo!("ROR") }
    /// Return from Interrupt
    fn RTI(&mut self) -> u8 { todo!("RTI") }
    /// Return from Subroutine
    fn RTS(&mut self) -> u8 { todo!("RTS") }
    
    /// Subtract Memory from Accumulator with Borrow
    fn SBC(&mut self) -> u8 { todo!("SBC") }
	/// Set Carry Flag
    pub fn SEC(&mut self) -> u8 { todo!("SEC") }
    /// Set Decimal Mode
    fn SED(&mut self) -> u8 { todo!("SED") }
    /// Set Interrupt Disable Status
    fn SEI(&mut self) -> u8 { todo!("SEI") }
    /// Store Accumulator in Memory
    fn STA(&mut self) -> u8 { todo!("STA") }
	/// Store Index X in Memory
    fn STX(&mut self) -> u8 { todo!("STX") }
    /// Store Index Y in Memory
    fn STY(&mut self) -> u8 { todo!("STY") }
    
    /// Transfer Accumulator to Index X
    fn TAX(&mut self) -> u8 { todo!("TAX") }
    /// Transfer Accumulator to Index Y
    fn TAY(&mut self) -> u8 { todo!("TAY") }
	/// Transfer Stack Pointer to Index X
    fn TSX(&mut self) -> u8 { todo!("TSX") }
    /// Transfer Index X to Accumulator
    fn TXA(&mut self) -> u8 { todo!("TXA") }
    /// Transfer Index X to Stack Pointer
    fn TXS(&mut self) -> u8 { todo!("TXS") }
    /// Transfer Index Y to Accumulator
    fn TYA(&mut self) -> u8 { todo!("TYA") }
}
