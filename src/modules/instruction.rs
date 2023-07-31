use crate::enums::address_mode::AddressMode;
use crate::enums::condition_type::ConditionType;
use crate::enums::instruction_type::InstructionType;
use crate::enums::register_type::RegisterType;

pub struct Instruction {
    pub ins_type: InstructionType,
    pub addr_mode: AddressMode,
    pub reg1: RegisterType,
    pub reg2: RegisterType,
    pub cond_type: ConditionType,
    pub param: u8,
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            ins_type: InstructionType::NONE,
            addr_mode: AddressMode::IMP,
            reg1: RegisterType::NONE,
            reg2: RegisterType::NONE,
            cond_type: ConditionType::NONE,
            param: 0,
        }
    }

    pub fn instruction_by_opcode(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction {
                ins_type: InstructionType::NOP,
                addr_mode: AddressMode::IMP,
                reg1: RegisterType::NONE,
                reg2: RegisterType::NONE,
                cond_type: ConditionType::NONE,
                param: 0,
            },
            0x05 => Instruction {
                ins_type: InstructionType::DEC,
                addr_mode: AddressMode::R,
                reg1: RegisterType::B,
                reg2: RegisterType::NONE,
                cond_type: ConditionType::NONE,
                param: 0,
            },
            0x0E => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD8,
                reg1: RegisterType::C,
                reg2: RegisterType::NONE,
                cond_type: ConditionType::NONE,
                param: 0,
            },
            0xAF => Instruction {
                ins_type: InstructionType::XOR,
                addr_mode: AddressMode::R,
                reg1: RegisterType::A,
                reg2: RegisterType::NONE,
                cond_type: ConditionType::NONE,
                param: 0,
            },
            0xC3 => Instruction {
                ins_type: InstructionType::JP,
                addr_mode: AddressMode::D16,
                reg1: RegisterType::NONE,
                reg2: RegisterType::NONE,
                cond_type: ConditionType::NONE,
                param: 0,
            },
            0xf3 => Instruction {
                ins_type: InstructionType::DI,
                addr_mode: AddressMode::IMP,
                reg1: RegisterType::NONE,
                reg2: RegisterType::NONE,
                cond_type: ConditionType::NONE,
                param: 0,
            },
            other => panic!("Unknown opcode: {:#04x}", other),
        }
    }

    pub fn instruction_name(ins_type: InstructionType) {}
}
