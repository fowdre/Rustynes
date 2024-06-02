use super::Mapper;

pub struct Mapper000 {
    prg_banks_count: u8,
    chr_banks_count: u8,
}

impl Mapper for Mapper000 {
    fn new(prg_banks_count: u8, chr_banks_count: u8) -> Self {
        Self {
            prg_banks_count,
            chr_banks_count,
        }
    }

    fn cpu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x8000 {
            *mapped_addr = if self.prg_banks_count > 1 {
                (addr & 0x7FFF) as u32
            } else {
                (addr & 0x3FFF) as u32
            };
            return true;
        }

        false
    }

    fn cpu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x8000 {
            
            *mapped_addr = if self.prg_banks_count > 1 {
                (addr & 0x7FFF) as u32
            } else {
                (addr & 0x3FFF) as u32
            };
            return true;
        }

        false
    }

    fn ppu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr <= 0x1FFF {
            
            *mapped_addr = addr as u32;
            return true;
        }

        false
    }

    fn ppu_map_write(&self, addr: u16, _mapped_addr: &mut u32) -> bool {
        false
    }
}
