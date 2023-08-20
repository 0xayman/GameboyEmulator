pub struct PPU {
    pub oam_ram: [u8; 0xA0],
    pub vram: [u8; 0x2000],
}

impl PPU {
    pub fn new() -> Self {
        Self {
            oam_ram: [0; 0xA0],
            vram: [0; 0x2000],
        }
    }

    pub fn oam_write(&mut self, address: u16, value: u8) {
        println!("OAM Write: {:#X} = {:#X}", address, value);
        let mut address = address as usize;
        if address >= 0xFE00 {
            address = address - 0xFE00;
        }

        self.oam_ram[address as usize] = value;
    }

    pub fn oam_read(&self, address: u16) -> u8 {
        let mut address = address as usize;
        if address >= 0xFE00 {
            address = address - 0xFE00;
        }

        self.oam_ram[address as usize]
    }

    pub fn vram_write(&mut self, address: u16, value: u8) {
        self.vram[(address - 0x8000) as usize] = value;
    }

    pub fn vram_read(&self, address: u16) -> u8 {
        self.vram[(address - 0x8000) as usize]
    }
}
