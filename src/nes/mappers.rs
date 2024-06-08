pub mod mapper_000;

pub trait Mapper {
    fn new(prg_banks_count: u8, chr_banks_count: u8) -> Self where Self: Sized;
    
    fn cpu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn cpu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    
    fn ppu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn ppu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
}
