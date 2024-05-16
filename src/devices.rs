mod addressing_modes;
mod opcodes;

pub use cpu6502::Cpu6502;

pub mod cpu6502 {
    use crate::bus;

    #[derive(Debug)]
    pub struct Cpu6502<'a> {
        bus: Option<bus::Bus<'a>>,
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

        lookup: [Instruction<'a>; 80],
    }

    #[derive(Debug)]
    pub struct Instruction<'a> {
        pub name: &'static str,
        pub cycles: u8,
        pub addr_mode: fn(&'a Cpu6502<'a>) -> u8,
        pub operate: fn(&'a mut Cpu6502<'a>) -> u8,
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

    impl<'a> Cpu6502<'a> {
        pub fn new() -> Self {
            Self {
                bus: None,
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
                ]
            }
        }

        fn connect_bus(&mut self, bus: bus::Bus<'a>) {
            self.bus = Some(bus);
        }

        fn write(&mut self, a: u16) {
            if let Some(bus) = &mut self.bus {
                bus.write(a, 0);
            }
        }
    
        fn read(&self, a: u16, d: u8) {
            if let Some(bus) = &self.bus {
                bus.read(a, false);
            }
        }

        fn get_flag(&self, f: Flags) -> u8 { todo!("get_flag") }
        
        fn set_flag(&self, f: Flags, v: bool) { todo!("set_flag") }

        fn fetch(&mut self) { todo!("fetch"); }

        /// Handle cycles
        fn clock(&self) { todo!("clock"); }

        /// Reset signal
        fn reset(&self) { todo!("reset"); }

        /// Interrupt request signal
        fn irq(&self) { todo!("irq"); }

        /// Non-maskable interrupt request signal
        fn nmi(&self) { todo!("nmi"); }

    }
}
