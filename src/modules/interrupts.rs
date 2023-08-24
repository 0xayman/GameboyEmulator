pub mod interrupt {
    use crate::{
        enums::interrupt_types::InterruptType,
        modules::{cpu::Cpu, stack::Stack},
    };

    fn map_interrupt_type_to_u8(interrupt_type: InterruptType) -> u8 {
        match interrupt_type {
            InterruptType::Vblank => 0x01,
            InterruptType::LcdStat => 0x02,
            InterruptType::Timer => 0x04,
            InterruptType::Serial => 0x08,
            InterruptType::Joybad => 0x10,
        }
    }

    pub fn request(cpu: &mut Cpu, interrupt_type: InterruptType) {
        cpu.interrupt_flags |= map_interrupt_type_to_u8(interrupt_type);
    }

    fn process(cpu: &mut Cpu, address: u16) {
        Stack::push16(cpu, cpu.registers.pc);
        cpu.registers.pc = address;
    }

    fn check(cpu: &mut Cpu, address: u16, interrupt_type: InterruptType) -> bool {
        let it: u8 = map_interrupt_type_to_u8(interrupt_type);
        if cpu.interrupt_flags & it != 0 && (cpu.ie_register & it) != 0 {
            process(cpu, address);
            cpu.interrupt_flags &= !it;
            cpu.halted = false;
            cpu.int_master_enabled = false;

            return true;
        }

        false
    }

    pub fn handle(cpu: &mut Cpu) {
        match () {
            _ if check(cpu, 0x40, InterruptType::Vblank) => (),
            _ if check(cpu, 0x48, InterruptType::LcdStat) => (),
            _ if check(cpu, 0x50, InterruptType::Timer) => (),
            _ if check(cpu, 0x58, InterruptType::Serial) => (),
            _ if check(cpu, 0x60, InterruptType::Joybad) => (),
            _ => (),
        }
    }
}
