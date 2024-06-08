use std::io::{Read, Seek};

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
    pub fn from_bytes(buffer: &[u8]) -> Self {
        unsafe { *(buffer.as_ptr() as *const Self) }
    }
}

#[derive(Debug)]
pub struct ComponentCartridge {
    /// Program ROM
    prg_rom: Vec<u8>,
    /// Character ROM
    chr_rom: Vec<u8>,
    mapper_id: u8,
    prg_banks_count: u8,
    chr_banks_count: u8,
}

impl ComponentCartridge {
    pub fn new() -> Self {
        Self {
            prg_rom: Vec::new(),
            chr_rom: Vec::new(),
            mapper_id: 0,
            prg_banks_count: 0,
            chr_banks_count: 0,
        }
    }

    pub fn from_path(path: &str) -> Self {
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer: [u8; 16] = [0; 16];
        
        file.read_exact(&mut buffer).unwrap();
        let header = HeaderCartridge::from_bytes(&buffer);

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

        Self {
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
