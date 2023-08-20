use crate::modules::cart::Cart;
use crate::modules::cpu::CPU;
use crate::modules::io::IO;
use crate::modules::ram::RAM;

use super::ppu::PPU;
pub struct Bus {
    pub cart: Cart,
    pub ram: RAM,
    pub io: IO,
    pub ppu: PPU,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cart: Cart::new(),
            ram: RAM::new(),
            io: IO::new(),
            ppu: PPU::new(),
        }
    }

    pub fn read(cpu: &CPU, address: u16) -> u8 {
        return match address {
            0x0000..=0x7FFF => Cart::read(&cpu.bus.cart, address), // ROM
            0x8000..=0x9FFF => PPU::vram_read(&cpu.bus.ppu, address), // CHAR DATA
            0xA000..=0xBFFF => Cart::read(&cpu.bus.cart, address), // CART RAM
            0xC000..=0xDFFF => RAM::wram_read(&cpu.bus.ram, address), // WRAM
            0xE000..=0xFDFF => 0,                                  // Reserverd ECHO RAM,
            0xFE00..=0xFE9F => {
                if cpu.dma.is_trasferring() {
                    return 0xFF;
                }

                return PPU::oam_read(&cpu.bus.ppu, address);
            }
            0xFEA0..=0xFEFF => 0, // Reserved
            0xFF00..=0xFF7F => {
                return IO::read(cpu, address);
            }
            0xFFFF => cpu.get_ie_register(), // CPU ENABLE REGISTER
            _ => RAM::hram_read(&cpu.bus.ram, address),
        };
    }

    pub fn write(cpu: &mut CPU, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => Cart::write(&mut cpu.bus.cart, address, value), // ROM
            0x8000..=0x9FFF => PPU::vram_write(&mut cpu.bus.ppu, address, value), // CHAR DATA
            0xA000..=0xBFFF => Cart::write(&mut cpu.bus.cart, address, value), // CART RAM
            0xC000..=0xDFFF => RAM::wram_write(&mut cpu.bus.ram, address, value), // WRAM
            0xE000..=0xFDFF => (), // Reserverd ECHO RAM,
            0xFE00..=0xFE9F => {
                if cpu.dma.is_trasferring() {
                    return;
                }

                PPU::oam_write(&mut cpu.bus.ppu, address, value);
            } // OAM
            0xFEA0..=0xFEFF => (), // Reserved
            0xFF00..=0xFF7F => {
                println!("IO Write: {:#X} = {:#X}", address, value);
                IO::write(cpu, address, value);
            }
            0xFFFF => cpu.set_ie_register(value), // CPU ENABLE REGISTER
            _ => RAM::hram_write(&mut cpu.bus.ram, address, value),
        }
    }

    pub fn read16(cpu: &CPU, address: u16) -> u16 {
        let lo: u16 = Self::read(cpu, address) as u16;
        let hi: u16 = Self::read(cpu, address + 1) as u16;

        return lo | (hi << 8);
    }

    pub fn write16(cpu: &mut CPU, address: u16, data: u16) {
        Self::write(cpu, address + 1, (data >> 8) as u8);
        Self::write(cpu, address, data as u8);
    }
}
