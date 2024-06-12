#[derive(Debug, Clone, Copy)]
pub struct EntryOA {
    pub y: u8,
    pub tile_index: u8,
    pub attributes: u8,
    pub x: u8,
}

impl EntryOA {
    pub const fn new() -> EntryOA {
        EntryOA {
            y: 0,
            tile_index: 0,
            attributes: 0,
            x: 0,
        }
    }

    pub fn set(&mut self, value: u8) {
        self.y = value;
        self.tile_index = value;
        self.attributes = value;
        self.x = value;
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub struct OAM {
    pub oam: [EntryOA; 64],
    pub address: u8,
}

impl OAM {
    pub const fn new() -> OAM {
        OAM {
            oam: [EntryOA::new(); 64],
            address: 0,
        }
    }

    pub fn get_entry(&self, index: u8) -> &EntryOA {
        &self.oam[index as usize]
    }

    pub fn write(&mut self, address: u8, data: u8) {
        match address % 4 {
            0 => self.oam[address as usize / 4].y = data,
            1 => self.oam[address as usize / 4].tile_index = data,
            2 => self.oam[address as usize / 4].attributes = data,
            3 => self.oam[address as usize / 4].x = data,
            _ => panic!("Invalid OAM address"),
        }
    }

    pub fn read(&self, data: &mut u8) {
        match self.address % 4 {
            0 => *data = self.oam[self.address as usize / 4].y,
            1 => *data = self.oam[self.address as usize / 4].tile_index,
            2 => *data = self.oam[self.address as usize / 4].attributes,
            3 => *data = self.oam[self.address as usize / 4].x,
            _ => panic!("Invalid OAM address"),
        }
    }

    pub const fn get_address(&self) -> u8 {
        self.address
    }

    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }
}