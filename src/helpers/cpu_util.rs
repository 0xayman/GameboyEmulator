pub mod util {

    use crate::{enums::register_type::RegisterType, modules::cpu::CPU};

    pub fn read_register(cpu: &CPU, reg_type: &RegisterType) -> u16 {
        match reg_type {
            RegisterType::A => return cpu.registers.a as u16,
            RegisterType::F => return cpu.registers.f as u16,
            RegisterType::B => return cpu.registers.b as u16,
            RegisterType::C => return cpu.registers.c as u16,
            RegisterType::D => return cpu.registers.d as u16,
            RegisterType::E => return cpu.registers.e as u16,
            RegisterType::H => return cpu.registers.h as u16,
            RegisterType::L => return cpu.registers.l as u16,

            RegisterType::AF => return (cpu.registers.a as u16) << 8 | cpu.registers.f as u16,
            RegisterType::BC => return (cpu.registers.b as u16) << 8 | cpu.registers.c as u16,
            RegisterType::DE => return (cpu.registers.d as u16) << 8 | cpu.registers.e as u16,
            RegisterType::HL => return (cpu.registers.h as u16) << 8 | cpu.registers.l as u16,

            RegisterType::SP => return cpu.registers.sp,
            RegisterType::PC => return cpu.registers.pc,
            other => panic!("Cannot read this register: {:#?}", other),
        }
    }
}
