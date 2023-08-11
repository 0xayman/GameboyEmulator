pub struct PPU {}

pub struct OAMEntry {
    pub y: u8,
    pub x: u8,
    pub tile: u8,

    pub f_cgb_pn: u8,
    pub f_cgb_vram_bank: u8,
    pub f_pn: u8,
    pub f_x_flip: u8,
    pub f_y_flip: u8,
    pub f_bgp: u8,
}

impl PPU {
    pub fn init() {}

    pub fn tick() {}
}
