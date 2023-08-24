use crate::modules::{cpu::Cpu, timer::Timer};

use super::lcd::Lcd;

pub struct IO {
    pub serial_data: [u8; 2],
}

impl IO {
    pub fn new() -> Self {
        Self {
            serial_data: [0; 2],
        }
    }

    pub fn read(cpu: &Cpu, address: u16) -> u8 {
        match address {
            0xFF01 => cpu.bus.io.serial_data[0],
            0xFF02 => cpu.bus.io.serial_data[1],
            0xFF04..=0xFF07 => Timer::read(cpu, address),
            0xFF0F => cpu.interrupt_flags,
            0xFF40..=0xFF4B => Lcd::read(cpu, address),

            _ => unimplemented!(),
        }
    }

    pub fn write(cpu: &mut Cpu, address: u16, value: u8) {
        match address {
            0xFF01 => cpu.bus.io.serial_data[0] = value,
            0xFF02 => cpu.bus.io.serial_data[1] = value,
            0xFF04..=0xFF07 => {
                Timer::write(cpu, address, value);
            }
            0xFF0F => {
                cpu.interrupt_flags = value;
            }
            0xFF40..=0xFF4B => {
                Lcd::write(cpu, address, value);
            }
            _ => {
                // println!("IO write not implemented for address: {:X}", address);
            }
        };
    }
}
