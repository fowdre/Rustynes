use super::Cpu6502;

#[allow(non_snake_case)]
impl Cpu6502<'_> {
    /// Accumulator addressing mode
    pub fn addr_ACC(&self) -> u8 { todo!("addr ACC") }

    /// Immediate addressing mode
    pub fn addr_IMM(&self) -> u8 { todo!("addr IMM") }
    
    /// Absolute addressing mode
    pub fn addr_ABS(&self) -> u8 { todo!("addr ABS") }
    
    /// Absolute addressing mode with X offset
    pub fn addr_ABSx(&self) -> u8 { todo!("addr ABSx") }
    
    /// Absolute addressing mode with Y offset
    pub fn addr_ABSy(&self) -> u8 { todo!("addr ABSy") }
    
    /// Zero Page addressing mode
    pub fn addr_ZPG(&self) -> u8 { todo!("addr ZP") }
    
    /// Zero Page addressing mode with X offset
    pub fn addr_ZPGx(&self) -> u8 { todo!("addr ZPx") }
    
    /// Zero Page addressing mode with Y offset
    pub fn addr_ZPGy(&self) -> u8 { todo!("addr ZPy") }
    
    /// Implied addressing mode
    pub fn addr_IMP(&self) -> u8 { todo!("addr IMP") }
    
    /// Relative addressing mode
    pub fn addr_REL(&self) -> u8 { todo!("addr REL") }
    
    /// Indirect addressing mode
    pub fn addr_IND(&self) -> u8 { todo!("addr IND") }
    
    /// Indirect addressing mode with X offset
    pub fn addr_INDx(&self) -> u8 { todo!("addr INDx") }
    
    /// Indirect addressing mode with Y offset
    pub fn addr_INDy(&self) -> u8 { todo!("addr INDy") }
}
