pub use cpu6502::Cpu6502;

pub mod cpu6502 {
    use crate::bus;
    
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

    #[derive(Debug, Default)]
    pub struct Cpu6502<'a> {
        bus: Option<bus::Bus<'a>>,
        a: u8,
        x: u8,
        y: u8,
        sp: u8,
        pc: u16,
        status: u8,
    }

    impl<'a> Cpu6502<'a> {
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
    }
}
