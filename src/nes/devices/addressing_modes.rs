use super::cpu6502::Cpu6502;

#[allow(non_snake_case)]
impl Cpu6502 {
    /// Accumulator addressing mode
    pub fn addr_ACC(&mut self) -> u8 { todo!("addr ACC") }

    /// Immediate addressing mode
    pub fn addr_IMM(&mut self) -> u8 { todo!("addr IMM") }
    
    /// Absolute addressing mode
    pub fn addr_ABS(&mut self) -> u8 { todo!("addr ABS") }
    
    /// Absolute addressing mode with X offset
    pub fn addr_ABSx(&mut self) -> u8 { todo!("addr ABSx") }
    
    /// Absolute addressing mode with Y offset
    pub fn addr_ABSy(&mut self) -> u8 { todo!("addr ABSy") }
    
    /// Zero Page addressing mode
    pub fn addr_ZPG(&mut self) -> u8 { todo!("addr ZP") }
    
    /// Zero Page addressing mode with X offset
    pub fn addr_ZPGx(&mut self) -> u8 { todo!("addr ZPx") }
    
    /// Zero Page addressing mode with Y offset
    pub fn addr_ZPGy(&mut self) -> u8 { todo!("addr ZPy") }
    
    /// Implied addressing mode
    pub fn addr_IMP(&mut self) -> u8 { todo!("addr IMP") }
    
    /// Relative addressing mode
    pub fn addr_REL(&mut self) -> u8 { todo!("addr REL") }
    
    /// Indirect addressing mode
    pub fn addr_IND(&mut self) -> u8 { todo!("addr IND") }
    
    /// Indirect addressing mode with X offset
    pub fn addr_INDx(&mut self) -> u8 { todo!("addr INDx") }
    
    /// Indirect addressing mode with Y offset
    pub fn addr_INDy(&mut self) -> u8 { todo!("addr INDy") }
}
