pub mod ncartridge {
    use crate::nes::mappers::*;
    use std::io::{Read, Seek};
    
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Header {
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

    impl Header {
        pub fn from_bytes(buffer: &[u8]) -> Self {
            unsafe { *(buffer.as_ptr() as *const Self) }
        }
    }

    #[derive(Debug, Default)]
    pub enum Mirror {
        #[default]
        Horizontal,
        Vertical,
        OneScreenLo,
        OneScreenHi,
    }

    #[allow(dead_code)]
    #[derive(Debug, Default)]
    pub struct Cartridge {
        prg_rom: Vec<u8>, /// Program ROM
        chr_rom: Vec<u8>, /// Character ROM

        mapper_id: u8,
        pub mirror: Mirror,
        prg_banks_count: u8,
        chr_banks_count: u8,

        mapper: Box<dyn Mapper>
    }

    impl Cartridge {
        pub fn from_path(path: &str) -> Self {
            let mut file = std::fs::File::open(path).unwrap();
            let mut buffer: [u8; 16] = [0; 16];
            
            file.read_exact(&mut buffer).unwrap();
            let header = Header::from_bytes(&buffer);

            if header.mapper1 & 0x04 != 0 {
                file.seek(std::io::SeekFrom::Current(512)).unwrap();
            }

            let mut prg_rom = Vec::new();
            let mut chr_rom = Vec::new();
            let mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
            let mirror = if header.mapper1 & 0x01 != 0 { Mirror::Vertical } else { Mirror::Horizontal };
            let mut prg_banks_count = 0;
            let mut chr_banks_count = 0;

            let file_type = 1;
            match file_type {
                0 => {},
                1 => {
                    prg_banks_count = header.prg_rom_chunks;
                    prg_rom.resize(prg_banks_count as usize * 16 * 1024, 0);
                    if file.read_exact(&mut prg_rom).is_err() {
                        println!("[WARN] PRG rom | no more data to read | Buffer size is {} bytes", prg_rom.len());
                    }

                    chr_banks_count = header.chr_rom_chunks;
                    chr_rom.resize(chr_banks_count as usize * 8 * 1024, 0);
                    if file.read_exact(&mut chr_rom).is_err() {
                        println!("[WARN] CHR rom | no more data to read | Buffer size is {} bytes", chr_rom.len());
                    }
                },
                2 => {},
                _ => {},
            }

            let mapper: Box<dyn Mapper> = match mapper_id {
                0 => {
                    Box::new(mapper_000::Mapper000::new(prg_banks_count, chr_banks_count))
                },
                _ => {
                    println!("[WARN] Mapper {} not implemented, using Mapper000 (program will probably not work)", mapper_id);
                    Box::new(mapper_000::Mapper000::new(prg_banks_count, chr_banks_count))
                }
            };

            Self {
                prg_rom,
                chr_rom,
                mapper_id,
                mirror,
                prg_banks_count,
                chr_banks_count,
                mapper
            }
        }

        pub fn cpu_read(&self, addr: u16, data: &mut u8) -> bool {
            let mut mapped_addr: u32 = 0x0000;

            if self.mapper.cpu_map_read(addr, &mut mapped_addr) {
                *data = self.prg_rom[mapped_addr as usize];
                return true;
            }

            false
        }

        pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
            let mut mapped_addr: u32 = 0x0000;

            if self.mapper.cpu_map_write(addr, &mut mapped_addr) {
                self.prg_rom[mapped_addr as usize] = data;
                return true;
            }

            false
        }

        pub fn ppu_read(&self, addr: u16, data: &mut u8) -> bool {
            let mut mapped_addr: u32 = 0x0000;

            if self.mapper.ppu_map_read(addr, &mut mapped_addr) {
                *data = self.chr_rom[mapped_addr as usize];
                return true;
            }

            false
        }

        #[allow(dead_code)]
        pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool {
            let mut mapped_addr: u32 = 0x0000;

            if self.mapper.ppu_map_write(addr, &mut mapped_addr) {
                self.chr_rom[mapped_addr as usize] = data;
                return true;
            }

            false
        }
    }

}
