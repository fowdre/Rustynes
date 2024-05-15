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

        lookup: [Instruction<'a>; 1],
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
                    Instruction { name: "XXX", cycles: 0, addr_mode: Self::addr_IMP, operate: Self::XXX }
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
