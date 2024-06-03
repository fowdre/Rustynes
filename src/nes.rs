mod cartridge;
mod mappers;
mod ppu;
mod cpu;
mod bus;

pub use cartridge::ncartridge::Cartridge;
pub use cpu::cpu6502::{Cpu6502, Flags};
pub use ppu::ppu2c02::Ppu2C02;
pub use bus::Bus;

#[derive(Debug)]
pub struct Nes {
    cartridge: Cartridge,
    cpu: Cpu6502,
    ppu: Ppu2C02,
    bus: Bus,
    selected_palette: u8,

    total_clock_ticks: u128,

    pub time: f32,
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
            cpu: Cpu6502::new(),
            ppu: Ppu2C02::new(),
            bus: Bus::new(),
            total_clock_ticks: 0,
            
            time: 0.0,
            pause: true,
            is_a_cpu_tick: false,
            selected_palette: 0,
        }
    }

    pub fn load_cartridge(&mut self, path: &str) {
        self.cartridge = Cartridge::from_path(path);
    }

    pub fn cycle_palette(&mut self) {
        self.selected_palette = (self.selected_palette + 1) & 0x07;
    }
    
    #[allow(dead_code)]
    pub fn ram_read(&mut self, addr: u16) -> u8 {
        self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, addr)
    }

    #[allow(dead_code)]
    pub fn ram_write(&mut self, addr: u16, data: u8) {
        self.cpu.write(&mut self.cartridge, &mut self.ppu, &mut self.bus, addr, data);
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.cartridge, &mut self.ppu, &mut self.bus);
        self.total_clock_ticks = 0;
    }

    pub fn tick(&mut self) {
        self.ppu.clock();
        self.is_a_cpu_tick = false;
        if self.total_clock_ticks % 3 == 0 {
            self.is_a_cpu_tick = true;
            self.cpu.clock(&mut self.cartridge, &mut self.ppu, &mut self.bus);
        }

        if self.ppu.nmi {
            self.ppu.nmi = false;
            self.cpu.nmi(&mut self.cartridge, &mut self.ppu, &mut self.bus);
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

    pub const fn is_cpu_instruction_complete(&self) -> bool {
        self.cpu.cycles == 0
    }

    pub const fn is_ppu_frame_complete(&self) -> bool {
        self.ppu.is_frame_complete
    }

    pub fn set_ppu_frame_complete(&mut self, value: bool) {
        self.ppu.is_frame_complete = value;
    }

    pub const fn get_ppu_screen(&self) -> &[ppu::ppu2c02::Color] {
        self.ppu.get_screen()
    }

    pub const fn get_ppu_name_table(&self, index: usize) -> &[u8] {
        &self.ppu.table_name[index]
    }

    pub fn get_ppu_pattern_table(&mut self, index: u8) -> &[ppu::ppu2c02::Color] {
        self.ppu.get_pattern_table(&self.cartridge, index, self.selected_palette)
    }

    #[allow(dead_code)]
    pub const fn get_ppu_pallete(&self) -> &[u8] {
        &self.ppu.table_pallete
    }

    pub fn get_instruction_string_range(&mut self, start: u16, end: u16) -> Vec<String> {
        let count = start;
        let mut instruction_string = Vec::new();

        let mut local_pc = start;
        for _ in count..end {
            let opcode = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc);
            let instruction = &self.cpu.lookup[opcode as usize];
            let mut format = format!("{local_pc:04X} | {opcode:02X} ");

            match instruction.addr_mode {
                cpu::cpu6502::ADDRESSING_MODES::ACC => {
                    format += &format!("(ACC) {}", instruction.name);
                    local_pc = local_pc.wrapping_add(1);
                }
                cpu::cpu6502::ADDRESSING_MODES::IMP => {
                    format += &format!("(IMP) {}", instruction.name);
                    local_pc = local_pc.wrapping_add(1);
                }
                cpu::cpu6502::ADDRESSING_MODES::IMM => {
                    let data = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(1));

                    format += &format!("(IMM) {} #${data:02X}", instruction.name);
                    local_pc = local_pc.wrapping_add(2);
                }
                cpu::cpu6502::ADDRESSING_MODES::ABS => {
                    let lo = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    format += &format!("(ABS) {} ${addr:04X}", instruction.name);
                    local_pc = local_pc.wrapping_add(3);
                }
                cpu::cpu6502::ADDRESSING_MODES::ABSX => {
                    let lo = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    format += &format!("(ABSx) {} ${addr:04X}, X", instruction.name);
                    local_pc = local_pc.wrapping_add(3);
                }
                cpu::cpu6502::ADDRESSING_MODES::ABSY => {
                    let lo = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(1));
                    let hi = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(2));
                    let addr = (hi as u16) << 8 | lo as u16;

                    format += &format!("{} ${addr:04X}, Y", instruction.name);
                    local_pc = local_pc.wrapping_add(3);
                }
                cpu::cpu6502::ADDRESSING_MODES::ZPG => {
                    let addr = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(1));

                    format += &format!("{} ${addr:02X}", instruction.name);
                    local_pc = local_pc.wrapping_add(2);
                }
                cpu::cpu6502::ADDRESSING_MODES::ZPGX => {
                    let addr = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(1));

                    format += &format!("{} ${addr:02X}, X", instruction.name);
                    local_pc = local_pc.wrapping_add(2);
                }
                cpu::cpu6502::ADDRESSING_MODES::ZPGY => {
                    let addr = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(1));

                    format += &format!("{} ${addr:02X}, Y", instruction.name);
                    local_pc = local_pc.wrapping_add(2);
                }
                cpu::cpu6502::ADDRESSING_MODES::REL => {
                    let addr = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc.wrapping_add(1));

                    format += &format!("(REL) {} ${addr:02X} [{:04X}]", instruction.name, local_pc.wrapping_add(2).wrapping_add((addr as i8) as u16));
                    local_pc = local_pc.wrapping_add(2);
                }
                cpu::cpu6502::ADDRESSING_MODES::IND => {
                    let old_lo = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc + 1);
                    let hi = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc + 2);
                    let ptr = (hi as u16) << 8 | old_lo as u16;
                    let lo = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, ptr);
                    let addr = if old_lo == 0xFF {
                        let hi = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, ptr & 0xFF00);
                        (hi as u16) << 8 | lo as u16
                    } else {
                        let hi = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, ptr + 1);
                        (hi as u16) << 8 | lo as u16
                    };

                    format += &format!("{} (${addr:04X})", instruction.name);
                    local_pc = local_pc.wrapping_add(3);
                }
                cpu::cpu6502::ADDRESSING_MODES::INDX => {
                    let addr = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc + 1);
                    let lo = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, (addr + self.cpu.x) as u16);
                    let hi = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, (addr + self.cpu.x + 1) as u16);
                    let ptr = (hi as u16) << 8 | lo as u16;

                    format += &format!("{} (${:02X}, X) @ {:02X} = {ptr:04X}", instruction.name, addr, addr + self.cpu.x);
                    local_pc = local_pc.wrapping_add(2);
                }
                cpu::cpu6502::ADDRESSING_MODES::INDY => {
                    let addr = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, local_pc + 1);
                    let lo = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, addr as u16);
                    let hi = self.cpu.read(&mut self.cartridge, &mut self.ppu, &self.bus, (addr + 1) as u16);
                    let ptr = (hi as u16) << 8 | lo as u16;

                    format += &format!("{} (${addr:02X}), Y = {:04X}", instruction.name, ptr + self.cpu.y as u16);
                    local_pc = local_pc.wrapping_add(2);
                }
            }
            instruction_string.push(format);
        }

        instruction_string
    }
}
