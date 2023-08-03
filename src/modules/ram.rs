pub struct RAM {
    wram: [u8; 0x2000],
    hram: [u8; 0x80],
}

impl RAM {
    pub fn new() -> Self {
        Self {
            wram: [0; 0x2000],
            hram: [0; 0x80],
        }
    }

    pub fn wram_read(&self, address: u16) -> u8 {
        let addr = address - 0xC000;
        if (addr >= 0x2000) {
            panic!("INVALID WRAM ADDRESS {:8X}", addr)
        }
        return self.wram[addr as usize];
    }

    pub fn wram_write(&mut self, address: u16, value: u8) {
        let addr = address - 0xC000;
        if (addr >= 0x2000) {
            panic!("INVALID WRAM ADDRESS {:8X}", addr)
        }
        self.wram[addr as usize] = value;
    }

    pub fn hram_read(&self, address: u16) -> u8 {
        let addr = address - 0xFF80;
        if (addr >= 0x80) {
            panic!("INVALID HRAM ADDRESS {:8X}", addr)
        }
        return self.hram[addr as usize];
    }

    pub fn hram_write(&mut self, address: u16, value: u8) {
        let addr = address - 0xFF80;
        if (addr >= 0x80) {
            panic!("INVALID HRAM ADDRESS {:8X}", addr)
        }
        self.hram[addr as usize] = value;
    }
}
