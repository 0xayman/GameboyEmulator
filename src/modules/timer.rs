use crate::{enums::interrupt_types::InterruptType, modules::interrupts::Interrupt};

use crate::modules::cpu::CPU;

pub struct Timer {
    pub div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    pub ticks: u64,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            ticks: 0,
        }
    }
}

impl Timer {
    pub fn init(&mut self) {
        self.div = 0xAC00;
    }

    pub fn tick(cpu: &mut CPU) {
        let prev_div = cpu.timer.div;
        cpu.timer.div.wrapping_add(1);

        let mut timer_update: bool = false;

        match (cpu.timer.tac & 0b11) {
            0b00 => {
                timer_update = (((prev_div & (1 << 9)) != 0) && (!(cpu.timer.div & (1 << 9)) != 0));
            }
            0b01 => {
                timer_update = (((prev_div & (1 << 3)) != 0) && (!(cpu.timer.div & (1 << 3)) != 0));
            }
            0b10 => {
                timer_update = (((prev_div & (1 << 5)) != 0) && (!(cpu.timer.div & (1 << 5)) != 0));
            }
            0b11 => {
                timer_update = (((prev_div & (1 << 7)) != 0) && (!(cpu.timer.div & (1 << 7)) != 0));
            }
            _ => {}
        }

        if (timer_update && ((cpu.timer.tac & (1 << 2)) == 1)) {
            cpu.timer.tima += 1;

            if cpu.timer.tima == 0xFF {
                cpu.timer.tima = cpu.timer.tma;
                Interrupt::request(cpu, InterruptType::TIMER);
            }
        }
    }

    pub fn cycles(cpu: &mut CPU, cycles: u64) {
        let n = cycles * 4;

        for _ in 0..n {
            cpu.timer.ticks += 1;
            Self::tick(cpu);
        }
    }

    pub fn write(cpu: &mut CPU, address: u16, value: u8) {
        match address {
            0xFF04 => {
                cpu.timer.div = 0;
            }
            0xFF05 => {
                cpu.timer.tima = value;
            }
            0xFF06 => {
                cpu.timer.tma = value;
            }
            0xFF07 => {
                cpu.timer.tac = value;
            }
            _ => {}
        }
    }

    pub fn read(cpu: &CPU, address: u16) -> u8 {
        match address {
            0xFF04 => (cpu.timer.div >> 8) as u8,
            0xFF05 => cpu.timer.tima,
            0xFF06 => cpu.timer.tma,
            0xFF07 => cpu.timer.tac,
            _ => 0,
        }
    }
}
