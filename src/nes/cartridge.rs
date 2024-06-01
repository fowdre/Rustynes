pub mod ncartridge {
    use std::io::{Read, Seek};
    
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Header {
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
            unsafe { *(buffer.as_ptr() as *const Header) }
        }
    }

    #[derive(Debug, Default)]
    pub struct Cartridge {
        prg_rom: Vec<u8>,
        chr_rom: Vec<u8>,

        mapper_id: u8,
        prg_banks_count: u8,
        chr_banks_count: u8,
    }

    impl Cartridge {
        pub fn from_path(path: &str) -> Self {
            let mut file = std::fs::File::open(path).unwrap();
            let mut buffer: [u8; 12] = [0; 12];
            
            file.read_exact(&mut buffer).unwrap();
            let header = Header::from_bytes(&buffer);

            if header.mapper1 & 0x04 != 0 {
                file.seek(std::io::SeekFrom::Current(512)).unwrap();
            }

            let mut prg_rom = Vec::new();
            let mut chr_rom = Vec::new();
            let mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
            let mut prg_banks_count = 0;
            let mut chr_banks_count = 0;

            let file_type = 1;
            match file_type {
                0 => {},
                1 => {
                    prg_banks_count = header.prg_rom_chunks;
                    prg_rom = vec![0; prg_banks_count as usize * 16 * 1024];
                    // file.read_exact(&mut prg_rom).unwrap();
                    file.read_to_end(&mut prg_rom).unwrap();

                    chr_banks_count = header.chr_rom_chunks;
                    chr_rom = vec![0; chr_banks_count as usize * 8 * 1024];
                    // file.read_exact(&mut chr_rom).unwrap();
                    file.read_to_end(&mut chr_rom).unwrap();
                },
                2 => {},
                _ => {},
            }

            Cartridge {
                prg_rom,
                chr_rom,
                mapper_id,
                prg_banks_count,
                chr_banks_count,
            }
        }

        pub fn cpu_read(&self, addr: u16, data: &u8) -> bool {
            false
        }

        pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
            false
        }

        pub fn ppu_read(&self, addr: u16, data: &u8) -> bool {
            false
        }

        pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool {
            false
        }
    }

}
