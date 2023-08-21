use sdl2::pixels::Color;

use crate::enums::interrupt_types::InterruptType;

use super::{cpu::CPU, dma::Dma, interrupts::interrupt};

const TILE_COLORS: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(175, 175, 175),
    Color::RGB(85, 85, 85),
    Color::RGB(0, 0, 0),
];

pub struct LCD {
    // Registers
    pub lcdc: u8,  // FF40
    pub lcds: u8,  // FF41
    pub scy: u8,   // FF42
    pub scx: u8,   // FF43
    pub ly: u8,    // FF44
    pub lyc: u8,   // FF45
    pub dma: u8,   // FF46
    pub bgp: u8,   // FF47
    pub objp0: u8, // FF48
    pub objp1: u8, // FF49
    pub wy: u8,    // FF4A
    pub wx: u8,    // FF4B

    // Other data ..
    bg_colors: [Color; 4],
    sp1_colors: [Color; 4],
    sp2_colors: [Color; 4],
}

impl LCD {
    pub fn new() -> Self {
        Self {
            lcdc: 0,
            lcds: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            objp0: 0,
            objp1: 0,
            wy: 0,
            wx: 0,
            bg_colors: [Color::RGB(0, 0, 0); 4],
            sp1_colors: [Color::RGB(0, 0, 0); 4],
            sp2_colors: [Color::RGB(0, 0, 0); 4],
        }
    }
    pub fn bgw_enabled(&self) -> u8 {
        self.lcdc & 0b0000_0001
    }

    pub fn obj_enabled(&self) -> u8 {
        self.lcdc & 0b0000_0010
    }

    pub fn obj_height(&self) -> u8 {
        if self.lcdc & 0b0000_0100 != 0 {
            16
        } else {
            8
        }
    }

    pub fn bg_map_area(&self) -> u16 {
        if self.lcdc & 0b0000_1000 != 0 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn bgw_data_area(&self) -> u16 {
        if self.lcdc & 0b0001_0000 != 0 {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn window_enabled(&self) -> u8 {
        self.lcdc & 0b0010_0000
    }

    pub fn window_map_area(&self) -> u16 {
        if self.lcdc & 0b0100_0000 != 0 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn lcd_enabled(&self) -> u8 {
        self.lcdc & 0b1000_0000
    }

    pub fn get_lcds_mode(&self) -> LCDMode {
        match self.lcds & 0b0000_0011 {
            0 => LCDMode::HBLANK,
            1 => LCDMode::VBLANK,
            2 => LCDMode::OAM,
            3 => LCDMode::XFER,
            _ => unreachable!(),
        }
    }

    pub fn set_lcds_mode(&mut self, mode: LCDMode) {
        self.lcds = (self.lcds & 0b1111_1100) | (mode as u8);
    }

    pub fn lyc(&self) -> u8 {
        self.lcds & 0b0000_0100
    }

    pub fn set_lyc(&mut self, value: u8) {
        self.lcds = (self.lcds & 0b1111_1011) | (value << 2);
    }

    pub fn stat_interrupt(&self, src: StatSrc) -> u8 {
        self.lcds & (src as u8)
    }

    pub fn increment_ly(cpu: &mut CPU) {
        cpu.bus.lcd.ly += 1;

        if cpu.bus.lcd.ly == cpu.bus.lcd.lyc {
            cpu.bus.lcd.set_lyc(1);

            if cpu.bus.lcd.lcds & StatSrc::LYC as u8 != 0 {
                interrupt::request(cpu, InterruptType::LCDSTAT);
            }
        } else {
            cpu.bus.lcd.set_lyc(0);
        }
    }

    pub fn init(&mut self) {
        self.lcdc = 0x91;
        self.scx = 0x00;
        self.scy = 0x00;
        self.ly = 0x00;
        self.lyc = 0x00;

        self.bgp = 0xFC;
        self.objp0 = 0xFF;
        self.objp1 = 0xFF;

        self.wy = 0x00;
        self.wx = 0x00;

        for i in 0..4 {
            self.bg_colors[i] = TILE_COLORS[i];
            self.sp1_colors[i] = TILE_COLORS[i];
            self.sp2_colors[i] = TILE_COLORS[i];
        }
    }

    pub fn read(cpu: &CPU, address: u16) -> u8 {
        let offset = address - 0xFF40;

        match offset {
            0x00 => cpu.bus.lcd.lcdc,
            0x01 => cpu.bus.lcd.lcds,
            0x02 => cpu.bus.lcd.scy,
            0x03 => cpu.bus.lcd.scx,
            0x04 => cpu.bus.lcd.ly,
            0x05 => cpu.bus.lcd.lyc,
            0x06 => cpu.bus.lcd.dma,
            0x07 => cpu.bus.lcd.bgp,
            0x08 => cpu.bus.lcd.objp0,
            0x09 => cpu.bus.lcd.objp1,
            0x0A => cpu.bus.lcd.wy,
            0x0B => cpu.bus.lcd.wx,
            _ => unreachable!(),
        }
    }

    pub fn write(cpu: &mut CPU, address: u16, value: u8) {
        let offset = address - 0xFF40;

        match offset {
            0x00 => cpu.bus.lcd.lcdc = value,
            0x01 => cpu.bus.lcd.lcds = value,
            0x02 => cpu.bus.lcd.scy = value,
            0x03 => cpu.bus.lcd.scx = value,
            0x04 => cpu.bus.lcd.ly = value,
            0x05 => cpu.bus.lcd.lyc = value,
            0x06 => cpu.bus.lcd.dma = value,
            0x07 => cpu.bus.lcd.bgp = value,
            0x08 => cpu.bus.lcd.objp0 = value,
            0x09 => cpu.bus.lcd.objp1 = value,
            0x0A => cpu.bus.lcd.wy = value,
            0x0B => cpu.bus.lcd.wx = value,
            _ => unreachable!(),
        }

        if offset == 0x06 {
            // 0xFF46 -> DMA
            Dma::start(cpu, value);
        }

        if address == 0xFF47 {
            cpu.bus.lcd.update_pallete(value, 0);
        } else if address == 0xFF48 {
            cpu.bus.lcd.update_pallete(value & 0b1111_1100, 1);
        } else if address == 0xFF49 {
            cpu.bus.lcd.update_pallete(value & 0b1111_1100, 2);
        }
    }

    fn update_pallete(&mut self, pdata: u8, pal: u8) {
        match pal {
            1 => self.bg_colors = self.sp1_colors,
            2 => self.bg_colors = self.sp2_colors,
            _ => (),
        }

        self.bg_colors[0] = TILE_COLORS[(pdata & 0b0000_0011) as usize];
        self.bg_colors[1] = TILE_COLORS[((pdata >> 2) & 0b0000_0011) as usize];
        self.bg_colors[2] = TILE_COLORS[((pdata >> 4) & 0b0000_0011) as usize];
        self.bg_colors[3] = TILE_COLORS[((pdata >> 6) & 0b0000_0011) as usize];
    }
}

pub enum LCDMode {
    HBLANK,
    VBLANK,
    OAM,
    XFER,
}

pub enum StatSrc {
    HBLANK = (1 << 3),
    VBLANK = (1 << 4),
    OAM = (1 << 5),
    LYC = (1 << 6),
}
