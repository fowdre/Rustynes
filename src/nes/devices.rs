mod addressing_modes;
mod opcodes;


pub mod cpu6502 {
    use super::super::Bus;

    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ADDRESSING_MODES {
        /// Accumulator
        ACC,
        /// Implied
        IMP,
        /// Immediate
        IMM,
        /// Zero Page
        ZP0,
        /// Zero Page with X Offset
        ZPX,
        /// Zero Page with Y Offset
        ZPY,
        /// Relative
        REL,
        /// Absolute
        ABS,
        /// Absolute with X Offset
        ABX,
        /// Absolute with Y Offset
        ABY,
        /// Indirect
        IND,
        /// Indirect with X Offset
        IZX,
        /// Indirect with Y Offset
        IZY,
    }

    impl ADDRESSING_MODES {
        pub fn get_operands_nb(&self) -> u8 {
            match self {
                ADDRESSING_MODES::ACC => 0,
                ADDRESSING_MODES::IMP => 0,
                ADDRESSING_MODES::IMM => 1,
                ADDRESSING_MODES::ZP0 => 1,
                ADDRESSING_MODES::ZPX => 1,
                ADDRESSING_MODES::ZPY => 1,
                ADDRESSING_MODES::REL => 1,
                ADDRESSING_MODES::ABS => 2,
                ADDRESSING_MODES::ABX => 2,
                ADDRESSING_MODES::ABY => 2,
                ADDRESSING_MODES::IND => 2,
                ADDRESSING_MODES::IZX => 1,
                ADDRESSING_MODES::IZY => 1,
            }
        }

        pub fn format_operands(&self, bytes: &[u8], pc: u16) -> String {
            if bytes.is_empty() {
                return "".to_string();
            }
            let index = 1;
            // dbg!(self);
            match self {
                ADDRESSING_MODES::ACC => format!("{}", bytes[index]),
                ADDRESSING_MODES::IMP => "        ".to_string(),
                ADDRESSING_MODES::IMM => format!("#${:02X}    ", bytes[index]),
                ADDRESSING_MODES::ZP0 => format!("${:02X} = 00", bytes[index]),
                ADDRESSING_MODES::ZPX => format!("${}, X", bytes[index]),
                ADDRESSING_MODES::ZPY => format!("${}, Y", bytes[index]),
                ADDRESSING_MODES::REL => format!("${:04X}   ", pc.wrapping_add(2).wrapping_add(bytes[index] as u16)),
                ADDRESSING_MODES::ABS => format!("${:02X}{:02X}   ", bytes[index + 1], bytes[index]),
                ADDRESSING_MODES::ABX => format!("${}, X", bytes[index]),
                ADDRESSING_MODES::ABY => format!("${}, Y", bytes[index]),
                ADDRESSING_MODES::IND => format!("(${})", bytes[index]),
                ADDRESSING_MODES::IZX => format!("(${}, X)", bytes[index]),
                ADDRESSING_MODES::IZY => format!("(${}), Y", bytes[index]),
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
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

    #[derive(Debug, Copy, Clone)]
    pub struct Instruction {
        pub name: &'static str,
        pub cycles: u8,
        pub addr_mode: ADDRESSING_MODES,
        pub addr_mode_fn: fn(&mut Cpu6502, &Bus) -> u8,
        pub operate: fn(&mut Cpu6502, &mut Bus) -> u8,
    }

    pub enum Flags {
        /// bit 0 | Carry
        C = (1 << 0),
        /// bit 1 | Zero
        Z = (1 << 1),
        /// bit 2 | Interrupt Disable
        I = (1 << 2),
        /// bit 3 | Decimal Mode
        D = (1 << 3),
        /// bit 4 | Break Command
        B = (1 << 4),
        /// bit 5 | Unused
        U = (1 << 5),
        /// bit 6 | Overflow
        V = (1 << 6),
        /// bit 7 | Negative
        N = (1 << 7),
    }

    impl Cpu6502 {
        pub fn new() -> Self {
            Self {
                a: 0,
                x: 0,
                y: 0,
                sp: 0xFD,
                pc: 0xC000,
                status: 0,
                
                opcode: 0,
                fetched: 0,
                addr_abs: 0,
                addr_rel: 0,
                cycles: 0,
                
                lookup: [
                    // Row 0
                    Instruction{name: "BRK", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::BRK, cycles: 7},
                    Instruction{name: "ORA", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::ORA, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // SLO
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::NOP, cycles: 3},
                    Instruction{name: "ORA", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ORA, cycles: 3},
                    Instruction{name: "ASL", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ASL, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // SLO
                    Instruction{name: "PHP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::PHP, cycles: 3},
                    Instruction{name: "ORA", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::ORA, cycles: 2},
                    Instruction{name: "ASL", addr_mode: ADDRESSING_MODES::ACC, addr_mode_fn: Self::addr_ACC, operate: Self::ASL, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // ANC
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::NOP, cycles: 4},
                    Instruction{name: "ORA", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ORA, cycles: 4},
                    Instruction{name: "ASL", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ASL, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // SLO

                    // Row 1
                    Instruction{name: "BPL", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BPL, cycles: 2},
                    Instruction{name: "ORA", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::ORA, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // SLO
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "ORA", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ORA, cycles: 4},
                    Instruction{name: "ASL", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ASL, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // SLO
                    Instruction{name: "CLC", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::CLC, cycles: 2},
                    Instruction{name: "ORA", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::ORA, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // SLO
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "ORA", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ORA, cycles: 4},
                    Instruction{name: "ASL", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ASL, cycles: 7},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // SLO*

                    // Row 2
                    Instruction{name: "JSR", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::JSR, cycles: 6},
                    Instruction{name: "AND", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::AND, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // RLA
                    Instruction{name: "BIT", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::BIT, cycles: 3},
                    Instruction{name: "AND", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::AND, cycles: 3},
                    Instruction{name: "ROL", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ROL, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // RLA
                    Instruction{name: "PLP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::PLP, cycles: 4},
                    Instruction{name: "AND", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::AND, cycles: 2},
                    Instruction{name: "ROL", addr_mode: ADDRESSING_MODES::ACC, addr_mode_fn: Self::addr_ACC, operate: Self::ROL, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // ANC
                    Instruction{name: "BIT", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::BIT, cycles: 4},
                    Instruction{name: "AND", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::AND, cycles: 4},
                    Instruction{name: "ROL", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ROL, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // RLA

                    // Row 3
                    Instruction{name: "BMI", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BMI, cycles: 2},
                    Instruction{name: "AND", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::AND, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // RLA
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "AND", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::AND, cycles: 4},
                    Instruction{name: "ROL", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ROL, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // RLA
                    Instruction{name: "SEC", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::SEC, cycles: 2},
                    Instruction{name: "AND", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::AND, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // RLA
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "AND", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::AND, cycles: 4},
                    Instruction{name: "ROL", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ROL, cycles: 7},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // RLA

                    // Row 4
                    Instruction{name: "RTI", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::RTI, cycles: 6},
                    Instruction{name: "EOR", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::EOR, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // SRE
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::NOP, cycles: 3},
                    Instruction{name: "EOR", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::EOR, cycles: 3},
                    Instruction{name: "LSR", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::LSR, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // SRE
                    Instruction{name: "PHA", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::PHA, cycles: 3},
                    Instruction{name: "EOR", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::EOR, cycles: 2},
                    Instruction{name: "LSR", addr_mode: ADDRESSING_MODES::ACC, addr_mode_fn: Self::addr_ACC, operate: Self::LSR, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // ALR
                    Instruction{name: "JMP", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::JMP, cycles: 3},
                    Instruction{name: "EOR", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::EOR, cycles: 4},
                    Instruction{name: "LSR", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::LSR, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // SRE

                    // Row 5
                    Instruction{name: "BVC", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BVC, cycles: 2},
                    Instruction{name: "EOR", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::EOR, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // SRE
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "EOR", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::EOR, cycles: 4},
                    Instruction{name: "LSR", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::LSR, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // SRE
                    Instruction{name: "CLI", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::CLI, cycles: 2},
                    Instruction{name: "EOR", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::EOR, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // SRE
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "EOR", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::EOR, cycles: 4},
                    Instruction{name: "LSR", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::LSR, cycles: 7},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // SRE

                    // Row 6
                    Instruction{name: "RTS", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::RTS, cycles: 6},
                    Instruction{name: "ADC", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::ADC, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // RRA
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::NOP, cycles: 3},
                    Instruction{name: "ADC", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ADC, cycles: 3},
                    Instruction{name: "ROR", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ROR, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // RRA
                    Instruction{name: "PLA", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::PLA, cycles: 4},
                    Instruction{name: "ADC", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::ADC, cycles: 2},
                    Instruction{name: "ROR", addr_mode: ADDRESSING_MODES::ACC, addr_mode_fn: Self::addr_ACC, operate: Self::ROR, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // ARR
                    Instruction{name: "JMP", addr_mode: ADDRESSING_MODES::IND, addr_mode_fn: Self::addr_IND, operate: Self::JMP, cycles: 5},
                    Instruction{name: "ADC", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ADC, cycles: 4},
                    Instruction{name: "ROR", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ROR, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // RRA

                    // Row 7
                    Instruction{name: "BVS", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BVS, cycles: 2},
                    Instruction{name: "ADC", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::ADC, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // RRA
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "ADC", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ADC, cycles: 4},
                    Instruction{name: "ROR", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ROR, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // RRA
                    Instruction{name: "SEI", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::SEI, cycles: 2},
                    Instruction{name: "ADC", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::ADC, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // RRA
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "ADC", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ADC, cycles: 4},
                    Instruction{name: "ROR", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ROR, cycles: 7},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // RRA

                    // Row 8
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2},
                    Instruction{name: "STA", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::STA, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 6}, // SAX
                    Instruction{name: "STY", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::STY, cycles: 3},
                    Instruction{name: "STA", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::STA, cycles: 3},
                    Instruction{name: "STX", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::STX, cycles: 3},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 3}, // SAX
                    Instruction{name: "DEY", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::DEY, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2},
                    Instruction{name: "TXA", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TXA, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // ANE
                    Instruction{name: "STY", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::STY, cycles: 4},
                    Instruction{name: "STA", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::STA, cycles: 4},
                    Instruction{name: "STX", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::STX, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 4}, // SAX

                    // Row 9
                    Instruction{name: "BCC", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BCC, cycles: 2},
                    Instruction{name: "STA", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::STA, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 6}, // SHA
                    Instruction{name: "STY", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::STY, cycles: 4},
                    Instruction{name: "STA", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::STA, cycles: 4},
                    Instruction{name: "STX", addr_mode: ADDRESSING_MODES::ZPY, addr_mode_fn: Self::addr_ZPY, operate: Self::STX, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPY, addr_mode_fn: Self::addr_ZPY, operate: Self::xxx, cycles: 4}, // SAX
                    Instruction{name: "TYA", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TYA, cycles: 2},
                    Instruction{name: "STA", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::STA, cycles: 5},
                    Instruction{name: "TXS", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TXS, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 5}, // TAS
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 5}, // SHY
                    Instruction{name: "STA", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::STA, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 5}, // SHX
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 5}, // SHA

                    // Row A
                    Instruction{name: "LDY", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::LDY, cycles: 2},
                    Instruction{name: "LDA", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::LDA, cycles: 6},
                    Instruction{name: "LDX", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::LDX, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 6}, // LAX
                    Instruction{name: "LDY", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::LDY, cycles: 3},
                    Instruction{name: "LDA", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::LDA, cycles: 3},
                    Instruction{name: "LDX", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::LDX, cycles: 3},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 3}, // LAX
                    Instruction{name: "TAY", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TAY, cycles: 2},
                    Instruction{name: "LDA", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::LDA, cycles: 2},
                    Instruction{name: "TAX", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TAX, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // LXA
                    Instruction{name: "LDY", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::LDY, cycles: 4},
                    Instruction{name: "LDA", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::LDA, cycles: 4},
                    Instruction{name: "LDX", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::LDX, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 4}, // LAX

                    // Row B
                    Instruction{name: "BCS", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BCS, cycles: 2},
                    Instruction{name: "LDA", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::LDA, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 5}, // LAX
                    Instruction{name: "LDY", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::LDY, cycles: 4},
                    Instruction{name: "LDA", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::LDA, cycles: 4},
                    Instruction{name: "LDX", addr_mode: ADDRESSING_MODES::ZPY, addr_mode_fn: Self::addr_ZPY, operate: Self::LDX, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPY, addr_mode_fn: Self::addr_ZPY, operate: Self::xxx, cycles: 4}, // LAX
                    Instruction{name: "CLV", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::CLV, cycles: 2},
                    Instruction{name: "LDA", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::LDA, cycles: 4},
                    Instruction{name: "TSX", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TSX, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 4}, // LAS
                    Instruction{name: "LDY", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::LDY, cycles: 4},
                    Instruction{name: "LDA", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::LDA, cycles: 4},
                    Instruction{name: "LDX", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::LDX, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 4}, // LAX

                    // Row C
                    Instruction{name: "CPY", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::CPY, cycles: 2},
                    Instruction{name: "CMP", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::CMP, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // DCP
                    Instruction{name: "CPY", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::CPY, cycles: 3},
                    Instruction{name: "CMP", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::CMP, cycles: 3},
                    Instruction{name: "DEC", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::DEC, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // DCP
                    Instruction{name: "INY", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::INY, cycles: 2},
                    Instruction{name: "CMP", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::CMP, cycles: 2},
                    Instruction{name: "DEX", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::DEX, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // SBX
                    Instruction{name: "CPY", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::CPY, cycles: 4},
                    Instruction{name: "CMP", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::CMP, cycles: 4},
                    Instruction{name: "DEC", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::DEC, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // DCP

                    // Row D
                    Instruction{name: "BNE", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BNE, cycles: 2},
                    Instruction{name: "CMP", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::CMP, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // DCP
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "CMP", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::CMP, cycles: 4},
                    Instruction{name: "DEC", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::DEC, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // DCP
                    Instruction{name: "CLD", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::CLD, cycles: 2},
                    Instruction{name: "CMP", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::CMP, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // DCP
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "CMP", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::CMP, cycles: 4},
                    Instruction{name: "DEC", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::DEC, cycles: 7},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // DCP

                    // Row E
                    Instruction{name: "CPX", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::CPX, cycles: 2},
                    Instruction{name: "SBC", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::SBC, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // ISC
                    Instruction{name: "CPX", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::CPX, cycles: 3},
                    Instruction{name: "SBC", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::SBC, cycles: 3},
                    Instruction{name: "INC", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::INC, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // ISC
                    Instruction{name: "INX", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::INX, cycles: 2},
                    Instruction{name: "SBC", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::SBC, cycles: 2},
                    Instruction{name: "NOP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::SBC, cycles: 2}, // USBC
                    Instruction{name: "CPX", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::CPX, cycles: 4},
                    Instruction{name: "SBC", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::SBC, cycles: 4},
                    Instruction{name: "INC", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::INC, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // ISC

                    // Row F
                    Instruction{name: "BEQ", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BEQ, cycles: 2},
                    Instruction{name: "SBC", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::SBC, cycles: 5},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // JAM
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // ISC
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "SBC", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::SBC, cycles: 4},
                    Instruction{name: "INC", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::INC, cycles: 6},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // ISC
                    Instruction{name: "SED", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::SED, cycles: 2},
                    Instruction{name: "SBC", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::SBC, cycles: 4},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // ISC
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4},
                    Instruction{name: "SBC", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::SBC, cycles: 4},
                    Instruction{name: "INC", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::INC, cycles: 7},
                    Instruction{name: "...", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // ISC
                ]
            }
        }
    
        pub const fn read(&self, bus: &Bus, addr: u16) -> u8 {
            bus.read(addr, false)
        }

        pub fn write(&mut self, bus: &mut Bus, addr: u16, data: u8) {
            bus.write(addr, data);
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

        pub fn fetch(&mut self, bus: &Bus) -> u8 {
            if (self.lookup[self.opcode as usize].addr_mode != ADDRESSING_MODES::ACC)
            || (self.lookup[self.opcode as usize].addr_mode != ADDRESSING_MODES::IMP) {
                self.fetched = self.read(bus, self.addr_abs);
            }
            self.fetched
        }

        /// Handle clock cycles
        pub fn clock(&mut self, bus: &mut Bus) {
            if self.cycles == 0 {
                self.opcode = self.read(bus, self.pc);
                
                self.set_flag(Flags::U, true);
                
                self.pc = self.pc.wrapping_add(1);
                self.cycles = self.lookup[self.opcode as usize].cycles;
                
                let bonus_cycles_addr_mode = (self.lookup[self.opcode as usize].addr_mode_fn)(self, bus);
                let bonus_cycles_operate = (self.lookup[self.opcode as usize].operate)(self, bus);
                self.cycles += bonus_cycles_addr_mode & bonus_cycles_operate;

                self.set_flag(Flags::U, true);
            }
            self.cycles -= 1;
        }

        /// Reset signal
        pub fn reset(&mut self, bus: &Bus) {
            // Reset registers
            self.a = 0;
            self.x = 0;
            self.y = 0;
            // Reset SP
            self.sp = 0xFD;
            
            // Reset PC address is hardcoded at 0xFFFC and 0xFFFD
            let lo = self.read(bus, 0xFFFC) as u16;
            let hi = self.read(bus, 0xFFFD) as u16;
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
        fn irq(&mut self, bus: &mut Bus) {
            if !self.get_flag(Flags::I) {
                // Push PC to stack (16 bits to write)
                self.write(bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.write(bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
                self.sp = self.sp.wrapping_sub(1);

                // Push Flags to stack
                self.set_flag(Flags::B, false);
                self.set_flag(Flags::U, true);
                self.set_flag(Flags::I, true);
                self.write(bus, 0x0100 + self.sp as u16, self.status);
                self.sp = self.sp.wrapping_sub(1);

                // New PC address to handle the interrupt is 0xFFFE and 0xFFFF
                let lo = self.read(bus, 0xFFFE) as u16;
                let hi = self.read(bus, 0xFFFF) as u16;
                self.pc = (hi << 8) | lo;

                // Manually set cycles because interrupt request takes time
                self.cycles = 7;
            }
        }

        /// Non-maskable interrupt request signal
        #[allow(dead_code)]
        fn nmi(&mut self, bus: &mut Bus) {
            // Push PC to stack (16 bits to write)
            self.write(bus, 0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.sp = self.sp.wrapping_sub(1);
            self.write(bus, 0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
            self.sp = self.sp.wrapping_sub(1);

            // Push Flags to stack
            self.set_flag(Flags::B, false);
            self.set_flag(Flags::U, true);
            self.set_flag(Flags::I, true);
            self.write(bus, 0x0100 + self.sp as u16, self.status);
            self.sp = self.sp.wrapping_sub(1);

            // New PC address to handle the interrupt is 0xFFFA and 0xFFFB
            let lo = self.read(bus, 0xFFFA) as u16;
            let hi = self.read(bus, 0xFFFB) as u16;
            self.pc = (hi << 8) | lo;

            // Manually set cycles because non-maskable interrupt request takes time
            self.cycles = 8;
        }

    }
}
