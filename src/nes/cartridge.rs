use std::io::{Read, Seek};

use crate::nes::mappers::{Mapper, mapper_000, mapper_002};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct HeaderCartridge {
    name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [u8; 5],
}

impl HeaderCartridge {
    pub const fn from_bytes(buffer: &[u8]) -> Self {
        unsafe { buffer.as_ptr().cast::<Self>().read_unaligned() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mirror {
    Hardware,
    Horizontal,
    Vertical,
    OneScreenLo,
    OneScreenHi,
}

#[derive(Debug)]
pub struct ComponentCartridge {
    /// Program ROM
    prg_rom: Vec<u8>,
    /// Character ROM
    chr_rom: Vec<u8>,
    
    mapper: Box<dyn Mapper>,
    pub hardware_mirror: Mirror,
}

impl ComponentCartridge {
    pub fn new() -> Self {
        Self {
            prg_rom: Vec::new(),
            chr_rom: Vec::new(),
            mapper: Box::new(mapper_000::Mapper000::new(0, 0)),
            hardware_mirror: Mirror::Horizontal,
        }
    }

    pub fn from_path(path: &str) -> Self {
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer: [u8; 16] = [0; 16];
        
        file.read_exact(&mut buffer).unwrap();
        let header = HeaderCartridge::from_bytes(&buffer);

        println!("Header unused bytes [{}{}{}{}{}]", header.unused[0] as char, header.unused[1] as char, header.unused[2] as char, header.unused[3] as char, header.unused[4] as char);
        println!("{:?}", header);

        if header.mapper1 & 0x04 != 0 {
            file.seek(std::io::SeekFrom::Current(512)).unwrap();
        }

        let mut prg_rom = Vec::new();
        let mut chr_rom = Vec::new();
        let mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
        let hardware_mirror = if header.mapper1 & 0x01 != 0 { Mirror::Vertical } else { Mirror::Horizontal };
        let prg_banks_count;
        let chr_banks_count;

        let file_type = if header.mapper2 & 0x0C == 0x08 { 2 } else { 1 };
        // let file_type = 1;
        match file_type {
            1 => {
                prg_banks_count = header.prg_rom_chunks;
                prg_rom.resize(prg_banks_count as usize * 16 * 1024, 0);
                if file.read_exact(&mut prg_rom).is_err() {
                    println!("[WARN] PRG rom | no more data to read | Buffer size is {} bytes", prg_rom.len());
                }

                chr_banks_count = header.chr_rom_chunks;
                dbg!(chr_banks_count);
                if chr_banks_count == 0 {
                    chr_rom.resize(8 * 1024, 0);
                } else {
                    chr_rom.resize(chr_banks_count as usize * 8 * 1024, 0);
                }
                if file.read_exact(&mut chr_rom).is_err() {
                    println!("[WARN] CHR rom | no more data to read | Buffer size is {} bytes", chr_rom.len());
                }
            }
            // 2 => {
            //     prg_banks_count = ((header.prg_ram_size & 0x0F).wrapping_shl(8)) | header.prg_rom_chunks;
            //     prg_rom.resize(prg_banks_count as usize * 16 * 1024, 0);
            //     if file.read_exact(&mut prg_rom).is_err() {
            //         println!("[WARN] PRG rom | no more data to read | Buffer size is {} bytes", prg_rom.len());
            //     }

            //     chr_banks_count = ((header.prg_ram_size & 0x38).wrapping_shr(8)) | header.chr_rom_chunks;
            //     chr_rom.resize(chr_banks_count as usize * 8 * 1024, 0);
            //     if file.read_exact(&mut chr_rom).is_err() {
            //         println!("[WARN] CHR rom | no more data to read | Buffer size is {} bytes", chr_rom.len());
            //     }
            // }
            _ => unimplemented!("File type {file_type} not implemented!"),
        }

        dbg!(prg_banks_count);
        dbg!(chr_banks_count);

        let mapper: Box<dyn Mapper> = match mapper_id {
            0 => Box::new(mapper_000::Mapper000::new(prg_banks_count, chr_banks_count)),
            2 => Box::new(mapper_002::Mapper002::new(prg_banks_count, chr_banks_count)),
            _ => todo!("Mapper {mapper_id} not implemented yet!"),
        };

        Self {
            prg_rom,
            chr_rom,
            mapper,
            hardware_mirror,
        }
    }

    pub fn mirror(&self) -> Mirror {
        let mirror = self.mapper.mirror();

        if mirror == Mirror::Hardware {
            return self.hardware_mirror;
        }

        mirror
    }

    pub fn cpu_read(&self, addr: u16, data: &mut u8) -> bool {
        let mut mapped_addr = 0x0000;

        if self.mapper.cpu_map_read(addr, &mut mapped_addr) {
            if mapped_addr == 0xFFFFFFFF {
                return true;
            } else {
                *data = self.prg_rom[mapped_addr as usize];
            }
            return true;
        }

        false
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        let mut mapped_addr = 0x0000;

        if self.mapper.cpu_map_write(addr, &mut mapped_addr, data) {
            if mapped_addr == 0xFFFFFFFF {
                return true;
            } else {
                self.prg_rom[mapped_addr as usize] = data;
            }
            return true;
        }

        false
    }

    pub fn ppu_read(&self, addr: u16, data: &mut u8) -> bool {
        let mut mapped_addr = 0x0000;

        if self.mapper.ppu_map_read(addr, &mut mapped_addr) {
            *data = self.chr_rom[mapped_addr as usize];
            return true;
        }

        false
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool {
        let mut mapped_addr = 0x000;

        if self.mapper.ppu_map_write(addr, &mut mapped_addr) {
            self.chr_rom[mapped_addr as usize] = data;
            return true;
        }

        false
    }

    pub fn reset(&mut self) {
        self.mapper.reset();
    }
}
