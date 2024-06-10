pub mod mapper_000;

pub trait Mapper {
    fn new(prg_banks_count: u8, chr_banks_count: u8) -> Self where Self: Sized;
    
    fn cpu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn cpu_map_write(&self, addr: u16, mapped_addr: &mut u32, data: u8) -> bool;
    
    fn ppu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn ppu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
}

impl Default for Box<dyn Mapper> {
    fn default() -> Self {
        Box::new(mapper_000::Mapper000::new(0, 0))
    }
}

impl core::fmt::Debug for dyn Mapper {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Mapper")
    }
}
