pub mod processor {
    use crate::enums::instruction_type::InstructionType;
    use crate::modules::common::set_bit;
    use crate::modules::cpu::CPU;

    fn process_none(cpu: &mut CPU) {
        println!("INVALID INSTRUCTION");
        return;
    }

    fn process_nop(cpu: &mut CPU) {
        //
    }

    fn process_di(cpu: &mut CPU) {
        cpu.int_master_enabled = false;
    }

    fn process_ld(cpu: &mut CPU) {
        // TODO: Implement
    }

    fn process_xor(cpu: &mut CPU) {
        cpu.registers.a ^= cpu.fetched_data as u8 & 0xFF;
        set_flags(cpu, (cpu.registers.a == 0) as i32, 0, 0, 0);
    }

    fn process_jp(cpu: &mut CPU) {}

    fn set_flags(cpu: &mut CPU, z: i32, n: i32, h: i32, c: i32) {
        if z != -1 {
            set_bit(cpu.registers.f, 7, z == 1);
        }
        if n != -1 {
            set_bit(cpu.registers.f, 6, n == 1);
        }
        if h != -1 {
            set_bit(cpu.registers.f, 5, h == 1);
        }
        if c != -1 {
            set_bit(cpu.registers.f, 4, c == 1);
        }
    }

    pub fn get_processor_by_instruction_type(ins_type: &InstructionType) -> fn(&mut CPU) {
        return match ins_type {
            InstructionType::NONE => process_none,
            InstructionType::NOP => process_nop,
            InstructionType::LD => process_ld,
            InstructionType::JP => process_jp,
            InstructionType::DI => process_di,
            InstructionType::XOR => process_xor,
            other => panic!("Cannot Process this instruction: {:#?}", other),
        };
    }
}
