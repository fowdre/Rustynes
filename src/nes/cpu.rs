mod addressing_modes;
mod opcodes;


pub mod cpu6502 {
    use crate::nes::Cartridge;
    use crate::nes::ppu::ppu2c02::Ppu2C02;
    use crate::nes::bus::Bus;
    
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    #[derive(Debug, PartialEq)]
    pub enum ADDRESSING_MODES {
        /// Accumulator
        ACC,
        /// Implied
        IMP,
        /// Immediate
        IMM,
        /// Zero Page
        ZPG,
        /// Zero Page with X Offset
        ZPGX,
        /// Zero Page with Y Offset
        ZPGY,
        /// Relative
        REL,
        /// Absolute
        ABS,
        /// Absolute with X Offset
        ABSX,
        /// Absolute with Y Offset
        ABSY,
        /// Indirect
        IND,
        /// Indirect with X Offset
        INDX,
        /// Indirect with Y Offset
        INDY,
    }

    #[derive(Debug)]
    pub struct Cpu6502 {
        pub a: u8,
        pub x: u8,
        pub y: u8,
        pub sp: u8,
        pub pc: u16,
        pub status: u8,
        
        pub opcode: u8,
        pub fetched: u8,
        /// Absolute address calculated from the addressing mode which will be
        /// used to read/write data during the instruction execution
        pub addr_abs: u16,
        /// Only changed by the relative addressing mode
        pub addr_rel: u16,
        pub cycles: u8,

        pub lookup: [Instruction; 256],
    }

    #[derive(Debug)]
    pub struct Instruction {
        pub name: &'static str,
        pub cycles: u8,
        pub addr_mode: ADDRESSING_MODES,
        pub fn_addr_mode: fn(&mut Cpu6502, &mut Cartridge, &mut Ppu2C02, &Bus) -> u8,
        pub fn_operate: fn(&mut Cpu6502, &mut Cartridge, &mut Ppu2C02, &mut Bus) -> u8,
    }

    pub enum Flags {
        /// Carry
        C = (1 << 0),
        /// Zero
        Z = (1 << 1),
        /// Interrupt Disable
        I = (1 << 2),
        /// Decimal Mode
        D = (1 << 3),
        /// Break Command
        B = (1 << 4),
        /// Unused
        U = (1 << 5),
        /// Overflow
        V = (1 << 6),
        /// Negative
        N = (1 << 7),
    }

    impl Cpu6502 {
        pub fn new() -> Self {
            use ADDRESSING_MODES::*;

            Self {
                a: 0,
                x: 0,
                y: 0,
                sp: 0,
                pc: 0x8000,
                status: 0,
                
                opcode: 0,
                fetched: 0,
                addr_abs: 0,
                addr_rel: 0,
                cycles: 0,

                lookup: [
                    // Row 0
                    Instruction{name: "BRK", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::BRK, cycles: 7},
                    Instruction{name: "ORA", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::ORA, cycles: 6},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::xxx, cycles: 8}, // SLO
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::NOP, cycles: 3},
                    Instruction{name: "ORA", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::ORA, cycles: 3},
                    Instruction{name: "ASL", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::ASL, cycles: 5},
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::xxx, cycles: 5}, // SLO
                    Instruction{name: "PHP", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::PHP, cycles: 3},
                    Instruction{name: "ORA", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::ORA, cycles: 2},
                    Instruction{name: "ASL", addr_mode: ACC,  fn_addr_mode: Self::addr_ACC,  fn_operate: Self::ASL, cycles: 2},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::xxx, cycles: 2}, // ANC
                    Instruction{name: "...", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "ORA", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::ORA, cycles: 4},
                    Instruction{name: "ASL", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::ASL, cycles: 6},
                    Instruction{name: "...", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::xxx, cycles: 6}, // SLO

                    // Row 1
                    Instruction{name: "BPL", addr_mode: REL,  fn_addr_mode: Self::addr_REL,  fn_operate: Self::BPL, cycles: 2},
                    Instruction{name: "ORA", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::ORA, cycles: 5},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::xxx, cycles: 8}, // SLO
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "ORA", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::ORA, cycles: 4},
                    Instruction{name: "ASL", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::ASL, cycles: 6},
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::xxx, cycles: 6}, // SLO
                    Instruction{name: "CLC", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::CLC, cycles: 2},
                    Instruction{name: "ORA", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::ORA, cycles: 4},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 7}, // SLO
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "ORA", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::ORA, cycles: 4},
                    Instruction{name: "ASL", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::ASL, cycles: 7},
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::xxx, cycles: 7}, // SLO*

                    // Row 2
                    Instruction{name: "JSR", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::JSR, cycles: 6},
                    Instruction{name: "AND", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::AND, cycles: 6},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::xxx, cycles: 8}, // RLA
                    Instruction{name: "BIT", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::BIT, cycles: 3},
                    Instruction{name: "AND", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::AND, cycles: 3},
                    Instruction{name: "ROL", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::ROL, cycles: 5},
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::xxx, cycles: 5}, // RLA
                    Instruction{name: "PLP", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::PLP, cycles: 4},
                    Instruction{name: "AND", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::AND, cycles: 2},
                    Instruction{name: "ROL", addr_mode: ACC,  fn_addr_mode: Self::addr_ACC,  fn_operate: Self::ROL, cycles: 2},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::xxx, cycles: 2}, // ANC
                    Instruction{name: "BIT", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::BIT, cycles: 4},
                    Instruction{name: "AND", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::AND, cycles: 4},
                    Instruction{name: "ROL", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::ROL, cycles: 6},
                    Instruction{name: "...", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::xxx, cycles: 6}, // RLA

                    // Row 3
                    Instruction{name: "BMI", addr_mode: REL,  fn_addr_mode: Self::addr_REL,  fn_operate: Self::BMI, cycles: 2},
                    Instruction{name: "AND", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::AND, cycles: 5},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::xxx, cycles: 8}, // RLA
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "AND", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::AND, cycles: 4},
                    Instruction{name: "ROL", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::ROL, cycles: 6},
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::xxx, cycles: 6}, // RLA
                    Instruction{name: "SEC", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::SEC, cycles: 2},
                    Instruction{name: "AND", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::AND, cycles: 4},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 7}, // RLA
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "AND", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::AND, cycles: 4},
                    Instruction{name: "ROL", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::ROL, cycles: 7},
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::xxx, cycles: 7}, // RLA

                    // Row 4
                    Instruction{name: "RTI", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::RTI, cycles: 6},
                    Instruction{name: "EOR", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::EOR, cycles: 6},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::xxx, cycles: 8}, // SRE
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::NOP, cycles: 3},
                    Instruction{name: "EOR", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::EOR, cycles: 3},
                    Instruction{name: "LSR", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::LSR, cycles: 5},
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::xxx, cycles: 5}, // SRE
                    Instruction{name: "PHA", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::PHA, cycles: 3},
                    Instruction{name: "EOR", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::EOR, cycles: 2},
                    Instruction{name: "LSR", addr_mode: ACC,  fn_addr_mode: Self::addr_ACC,  fn_operate: Self::LSR, cycles: 2},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::xxx, cycles: 2}, // ALR
                    Instruction{name: "JMP", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::JMP, cycles: 3},
                    Instruction{name: "EOR", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::EOR, cycles: 4},
                    Instruction{name: "LSR", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::LSR, cycles: 6},
                    Instruction{name: "...", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::xxx, cycles: 6}, // SRE

                    // Row 5
                    Instruction{name: "BVC", addr_mode: REL,  fn_addr_mode: Self::addr_REL,  fn_operate: Self::BVC, cycles: 2},
                    Instruction{name: "EOR", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::EOR, cycles: 5},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::xxx, cycles: 8}, // SRE
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "EOR", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::EOR, cycles: 4},
                    Instruction{name: "LSR", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::LSR, cycles: 6},
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::xxx, cycles: 6}, // SRE
                    Instruction{name: "CLI", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::CLI, cycles: 2},
                    Instruction{name: "EOR", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::EOR, cycles: 4},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 7}, // SRE
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "EOR", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::EOR, cycles: 4},
                    Instruction{name: "LSR", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::LSR, cycles: 7},
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::xxx, cycles: 7}, // SRE

                    // Row 6
                    Instruction{name: "RTS", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::RTS, cycles: 6},
                    Instruction{name: "ADC", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::ADC, cycles: 6},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::xxx, cycles: 8}, // RRA
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::NOP, cycles: 3},
                    Instruction{name: "ADC", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::ADC, cycles: 3},
                    Instruction{name: "ROR", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::ROR, cycles: 5},
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::xxx, cycles: 5}, // RRA
                    Instruction{name: "PLA", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::PLA, cycles: 4},
                    Instruction{name: "ADC", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::ADC, cycles: 2},
                    Instruction{name: "ROR", addr_mode: ACC,  fn_addr_mode: Self::addr_ACC,  fn_operate: Self::ROR, cycles: 2},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::xxx, cycles: 2}, // ARR
                    Instruction{name: "JMP", addr_mode: IND,  fn_addr_mode: Self::addr_IND,  fn_operate: Self::JMP, cycles: 5},
                    Instruction{name: "ADC", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::ADC, cycles: 4},
                    Instruction{name: "ROR", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::ROR, cycles: 6},
                    Instruction{name: "...", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::xxx, cycles: 6}, // RRA

                    // Row 7
                    Instruction{name: "BVS", addr_mode: REL,  fn_addr_mode: Self::addr_REL,  fn_operate: Self::BVS, cycles: 2},
                    Instruction{name: "ADC", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::ADC, cycles: 5},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::xxx, cycles: 8}, // RRA
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "ADC", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::ADC, cycles: 4},
                    Instruction{name: "ROR", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::ROR, cycles: 6},
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::xxx, cycles: 6}, // RRA
                    Instruction{name: "SEI", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::SEI, cycles: 2},
                    Instruction{name: "ADC", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::ADC, cycles: 4},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 7}, // RRA
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "ADC", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::ADC, cycles: 4},
                    Instruction{name: "ROR", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::ROR, cycles: 7},
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::xxx, cycles: 7}, // RRA

                    // Row 8
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "STA", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::STA, cycles: 6},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::xxx, cycles: 6}, // SAX
                    Instruction{name: "STY", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::STY, cycles: 3},
                    Instruction{name: "STA", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::STA, cycles: 3},
                    Instruction{name: "STX", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::STX, cycles: 3},
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::xxx, cycles: 3}, // SAX
                    Instruction{name: "DEY", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::DEY, cycles: 2},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "TXA", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::TXA, cycles: 2},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::xxx, cycles: 2}, // ANE
                    Instruction{name: "STY", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::STY, cycles: 4},
                    Instruction{name: "STA", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::STA, cycles: 4},
                    Instruction{name: "STX", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::STX, cycles: 4},
                    Instruction{name: "...", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::xxx, cycles: 4}, // SAX

                    // Row 9
                    Instruction{name: "BCC", addr_mode: REL,  fn_addr_mode: Self::addr_REL,  fn_operate: Self::BCC, cycles: 2},
                    Instruction{name: "STA", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::STA, cycles: 6},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::xxx, cycles: 6}, // SHA
                    Instruction{name: "STY", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::STY, cycles: 4},
                    Instruction{name: "STA", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::STA, cycles: 4},
                    Instruction{name: "STX", addr_mode: ZPGY, fn_addr_mode: Self::addr_ZPGy, fn_operate: Self::STX, cycles: 4},
                    Instruction{name: "...", addr_mode: ZPGY, fn_addr_mode: Self::addr_ZPGy, fn_operate: Self::xxx, cycles: 4}, // SAX
                    Instruction{name: "TYA", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::TYA, cycles: 2},
                    Instruction{name: "STA", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::STA, cycles: 5},
                    Instruction{name: "TXS", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::TXS, cycles: 2},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 5}, // TAS
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::NOP, cycles: 5}, // SHY
                    Instruction{name: "STA", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::STA, cycles: 5},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 5}, // SHX
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 5}, // SHA

                    // Row A
                    Instruction{name: "LDY", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::LDY, cycles: 2},
                    Instruction{name: "LDA", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::LDA, cycles: 6},
                    Instruction{name: "LDX", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::LDX, cycles: 2},
                    Instruction{name: "...", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::xxx, cycles: 6}, // LAX
                    Instruction{name: "LDY", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::LDY, cycles: 3},
                    Instruction{name: "LDA", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::LDA, cycles: 3},
                    Instruction{name: "LDX", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::LDX, cycles: 3},
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::xxx, cycles: 3}, // LAX
                    Instruction{name: "TAY", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::TAY, cycles: 2},
                    Instruction{name: "LDA", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::LDA, cycles: 2},
                    Instruction{name: "TAX", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::TAX, cycles: 2},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::xxx, cycles: 2}, // LXA
                    Instruction{name: "LDY", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::LDY, cycles: 4},
                    Instruction{name: "LDA", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::LDA, cycles: 4},
                    Instruction{name: "LDX", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::LDX, cycles: 4},
                    Instruction{name: "...", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::xxx, cycles: 4}, // LAX

                    // Row B
                    Instruction{name: "BCS", addr_mode: REL,  fn_addr_mode: Self::addr_REL,  fn_operate: Self::BCS, cycles: 2},
                    Instruction{name: "LDA", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::LDA, cycles: 5},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::xxx, cycles: 5}, // LAX
                    Instruction{name: "LDY", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::LDY, cycles: 4},
                    Instruction{name: "LDA", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::LDA, cycles: 4},
                    Instruction{name: "LDX", addr_mode: ZPGY, fn_addr_mode: Self::addr_ZPGy, fn_operate: Self::LDX, cycles: 4},
                    Instruction{name: "...", addr_mode: ZPGY, fn_addr_mode: Self::addr_ZPGy, fn_operate: Self::xxx, cycles: 4}, // LAX
                    Instruction{name: "CLV", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::CLV, cycles: 2},
                    Instruction{name: "LDA", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::LDA, cycles: 4},
                    Instruction{name: "TSX", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::TSX, cycles: 2},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 4}, // LAS
                    Instruction{name: "LDY", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::LDY, cycles: 4},
                    Instruction{name: "LDA", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::LDA, cycles: 4},
                    Instruction{name: "LDX", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::LDX, cycles: 4},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 4}, // LAX

                    // Row C
                    Instruction{name: "CPY", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::CPY, cycles: 2},
                    Instruction{name: "CMP", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::CMP, cycles: 6},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::xxx, cycles: 8}, // DCP
                    Instruction{name: "CPY", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::CPY, cycles: 3},
                    Instruction{name: "CMP", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::CMP, cycles: 3},
                    Instruction{name: "DEC", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::DEC, cycles: 5},
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::xxx, cycles: 5}, // DCP
                    Instruction{name: "INY", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::INY, cycles: 2},
                    Instruction{name: "CMP", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::CMP, cycles: 2},
                    Instruction{name: "DEX", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::DEX, cycles: 2},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::xxx, cycles: 2}, // SBX
                    Instruction{name: "CPY", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::CPY, cycles: 4},
                    Instruction{name: "CMP", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::CMP, cycles: 4},
                    Instruction{name: "DEC", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::DEC, cycles: 6},
                    Instruction{name: "...", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::xxx, cycles: 6}, // DCP

                    // Row D
                    Instruction{name: "BNE", addr_mode: REL,  fn_addr_mode: Self::addr_REL,  fn_operate: Self::BNE, cycles: 2},
                    Instruction{name: "CMP", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::CMP, cycles: 5},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::xxx, cycles: 8}, // DCP
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "CMP", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::CMP, cycles: 4},
                    Instruction{name: "DEC", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::DEC, cycles: 6},
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::xxx, cycles: 6}, // DCP
                    Instruction{name: "CLD", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::CLD, cycles: 2},
                    Instruction{name: "CMP", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::CMP, cycles: 4},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 7}, // DCP
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "CMP", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::CMP, cycles: 4},
                    Instruction{name: "DEC", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::DEC, cycles: 7},
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::xxx, cycles: 7}, // DCP

                    // Row E
                    Instruction{name: "CPX", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::CPX, cycles: 2},
                    Instruction{name: "SBC", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::SBC, cycles: 6},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: INDX, fn_addr_mode: Self::addr_INDx, fn_operate: Self::xxx, cycles: 8}, // ISC
                    Instruction{name: "CPX", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::CPX, cycles: 3},
                    Instruction{name: "SBC", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::SBC, cycles: 3},
                    Instruction{name: "INC", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::INC, cycles: 5},
                    Instruction{name: "...", addr_mode: ZPG,  fn_addr_mode: Self::addr_ZPG,  fn_operate: Self::xxx, cycles: 5}, // ISC
                    Instruction{name: "INX", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::INX, cycles: 2},
                    Instruction{name: "SBC", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::SBC, cycles: 2},
                    Instruction{name: "NOP", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: IMM,  fn_addr_mode: Self::addr_IMM,  fn_operate: Self::SBC, cycles: 2}, // USBC
                    Instruction{name: "CPX", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::CPX, cycles: 4},
                    Instruction{name: "SBC", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::SBC, cycles: 4},
                    Instruction{name: "INC", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::INC, cycles: 6},
                    Instruction{name: "...", addr_mode: ABS,  fn_addr_mode: Self::addr_ABS,  fn_operate: Self::xxx, cycles: 6}, // ISC

                    // Row F
                    Instruction{name: "BEQ", addr_mode: REL,  fn_addr_mode: Self::addr_REL,  fn_operate: Self::BEQ, cycles: 2},
                    Instruction{name: "SBC", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::SBC, cycles: 5},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: INDY, fn_addr_mode: Self::addr_INDy, fn_operate: Self::xxx, cycles: 8}, // ISC
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "SBC", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::SBC, cycles: 4},
                    Instruction{name: "INC", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::INC, cycles: 6},
                    Instruction{name: "...", addr_mode: ZPGX, fn_addr_mode: Self::addr_ZPGx, fn_operate: Self::xxx, cycles: 6}, // ISC
                    Instruction{name: "SED", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::SED, cycles: 2},
                    Instruction{name: "SBC", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::SBC, cycles: 4},
                    Instruction{name: "...", addr_mode: IMP,  fn_addr_mode: Self::addr_IMP,  fn_operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ABSY, fn_addr_mode: Self::addr_ABSy, fn_operate: Self::xxx, cycles: 7}, // ISC
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::NOP, cycles: 4},
                    Instruction{name: "SBC", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::SBC, cycles: 4},
                    Instruction{name: "INC", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::INC, cycles: 7},
                    Instruction{name: "...", addr_mode: ABSX, fn_addr_mode: Self::addr_ABSx, fn_operate: Self::xxx, cycles: 7}, // ISC
                ]
            }
        }
    
        pub fn read(&self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &Bus, addr: u16) -> u8 {
            bus.cpu_read(cartridge, ppu, addr, false)
        }

        pub fn write(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus, addr: u16, data: u8) {
            bus.cpu_write(cartridge, ppu, addr, data);
        }

        pub const fn get_flag(&self, flag: Flags) -> bool {
            self.status & (flag as u8) > 0
        }
        
        pub fn set_flag(&mut self, flag: Flags, value: bool) {
            if value {
                self.status |= flag as u8;
            } else {
                self.status &= !(flag as u8);
            }
        }

        pub fn fetch(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &Bus) -> u8 {
            if (self.lookup[self.opcode as usize].addr_mode != ADDRESSING_MODES::ACC)
            || (self.lookup[self.opcode as usize].addr_mode != ADDRESSING_MODES::IMP) {
                self.fetched = self.read(cartridge, ppu, bus, self.addr_abs);
            }
            self.fetched
        }

        /// Handle clock cycles
        pub fn clock(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) {
            if self.cycles == 0 {
                self.opcode = self.read(cartridge, ppu, bus, self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.cycles = self.lookup[self.opcode as usize].cycles;
                
                let bonus_cycles_addr_mode = (self.lookup[self.opcode as usize].fn_addr_mode)(self, cartridge, ppu, bus);
                let bonus_cycles_operate = (self.lookup[self.opcode as usize].fn_operate)(self, cartridge, ppu, bus);
                self.cycles += bonus_cycles_addr_mode & bonus_cycles_operate;
            }
            self.cycles -= 1;
        }

        /// Reset signal
        pub fn reset(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) {
            // Reset registers
            self.a = 0;
            self.x = 0;
            self.y = 0;
            // Reset SP
            self.sp = 0xFD;
            
            // Reset PC address is hardcoded at 0xFFFC and 0xFFFD
            let lo = self.read(cartridge, ppu, bus, 0xFFFC) as u16;
            let hi = self.read(cartridge, ppu, bus, 0xFFFD) as u16;
            self.pc = (hi << 8) | lo;
            
            // Reset Flags
            self.status = Flags::U as u8;

            // Reset custom variables
            self.fetched = 0;
            self.addr_abs = 0;
            self.addr_rel = 0;

            // Manually set cycles because reset takes time
            self.cycles = 8;
        }

        /// Interrupt request signal
        #[allow(dead_code)]
        fn irq(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) {
            if !self.get_flag(Flags::I) {
                // Push PC to stack (16 bits to write)
                self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
                self.sp = self.sp.wrapping_sub(1);

                // Push Flags to stack
                self.set_flag(Flags::B, false);
                self.set_flag(Flags::U, true);
                self.set_flag(Flags::I, true);
                self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, self.status);
                self.sp = self.sp.wrapping_sub(1);

                // New PC address to handle the interrupt is 0xFFFE and 0xFFFF
                let lo = self.read(cartridge, ppu, bus, 0xFFFE) as u16;
                let hi = self.read(cartridge, ppu, bus, 0xFFFF) as u16;
                self.pc = (hi << 8) | lo;

                // Manually set cycles because interrupt request takes time
                self.cycles = 7;
            }
        }

        /// Non-maskable interrupt request signal
        #[allow(dead_code)]
        fn nmi(&mut self, cartridge: &mut Cartridge, ppu: &mut Ppu2C02, bus: &mut Bus) {
            // Push PC to stack (16 bits to write)
            self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.sp = self.sp.wrapping_sub(1);
            self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
            self.sp = self.sp.wrapping_sub(1);

            // Push Flags to stack
            self.set_flag(Flags::B, false);
            self.set_flag(Flags::U, true);
            self.set_flag(Flags::I, true);
            self.write(cartridge, ppu, bus, 0x0100 + self.sp as u16, self.status);
            self.sp = self.sp.wrapping_sub(1);

            // New PC address to handle the interrupt is 0xFFFA and 0xFFFB
            let lo = self.read(cartridge, ppu, bus, 0xFFFA) as u16;
            let hi = self.read(cartridge, ppu, bus, 0xFFFB) as u16;
            self.pc = (hi << 8) | lo;

            // Manually set cycles because non-maskable interrupt request takes time
            self.cycles = 8;
        }

    }
}
