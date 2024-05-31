mod devices;
mod bus;

pub use bus::Bus;
pub use devices::cpu6502::{Cpu6502, Flags};

#[derive(Debug)]
pub struct Nes {
    cpu: devices::cpu6502::Cpu6502,
    bus: bus::Bus,
}

pub struct CpuInfo {
    pub program_counter: u16,
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub stack_pointer: u8,
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

    pub fn cpu_tick(&mut self) {
        self.cpu.clock(&mut self.bus);
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
        }
    }

    pub fn get_cpu_flags(&self) -> u8 {
        self.cpu.status
    }

    pub fn get_next_instruction_string(&self) -> String {
        let opcode = self.cpu.read(&self.bus, self.cpu.pc);
        let instruction = &self.cpu.lookup[opcode as usize];
        let mut instruction_string = String::new();
        
        // if instruction.addr_mode as usize != devices::cpu6502::Cpu6502::addr_IMP as usize {
        //     let data = self.cpu.read(&self.bus, self.cpu.pc + 1);
        //     instruction_string.push_str(&format!(" {:02X}", data));
        // }

        match instruction.addr_mode as usize {
            mode if mode == devices::cpu6502::Cpu6502::addr_ACC as usize => {
                instruction_string.push_str(&format!("{opcode:02X} ({}) {}", instruction.name, self.cpu.a));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_IMP as usize => {
                instruction_string.push_str(&format!("{opcode:02X} {}", instruction.name));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_IMM as usize => {
                let data = self.cpu.read(&self.bus, self.cpu.pc + 1);
                instruction_string.push_str(&format!("{opcode:02X} {} #${data:02X}", instruction.name ));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_ABS as usize => {
                let lo = self.cpu.read(&self.bus, self.cpu.pc + 1);
                let hi = self.cpu.read(&self.bus, self.cpu.pc + 2);
                let addr = (hi as u16) << 8 | lo as u16;
                instruction_string.push_str(&format!("{:02X} {} ${:04X}", opcode, instruction.name, addr));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_ABSx as usize => {
                let lo = self.cpu.read(&self.bus, self.cpu.pc + 1);
                let hi = self.cpu.read(&self.bus, self.cpu.pc + 2);
                let addr = (hi as u16) << 8 | lo as u16;
                instruction_string.push_str(&format!("{:02X} {} ${:04X},X", opcode, instruction.name, addr));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_ABSy as usize => {
                let lo = self.cpu.read(&self.bus, self.cpu.pc + 1);
                let hi = self.cpu.read(&self.bus, self.cpu.pc + 2);
                let addr = (hi as u16) << 8 | lo as u16;
                instruction_string.push_str(&format!("{:02X} {} ${:04X},Y", opcode, instruction.name, addr));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_ZPG as usize => {
                let addr = self.cpu.read(&self.bus, self.cpu.pc + 1);
                instruction_string.push_str(&format!("{:02X} {} ${:02X}", opcode, instruction.name, addr));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_ZPGx as usize => {
                let addr = self.cpu.read(&self.bus, self.cpu.pc + 1);
                instruction_string.push_str(&format!("{:02X} {} ${:02X},X", opcode, instruction.name, addr));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_ZPGy as usize => {
                let addr = self.cpu.read(&self.bus, self.cpu.pc + 1);
                instruction_string.push_str(&format!("{:02X} {} ${:02X},Y", opcode, instruction.name, addr));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_REL as usize => {
                let addr = self.cpu.read(&self.bus, self.cpu.pc + 1);
                instruction_string.push_str(&format!("{:02X} {} ${:02X}", opcode, instruction.name, addr));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_IND as usize => {
                let lo = self.cpu.read(&self.bus, self.cpu.pc + 1);
                let hi = self.cpu.read(&self.bus, self.cpu.pc + 2);
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
                instruction_string.push_str(&format!("{:02X} {} (${:04X})", opcode, instruction.name, addr));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_INDx as usize => {
                let addr = self.cpu.read(&self.bus, self.cpu.pc + 1);
                let lo = self.cpu.read(&self.bus, (addr + self.cpu.x) as u16);
                let hi = self.cpu.read(&self.bus, (addr + self.cpu.x + 1) as u16);
                let ptr = (hi as u16) << 8 | lo as u16;
                instruction_string.push_str(&format!("{:02X} {} (${:02X},X) @ {:02X} = {:04X}", opcode, instruction.name, addr, addr + self.cpu.x, ptr));
            }
            mode if mode == devices::cpu6502::Cpu6502::addr_INDy as usize => {
                let addr = self.cpu.read(&self.bus, self.cpu.pc + 1);
                let lo = self.cpu.read(&self.bus, addr as u16);
                let hi = self.cpu.read(&self.bus, (addr + 1) as u16);
                let ptr = (hi as u16) << 8 | lo as u16;
                instruction_string.push_str(&format!("{:02X} {} (${:02X}),Y = {:04X}", opcode, instruction.name, addr, ptr + self.cpu.y as u16));
            }
            _ => {
                instruction_string.push_str(&format!("{:02X} {}", opcode, instruction.name));
            }
        }

        instruction_string
    }
}
