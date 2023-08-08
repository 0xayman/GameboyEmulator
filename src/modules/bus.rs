use crate::modules::cart::Cart;
use crate::modules::cpu::CPU;
use crate::modules::ram::RAM;
pub struct Bus<'a> {
    cart: &'a mut Cart,
    ram: &'a mut RAM,
}

impl<'a> Bus<'a> {
    pub fn new(cart: &'a mut Cart, ram: &'a mut RAM) -> Self {
        Self {
            cart: cart,
            ram: ram,
        }
    }

    pub fn read(cpu: &CPU, address: u16) -> u8 {
        return match address {
            0x0000..=0x7FFF => cpu.bus.cart.read(address), // ROM
            0x8000..=0x9FFF => panic!("PPU read not implemented for address: {:X}", address), // CHAR DATA
            0xA000..=0xBFFF => cpu.bus.cart.read(address), // CART RAM
            0xC000..=0xDFFF => cpu.bus.ram.wram_read(address), // WRAM
            0xE000..=0xFDFF => 0,                          // Reserverd ECHO RAM,
            0xFE00..=0xFE9F => {
                println!("OAM read not implemented for address: {:X}", address); // OAM
                return 0x0;
            }
            0xFEA0..=0xFEFF => 0, // Reserved
            0xFF00..=0xFF7F => {
                println!("IO read not implemented for address: {:X}", address); // IO
                return 0x0;
            }
            0xFFFF => cpu.get_ie_register(), // CPU ENABLE REGISTER
            _ => cpu.bus.ram.hram_read(address),
        };
        panic!("Bus read not implemented for address: {:X}", address);
    }

    pub fn write(cpu: &mut CPU, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => cpu.bus.cart.write(address, value), // ROM
            0x8000..=0x9FFF => println!("PPU write not implemented for address: {:X}", address), // CHAR DATA
            0xA000..=0xBFFF => cpu.bus.cart.write(address, value), // CART RAM
            0xC000..=0xDFFF => cpu.bus.ram.wram_write(address, value), // WRAM
            0xE000..=0xFDFF => (),                                 // Reserverd ECHO RAM,
            0xFE00..=0xFE9F => println!("OAM write not implemented for address: {:X}", address), // OAM
            0xFEA0..=0xFEFF => (), // Reserved
            0xFF00..=0xFF7F => println!("IO write not implemented for address: {:X}", address), // IO
            0xFFFF => cpu.set_ie_register(value), // CPU ENABLE REGISTER
            _ => cpu.bus.ram.hram_write(address, value),
        }
    }

    pub fn read16(cpu: &CPU, address: u16) -> u16 {
        let lo: u16 = Self::read(cpu, address) as u16;
        let hi: u16 = Self::read(cpu, address + 1) as u16;

        return lo | (hi << 8);
    }

    pub fn write16(cpu: &mut CPU, address: u16, data: u16) {
        Self::write(cpu, address + 1, ((data >> 8) & 0xFF) as u8);
        Self::write(cpu, address, (data & 0xFF) as u8);
    }
}
