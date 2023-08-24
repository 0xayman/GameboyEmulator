use crate::enums::register_type::RegisterType;
use crate::modules::bus::Bus;
use crate::modules::cpu::Cpu;

impl Cpu {
    pub fn read_register(&self, reg_type: RegisterType) -> u16 {
        match reg_type {
            RegisterType::A => self.registers.a as u16,
            RegisterType::F => self.registers.f as u16,
            RegisterType::B => self.registers.b as u16,
            RegisterType::C => self.registers.c as u16,
            RegisterType::D => self.registers.d as u16,
            RegisterType::E => self.registers.e as u16,
            RegisterType::H => self.registers.h as u16,
            RegisterType::L => self.registers.l as u16,

            RegisterType::Af => ((self.registers.a as u16) << 8) | (self.registers.f as u16),
            RegisterType::Bc => ((self.registers.b as u16) << 8) | (self.registers.c as u16),
            RegisterType::De => ((self.registers.d as u16) << 8) | (self.registers.e as u16),
            RegisterType::Hl => ((self.registers.h as u16) << 8) | (self.registers.l as u16),

            RegisterType::Sp => self.registers.sp,
            RegisterType::Pc => self.registers.pc,
            RegisterType::None => 0,
        }
    }

    pub fn read_register_8bits(&self, reg_type: RegisterType) -> u8 {
        match reg_type {
            RegisterType::A => self.registers.a,
            RegisterType::F => self.registers.f,
            RegisterType::B => self.registers.b,
            RegisterType::C => self.registers.c,
            RegisterType::D => self.registers.d,
            RegisterType::E => self.registers.e,
            RegisterType::H => self.registers.h,
            RegisterType::L => self.registers.l,
            RegisterType::Hl => Bus::read(self, self.read_register(RegisterType::Hl)),
            _other => panic!("INVALID REGISTER TYPE: {:?}", reg_type),
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

            RegisterType::Af => {
                self.registers.a = ((data & 0xFF00) >> 8) as u8;
                self.registers.f = data as u8;
            }
            RegisterType::Bc => {
                self.registers.b = ((data & 0xFF00) >> 8) as u8;
                self.registers.c = data as u8;
            }
            RegisterType::De => {
                self.registers.d = ((data & 0xFF00) >> 8) as u8;
                self.registers.e = data as u8;
            }
            RegisterType::Hl => {
                self.registers.h = ((data & 0xFF00) >> 8) as u8;
                self.registers.l = data as u8;
            }
            RegisterType::Sp => self.registers.sp = data,
            RegisterType::Pc => self.registers.pc = data,
            RegisterType::None => (),
        }
    }

    pub fn set_register_8bits(&mut self, reg_type: RegisterType, data: u8) {
        match reg_type {
            RegisterType::A => self.registers.a = data,
            RegisterType::F => self.registers.f = data,
            RegisterType::B => self.registers.b = data,
            RegisterType::C => self.registers.c = data,
            RegisterType::D => self.registers.d = data,
            RegisterType::E => self.registers.e = data,
            RegisterType::H => self.registers.h = data,
            RegisterType::L => self.registers.l = data,
            RegisterType::Hl => Bus::write(self, self.read_register(RegisterType::Hl), data),
            _other => panic!("INVALID REGISTER TYPE: {:?}", reg_type),
        }
    }
}
