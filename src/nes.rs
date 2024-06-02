mod cartridge;
mod mappers;
mod ppu;
mod cpu;
mod bus;

pub use cartridge::ncartridge::Cartridge;
pub use bus::Bus;
pub use cpu::cpu6502::{Cpu6502, Flags};

#[derive(Debug)]
pub struct Nes {
    cartridge: Cartridge,
    cpu: cpu::cpu6502::Cpu6502,
    ppu: ppu::ppu2c02::Ppu2C02,
    bus: bus::Bus,

    total_clock_ticks: u128,

    pub pause: bool,
    pub is_a_cpu_tick: bool,
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
            cartridge: Cartridge::default(),
            cpu: cpu::cpu6502::Cpu6502::new(),
            ppu: ppu::ppu2c02::Ppu2C02::new(),
            bus: bus::Bus::new(),
            total_clock_ticks: 0,
            
            pause: true,
            is_a_cpu_tick: false,
        }
    }

    pub fn load_cartridge(&mut self, path: &str) {
        self.cartridge = Cartridge::from_path(path);
    }
    
    pub fn ram_read(&self, addr: u16) -> u8 {
        self.cpu.read(&self.bus, &self.cartridge, addr)
    }

    pub fn ram_write(&mut self, addr: u16, data: u8) {
        self.cpu.write(&mut self.bus, &mut self.cartridge, addr, data);
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&self.bus, &self.cartridge);
        self.total_clock_ticks = 0;
    }

    pub fn tick(&mut self) {
        self.ppu.clock();
        self.is_a_cpu_tick = false;
        if self.total_clock_ticks % 3 == 0 {
            self.is_a_cpu_tick = true;
            self.cpu.clock(&mut self.bus, &mut self.cartridge);
        }
        self.total_clock_ticks = self.total_clock_ticks.wrapping_add(1);
    }

    // Draw helper methods

    pub fn get_ram(&self, low: u16, high: u16) -> (u16, u16, &[u8]) {
        if low > high {
            return (low, high, &[]); // Otherwise taking the slice panics
        }
        (low, high, &self.bus.cpu_ram[(low & 0x07FF) as usize..(high & 0x07FF) as usize])
    }

    pub const fn get_cpu_info(&self) -> CpuInfo {
        CpuInfo {
            program_counter: self.cpu.pc,
            reg_a: self.cpu.a,
            reg_x: self.cpu.x,
            reg_y: self.cpu.y,
            stack_pointer: self.cpu.sp,
            cycles: self.cpu.cycles,
        }
    }

    pub const fn get_cpu_flags(&self) -> u8 {
        self.cpu.status
    }

    pub fn is_cpu_instruction_complete(&self) -> bool {
        self.cpu.cycles == 0
    }

    pub fn is_ppu_frame_complete(&self) -> bool {
        self.ppu.is_frame_complete
    }

    pub fn set_ppu_frame_complete(&mut self, value: bool) {
        self.ppu.is_frame_complete = value;
    }

    pub fn get_ppu_screen(&self) -> &[ppu::ppu2c02::Color] {
        &self.ppu.screen
    }

    pub fn get_ppu_name_table(&self, index: usize) -> &[u8] {
        &self.ppu.table_name[index]
    }

    pub fn get_ppu_pattern_table(&self, index: usize) -> &[u8] {
        &self.ppu.table_pattern[index]
    }

    pub fn get_ppu_pallete(&self) -> &[u8] {
        &self.ppu.table_pallete
    }

    pub fn get_instruction_string_range(&self, start: u16, end: u16) -> Vec<String> {
        let count = start;
        let mut instruction_string = Vec::new();

        let mut local_pc = start;
        for _ in count..end {
            let opcode = self.cpu.read(&self.bus, &self.cartridge, local_pc);
            let instruction = &self.cpu.lookup[opcode as usize];
            
            match instruction.addr_mode as usize {
                mode if mode == cpu::cpu6502::Cpu6502::addr_ACC as usize => {
                    instruction_string.push(format!("{opcode:02X} (ACC) {}", instruction.name));
                    local_pc = local_pc.wrapping_add(1);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_IMP as usize => {
                    instruction_string.push(format!("{opcode:02X} (IMP) {}", instruction.name));
                    local_pc = local_pc.wrapping_add(1);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_IMM as usize => {
                    let data = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} (IMM) {} #${data:02X}", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_ABS as usize => {
                    let lo = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} (ABS) {} ${addr:04X}", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_ABSx as usize => {
                    let lo = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} (ABSx) {} ${addr:04X}, X", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_ABSy as usize => {
                    let lo = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} ${addr:04X}, Y", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_ZPG as usize => {
                    let addr = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_ZPGx as usize => {
                    let addr = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}, X", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_ZPGy as usize => {
                    let addr = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}, Y", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_REL as usize => {
                    let addr = self.cpu.read(&self.bus, &self.cartridge, local_pc.wrapping_add(1));

                    instruction_string.push(format!("{opcode:02X} (REL) {} ${addr:02X} [{:04X}]", instruction.name, local_pc.wrapping_add(2).wrapping_add(addr as u16)));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_IND as usize => {
                    let old_lo = self.cpu.read(&self.bus, &self.cartridge, local_pc + 1);
                    let hi = self.cpu.read(&self.bus, &self.cartridge, local_pc + 2);
                    let ptr = (hi as u16) << 8 | old_lo as u16;
                    let lo = self.cpu.read(&self.bus, &self.cartridge, ptr);
                    let addr = if old_lo == 0xFF {
                        let hi = self.cpu.read(&self.bus, &self.cartridge, ptr & 0xFF00);
                        (hi as u16) << 8 | lo as u16
                    } else {
                        let hi = self.cpu.read(&self.bus, &self.cartridge, ptr + 1);
                        (hi as u16) << 8 | lo as u16
                    };

                    instruction_string.push(format!("{opcode:02X} {} (${addr:04X})", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_INDx as usize => {
                    let addr = self.cpu.read(&self.bus, &self.cartridge, local_pc + 1);
                    let lo = self.cpu.read(&self.bus, &self.cartridge, (addr + self.cpu.x) as u16);
                    let hi = self.cpu.read(&self.bus, &self.cartridge, (addr + self.cpu.x + 1) as u16);
                    let ptr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} (${:02X}, X) @ {:02X} = {ptr:04X}", instruction.name, addr, addr + self.cpu.x));
                    local_pc = local_pc.wrapping_add(2);
                }
                mode if mode == cpu::cpu6502::Cpu6502::addr_INDy as usize => {
                    let addr = self.cpu.read(&self.bus, &self.cartridge, local_pc + 1);
                    let lo = self.cpu.read(&self.bus, &self.cartridge, addr as u16);
                    let hi = self.cpu.read(&self.bus, &self.cartridge, (addr + 1) as u16);
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
