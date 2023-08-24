use crate::modules::cpu::Cpu;

use super::bus::Bus;

pub struct Stack {}

impl Stack {
    pub fn push(cpu: &mut Cpu, data: u8) {
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        Bus::write(cpu, cpu.registers.sp, data);
    }

    pub fn pop(cpu: &mut Cpu) -> u8 {
        let val = Bus::read(cpu, cpu.registers.sp);
        cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
        val
    }

    pub fn push16(cpu: &mut Cpu, data: u16) {
        Self::push(cpu, ((data >> 8) & 0xFF) as u8);
        Self::push(cpu, (data & 0xFF) as u8);
    }

    pub fn pop16(cpu: &mut Cpu) -> u16 {
        let lo: u16 = Self::pop(cpu) as u16;
        let hi: u16 = Self::pop(cpu) as u16;

        (hi << 8) | lo
    }
}
