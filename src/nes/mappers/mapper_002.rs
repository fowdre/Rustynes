use crate::nes::mappers::Mapper;

pub struct Mapper002 {
    prg_banks_count: u8,
    chr_banks_count: u8,

    // prg_bank_select_high: u8,
    // prg_bank_select_low: u8,

    prg_top_bank: u8
}

impl Mapper for Mapper002 {
    fn new(prg_banks_count: u8, chr_banks_count: u8) -> Self {
        // panic!("Mapper002 not implemented yet!");
        Self {
            prg_banks_count,
            chr_banks_count,

            // prg_bank_select_high: 0,
            // prg_bank_select_low: 0,

            prg_top_bank: 0
        }
    }
    
    fn cpu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        match addr {
            // 0x8000..=0xBFFF => {
            //     *mapped_addr = (self.prg_bank_select_low as u16).wrapping_mul(0x4000).wrapping_add(addr & 0x3FFF) as u32;
            //     return true
            // }
            // 0xC000..=0xFFFF => {
            //     *mapped_addr = (self.prg_bank_select_high as u16).wrapping_mul(0x4000).wrapping_add(addr & 0x3FFF) as u32;
            //     return true
            // }

            0x8000..=0xFFFF => {
                let mut bank = if addr >= 0x8000 && addr <= 0xBFFF {
                    self.prg_top_bank & 0x0F
                } else if addr >= 0xC000 {
                    self.prg_banks_count - 1
                } else {
                    unreachable!()
                } as usize;

                bank %= self.prg_banks_count as usize;

                let start_of_bank = bank * 0x4000;

                *mapped_addr = (start_of_bank + (addr & 0x3FFF) as usize) as u32;
            }
            _ => {}
        }

        false
    }
    fn cpu_map_write(&mut self, addr: u16, _mapped_addr: &mut u32, data: u8) -> bool {
        if addr >= 0x8000 {
            self.prg_top_bank = data;
        }

        false
    }
    
    fn ppu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr < 0x2000 {
            *mapped_addr = addr as u32;
            return true
        }
        
        false
    }
    fn ppu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        // if addr < 0x2000 && self.chr_banks_count == 0 {
        //     *mapped_addr = addr as u32;
        //     return true
        // }

        if addr < 0x1FFF && self.chr_banks_count == 0 {
            *mapped_addr = addr as u32;
            return true
        }

        false
    }

    fn reset(&mut self) {
        // self.prg_bank_select_low = self.prg_banks_count - 1;
        // self.prg_bank_select_high = 0;

        self.prg_top_bank = 0;
    }
}
