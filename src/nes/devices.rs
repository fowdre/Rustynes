mod addressing_modes;
mod opcodes;


pub mod cpu6502 {
    use super::super::bus::Bus;

    #[derive(Debug)]
    pub struct Cpu6502 {
        a: u8,
        x: u8,
        y: u8,
        sp: u8,
        pc: u16,
        status: u8,
        
        opcode: u8,
        fetched: u8,
        addr_abs: u16,
        addr_rel: u16,
        cycles: u8,

        lookup: [Instruction; 256],
    }

    #[derive(Debug)]
    pub struct Instruction {
        pub name: &'static str,
        pub cycles: u8,
        pub addr_mode: fn(&mut Cpu6502) -> u8,
        pub operate: fn(&mut Cpu6502) -> u8,
    }

    enum Flags {
        C = (1 << 0), // Carry
        Z = (1 << 1), // Zero
        I = (1 << 2), // Interrupt Disable
        D = (1 << 3), // Decimal
        B = (1 << 4),
        U = (1 << 5),
        V = (1 << 6), // Overflow
        N = (1 << 7), // Negative
    }

    impl Cpu6502 {
        pub fn new() -> Self {
            Self {
                a: 0,
                x: 0,
                y: 0,
                sp: 0,
                pc: 0,
                status: 0,
                
                opcode: 0,
                fetched: 0,
                addr_abs: 0,
                addr_rel: 0,
                cycles: 0,
                
                lookup: [
                    // Row 0
                    Instruction{name: "BRK", addr_mode: Self::addr_IMP,  operate: Self::BRK, cycles: 7},
                    Instruction{name: "ORA", addr_mode: Self::addr_INDx, operate: Self::ORA, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDx, operate: Self::xxx, cycles: 8}, // SLO
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::NOP, cycles: 3},
                    Instruction{name: "ORA", addr_mode: Self::addr_ZPG,  operate: Self::ORA, cycles: 3},
                    Instruction{name: "ASL", addr_mode: Self::addr_ZPG,  operate: Self::ASL, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::xxx, cycles: 5}, // SLO
                    Instruction{name: "PHP", addr_mode: Self::addr_IMP,  operate: Self::PHP, cycles: 3},
                    Instruction{name: "ORA", addr_mode: Self::addr_IMM,  operate: Self::ORA, cycles: 2},
                    Instruction{name: "ASL", addr_mode: Self::addr_ACC,  operate: Self::ASL, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::xxx, cycles: 2}, // ANC
                    Instruction{name: "...", addr_mode: Self::addr_ABS,  operate: Self::NOP, cycles: 4},
                    Instruction{name: "ORA", addr_mode: Self::addr_ABS,  operate: Self::ORA, cycles: 4},
                    Instruction{name: "ASL", addr_mode: Self::addr_ABS,  operate: Self::ASL, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ABS,  operate: Self::xxx, cycles: 6}, // SLO

                    // Row 1
                    Instruction{name: "BPL", addr_mode: Self::addr_REL,  operate: Self::BPL, cycles: 2},
                    Instruction{name: "ORA", addr_mode: Self::addr_INDy, operate: Self::ORA, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDy, operate: Self::xxx, cycles: 8}, // SLO
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "ORA", addr_mode: Self::addr_ZPGx, operate: Self::ORA, cycles: 4},
                    Instruction{name: "ASL", addr_mode: Self::addr_ZPGx, operate: Self::ASL, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::xxx, cycles: 6}, // SLO
                    Instruction{name: "CLC", addr_mode: Self::addr_IMP,  operate: Self::CLC, cycles: 2},
                    Instruction{name: "ORA", addr_mode: Self::addr_ABSy, operate: Self::ORA, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 7}, // SLO
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "ORA", addr_mode: Self::addr_ABSx, operate: Self::ORA, cycles: 4},
                    Instruction{name: "ASL", addr_mode: Self::addr_ABSx, operate: Self::ASL, cycles: 7},
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::xxx, cycles: 7}, // SLO*

                    // Row 2
                    Instruction{name: "JSR", addr_mode: Self::addr_ABS,  operate: Self::JSR, cycles: 6},
                    Instruction{name: "AND", addr_mode: Self::addr_INDx, operate: Self::AND, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDx, operate: Self::xxx, cycles: 8}, // RLA
                    Instruction{name: "BIT", addr_mode: Self::addr_ZPG,  operate: Self::BIT, cycles: 3},
                    Instruction{name: "AND", addr_mode: Self::addr_ZPG,  operate: Self::AND, cycles: 3},
                    Instruction{name: "ROL", addr_mode: Self::addr_ZPG,  operate: Self::ROL, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::xxx, cycles: 5}, // RLA
                    Instruction{name: "PLP", addr_mode: Self::addr_IMP,  operate: Self::PLP, cycles: 4},
                    Instruction{name: "AND", addr_mode: Self::addr_IMM,  operate: Self::AND, cycles: 2},
                    Instruction{name: "ROL", addr_mode: Self::addr_ACC,  operate: Self::ROL, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::xxx, cycles: 2}, // ANC
                    Instruction{name: "BIT", addr_mode: Self::addr_ABS,  operate: Self::BIT, cycles: 4},
                    Instruction{name: "AND", addr_mode: Self::addr_ABS,  operate: Self::AND, cycles: 4},
                    Instruction{name: "ROL", addr_mode: Self::addr_ABS,  operate: Self::ROL, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ABS,  operate: Self::xxx, cycles: 6}, // RLA

                    // Row 3
                    Instruction{name: "BMI", addr_mode: Self::addr_REL,  operate: Self::BMI, cycles: 2},
                    Instruction{name: "AND", addr_mode: Self::addr_INDy, operate: Self::AND, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDy, operate: Self::xxx, cycles: 8}, // RLA
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "AND", addr_mode: Self::addr_ZPGx, operate: Self::AND, cycles: 4},
                    Instruction{name: "ROL", addr_mode: Self::addr_ZPGx, operate: Self::ROL, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::xxx, cycles: 6}, // RLA
                    Instruction{name: "SEC", addr_mode: Self::addr_IMP,  operate: Self::SEC, cycles: 2},
                    Instruction{name: "AND", addr_mode: Self::addr_ABSy, operate: Self::AND, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 7}, // RLA
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "AND", addr_mode: Self::addr_ABSx, operate: Self::AND, cycles: 4},
                    Instruction{name: "ROL", addr_mode: Self::addr_ABSx, operate: Self::ROL, cycles: 7},
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::xxx, cycles: 7}, // RLA

                    // Row 4
                    Instruction{name: "RTI", addr_mode: Self::addr_IMP,  operate: Self::RTI, cycles: 6},
                    Instruction{name: "EOR", addr_mode: Self::addr_INDx, operate: Self::EOR, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDx, operate: Self::xxx, cycles: 8}, // SRE
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::NOP, cycles: 3},
                    Instruction{name: "EOR", addr_mode: Self::addr_ZPG,  operate: Self::EOR, cycles: 3},
                    Instruction{name: "LSR", addr_mode: Self::addr_ZPG,  operate: Self::LSR, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::xxx, cycles: 5}, // SRE
                    Instruction{name: "PHA", addr_mode: Self::addr_IMP,  operate: Self::PHA, cycles: 3},
                    Instruction{name: "EOR", addr_mode: Self::addr_IMM,  operate: Self::EOR, cycles: 2},
                    Instruction{name: "LSR", addr_mode: Self::addr_ACC,  operate: Self::LSR, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::xxx, cycles: 2}, // ALR
                    Instruction{name: "JMP", addr_mode: Self::addr_ABS,  operate: Self::JMP, cycles: 3},
                    Instruction{name: "EOR", addr_mode: Self::addr_ABS,  operate: Self::EOR, cycles: 4},
                    Instruction{name: "LSR", addr_mode: Self::addr_ABS,  operate: Self::LSR, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ABS,  operate: Self::xxx, cycles: 6}, // SRE

                    // Row 5
                    Instruction{name: "BVC", addr_mode: Self::addr_REL,  operate: Self::BVC, cycles: 2},
                    Instruction{name: "EOR", addr_mode: Self::addr_INDy, operate: Self::EOR, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDy, operate: Self::xxx, cycles: 8}, // SRE
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "EOR", addr_mode: Self::addr_ZPGx, operate: Self::EOR, cycles: 4},
                    Instruction{name: "LSR", addr_mode: Self::addr_ZPGx, operate: Self::LSR, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::xxx, cycles: 6}, // SRE
                    Instruction{name: "CLI", addr_mode: Self::addr_IMP,  operate: Self::CLI, cycles: 2},
                    Instruction{name: "EOR", addr_mode: Self::addr_ABSy, operate: Self::EOR, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 7}, // SRE
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "EOR", addr_mode: Self::addr_ABSx, operate: Self::EOR, cycles: 4},
                    Instruction{name: "LSR", addr_mode: Self::addr_ABSx, operate: Self::LSR, cycles: 7},
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::xxx, cycles: 7}, // SRE

                    // Row 6
                    Instruction{name: "RTS", addr_mode: Self::addr_IMP,  operate: Self::RTS, cycles: 6},
                    Instruction{name: "ADC", addr_mode: Self::addr_INDx, operate: Self::ADC, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDx, operate: Self::xxx, cycles: 8}, // RRA
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::NOP, cycles: 3},
                    Instruction{name: "ADC", addr_mode: Self::addr_ZPG,  operate: Self::ADC, cycles: 3},
                    Instruction{name: "ROR", addr_mode: Self::addr_ZPG,  operate: Self::ROR, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::xxx, cycles: 5}, // RRA
                    Instruction{name: "PLA", addr_mode: Self::addr_IMP,  operate: Self::PLA, cycles: 4},
                    Instruction{name: "ADC", addr_mode: Self::addr_IMM,  operate: Self::ADC, cycles: 2},
                    Instruction{name: "ROR", addr_mode: Self::addr_ACC,  operate: Self::ROR, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::xxx, cycles: 2}, // ARR
                    Instruction{name: "JMP", addr_mode: Self::addr_IND,  operate: Self::JMP, cycles: 5},
                    Instruction{name: "ADC", addr_mode: Self::addr_ABS,  operate: Self::ADC, cycles: 4},
                    Instruction{name: "ROR", addr_mode: Self::addr_ABS,  operate: Self::ROR, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ABS,  operate: Self::xxx, cycles: 6}, // RRA

                    // Row 7
                    Instruction{name: "BVS", addr_mode: Self::addr_REL,  operate: Self::BVS, cycles: 2},
                    Instruction{name: "ADC", addr_mode: Self::addr_INDy, operate: Self::ADC, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDy, operate: Self::xxx, cycles: 8}, // RRA
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "ADC", addr_mode: Self::addr_ZPGx, operate: Self::ADC, cycles: 4},
                    Instruction{name: "ROR", addr_mode: Self::addr_ZPGx, operate: Self::ROR, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::xxx, cycles: 6}, // RRA
                    Instruction{name: "SEI", addr_mode: Self::addr_IMP,  operate: Self::SEI, cycles: 2},
                    Instruction{name: "ADC", addr_mode: Self::addr_ABSy, operate: Self::ADC, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 7}, // RRA
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "ADC", addr_mode: Self::addr_ABSx, operate: Self::ADC, cycles: 4},
                    Instruction{name: "ROR", addr_mode: Self::addr_ABSx, operate: Self::ROR, cycles: 7},
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::xxx, cycles: 7}, // RRA

                    // Row 8
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "STA", addr_mode: Self::addr_INDx, operate: Self::STA, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_INDx, operate: Self::xxx, cycles: 6}, // SAX
                    Instruction{name: "STY", addr_mode: Self::addr_ZPG,  operate: Self::STY, cycles: 3},
                    Instruction{name: "STA", addr_mode: Self::addr_ZPG,  operate: Self::STA, cycles: 3},
                    Instruction{name: "STX", addr_mode: Self::addr_ZPG,  operate: Self::STX, cycles: 3},
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::xxx, cycles: 3}, // SAX
                    Instruction{name: "DEY", addr_mode: Self::addr_IMP,  operate: Self::DEY, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "TXA", addr_mode: Self::addr_IMP,  operate: Self::TXA, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::xxx, cycles: 2}, // ANE
                    Instruction{name: "STY", addr_mode: Self::addr_ABS,  operate: Self::STY, cycles: 4},
                    Instruction{name: "STA", addr_mode: Self::addr_ABS,  operate: Self::STA, cycles: 4},
                    Instruction{name: "STX", addr_mode: Self::addr_ABS,  operate: Self::STX, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_ABS,  operate: Self::xxx, cycles: 4}, // SAX

                    // Row 9
                    Instruction{name: "BCC", addr_mode: Self::addr_REL,  operate: Self::BCC, cycles: 2},
                    Instruction{name: "STA", addr_mode: Self::addr_INDy, operate: Self::STA, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDy, operate: Self::xxx, cycles: 6}, // SHA
                    Instruction{name: "STY", addr_mode: Self::addr_ZPGx, operate: Self::STY, cycles: 4},
                    Instruction{name: "STA", addr_mode: Self::addr_ZPGx, operate: Self::STA, cycles: 4},
                    Instruction{name: "STX", addr_mode: Self::addr_ZPGy, operate: Self::STX, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_ZPGy, operate: Self::xxx, cycles: 4}, // SAX
                    Instruction{name: "TYA", addr_mode: Self::addr_IMP,  operate: Self::TYA, cycles: 2},
                    Instruction{name: "STA", addr_mode: Self::addr_ABSy, operate: Self::STA, cycles: 5},
                    Instruction{name: "TXS", addr_mode: Self::addr_IMP,  operate: Self::TXS, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 5}, // TAS
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::NOP, cycles: 5}, // SHY
                    Instruction{name: "STA", addr_mode: Self::addr_ABSx, operate: Self::STA, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 5}, // SHX
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 5}, // SHA

                    // Row A
                    Instruction{name: "LDY", addr_mode: Self::addr_IMM,  operate: Self::LDY, cycles: 2},
                    Instruction{name: "LDA", addr_mode: Self::addr_INDx, operate: Self::LDA, cycles: 6},
                    Instruction{name: "LDX", addr_mode: Self::addr_IMM,  operate: Self::LDX, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_INDx, operate: Self::xxx, cycles: 6}, // LAX
                    Instruction{name: "LDY", addr_mode: Self::addr_ZPG,  operate: Self::LDY, cycles: 3},
                    Instruction{name: "LDA", addr_mode: Self::addr_ZPG,  operate: Self::LDA, cycles: 3},
                    Instruction{name: "LDX", addr_mode: Self::addr_ZPG,  operate: Self::LDX, cycles: 3},
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::xxx, cycles: 3}, // LAX
                    Instruction{name: "TAY", addr_mode: Self::addr_IMP,  operate: Self::TAY, cycles: 2},
                    Instruction{name: "LDA", addr_mode: Self::addr_IMM,  operate: Self::LDA, cycles: 2},
                    Instruction{name: "TAX", addr_mode: Self::addr_IMP,  operate: Self::TAX, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::xxx, cycles: 2}, // LXA
                    Instruction{name: "LDY", addr_mode: Self::addr_ABS,  operate: Self::LDY, cycles: 4},
                    Instruction{name: "LDA", addr_mode: Self::addr_ABS,  operate: Self::LDA, cycles: 4},
                    Instruction{name: "LDX", addr_mode: Self::addr_ABS,  operate: Self::LDX, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_ABS,  operate: Self::xxx, cycles: 4}, // LAX

                    // Row B
                    Instruction{name: "BCS", addr_mode: Self::addr_REL,  operate: Self::BCS, cycles: 2},
                    Instruction{name: "LDA", addr_mode: Self::addr_INDy, operate: Self::LDA, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDy, operate: Self::xxx, cycles: 5}, // LAX
                    Instruction{name: "LDY", addr_mode: Self::addr_ZPGx, operate: Self::LDY, cycles: 4},
                    Instruction{name: "LDA", addr_mode: Self::addr_ZPGx, operate: Self::LDA, cycles: 4},
                    Instruction{name: "LDX", addr_mode: Self::addr_ZPGy, operate: Self::LDX, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_ZPGy, operate: Self::xxx, cycles: 4}, // LAX
                    Instruction{name: "CLV", addr_mode: Self::addr_IMP,  operate: Self::CLV, cycles: 2},
                    Instruction{name: "LDA", addr_mode: Self::addr_ABSy, operate: Self::LDA, cycles: 4},
                    Instruction{name: "TSX", addr_mode: Self::addr_IMP,  operate: Self::TSX, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 4}, // LAS
                    Instruction{name: "LDY", addr_mode: Self::addr_ABSx, operate: Self::LDY, cycles: 4},
                    Instruction{name: "LDA", addr_mode: Self::addr_ABSx, operate: Self::LDA, cycles: 4},
                    Instruction{name: "LDX", addr_mode: Self::addr_ABSy, operate: Self::LDX, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 4}, // LAX

                    // Row C
                    Instruction{name: "CPY", addr_mode: Self::addr_IMM,  operate: Self::CPY, cycles: 2},
                    Instruction{name: "CMP", addr_mode: Self::addr_INDx, operate: Self::CMP, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_INDx, operate: Self::xxx, cycles: 8}, // DCP
                    Instruction{name: "CPY", addr_mode: Self::addr_ZPG,  operate: Self::CPY, cycles: 3},
                    Instruction{name: "CMP", addr_mode: Self::addr_ZPG,  operate: Self::CMP, cycles: 3},
                    Instruction{name: "DEC", addr_mode: Self::addr_ZPG,  operate: Self::DEC, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::xxx, cycles: 5}, // DCP
                    Instruction{name: "INY", addr_mode: Self::addr_IMP,  operate: Self::INY, cycles: 2},
                    Instruction{name: "CMP", addr_mode: Self::addr_IMM,  operate: Self::CMP, cycles: 2},
                    Instruction{name: "DEX", addr_mode: Self::addr_IMP,  operate: Self::DEX, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::xxx, cycles: 2}, // SBX
                    Instruction{name: "CPY", addr_mode: Self::addr_ABS,  operate: Self::CPY, cycles: 4},
                    Instruction{name: "CMP", addr_mode: Self::addr_ABS,  operate: Self::CMP, cycles: 4},
                    Instruction{name: "DEC", addr_mode: Self::addr_ABS,  operate: Self::DEC, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ABS,  operate: Self::xxx, cycles: 6}, // DCP

                    // Row D
                    Instruction{name: "BNE", addr_mode: Self::addr_REL,  operate: Self::BNE, cycles: 2},
                    Instruction{name: "CMP", addr_mode: Self::addr_INDy, operate: Self::CMP, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDy, operate: Self::xxx, cycles: 8}, // DCP
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "CMP", addr_mode: Self::addr_ZPGx, operate: Self::CMP, cycles: 4},
                    Instruction{name: "DEC", addr_mode: Self::addr_ZPGx, operate: Self::DEC, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::xxx, cycles: 6}, // DCP
                    Instruction{name: "CLD", addr_mode: Self::addr_IMP,  operate: Self::CLD, cycles: 2},
                    Instruction{name: "CMP", addr_mode: Self::addr_ABSy, operate: Self::CMP, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 7}, // DCP
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "CMP", addr_mode: Self::addr_ABSx, operate: Self::CMP, cycles: 4},
                    Instruction{name: "DEC", addr_mode: Self::addr_ABSx, operate: Self::DEC, cycles: 7},
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::xxx, cycles: 7}, // DCP

                    // Row E
                    Instruction{name: "CPX", addr_mode: Self::addr_IMM,  operate: Self::CPX, cycles: 2},
                    Instruction{name: "SBC", addr_mode: Self::addr_INDx, operate: Self::SBC, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_INDx, operate: Self::xxx, cycles: 8}, // ISC
                    Instruction{name: "CPX", addr_mode: Self::addr_ZPG,  operate: Self::CPX, cycles: 3},
                    Instruction{name: "SBC", addr_mode: Self::addr_ZPG,  operate: Self::SBC, cycles: 3},
                    Instruction{name: "INC", addr_mode: Self::addr_ZPG,  operate: Self::INC, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_ZPG,  operate: Self::xxx, cycles: 5}, // ISC
                    Instruction{name: "INX", addr_mode: Self::addr_IMP,  operate: Self::INX, cycles: 2},
                    Instruction{name: "SBC", addr_mode: Self::addr_IMM,  operate: Self::SBC, cycles: 2},
                    Instruction{name: "NOP", addr_mode: Self::addr_IMP,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_IMM,  operate: Self::SBC, cycles: 2}, // USBC
                    Instruction{name: "CPX", addr_mode: Self::addr_ABS,  operate: Self::CPX, cycles: 4},
                    Instruction{name: "SBC", addr_mode: Self::addr_ABS,  operate: Self::SBC, cycles: 4},
                    Instruction{name: "INC", addr_mode: Self::addr_ABS,  operate: Self::INC, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ABS,  operate: Self::xxx, cycles: 6}, // ISC

                    // Row F
                    Instruction{name: "BEQ", addr_mode: Self::addr_REL,  operate: Self::BEQ, cycles: 2},
                    Instruction{name: "SBC", addr_mode: Self::addr_INDy, operate: Self::SBC, cycles: 5},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: Self::addr_INDy, operate: Self::xxx, cycles: 8}, // ISC
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "SBC", addr_mode: Self::addr_ZPGx, operate: Self::SBC, cycles: 4},
                    Instruction{name: "INC", addr_mode: Self::addr_ZPGx, operate: Self::INC, cycles: 6},
                    Instruction{name: "...", addr_mode: Self::addr_ZPGx, operate: Self::xxx, cycles: 6}, // ISC
                    Instruction{name: "SED", addr_mode: Self::addr_IMP,  operate: Self::SED, cycles: 2},
                    Instruction{name: "SBC", addr_mode: Self::addr_ABSy, operate: Self::SBC, cycles: 4},
                    Instruction{name: "...", addr_mode: Self::addr_IMP,  operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: Self::addr_ABSy, operate: Self::xxx, cycles: 7}, // ISC
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::NOP, cycles: 4},
                    Instruction{name: "SBC", addr_mode: Self::addr_ABSx, operate: Self::SBC, cycles: 4},
                    Instruction{name: "INC", addr_mode: Self::addr_ABSx, operate: Self::INC, cycles: 7},
                    Instruction{name: "...", addr_mode: Self::addr_ABSx, operate: Self::xxx, cycles: 7}, // ISC
                ]
            }
        }
    
        pub fn read(&self, bus: &Bus, addr: u16) -> u8 {
            bus.read(addr, false)
        }

        pub fn write(&mut self, bus: &mut Bus, addr: u16, data: u8) {
            bus.write(addr, data);
        }

        fn get_flag(&self, f: Flags) -> u8 { todo!("get_flag") }
        
        fn set_flag(&self, f: Flags, v: bool) { todo!("set_flag") }

        fn fetch(&mut self) { todo!("fetch"); }

        /// Handle clock cycles
        pub fn clock(&mut self, bus: &Bus) {
            if self.cycles == 0 {
                self.opcode = self.read(bus, self.pc);
                self.pc += 1;
                self.cycles = self.lookup[self.opcode as usize].cycles;
                
                let bonus_cycles_addr_mode = (self.lookup[self.opcode as usize].addr_mode)(self);
                let bonus_cycles_operate = (self.lookup[self.opcode as usize].operate)(self);
                self.cycles += bonus_cycles_addr_mode & bonus_cycles_operate;
            }
            self.cycles -= 1;
        }

        /// Reset signal
        fn reset(&self) { todo!("reset"); }

        /// Interrupt request signal
        fn irq(&self) { todo!("irq"); }

        /// Non-maskable interrupt request signal
        fn nmi(&self) { todo!("nmi"); }

    }
}
