mod cartridge;
mod mappers;
mod cpu;
mod ppu;
mod bus;

pub use cartridge::ComponentCartridge;
pub use mappers::Mapper;
pub use cpu::{Component6502, Flags, ADDRESSING_MODES};
pub use ppu::{Component2C02, ScreenData};
pub use bus::Bus;

use raylib::color::Color;
use crate::constants::*;

#[cfg(feature = "nestest")]
pub struct Snapshot {
    cpu: Component6502,
    ppu: Component2C02,
    bus: bus::Bus,
}

#[derive(Debug)]
pub struct Nes {
    cartridge: ComponentCartridge,
    cpu: Component6502,
    ppu: Component2C02,
    bus: bus::Bus,

    screen: ScreenData,
    pub pause: bool,
    pub timer: f32,
    pub total_clock_ticks: u128,
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
            cartridge: ComponentCartridge::new(),
            cpu: Component6502::new(),
            ppu: Component2C02::new(),
            bus: bus::Bus {
                ram: [0; 64 * 1024],
            },

            screen: ScreenData::new(),
            pause: true,
            timer: 0.0,
            total_clock_ticks: 0,
        }
    }
    
    #[allow(dead_code)]
    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        self.cpu.read(addr, &mut self.cartridge, &mut self.ppu, &self.bus)
    }

    #[allow(dead_code)]
    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        self.cpu.write(addr, data, &mut self.cartridge, &mut self.ppu, &mut self.bus);
    }

    pub fn load_cartridge(&mut self, path: &str) {
        self.cartridge = ComponentCartridge::from_path(path);
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.cartridge, &mut self.ppu, &self.bus);
        self.total_clock_ticks = 0;
    }

    pub fn tick(&mut self) {
        #[cfg(feature = "nestest")]
        let snapshot = Snapshot {
            cpu: self.cpu,
            bus: self.bus,
            ppu: self.ppu,
        };

        #[cfg(feature = "nestest")]
        let display_log = self.cpu.cycles == 0;

        self.ppu.tick(&mut self.screen);
        if self.total_clock_ticks % 3 == 0 {
            self.cpu.tick(&mut self.cartridge, &mut self.ppu, &mut self.bus);
        }
        self.total_clock_ticks += 1;
        
        #[cfg(feature = "nestest")]
        if display_log {
            Self::nestest_format_log(&snapshot);
        }
    }

    #[cfg(feature = "nestest")]
    pub fn nestest_format_log(snapshot: &Snapshot) {
        use std::fmt::Write;
        
        let mut line = String::new();

        let instruction = &snapshot.cpu.lookup[snapshot.cpu.read(&snapshot.bus, snapshot.cpu.pc) as usize];

        // opcode
        write!(&mut line, "{:04X}  ", snapshot.cpu.pc).unwrap();
        // operands
        let operands_nb = instruction.addr_mode.get_operands_nb();
        let operands = (0..=operands_nb).map(|i| snapshot.cpu.read(&snapshot.bus, snapshot.cpu.pc.wrapping_add(i as u16)));
        for operand in operands.clone() {
            write!(&mut line, "{operand:02X} ").unwrap();
        }
        match operands_nb {
            0 => write!(&mut line, "      ").unwrap(),
            1 => write!(&mut line, "   ").unwrap(),
            2 => write!(&mut line, "").unwrap(),
            _ => {}
        }
        // instruction name
        write!(&mut line, "{} ", instruction.name).unwrap();
        // detailed operands
        let operands_vec = operands.collect::<Vec<u8>>();
        write!(&mut line, "{}", instruction.addr_mode.format_operands(&operands_vec, &snapshot.bus.ram, operands_vec[0], snapshot.cpu.pc, snapshot.cpu.x, snapshot.cpu.y)).unwrap();
        write!(&mut line, "  ").unwrap();
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

        println!("{line}");
    }

    pub const fn is_current_tick_cpu(&self) -> bool {
        self.cpu.cycles == 0
    }

    pub const fn is_cpu_instruction_complete(&self) -> bool {
        self.cpu.cycles == 0
    }

    pub const fn is_ppu_frame_complete(&self) -> bool {
        self.ppu.is_frame_complete
    }

    pub fn set_ppu_frame_complete(&mut self, value: bool) {
        self.ppu.is_frame_complete = value;
    }

    pub fn get_ram(&self, low: u16, high: u16) -> (u16, u16, &[u8]) {
        if low > high {
            return (low, high, &[]); // Otherwise taking the slice panics
        }
        (low, high, &self.bus.ram[low as usize..high as usize])
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

    pub const fn get_screen(&self) -> &[Color; NES_SCREEN_WIDTH as usize * NES_SCREEN_HEIGHT as usize] {
        &self.screen.displayable_screen
    }

    pub fn get_pattern_table(&mut self, index: u8) -> &[Color; 128 * 128] {
        self.ppu.fill_pattern_table(index, &mut self.screen, &self.cartridge);

        &self.screen.displayable_pattern_table[index as usize]
    }

    pub fn get_instruction_string_range(&mut self, start: u16, end: u16) -> Vec<String> {
        let count = start;
        let mut instruction_string = Vec::new();

        let mut local_pc = start;
        for _ in count..end {
            let opcode = self.cpu.read(local_pc, &mut self.cartridge, &mut self.ppu, &self.bus);
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
                    let data = self.cpu.read(local_pc.wrapping_add(1), &mut self.cartridge, &mut self.ppu, &self.bus);

                    instruction_string.push(format!("{opcode:02X} (IMM) {} #${data:02X}", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::ABS => {
                    let lo = self.cpu.read(local_pc.wrapping_add(1), &mut self.cartridge, &mut self.ppu, &self.bus);
                    let hi = self.cpu.read(local_pc.wrapping_add(2), &mut self.cartridge, &mut self.ppu, &self.bus);
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} (ABS) {} ${addr:04X}", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                ADDRESSING_MODES::ABX => {
                    let lo = self.cpu.read(local_pc.wrapping_add(1), &mut self.cartridge, &mut self.ppu, &self.bus);
                    let hi = self.cpu.read(local_pc.wrapping_add(2), &mut self.cartridge, &mut self.ppu, &self.bus);
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} (ABSx) {} ${addr:04X}, X", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                ADDRESSING_MODES::ABY => {
                    let lo = self.cpu.read(local_pc.wrapping_add(1), &mut self.cartridge, &mut self.ppu, &self.bus);
                    let hi = self.cpu.read(local_pc.wrapping_add(2), &mut self.cartridge, &mut self.ppu, &self.bus);
                    let addr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} ${addr:04X}, Y", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                ADDRESSING_MODES::ZP0 => {
                    let addr = self.cpu.read(local_pc.wrapping_add(1), &mut self.cartridge, &mut self.ppu, &self.bus);

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::ZPX => {
                    let addr = self.cpu.read(local_pc.wrapping_add(1), &mut self.cartridge, &mut self.ppu, &self.bus);

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}, X", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::ZPY => {
                    let addr = self.cpu.read(local_pc.wrapping_add(1), &mut self.cartridge, &mut self.ppu, &self.bus);

                    instruction_string.push(format!("{opcode:02X} {} ${addr:02X}, Y", instruction.name));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::REL => {
                    let addr = self.cpu.read(local_pc.wrapping_add(1), &mut self.cartridge, &mut self.ppu, &self.bus);

                    instruction_string.push(format!("{opcode:02X} (REL) {} ${addr:02X} [{:04X}]", instruction.name, local_pc.wrapping_add(2).wrapping_add(addr as u16)));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::IND => {
                    let lo = self.cpu.read(local_pc + 1, &mut self.cartridge, &mut self.ppu, &self.bus);
                    let hi = self.cpu.read(local_pc + 2, &mut self.cartridge, &mut self.ppu, &self.bus);
                    let ptr = (hi as u16) << 8 | lo as u16;
                    let addr = if lo == 0xFF {
                        (self.cpu.read(ptr & 0xFF00, &mut self.cartridge, &mut self.ppu, &self.bus) as u16) | (self.cpu.read(ptr, &mut self.cartridge, &mut self.ppu, &self.bus) as u16) << 8
                    } else {
                        (self.cpu.read(ptr + 1, &mut self.cartridge, &mut self.ppu, &self.bus) as u16) << 8 | self.cpu.read(ptr, &mut self.cartridge, &mut self.ppu, &self.bus) as u16
                    };

                    instruction_string.push(format!("{opcode:02X} {} (${addr:04X})", instruction.name));
                    local_pc = local_pc.wrapping_add(3);
                }
                ADDRESSING_MODES::IZX => {
                    let addr = self.cpu.read(local_pc + 1, &mut self.cartridge, &mut self.ppu, &self.bus) as u16;
                    let lo = self.cpu.read((addr + self.cpu.x as u16) & 0x00FF, &mut self.cartridge, &mut self.ppu, &self.bus);
                    let hi = self.cpu.read((addr + self.cpu.x as u16 + 1) & 0x00FF, &mut self.cartridge, &mut self.ppu, &self.bus);
                    let ptr = (hi as u16) << 8 | lo as u16;

                    instruction_string.push(format!("{opcode:02X} {} (${:02X}, X) @ {:02X} = {ptr:04X}", instruction.name, addr, addr + self.cpu.x as u16));
                    local_pc = local_pc.wrapping_add(2);
                }
                ADDRESSING_MODES::IZY => {
                    let addr = self.cpu.read(local_pc + 1, &mut self.cartridge, &mut self.ppu, &self.bus) as u16;
                    let lo = self.cpu.read(addr & 0x00FF, &mut self.cartridge, &mut self.ppu, &self.bus);
                    let hi = self.cpu.read((addr + 1) & 0x00FF, &mut self.cartridge, &mut self.ppu, &self.bus);
                    let mut ptr = (hi as u16) << 8 | lo as u16;
                    ptr = ptr.wrapping_add(self.cpu.y as u16);

                    instruction_string.push(format!("{opcode:02X} {} (${addr:02X}), Y = {:04X}", instruction.name, ptr + self.cpu.y as u16));
                    local_pc = local_pc.wrapping_add(2);
                }
            }
        }

        instruction_string
    }
}
