use crate::enums::interrupt_types::InterruptType;

use super::{
    cpu::CPU,
    emu::Emu,
    interrupts::interrupt,
    lcd::{LCDMode, StatSrc, LCD},
};

const LINES_PER_FRAME: u32 = 154;
const TICKS_PER_LINE: u32 = 456;
const YRES: u32 = 144;
const XRES: u32 = 160;

pub struct PPU {
    pub oam_ram: [u8; 0xA0],
    pub vram: [u8; 0x2000],

    pub current_frame: u32,
    pub line_ticks: u32,
    pub video_buffer: [u32; (XRES * YRES) as usize],

    pub target_frame_time: u64,
    pub prev_frame_time: u64,
    pub start_time: u64,
    pub frame_count: u64,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            oam_ram: [0; 0xA0],
            vram: [0; 0x2000],

            current_frame: 0,
            line_ticks: 0,
            video_buffer: [0; (XRES * YRES) as usize],

            target_frame_time: 1000 / 60,
            prev_frame_time: 0,
            start_time: 0,
            frame_count: 0,
        }
    }

    pub fn init(cpu: &mut CPU) {
        cpu.bus.ppu.current_frame = 0;
        cpu.bus.ppu.line_ticks = 0;
        cpu.bus.ppu.video_buffer = [0; (XRES * YRES) as usize];

        LCD::init(&mut cpu.bus.lcd);
        LCD::set_lcds_mode(&mut cpu.bus.lcd, super::lcd::LCDMode::OAM);

        cpu.bus.ppu.oam_ram = [0; 0xA0];
    }

    pub fn tick(cpu: &mut CPU) {
        cpu.bus.ppu.line_ticks += 1;

        match cpu.bus.lcd.get_lcds_mode() {
            LCDMode::OAM => Self::mode_oam(cpu),
            LCDMode::XFER => Self::mode_xfer(cpu),
            LCDMode::VBLANK => Self::mode_vblank(cpu),
            LCDMode::HBLANK => Self::mode_hblank(cpu),
        }
    }

    pub fn oam_write(&mut self, address: u16, value: u8) {
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

    fn mode_oam(cpu: &mut CPU) {
        if cpu.bus.ppu.line_ticks >= 80 {
            cpu.bus.lcd.set_lcds_mode(LCDMode::XFER);
        }
    }
    fn mode_xfer(cpu: &mut CPU) {
        if cpu.bus.ppu.line_ticks >= 80 + 172 {
            cpu.bus.lcd.set_lcds_mode(LCDMode::HBLANK);
        }
    }
    fn mode_vblank(cpu: &mut CPU) {
        if cpu.bus.ppu.line_ticks >= TICKS_PER_LINE {
            LCD::increment_ly(cpu);

            if cpu.bus.lcd.ly >= (LINES_PER_FRAME as u8) {
                cpu.bus.lcd.set_lcds_mode(LCDMode::OAM);
                cpu.bus.lcd.ly = 0;
            }

            cpu.bus.ppu.line_ticks = 0;
        }
    }
    fn mode_hblank(cpu: &mut CPU) {
        if cpu.bus.ppu.line_ticks >= TICKS_PER_LINE {
            LCD::increment_ly(cpu);

            if cpu.bus.lcd.ly >= YRES as u8 {
                cpu.bus.lcd.set_lcds_mode(LCDMode::VBLANK);

                interrupt::request(cpu, InterruptType::VBLANK);

                if cpu.bus.lcd.stat_interrupt(StatSrc::VBLANK) != 0 {
                    interrupt::request(cpu, InterruptType::LCDSTAT);
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
                cpu.bus.lcd.set_lcds_mode(LCDMode::OAM);
            }

            cpu.bus.ppu.line_ticks = 0;
        }
    }
}
