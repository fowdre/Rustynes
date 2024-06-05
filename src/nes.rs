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
            
            match instruction.addr_mode as usize {
                mode if mode == devices::cpu6502::Cpu6502::addr_ACC as usize => {
                    instruction_string.push(format!("{opcode:02X} (ACC) {}", instruction.name));
                    local_pc = local_pc.wrapping_add(1);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_IMP as usize => {
                    instruction_string.push(format!("{opcode:02X} (IMP) {}", instruction.name));
                    local_pc = local_pc.wrapping_add(1);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_IMM as usize => {
                    let data = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} (IMM) {} #${data:02X}", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_ABS as usize => {
                    let lo = self.cpu.read(&self.bus, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&self.bus, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} (ABS) {} ${addr:04X}", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_ABX as usize => {
                    let lo = self.cpu.read(&self.bus, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&self.bus, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} (ABSx) {} ${addr:04X}, X", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_ABY as usize => {
                    let lo = self.cpu.read(&self.bus, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&self.bus, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} ${addr:04X}, Y", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_ZP0 as usize => {
                    let addr = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_ZPX as usize => {
                    let addr = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}, X", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_ZPY as usize => {
                    let addr = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}, Y", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_REL as usize => {
                    let addr = self.cpu.read(&self.bus, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} (REL) {} ${addr:02X} [{:04X}]", instruction.name, local_pc.wrapping_add(2).wrapping_add(addr as u16)));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_IND as usize => {
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
                mode if mode == devices::cpu6502::Cpu6502::addr_INDx as usize => {
                    let addr = self.cpu.read(&self.bus, local_pc + 1);
                    let lo = self.cpu.read(&self.bus, (addr + self.cpu.x) as u16);
                    let hi = self.cpu.read(&self.bus, (addr + self.cpu.x + 1) as u16);
                    let ptr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} (${:02X}, X) @ {:02X} = {ptr:04X}", instruction.name, addr, addr + self.cpu.x));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == devices::cpu6502::Cpu6502::addr_INDy as usize => {
                    let addr = self.cpu.read(&self.bus, local_pc + 1);
                    let lo = self.cpu.read(&self.bus, addr as u16);
                    let hi = self.cpu.read(&self.bus, (addr + 1) as u16);
                    let ptr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} (${addr:02X}), Y = {:04X}", instruction.name, ptr + self.cpu.y as u16));
                    local_pc = local_pc.wrapping_add(2);
                }
                _ => {
                    instruction_string.push("Unknown instruction".to_string());
                }
            }
        }

        instruction_string
    }
}
