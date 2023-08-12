pub mod Interrupt {
    use crate::{
        enums::interrupt_types::InterruptType,
        modules::{cpu::CPU, stack::Stack},
    };

    fn map_interrupt_type_to_u8(interrupt_type: InterruptType) -> u8 {
        return match interrupt_type {
            InterruptType::VBLANK => 0x01,
            InterruptType::LCDSTAT => 0x02,
            InterruptType::TIMER => 0x04,
            InterruptType::SERIAL => 0x08,
            InterruptType::JOYPAD => 0x10,
        };
    }

    pub fn request(cpu: &mut CPU, interrupt_type: InterruptType) {
        cpu.interrupt_flags |= map_interrupt_type_to_u8(interrupt_type);
    }

    fn process(cpu: &mut CPU, address: u16) {
        Stack::push16(cpu, cpu.registers.pc);
        cpu.registers.pc = address;
    }

    fn check(cpu: &mut CPU, address: u16, interrupt_type: InterruptType) -> bool {
        let it: u8 = map_interrupt_type_to_u8(interrupt_type);
        if ((cpu.interrupt_flags & it) != 0 && (cpu.ie_register & it) != 0) {
            process(cpu, address);
            cpu.interrupt_flags &= !it;
            cpu.halted = false;
            cpu.int_master_enabled = false;

            return true;
        }

        return false;
    }

    pub fn handle(cpu: &mut CPU) {
        if (check(cpu, 0x40, InterruptType::VBLANK)) {
            return;
        } else if (check(cpu, 0x48, InterruptType::LCDSTAT)) {
            return;
        } else if (check(cpu, 0x50, InterruptType::TIMER)) {
            return;
        } else if (check(cpu, 0x58, InterruptType::SERIAL)) {
            return;
        } else if (check(cpu, 0x60, InterruptType::JOYPAD)) {
            return;
        }
    }
}
