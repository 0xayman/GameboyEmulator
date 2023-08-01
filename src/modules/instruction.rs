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

impl Default for Instruction {
    fn default() -> Self {
        Self {
            ins_type: InstructionType::NONE,
            addr_mode: AddressMode::IMP,
            reg1: RegisterType::NONE,
            reg2: RegisterType::NONE,
            cond_type: ConditionType::NONE,
            param: 0,
        }
    }
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
            0x00 => Instruction::default(),
            0x01 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD16,
                reg1: RegisterType::BC,
                ..Instruction::default()
            },
            0x02 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::BC,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x05 => Instruction {
                ins_type: InstructionType::DEC,
                addr_mode: AddressMode::R,
                reg1: RegisterType::B,
                ..Instruction::default()
            },
            0x06 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD8,
                reg1: RegisterType::B,
                ..Instruction::default()
            },
            0x08 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::A16R,
                reg2: RegisterType::SP,
                ..Instruction::default()
            },
            0x0A => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::A,
                reg2: RegisterType::BC,
                ..Instruction::default()
            },
            0x0E => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD8,
                reg1: RegisterType::C,
                ..Instruction::default()
            },
            0x11 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD16,
                reg1: RegisterType::DE,
                ..Instruction::default()
            },
            0x12 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::DE,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x15 => Instruction {
                ins_type: InstructionType::DEC,
                addr_mode: AddressMode::R,
                reg1: RegisterType::D,
                ..Instruction::default()
            },
            0x16 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD8,
                reg1: RegisterType::D,
                ..Instruction::default()
            },
            0x1A => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::A,
                reg2: RegisterType::DE,
                ..Instruction::default()
            },
            0x1E => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD8,
                reg1: RegisterType::E,
                ..Instruction::default()
            },

            0x21 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD16,
                reg1: RegisterType::HL,
                ..Instruction::default()
            },
            0x22 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::HLIR,
                reg1: RegisterType::HL,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x25 => Instruction {
                ins_type: InstructionType::DEC,
                addr_mode: AddressMode::R,
                reg1: RegisterType::H,
                ..Instruction::default()
            },
            0x26 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD8,
                reg1: RegisterType::H,
                ..Instruction::default()
            },
            0x2A => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RHLI,
                reg1: RegisterType::A,
                reg2: RegisterType::HL,
                ..Instruction::default()
            },
            0x2E => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD8,
                reg1: RegisterType::L,
                ..Instruction::default()
            },

            0x31 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD16,
                reg1: RegisterType::SP,
                ..Instruction::default()
            },
            0x32 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::HLDR,
                reg1: RegisterType::HL,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x35 => Instruction {
                ins_type: InstructionType::DEC,
                addr_mode: AddressMode::R,
                reg1: RegisterType::HL,
                ..Instruction::default()
            },
            0x36 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRD8,
                reg1: RegisterType::HL,
                ..Instruction::default()
            },
            0x3A => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RHLD,
                reg1: RegisterType::A,
                reg2: RegisterType::HL,
                ..Instruction::default()
            },
            0x3E => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RD8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },

            0x40 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::B,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x41 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::B,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x42 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::B,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x43 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::B,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x44 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::B,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x45 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::B,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x46 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::B,
                reg2: RegisterType::HL,
                ..Instruction::default()
            },
            0x47 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::B,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x48 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::C,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x49 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::C,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x4A => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::C,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x4B => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::C,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x4C => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::C,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x4D => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::C,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x4E => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::C,
                reg2: RegisterType::HL,
                ..Instruction::default()
            },
            0x4F => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::C,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            0x50 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::D,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x51 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::D,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x52 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::D,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x53 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::D,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x54 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::D,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x55 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::D,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x56 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::D,
                reg2: RegisterType::HL,
                ..Instruction::default()
            },
            0x57 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::D,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x58 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::E,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x59 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::E,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x5A => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::E,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x5B => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::E,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x5C => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::E,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x5D => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::E,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x5E => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::E,
                reg2: RegisterType::HL,
                ..Instruction::default()
            },
            0x5F => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::E,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            0x60 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::H,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x61 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::H,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x62 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::H,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x63 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::H,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x64 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::H,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x65 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::H,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x66 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::H,
                reg2: RegisterType::HL,
                ..Instruction::default()
            },
            0x67 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::H,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x68 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::L,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x69 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::L,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x6A => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::L,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x6B => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::L,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x6C => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::L,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x6D => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::L,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x6E => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::L,
                reg2: RegisterType::HL,
                ..Instruction::default()
            },
            0x6F => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::L,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            0x70 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::HL,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x71 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::HL,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x72 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::HL,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x73 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::HL,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x74 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::HL,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x75 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::HL,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x76 => Instruction {
                ins_type: InstructionType::HALT,
                ..Instruction::default()
            },
            0x77 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::HL,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x78 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x79 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x7A => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x7B => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x7C => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x7D => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x7E => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::A,
                reg2: RegisterType::HL,
                ..Instruction::default()
            },
            0x7F => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RR,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            0xAF => Instruction {
                ins_type: InstructionType::XOR,
                addr_mode: AddressMode::R,
                reg1: RegisterType::A,
                ..Instruction::default()
            },

            0xC3 => Instruction {
                ins_type: InstructionType::JP,
                addr_mode: AddressMode::D16,
                ..Instruction::default()
            },

            0xE2 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::MRR,
                reg1: RegisterType::C,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0xEA => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::A16R,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            0xF2 => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RMR,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0xF3 => Instruction {
                ins_type: InstructionType::DI,
                addr_mode: AddressMode::IMP,
                ..Instruction::default()
            },
            0xFA => Instruction {
                ins_type: InstructionType::LD,
                addr_mode: AddressMode::RA16,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            other => panic!("Unknown opcode: {:#04x}", other),
        }
    }

    pub fn instruction_name(ins_type: InstructionType) {}
}
