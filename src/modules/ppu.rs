use std::{cell::RefCell, rc::Rc};

use sdl2::pixels::Color;

use crate::enums::interrupt_types::InterruptType;

use super::{
    bus::Bus,
    cpu::Cpu,
    emu::Emu,
    interrupts::interrupt,
    lcd::{LCDMode, Lcd, StatSrc},
};

const LINES_PER_FRAME: u32 = 154;
const TICKS_PER_LINE: u32 = 456;
const YRES: i32 = 144;
const XRES: i32 = 160;

const COLORS_DEFAULT: [u32; 4] = [0xFFFFFFFF, 0xFFAAAAAA, 0xFF555555, 0xFF000000];

const TILE_COLORS: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(175, 175, 175),
    Color::RGB(85, 85, 85),
    Color::RGB(0, 0, 0),
];

pub struct FiFoEntry {
    next: Option<Rc<RefCell<FiFoEntry>>>,
    value: Color, // color value
}

pub struct FiFo {
    head: Option<Rc<RefCell<FiFoEntry>>>,
    tail: Option<Rc<RefCell<FiFoEntry>>>, // or *mut FiFoEntry, will see later which is better
    size: u32,
}

impl FiFo {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
}

pub struct PixelFiFo {
    current_fetch_state: FetchState,
    fifo: FiFo,
    line_x: u8,
    pushed_x: u8,
    fetch_x: u8,
    bgw_fetch_data: [u8; 3],
    fetch_entry_data: [u8; 6],
    map_y: u8,
    map_x: u8,
    tile_y: u8,
    fifo_x: u8,
}

impl PixelFiFo {
    pub fn new() -> Self {
        Self {
            current_fetch_state: FetchState::Tile,
            fifo: FiFo::new(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            bgw_fetch_data: [0; 3],
            fetch_entry_data: [0; 6],
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
        }
    }
}

pub struct Ppu {
    pub oam_ram: [u8; 0xA0],
    pub vram: [u8; 0x2000],

    pub current_frame: u32,
    pub line_ticks: u32,
    pub video_buffer: [Color; (XRES * YRES) as usize],

    pub target_frame_time: u64,
    pub prev_frame_time: u64,
    pub start_time: u64,
    pub frame_count: u64,

    pub pfc: PixelFiFo,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            oam_ram: [0; 0xA0],
            vram: [0; 0x2000],

            current_frame: 0,
            line_ticks: 0,
            video_buffer: [TILE_COLORS[0]; (XRES * YRES) as usize],

            target_frame_time: 1000 / 60,
            prev_frame_time: 0,
            start_time: 0,
            frame_count: 0,

            pfc: PixelFiFo::new(),
        }
    }

    pub fn init(cpu: &mut Cpu) {
        cpu.bus.ppu.current_frame = 0;
        cpu.bus.ppu.line_ticks = 0;
        cpu.bus.ppu.video_buffer = [TILE_COLORS[0]; (XRES * YRES) as usize];

        Lcd::init(&mut cpu.bus.lcd);
        Lcd::set_lcds_mode(&mut cpu.bus.lcd, super::lcd::LCDMode::Oam);

        cpu.bus.ppu.oam_ram = [0; 0xA0];
    }

    pub fn tick(cpu: &mut Cpu) {
        cpu.bus.ppu.line_ticks += 1;

        match cpu.bus.lcd.get_lcds_mode() {
            LCDMode::Oam => Self::mode_oam(cpu),
            LCDMode::Xfer => Self::mode_xfer(cpu),
            LCDMode::Vblank => Self::mode_vblank(cpu),
            LCDMode::Hblank => Self::mode_hblank(cpu),
        }
    }

    pub fn oam_write(&mut self, address: u16, value: u8) {
        let mut address = address as usize;
        if address >= 0xFE00 {
            address -= 0xFE00;
        }

        self.oam_ram[address] = value;
    }

    pub fn oam_read(&self, address: u16) -> u8 {
        let mut address = address as usize;
        if address >= 0xFE00 {
            address -= 0xFE00;
        }

        self.oam_ram[address]
    }

    pub fn vram_write(&mut self, address: u16, value: u8) {
        self.vram[(address - 0x8000) as usize] = value;
    }

    pub fn vram_read(&self, address: u16) -> u8 {
        self.vram[(address - 0x8000) as usize]
    }

    fn mode_oam(cpu: &mut Cpu) {
        if cpu.bus.ppu.line_ticks >= 80 {
            cpu.bus.lcd.set_lcds_mode(LCDMode::Xfer);

            cpu.bus.ppu.pfc.current_fetch_state = FetchState::Tile;
            cpu.bus.ppu.pfc.line_x = 0;
            cpu.bus.ppu.pfc.fetch_x = 0;
            cpu.bus.ppu.pfc.pushed_x = 0;
            cpu.bus.ppu.pfc.fifo_x = 0;
        }
    }

    fn mode_xfer(cpu: &mut Cpu) {
        Self::pipeline_process(cpu);
        if cpu.bus.ppu.pfc.pushed_x >= XRES as u8 {
            Self::pipeline_fifo_reset(cpu);

            cpu.bus.lcd.set_lcds_mode(LCDMode::Hblank);

            if cpu.bus.lcd.stat_interrupt(StatSrc::Hblank) != 0 {
                interrupt::request(cpu, InterruptType::LcdStat);
            }
        }
    }

    fn mode_vblank(cpu: &mut Cpu) {
        if cpu.bus.ppu.line_ticks >= TICKS_PER_LINE {
            Lcd::increment_ly(cpu);

            if cpu.bus.lcd.ly >= (LINES_PER_FRAME as u8) {
                cpu.bus.lcd.set_lcds_mode(LCDMode::Oam);
                cpu.bus.lcd.ly = 0;
            }

            cpu.bus.ppu.line_ticks = 0;
        }
    }
    fn mode_hblank(cpu: &mut Cpu) {
        if cpu.bus.ppu.line_ticks >= TICKS_PER_LINE {
            Lcd::increment_ly(cpu);

            if cpu.bus.lcd.ly >= YRES as u8 {
                cpu.bus.lcd.set_lcds_mode(LCDMode::Vblank);

                interrupt::request(cpu, InterruptType::Vblank);

                if cpu.bus.lcd.stat_interrupt(StatSrc::Vblank) != 0 {
                    interrupt::request(cpu, InterruptType::LcdStat);
                }

                cpu.bus.ppu.current_frame += 1;

                // calc FPS
                let end = Emu::get_ticks();
                let frame_time = end - cpu.bus.ppu.prev_frame_time;

                if frame_time < cpu.bus.ppu.target_frame_time {
                    Emu::delay(cpu.bus.ppu.target_frame_time - frame_time);
                }

                if end - cpu.bus.ppu.start_time >= 1000 {
                    // let fps = cpu.bus.ppu.frame_count;
                    cpu.bus.ppu.start_time = end;
                    cpu.bus.ppu.frame_count = 0;

                    // println!("FPS: {}", fps);
                }

                cpu.bus.ppu.frame_count += 1;
                cpu.bus.ppu.prev_frame_time = Emu::get_ticks();
            } else {
                cpu.bus.lcd.set_lcds_mode(LCDMode::Oam);
            }

            cpu.bus.ppu.line_ticks = 0;
        }
    }

    fn pixel_fifo_push(cpu: &mut Cpu, value: Color) {
        let next = Rc::new(RefCell::new(FiFoEntry { next: None, value }));

        if cpu.bus.ppu.pfc.fifo.head.is_none() {
            cpu.bus.ppu.pfc.fifo.head = Some(next.clone());
            cpu.bus.ppu.pfc.fifo.tail = Some(next.clone());
        } else {
            cpu.bus
                .ppu
                .pfc
                .fifo
                .tail
                .as_ref()
                .unwrap()
                .borrow_mut()
                .next = Some(next.clone());
            cpu.bus.ppu.pfc.fifo.tail = Some(next.clone());
        }

        cpu.bus.ppu.pfc.fifo.size += 1;
    }

    fn pixel_fifo_pop(cpu: &mut Cpu) -> Color {
        if cpu.bus.ppu.pfc.fifo.size == 0 {
            panic!("FIFO underflow");
        }

        if cpu.bus.ppu.pfc.fifo.head.is_none() {
            panic!("FIFO head is None");
        }

        let head = cpu.bus.ppu.pfc.fifo.head.clone().unwrap();
        let val = head.borrow().value;
        cpu.bus.ppu.pfc.fifo.head = head.borrow().next.clone();

        cpu.bus.ppu.pfc.fifo.size -= 1;
        val
    }

    fn pipeline_push_pixel(cpu: &mut Cpu) {
        if cpu.bus.ppu.pfc.fifo.size > 8 {
            let pixel_data = Self::pixel_fifo_pop(cpu);

            if cpu.bus.ppu.pfc.line_x >= cpu.bus.lcd.scx % 8 {
                cpu.bus.ppu.video_buffer[cpu
                    .bus
                    .ppu
                    .pfc
                    .pushed_x
                    .wrapping_add((cpu.bus.lcd.ly as i32 * XRES) as u8)
                    as usize] = pixel_data;

                cpu.bus.ppu.pfc.pushed_x += 1;
            }

            cpu.bus.ppu.pfc.line_x += 1;
        }
    }

    fn pipeline_fetch(cpu: &mut Cpu) {
        let fetch_state = &cpu.bus.ppu.pfc.current_fetch_state;

        match fetch_state {
            FetchState::Tile => {
                if cpu.bus.lcd.bgw_enabled() != 0 {
                    let address = cpu.bus.lcd.bg_map_area()
                        + (cpu.bus.ppu.pfc.map_x / 8) as u16
                        + ((cpu.bus.ppu.pfc.map_y / 8) as u16 * 32);
                    cpu.bus.ppu.pfc.bgw_fetch_data[0] = Bus::read(cpu, address);

                    if cpu.bus.lcd.bgw_data_area() == 0x8800 {
                        cpu.bus.ppu.pfc.bgw_fetch_data[0] =
                            cpu.bus.ppu.pfc.bgw_fetch_data[0].wrapping_add(128);
                    }
                }

                cpu.bus.ppu.pfc.current_fetch_state = FetchState::Data0;
                cpu.bus.ppu.pfc.fetch_x = cpu.bus.ppu.pfc.fetch_x.wrapping_add(8);
            }
            FetchState::Data0 => {
                let address = cpu.bus.lcd.bgw_data_area()
                    + (cpu.bus.ppu.pfc.bgw_fetch_data[0] as u16 * 16)
                    + (cpu.bus.ppu.pfc.tile_y) as u16;
                cpu.bus.ppu.pfc.bgw_fetch_data[1] = Bus::read(cpu, address);

                cpu.bus.ppu.pfc.current_fetch_state = FetchState::Data1;
            }
            FetchState::Data1 => {
                let address = cpu.bus.lcd.bgw_data_area()
                    + (cpu.bus.ppu.pfc.bgw_fetch_data[0] as u16 * 16)
                    + (cpu.bus.ppu.pfc.tile_y + 1) as u16;
                cpu.bus.ppu.pfc.bgw_fetch_data[2] = Bus::read(cpu, address);

                cpu.bus.ppu.pfc.current_fetch_state = FetchState::Idle;
            }
            FetchState::Idle => {
                cpu.bus.ppu.pfc.current_fetch_state = FetchState::Push;
            }
            FetchState::Push => {
                if Self::pipeline_fifo_add(cpu) {
                    cpu.bus.ppu.pfc.current_fetch_state = FetchState::Tile;
                }
            }
        }
    }

    fn pipeline_fifo_add(cpu: &mut Cpu) -> bool {
        if cpu.bus.ppu.pfc.fifo.size > 8 {
            // FiFo is Full
            return false;
        }

        let x: i8 = (cpu
            .bus
            .ppu
            .pfc
            .fetch_x
            .wrapping_sub(8 - (cpu.bus.lcd.scx % 8))) as i8;

        for i in 0..8 {
            let bit = 7 - i;
            let lo: u8 = (cpu.bus.ppu.pfc.bgw_fetch_data[1] & (1 << bit) != 0) as u8;
            let hi: u8 = ((cpu.bus.ppu.pfc.bgw_fetch_data[2] & (1 << bit)) << 1 != 0) as u8;

            let color: Color = TILE_COLORS[(hi | lo) as usize];

            if x >= 0 {
                Self::pixel_fifo_push(cpu, color);
                cpu.bus.ppu.pfc.fifo_x += 1;
            }
        }

        true
    }

    fn pipeline_process(cpu: &mut Cpu) {
        let ppu = &mut cpu.bus.ppu;
        let lcd = &mut cpu.bus.lcd;

        ppu.pfc.map_y = lcd.ly.wrapping_add(lcd.scy);
        ppu.pfc.map_x = ppu.pfc.fetch_x.wrapping_add(lcd.scx);
        ppu.pfc.tile_y = ((lcd.ly.wrapping_add(lcd.scy)) % 8) * 2;

        if ppu.line_ticks & 1 == 0 {
            Self::pipeline_fetch(cpu);
        }

        Self::pipeline_push_pixel(cpu);
    }

    fn pipeline_fifo_reset(cpu: &mut Cpu) {
        while cpu.bus.ppu.pfc.fifo.size > 0 {
            Self::pixel_fifo_pop(cpu);
        }

        cpu.bus.ppu.pfc.fifo.head = None;
    }
}

pub enum FetchState {
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}
