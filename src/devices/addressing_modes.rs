use super::Cpu6502;

#[allow(non_snake_case)]
impl Cpu6502<'_> {
    /// Immediate addressing mode
    fn addr_IMM(&self) -> u8 { todo!("addr IMM") }
    
    /// Absolute addressing mode
    fn addr_ABS(&self) -> u8 { todo!("addr ABS") }
    
    /// Absolute addressing mode with X offset
    fn addr_ABSx(&self) -> u8 { todo!("addr ABSx") }
    
    /// Absolute addressing mode with Y offset
    fn addr_ABSy(&self) -> u8 { todo!("addr ABSy") }
    
    /// Zero Page addressing mode
    fn addr_ZP(&self) -> u8 { todo!("addr ZP") }
    
    /// Zero Page addressing mode with X offset
    fn addr_ZPx(&self) -> u8 { todo!("addr ZPx") }
    
    /// Zero Page addressing mode with Y offset
    fn addr_ZPy(&self) -> u8 { todo!("addr ZPy") }
    
    /// Implied addressing mode
    pub fn addr_IMP(&self) -> u8 { todo!("addr IMP") }
    
    /// Relative addressing mode
    fn addr_REL(&self) -> u8 { todo!("addr REL") }
    
    /// Indirect addressing mode
    fn addr_IND(&self) -> u8 { todo!("addr IND") }
    
    /// Indirect addressing mode with X offset
    fn addr_INDx(&self) -> u8 { todo!("addr INDx") }
    
    /// Indirect addressing mode with Y offset
    fn addr_INDy(&self) -> u8 { todo!("addr INDy") }
}
