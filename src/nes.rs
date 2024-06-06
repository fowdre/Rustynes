mod devices;
mod bus;

pub use bus::Bus;
pub use devices::cpu6502::{Cpu6502, Flags, ADDRESSING_MODES};

pub struct NesSnapshot {
    cpu: devices::cpu6502::Cpu6502,
    bus: bus::Bus,
}

impl NesSnapshot {
    pub fn update_internals(&mut self, cpu: &Cpu6502) {
        self.cpu.a = cpu.a;
        self.cpu.x = cpu.x;
        self.cpu.y = cpu.y;
        self.cpu.sp = cpu.sp;
    }
}

#[derive(Debug)]
pub struct Nes {
    pub cpu: devices::cpu6502::Cpu6502,
    bus: bus::Bus,
}

pub struct CpuInfo {
    pub program_counter: u16,
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub stack_pointer: u8,
    pub cycles: u8,
}

impl Nes {
    pub fn new() -> Self {
        Self {
            cpu: devices::cpu6502::Cpu6502::new(),
            bus: bus::Bus {
                ram: [0; 64 * 1024],
            },
        }
    }
    
    #[allow(dead_code)]
    pub const fn cpu_read(&self, addr: u16) -> u8 {
        self.cpu.read(&self.bus, addr)
    }

    #[allow(dead_code)]
    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        self.cpu.write(&mut self.bus, addr, data);
    }

    pub fn nestest_format_log(&mut self, snapshot: &NesSnapshot) {
        use std::fmt::Write;
        
        let mut line = String::new();

        let instruction = &snapshot.cpu.lookup[snapshot.cpu.read(&snapshot.bus, snapshot.cpu.pc) as usize];

        // opcode
        write!(&mut line, "{:04X}  ", snapshot.cpu.pc).unwrap();
        // operands
        let operands_nb = instruction.addr_mode.get_operands_nb();
        let operands = (0..operands_nb + 1).map(|i| snapshot.cpu.read(&snapshot.bus, snapshot.cpu.pc + i as u16));
        for operand in operands.clone() {
            write!(&mut line, "{:02X} ", operand).unwrap();
        }
        match operands_nb {
            0 => write!(&mut line, "       ").unwrap(),
            1 => write!(&mut line, "    ").unwrap(),
            2 => write!(&mut line, " ").unwrap(),
            _ => {}
        }
        // instruction name
        write!(&mut line, "{} ", instruction.name).unwrap();
        // detailed operands
        match instruction.addr_mode {
            ADDRESSING_MODES::ACC => todo!("ACC"),
            ADDRESSING_MODES::ZPX => todo!("ZPX"),
            ADDRESSING_MODES::ZPY => todo!("ZPY"),
            ADDRESSING_MODES::ABX => todo!("ABX"),
            ADDRESSING_MODES::ABY => todo!("ABY"),
            ADDRESSING_MODES::IND => todo!("IND"),
            ADDRESSING_MODES::IZX => todo!("IZX"),
            ADDRESSING_MODES::IZY => todo!("IZY"),
            _ => {}
        }
        write!(&mut line, "{}", instruction.addr_mode.format_operands(&operands.collect::<Vec<u8>>(), snapshot.cpu.pc)).unwrap();
        write!(&mut line, "                    ").unwrap();
        // accumulator
        write!(&mut line, "A:{:02X} ", snapshot.cpu.a).unwrap();
        // x register
        write!(&mut line, "X:{:02X} ", snapshot.cpu.x).unwrap();
        // y register
        write!(&mut line, "Y:{:02X} ", snapshot.cpu.y).unwrap();
        // flags
        write!(&mut line, "P:{:02X} ", snapshot.cpu.status).unwrap();
        // stack pointer
        write!(&mut line, "SP:{:02X}", snapshot.cpu.sp).unwrap();

        println!("{}", line);
    }

    pub fn cpu_tick(&mut self) {
        
        let mut snapshot = NesSnapshot {
            cpu: self.cpu,
            bus: self.bus,
        };

        let mut display_nestest = false;
        if self.cpu.cycles == 0 {
            display_nestest = true;
        }
        
        // dbg!(self.cpu.status);
        self.cpu.clock(&mut self.bus);
        // dbg!(self.cpu.status);

        if display_nestest {
            snapshot.update_internals(&self.cpu);
            self.nestest_format_log(&snapshot);
        }
            
    }

    pub fn reset(&mut self) {
        // self.cpu.reset(&mut self.bus);
        self.cpu.pc = 0xC000;
        self.cpu.sp = 0xFD;
        self.cpu.status = 0;
        self.cpu.set_flag(Flags::U, true);
        self.cpu.set_flag(Flags::I, true);
    }

    pub fn get_ram(&self, low: u16, high: u16) -> (u16, u16, &[u8]) {
        if low > high {
            return (low, high, &[]); // Otherwise taking the slice panics
        }
        (low, high, &self.bus.ram[low as usize..high as usize])
    }

    pub fn get_cpu_info(&self) -> CpuInfo {
        CpuInfo {
            program_counter: self.cpu.pc,
            reg_a: self.cpu.a,
            reg_x: self.cpu.x,
            reg_y: self.cpu.y,
            stack_pointer: self.cpu.sp,
            cycles: self.cpu.cycles,
        }
    }

    pub fn get_cpu_flags(&self) -> u8 {
        self.cpu.status
    }

    pub fn get_instruction_string_range(&self, start: u16, end: u16) -> Vec<String> {
        let count = start;
        let mut instruction_string = Vec::new();

        let mut local_pc = start;
        for _ in count..end {
            let opcode = self.cpu.read(&self.bus, local_pc);
            let instruction = &self.cpu.lookup[opcode as usize];
            
            match instruction.addr_mode {
                ADDRESSING_MODES::ACC => {
                    instruction_string.push(format!("{opcode:02X} (ACC) {}", instruction.name));
                    local_pc = local_pc.wrapping_add(1);
                }
                ADDRESSING_MODES::IMP => {
                    instruction_string.push(format!("{opcode:02X} (IMP) {}", instruction.name));
                    local_pc = local_pc.wrapping_add(1);
                }
                ADDRESSING_MODES::IMM => {
                    let data = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} (IMM) {} #${data:02X}", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::ABS => {
                    let lo = self.cpu.read(&self.bus, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&self.bus, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} (ABS) {} ${addr:04X}", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                ADDRESSING_MODES::ABX => {
                    let lo = self.cpu.read(&self.bus, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&self.bus, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} (ABSx) {} ${addr:04X}, X", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                ADDRESSING_MODES::ABY => {
                    let lo = self.cpu.read(&self.bus, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&self.bus, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} ${addr:04X}, Y", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                ADDRESSING_MODES::ZP0 => {
                    let addr = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::ZPX => {
                    let addr = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}, X", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::ZPY => {
                    let addr = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}, Y", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::REL => {
                    let addr = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} (REL) {} ${addr:02X} [{:04X}]", instruction.name, local_pc.wrapping_add(2).wrapping_add(addr as u16)));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::IND => {
                    let lo = self.cpu.read(&self.bus, local_pc + 1);
                    let hi = self.cpu.read(&self.bus, local_pc + 2);
                    let ptr = (hi as u16) << 8 | lo as u16;
                    let addr = if lo == 0xFF {
                        let lo = self.cpu.read(&self.bus, ptr);
                        let hi = self.cpu.read(&self.bus, ptr & 0xFF00);
                        (hi as u16) << 8 | lo as u16
                    } else {
                        let lo = self.cpu.read(&self.bus, ptr);
                        let hi = self.cpu.read(&self.bus, ptr + 1);
                        (hi as u16) << 8 | lo as u16
                    };

                    instruction_string.push(format!("{opcode:02X} {} (${addr:04X})", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                ADDRESSING_MODES::IZX => {
                    let addr = self.cpu.read(&self.bus, local_pc + 1);
                    let lo = self.cpu.read(&self.bus, (addr + self.cpu.x) as u16);
                    let hi = self.cpu.read(&self.bus, (addr + self.cpu.x + 1) as u16);
                    let ptr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} (${:02X}, X) @ {:02X} = {ptr:04X}", instruction.name, addr, addr + self.cpu.x));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::IZY => {
                    let addr = self.cpu.read(&self.bus, local_pc + 1);
                    let lo = self.cpu.read(&self.bus, addr as u16);
                    let hi = self.cpu.read(&self.bus, (addr + 1) as u16);
                    let ptr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} (${addr:02X}), Y = {:04X}", instruction.name, ptr + self.cpu.y as u16));
                    local_pc = local_pc.wrapping_add(2);
                }
            }
        }

        instruction_string
    }
}
