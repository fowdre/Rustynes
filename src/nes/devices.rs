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

        pub fn format_operands(&self, bytes: &[u8], ram: &[u8], opcode: u8, pc: u16, x: u8, y: u8) -> String {
            if bytes.is_empty() {
                return "".to_string();
            }
            let index = 1;
            let special_case = match opcode {
                0x20 => format!("${:02X}{:02X}                     ", bytes[index + 1], bytes[index]),
                0x4C => format!("${:02X}{:02X}                     ", bytes[index + 1], bytes[index]),
                _ => "".to_string(),
            };
            if !special_case.is_empty() {
                return special_case;
            }
            match self {
                ADDRESSING_MODES::ACC => "A                         ".to_string(),
                ADDRESSING_MODES::IMP => "                          ".to_string(),
                ADDRESSING_MODES::IMM => format!("#${:02X}                      ", bytes[index]),
                ADDRESSING_MODES::ZP0 => format!("${:02X} = {:02X}                  ", bytes[index], ram[bytes[index] as usize]),
                ADDRESSING_MODES::ZPX => {
                    let addr = bytes[index].wrapping_add(x) as u16;
                    format!("${:02X},X @ {:02X} = {:02X}           ", bytes[index], addr, ram[(addr & 0x00FF) as usize])
                }
                ADDRESSING_MODES::ZPY => {
                    let addr = bytes[index].wrapping_add(y) as u16;
                    format!("${:02X},Y @ {:02X} = {:02X}           ", bytes[index], addr, ram[(addr & 0x00FF) as usize])
                }
                ADDRESSING_MODES::REL => {
                    let pc = pc.wrapping_add(2);
                    let addr = (pc as i32 + bytes[index] as i8 as i32) as u16;

                    format!("${:04X}                     ", addr)
                }
                ADDRESSING_MODES::ABS => format!("${:02X}{:02X} = {:02X}                ", bytes[index + 1], bytes[index], ram[((bytes[index + 1] as u16) << 8 | bytes[index] as u16) as usize]),
                ADDRESSING_MODES::ABX => {
                    let addr = ((bytes[index + 1] as u16) << 8 | bytes[index] as u16).wrapping_add(x as u16);
                    format!("${:02X}{:02X},X @ {:04X} = {:02X}       ", bytes[index + 1], bytes[index], addr, ram[addr as usize])
                }
                ADDRESSING_MODES::ABY => {
                    let addr = ((bytes[index + 1] as u16) << 8 | bytes[index] as u16).wrapping_add(y as u16);
                    format!("${:02X}{:02X},Y @ {:04X} = {:02X}       ", bytes[index + 1], bytes[index], addr, ram[addr as usize])
                }
                ADDRESSING_MODES::IND => {
                    let ptr_lo = ram[(pc + 1) as usize] as u16;
                    let ptr_hi = ram[(pc + 2) as usize] as u16;

                    let ptr = (ptr_hi << 8) | ptr_lo;

                    let addr = if ptr_lo == 0x00FF { // Simulate bug :D
                        (ram[(ptr & 0xFF00) as usize] as u16) << 8 | ram[ptr as usize] as u16
                    } else {
                        (ram[(ptr + 1) as usize] as u16) << 8 | ram[ptr as usize] as u16
                    };

                    format!("(${:02X}{:02X}) = {addr:04X}            ", bytes[index + 1], bytes[index])
                }
                ADDRESSING_MODES::IZX => {
                    let lo = ram[((bytes[index] as u16 + x as u16) & 0x00FF) as usize] as u16;
                    let hi = ram[((bytes[index] as u16 + x as u16 + 1) & 0x00FF) as usize] as u16;
                    let addr = (hi << 8) | lo;
                    
                    format!("(${:02X},X) @ {:02X} = {:04X} = {:02X}  ", bytes[index], (bytes[index] as u16 + x as u16) & 0x00FF, addr, ram[addr as usize])
                }
                ADDRESSING_MODES::IZY => {
                    let lo = ram[(bytes[index] as u16 & 0x00FF) as usize] as u16;
                    let hi = ram[((bytes[index] as u16 + 1) & 0x00FF) as usize] as u16;
                    let mut addr = (hi << 8) | lo;
                    addr = addr.wrapping_add(y as u16);
                    
                    format!("(${:02X}),Y = {:04X} @ {:04X} = {:02X}", bytes[index], addr.wrapping_sub(y as u16), addr, ram[addr as usize])
                }
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
        /// bit 4 | Doesn't actually exists | Break Command
        B = (1 << 4),
        /// bit 5 | Doesn't actually exists | Unused
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
                status: Flags::U as u8, // U flag is always set
                
                opcode: 0,
                fetched: 0,
                addr_abs: 0,
                addr_rel: 0,
                cycles: 0,
                
                lookup: [
                    // Row 0
                    Instruction{name: " BRK", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::BRK, cycles: 7}, // 00
                    Instruction{name: " ORA", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::ORA, cycles: 6}, // 01
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // 02
                    Instruction{name: "*SLO", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // 03
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::NOP, cycles: 3}, // 04
                    Instruction{name: " ORA", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ORA, cycles: 3}, // 05
                    Instruction{name: " ASL", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ASL, cycles: 5}, // 06
                    Instruction{name: "*SLO", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // 07
                    Instruction{name: " PHP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::PHP, cycles: 3}, // 08
                    Instruction{name: " ORA", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::ORA, cycles: 2}, // 09
                    Instruction{name: " ASL", addr_mode: ADDRESSING_MODES::ACC, addr_mode_fn: Self::addr_ACC, operate: Self::ASL, cycles: 2}, // 0A
                    Instruction{name: "*ANC", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // 0B
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::NOP, cycles: 4}, // 0C
                    Instruction{name: " ORA", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ORA, cycles: 4}, // 0D
                    Instruction{name: " ASL", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ASL, cycles: 6}, // 0E
                    Instruction{name: "*SLO", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // 0F

                    // Row 1
                    Instruction{name: " BPL", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BPL, cycles: 2}, // 10
                    Instruction{name: " ORA", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::ORA, cycles: 5}, // 11
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // 12
                    Instruction{name: "*SLO", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // 13
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4}, // 14
                    Instruction{name: " ORA", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ORA, cycles: 4}, // 15
                    Instruction{name: " ASL", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ASL, cycles: 6}, // 16
                    Instruction{name: "*SLO", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // 17
                    Instruction{name: " CLC", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::CLC, cycles: 2}, // 18
                    Instruction{name: " ORA", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::ORA, cycles: 4}, // 19
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2}, // 1A
                    Instruction{name: "*SLO", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // 1B
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4}, // 1C
                    Instruction{name: " ORA", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ORA, cycles: 4}, // 1D
                    Instruction{name: " ASL", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ASL, cycles: 7}, // 1E
                    Instruction{name: "*SLO", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // 1F

                    // Row 2
                    Instruction{name: " JSR", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::JSR, cycles: 6}, // 20
                    Instruction{name: " AND", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::AND, cycles: 6}, // 21
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // 22
                    Instruction{name: "*RLA", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // 23
                    Instruction{name: " BIT", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::BIT, cycles: 3}, // 24
                    Instruction{name: " AND", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::AND, cycles: 3}, // 25
                    Instruction{name: " ROL", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ROL, cycles: 5}, // 26
                    Instruction{name: "*RLA", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // 27
                    Instruction{name: " PLP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::PLP, cycles: 4}, // 28
                    Instruction{name: " AND", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::AND, cycles: 2}, // 29
                    Instruction{name: " ROL", addr_mode: ADDRESSING_MODES::ACC, addr_mode_fn: Self::addr_ACC, operate: Self::ROL, cycles: 2}, // 2A
                    Instruction{name: "*ANC", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // 2B
                    Instruction{name: " BIT", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::BIT, cycles: 4}, // 2C
                    Instruction{name: " AND", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::AND, cycles: 4}, // 2D
                    Instruction{name: " ROL", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ROL, cycles: 6}, // 2E
                    Instruction{name: "*RLA", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // 2F

                    // Row 3
                    Instruction{name: " BMI", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BMI, cycles: 2}, // 30
                    Instruction{name: " AND", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::AND, cycles: 5}, // 31
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // 32
                    Instruction{name: "*RLA", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // 33
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4}, // 34
                    Instruction{name: " AND", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::AND, cycles: 4}, // 35
                    Instruction{name: " ROL", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ROL, cycles: 6}, // 36
                    Instruction{name: "*RLA", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // 37
                    Instruction{name: " SEC", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::SEC, cycles: 2}, // 38
                    Instruction{name: " AND", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::AND, cycles: 4}, // 39
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2}, // 3A
                    Instruction{name: "*RLA", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // 3B
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4}, // 3C
                    Instruction{name: " AND", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::AND, cycles: 4}, // 3D
                    Instruction{name: " ROL", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ROL, cycles: 7}, // 3E
                    Instruction{name: "*RLA", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // 3F

                    // Row 4
                    Instruction{name: " RTI", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::RTI, cycles: 6}, // 40
                    Instruction{name: " EOR", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::EOR, cycles: 6}, // 41
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // 42
                    Instruction{name: "*SRE", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // 43
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::NOP, cycles: 3}, // 44
                    Instruction{name: " EOR", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::EOR, cycles: 3}, // 45
                    Instruction{name: " LSR", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::LSR, cycles: 5}, // 46
                    Instruction{name: "*SRE", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // 47
                    Instruction{name: " PHA", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::PHA, cycles: 3}, // 48
                    Instruction{name: " EOR", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::EOR, cycles: 2}, // 49
                    Instruction{name: " LSR", addr_mode: ADDRESSING_MODES::ACC, addr_mode_fn: Self::addr_ACC, operate: Self::LSR, cycles: 2}, // 4A
                    Instruction{name: "*ALR", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // 4B
                    Instruction{name: " JMP", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::JMP, cycles: 3}, // 4C
                    Instruction{name: " EOR", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::EOR, cycles: 4}, // 4D
                    Instruction{name: " LSR", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::LSR, cycles: 6}, // 4E
                    Instruction{name: "*SRE", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // 4F

                    // Row 5
                    Instruction{name: " BVC", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BVC, cycles: 2}, // 50
                    Instruction{name: " EOR", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::EOR, cycles: 5}, // 51
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // 52
                    Instruction{name: "*SRE", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // 53
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4}, // 54
                    Instruction{name: " EOR", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::EOR, cycles: 4}, // 55
                    Instruction{name: " LSR", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::LSR, cycles: 6}, // 56
                    Instruction{name: "*SRE", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // 57
                    Instruction{name: " CLI", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::CLI, cycles: 2}, // 58
                    Instruction{name: " EOR", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::EOR, cycles: 4}, // 59
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2}, // 5A
                    Instruction{name: "*SRE", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // 5B
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4}, // 5C
                    Instruction{name: " EOR", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::EOR, cycles: 4}, // 5D
                    Instruction{name: " LSR", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::LSR, cycles: 7}, // 5E
                    Instruction{name: "*SRE", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // 5F

                    // Row 6
                    Instruction{name: " RTS", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::RTS, cycles: 6}, // 60
                    Instruction{name: " ADC", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::ADC, cycles: 6}, // 61
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // 62
                    Instruction{name: "*RRA", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // 63
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::NOP, cycles: 3}, // 64
                    Instruction{name: " ADC", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ADC, cycles: 3}, // 65
                    Instruction{name: " ROR", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::ROR, cycles: 5}, // 66
                    Instruction{name: "*RRA", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // 67
                    Instruction{name: " PLA", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::PLA, cycles: 4}, // 68
                    Instruction{name: " ADC", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::ADC, cycles: 2}, // 69
                    Instruction{name: " ROR", addr_mode: ADDRESSING_MODES::ACC, addr_mode_fn: Self::addr_ACC, operate: Self::ROR, cycles: 2}, // 6A
                    Instruction{name: "*ARR", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // 6B
                    Instruction{name: " JMP", addr_mode: ADDRESSING_MODES::IND, addr_mode_fn: Self::addr_IND, operate: Self::JMP, cycles: 5}, // 6C
                    Instruction{name: " ADC", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ADC, cycles: 4}, // 6D
                    Instruction{name: " ROR", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::ROR, cycles: 6}, // 6E
                    Instruction{name: "*RRA", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // 6F

                    // Row 7
                    Instruction{name: " BVS", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BVS, cycles: 2}, // 70
                    Instruction{name: " ADC", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::ADC, cycles: 5}, // 71
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // 72
                    Instruction{name: "*RRA", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // 73
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4}, // 74
                    Instruction{name: " ADC", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ADC, cycles: 4}, // 75
                    Instruction{name: " ROR", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::ROR, cycles: 6}, // 76
                    Instruction{name: "*RRA", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // 77
                    Instruction{name: " SEI", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::SEI, cycles: 2}, // 78
                    Instruction{name: " ADC", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::ADC, cycles: 4}, // 79
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2}, // 7A
                    Instruction{name: "*RRA", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // 7B
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4}, // 7C
                    Instruction{name: " ADC", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ADC, cycles: 4}, // 7D
                    Instruction{name: " ROR", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::ROR, cycles: 7}, // 7E
                    Instruction{name: "*RRA", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // 7F

                    // Row 8
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2}, // 80
                    Instruction{name: " STA", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::STA, cycles: 6}, // 81
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2}, // 82
                    Instruction{name: "*SAX", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 6}, // 83
                    Instruction{name: " STY", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::STY, cycles: 3}, // 84
                    Instruction{name: " STA", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::STA, cycles: 3}, // 85
                    Instruction{name: " STX", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::STX, cycles: 3}, // 86
                    Instruction{name: "*SAX", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 3}, // 87
                    Instruction{name: " DEY", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::DEY, cycles: 2}, // 88
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2}, // 89
                    Instruction{name: " TXA", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TXA, cycles: 2}, // 8A
                    Instruction{name: "*ANE", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // 8B
                    Instruction{name: " STY", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::STY, cycles: 4}, // 8C
                    Instruction{name: " STA", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::STA, cycles: 4}, // 8D
                    Instruction{name: " STX", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::STX, cycles: 4}, // 8E
                    Instruction{name: "*SAX", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 4}, // 8F

                    // Row 9
                    Instruction{name: " BCC", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BCC, cycles: 2}, // 90
                    Instruction{name: " STA", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::STA, cycles: 6}, // 91
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // 92
                    Instruction{name: "*SHA", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 6}, // 93
                    Instruction{name: " STY", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::STY, cycles: 4}, // 94
                    Instruction{name: " STA", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::STA, cycles: 4}, // 95
                    Instruction{name: " STX", addr_mode: ADDRESSING_MODES::ZPY, addr_mode_fn: Self::addr_ZPY, operate: Self::STX, cycles: 4}, // 96
                    Instruction{name: "*SAX", addr_mode: ADDRESSING_MODES::ZPY, addr_mode_fn: Self::addr_ZPY, operate: Self::xxx, cycles: 4}, // 97
                    Instruction{name: " TYA", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TYA, cycles: 2}, // 98
                    Instruction{name: " STA", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::STA, cycles: 5}, // 99
                    Instruction{name: " TXS", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TXS, cycles: 2}, // 9A
                    Instruction{name: "*TAS", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 5}, // 9B
                    Instruction{name: "*SHY", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 5}, // 9C
                    Instruction{name: " STA", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::STA, cycles: 5}, // 9D
                    Instruction{name: "*SHX", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 5}, // 9E
                    Instruction{name: "*SHA", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 5}, // 9F

                    // Row A
                    Instruction{name: " LDY", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::LDY, cycles: 2}, // A0
                    Instruction{name: " LDA", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::LDA, cycles: 6}, // A1
                    Instruction{name: " LDX", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::LDX, cycles: 2}, // A2
                    Instruction{name: "*LAX", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 6}, // A3
                    Instruction{name: " LDY", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::LDY, cycles: 3}, // A4
                    Instruction{name: " LDA", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::LDA, cycles: 3}, // A5
                    Instruction{name: " LDX", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::LDX, cycles: 3}, // A6
                    Instruction{name: "*LAX", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 3}, // A7
                    Instruction{name: " TAY", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TAY, cycles: 2}, // A8
                    Instruction{name: " LDA", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::LDA, cycles: 2}, // A9
                    Instruction{name: " TAX", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TAX, cycles: 2}, // AA
                    Instruction{name: "*LXA", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // AB
                    Instruction{name: " LDY", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::LDY, cycles: 4}, // AC
                    Instruction{name: " LDA", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::LDA, cycles: 4}, // AD
                    Instruction{name: " LDX", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::LDX, cycles: 4}, // AE
                    Instruction{name: "*LAX", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 4}, // AF

                    // Row B
                    Instruction{name: " BCS", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BCS, cycles: 2}, // B0
                    Instruction{name: " LDA", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::LDA, cycles: 5}, // B1
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // B2
                    Instruction{name: "*LAX", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 5}, // B3
                    Instruction{name: " LDY", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::LDY, cycles: 4}, // B4
                    Instruction{name: " LDA", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::LDA, cycles: 4}, // B5
                    Instruction{name: " LDX", addr_mode: ADDRESSING_MODES::ZPY, addr_mode_fn: Self::addr_ZPY, operate: Self::LDX, cycles: 4}, // B6
                    Instruction{name: "*LAX", addr_mode: ADDRESSING_MODES::ZPY, addr_mode_fn: Self::addr_ZPY, operate: Self::xxx, cycles: 4}, // B7
                    Instruction{name: " CLV", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::CLV, cycles: 2}, // B8
                    Instruction{name: " LDA", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::LDA, cycles: 4}, // B9
                    Instruction{name: " TSX", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::TSX, cycles: 2}, // BA
                    Instruction{name: "*LAS", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 4}, // BB
                    Instruction{name: " LDY", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::LDY, cycles: 4}, // BC
                    Instruction{name: " LDA", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::LDA, cycles: 4}, // BD
                    Instruction{name: " LDX", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::LDX, cycles: 4}, // BE
                    Instruction{name: "*LAX", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 4}, // BF

                    // Row C
                    Instruction{name: " CPY", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::CPY, cycles: 2}, // C0
                    Instruction{name: " CMP", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::CMP, cycles: 6}, // C1
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2}, // C2
                    Instruction{name: "*DCP", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // C3
                    Instruction{name: " CPY", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::CPY, cycles: 3}, // C4
                    Instruction{name: " CMP", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::CMP, cycles: 3}, // C5
                    Instruction{name: " DEC", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::DEC, cycles: 5}, // C6
                    Instruction{name: "*DCP", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // C7
                    Instruction{name: " INY", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::INY, cycles: 2}, // C8
                    Instruction{name: " CMP", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::CMP, cycles: 2}, // C9
                    Instruction{name: " DEX", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::DEX, cycles: 2}, // CA
                    Instruction{name: "*SBX", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::xxx, cycles: 2}, // CB
                    Instruction{name: " CPY", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::CPY, cycles: 4}, // CC
                    Instruction{name: " CMP", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::CMP, cycles: 4}, // CD
                    Instruction{name: " DEC", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::DEC, cycles: 6}, // CE
                    Instruction{name: "*DCP", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // CF

                    // Row D
                    Instruction{name: " BNE", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BNE, cycles: 2}, // D0
                    Instruction{name: " CMP", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::CMP, cycles: 5}, // D1
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // D2
                    Instruction{name: "*DCP", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // D3
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4}, // D4
                    Instruction{name: " CMP", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::CMP, cycles: 4}, // D5
                    Instruction{name: " DEC", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::DEC, cycles: 6}, // D6
                    Instruction{name: "*DCP", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // D7
                    Instruction{name: " CLD", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::CLD, cycles: 2}, // D8
                    Instruction{name: " CMP", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::CMP, cycles: 4}, // D9
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2}, // DA
                    Instruction{name: "*DCP", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // DB
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4}, // DC
                    Instruction{name: " CMP", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::CMP, cycles: 4}, // DD
                    Instruction{name: " DEC", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::DEC, cycles: 7}, // DE
                    Instruction{name: "*DCP", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // DF

                    // Row E
                    Instruction{name: " CPX", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::CPX, cycles: 2}, // E0
                    Instruction{name: " SBC", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::SBC, cycles: 6}, // E1
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::NOP, cycles: 2}, // E2
                    Instruction{name: "*ISC", addr_mode: ADDRESSING_MODES::IZX, addr_mode_fn: Self::addr_IZX, operate: Self::xxx, cycles: 8}, // E3
                    Instruction{name: " CPX", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::CPX, cycles: 3}, // E4
                    Instruction{name: " SBC", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::SBC, cycles: 3}, // E5
                    Instruction{name: " INC", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::INC, cycles: 5}, // E6
                    Instruction{name: "*ISC", addr_mode: ADDRESSING_MODES::ZP0, addr_mode_fn: Self::addr_ZP0, operate: Self::xxx, cycles: 5}, // E7
                    Instruction{name: " INX", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::INX, cycles: 2}, // E8
                    Instruction{name: " SBC", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::SBC, cycles: 2}, // E9
                    Instruction{name: " NOP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2}, // EA
                    Instruction{name: "USBC", addr_mode: ADDRESSING_MODES::IMM, addr_mode_fn: Self::addr_IMM, operate: Self::SBC, cycles: 2}, // EB
                    Instruction{name: " CPX", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::CPX, cycles: 4}, // EC
                    Instruction{name: " SBC", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::SBC, cycles: 4}, // ED
                    Instruction{name: " INC", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::INC, cycles: 6}, // EE
                    Instruction{name: "*ISC", addr_mode: ADDRESSING_MODES::ABS, addr_mode_fn: Self::addr_ABS, operate: Self::xxx, cycles: 6}, // EF

                    // Row F
                    Instruction{name: " BEQ", addr_mode: ADDRESSING_MODES::REL, addr_mode_fn: Self::addr_REL, operate: Self::BEQ, cycles: 2}, // F0
                    Instruction{name: " SBC", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::SBC, cycles: 5}, // F1
                    Instruction{name: "/!\\", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::xxx, cycles: 2}, // F2
                    Instruction{name: "*ISC", addr_mode: ADDRESSING_MODES::IZY, addr_mode_fn: Self::addr_IZY, operate: Self::xxx, cycles: 8}, // F3
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::NOP, cycles: 4}, // F4
                    Instruction{name: " SBC", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::SBC, cycles: 4}, // F5
                    Instruction{name: " INC", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::INC, cycles: 6}, // F6
                    Instruction{name: "*ISC", addr_mode: ADDRESSING_MODES::ZPX, addr_mode_fn: Self::addr_ZPX, operate: Self::xxx, cycles: 6}, // F7
                    Instruction{name: " SED", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::SED, cycles: 2}, // F8
                    Instruction{name: " SBC", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::SBC, cycles: 4}, // F9
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::IMP, addr_mode_fn: Self::addr_IMP, operate: Self::NOP, cycles: 2}, // FA
                    Instruction{name: "*ISC", addr_mode: ADDRESSING_MODES::ABY, addr_mode_fn: Self::addr_ABY, operate: Self::xxx, cycles: 7}, // FB
                    Instruction{name: "*NOP", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::NOP, cycles: 4}, // FC
                    Instruction{name: " SBC", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::SBC, cycles: 4}, // FD
                    Instruction{name: " INC", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::INC, cycles: 7}, // FE
                    Instruction{name: "*ISC", addr_mode: ADDRESSING_MODES::ABX, addr_mode_fn: Self::addr_ABX, operate: Self::xxx, cycles: 7}, // FF
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
                
                self.pc = self.pc.wrapping_add(1);
                self.cycles = self.lookup[self.opcode as usize].cycles;
                
                let bonus_cycles_addr_mode = (self.lookup[self.opcode as usize].addr_mode_fn)(self, bus);
                let bonus_cycles_operate = (self.lookup[self.opcode as usize].operate)(self, bus);
                self.cycles += bonus_cycles_addr_mode & bonus_cycles_operate;
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
