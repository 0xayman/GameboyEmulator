use crate::enums::{address_mode::AddressMode, register_type::RegisterType};
use crate::modules::{cpu::CPU, emu::Emu};

impl<'a> CPU<'a> {
    pub fn read_register(&self, reg_type: RegisterType) -> u16 {
        match reg_type {
            RegisterType::A => return self.registers.a as u16,
            RegisterType::F => return self.registers.f as u16,
            RegisterType::B => return self.registers.b as u16,
            RegisterType::C => return self.registers.c as u16,
            RegisterType::D => return self.registers.d as u16,
            RegisterType::E => return self.registers.e as u16,
            RegisterType::H => return self.registers.h as u16,
            RegisterType::L => return self.registers.l as u16,

            RegisterType::AF => {
                return ((self.registers.a as u16) << 8) | (self.registers.f as u16)
            }
            RegisterType::BC => {
                return ((self.registers.b as u16) << 8) | (self.registers.c as u16)
            }
            RegisterType::DE => {
                return ((self.registers.d as u16) << 8) | (self.registers.e as u16)
            }
            RegisterType::HL => {
                return ((self.registers.h as u16) << 8) | (self.registers.l as u16)
            }

            RegisterType::SP => return self.registers.sp,
            RegisterType::PC => return self.registers.pc,
            RegisterType::NONE => return 0,
        }
    }

    pub fn set_register(&mut self, reg_type: RegisterType, data: u16) {
        match reg_type {
            RegisterType::A => self.registers.a = data as u8,
            RegisterType::F => self.registers.f = data as u8,
            RegisterType::B => self.registers.b = data as u8,
            RegisterType::C => self.registers.c = data as u8,
            RegisterType::D => self.registers.d = data as u8,
            RegisterType::E => self.registers.e = data as u8,
            RegisterType::H => self.registers.h = data as u8,
            RegisterType::L => self.registers.l = data as u8,

            RegisterType::AF => {
                self.registers.a = ((data & 0xFF00) >> 8) as u8;
                self.registers.f = data as u8;
            }
            RegisterType::BC => {
                self.registers.b = ((data & 0xFF00) >> 8) as u8;
                self.registers.c = data as u8;
            }
            RegisterType::DE => {
                self.registers.d = ((data & 0xFF00) >> 8) as u8;
                self.registers.e = data as u8;
            }
            RegisterType::HL => {
                self.registers.h = ((data & 0xFF00) >> 8) as u8;
                self.registers.l = data as u8;
            }
            RegisterType::SP => self.registers.sp = data,
            RegisterType::PC => self.registers.pc = data,
            RegisterType::NONE => return,
        }
    }
}
