use crate::enums::address_mode::AddressMode;
use crate::enums::condition_type::ConditionType;
use crate::enums::instruction_type::InstructionType;
use crate::enums::register_type::RegisterType;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub ins_type: InstructionType,
    pub addr_mode: AddressMode,
    pub reg1: RegisterType,
    pub reg2: RegisterType,
    pub cond_type: ConditionType,
    pub param: Option<u8>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            ins_type: InstructionType::None,
            addr_mode: AddressMode::Imp,
            reg1: RegisterType::None,
            reg2: RegisterType::None,
            cond_type: ConditionType::None,
            param: None,
        }
    }
}

impl Instruction {
    pub fn instruction_by_opcode(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction {
                ins_type: InstructionType::Nop,
                ..Instruction::default()
            },
            0x01 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd16,
                reg1: RegisterType::Bc,
                ..Instruction::default()
            },
            0x02 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::Bc,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x03 => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Bc,
                ..Instruction::default()
            },
            0x04 => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::B,
                ..Instruction::default()
            },
            0x05 => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::B,
                ..Instruction::default()
            },
            0x06 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::B,
                ..Instruction::default()
            },
            0x07 => Instruction {
                ins_type: InstructionType::Rlca,
                ..Instruction::default()
            },
            0x08 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::A16R,
                reg2: RegisterType::Sp,
                ..Instruction::default()
            },
            0x09 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::Hl,
                reg2: RegisterType::Bc,
                ..Instruction::default()
            },
            0x0A => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Bc,
                ..Instruction::default()
            },
            0x0B => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Bc,
                ..Instruction::default()
            },
            0x0C => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::C,
                ..Instruction::default()
            },
            0x0D => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::C,
                ..Instruction::default()
            },
            0x0E => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::C,
                ..Instruction::default()
            },
            0x0F => Instruction {
                ins_type: InstructionType::Rrca,
                ..Instruction::default()
            },

            //0x1X
            0x10 => Instruction {
                ins_type: InstructionType::Stop,
                ..Instruction::default()
            },
            0x11 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd16,
                reg1: RegisterType::De,
                ..Instruction::default()
            },
            0x12 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::De,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x13 => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::De,
                ..Instruction::default()
            },
            0x14 => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::D,
                ..Instruction::default()
            },
            0x15 => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::D,
                ..Instruction::default()
            },
            0x16 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::D,
                ..Instruction::default()
            },
            0x17 => Instruction {
                ins_type: InstructionType::Rla,
                ..Instruction::default()
            },
            0x18 => Instruction {
                ins_type: InstructionType::Jr,
                addr_mode: AddressMode::D8,
                ..Instruction::default()
            },
            0x19 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::Hl,
                reg2: RegisterType::De,
                ..Instruction::default()
            },
            0x1A => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::De,
                ..Instruction::default()
            },
            0x1B => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::De,
                ..Instruction::default()
            },
            0x1C => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::E,
                ..Instruction::default()
            },
            0x1D => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::E,
                ..Instruction::default()
            },
            0x1E => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::E,
                ..Instruction::default()
            },
            0x1F => Instruction {
                ins_type: InstructionType::Rra,
                ..Instruction::default()
            },

            //0x2X
            0x20 => Instruction {
                ins_type: InstructionType::Jr,
                addr_mode: AddressMode::D8,
                cond_type: ConditionType::Nz,
                ..Instruction::default()
            },
            0x21 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd16,
                reg1: RegisterType::Hl,
                ..Instruction::default()
            },
            0x22 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::HliR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x23 => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Hl,
                ..Instruction::default()
            },
            0x24 => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::H,
                ..Instruction::default()
            },
            0x25 => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::H,
                ..Instruction::default()
            },
            0x26 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::H,
                ..Instruction::default()
            },
            0x27 => Instruction {
                ins_type: InstructionType::Daa,
                ..Instruction::default()
            },
            0x28 => Instruction {
                ins_type: InstructionType::Jr,
                addr_mode: AddressMode::D8,
                cond_type: ConditionType::Z,
                ..Instruction::default()
            },
            0x29 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::Hl,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x2A => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rhli,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x2B => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Hl,
                ..Instruction::default()
            },
            0x2C => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::L,
                ..Instruction::default()
            },
            0x2D => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::L,
                ..Instruction::default()
            },
            0x2E => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::L,
                ..Instruction::default()
            },
            0x2F => Instruction {
                ins_type: InstructionType::Cpl,
                ..Instruction::default()
            },

            //0x3X
            0x30 => Instruction {
                ins_type: InstructionType::Jr,
                addr_mode: AddressMode::D8,
                cond_type: ConditionType::Nc,
                ..Instruction::default()
            },
            0x31 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd16,
                reg1: RegisterType::Sp,
                ..Instruction::default()
            },
            0x32 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::HldR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x33 => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Sp,
                ..Instruction::default()
            },
            0x34 => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::Mr,
                reg1: RegisterType::Hl,
                ..Instruction::default()
            },
            0x35 => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::Mr,
                reg1: RegisterType::Hl,
                ..Instruction::default()
            },
            0x36 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrD8,
                reg1: RegisterType::Hl,
                ..Instruction::default()
            },
            0x37 => Instruction {
                ins_type: InstructionType::Scf,
                ..Instruction::default()
            },
            0x38 => Instruction {
                ins_type: InstructionType::Jr,
                addr_mode: AddressMode::D8,
                cond_type: ConditionType::C,
                ..Instruction::default()
            },
            0x39 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::Hl,
                reg2: RegisterType::Sp,
                ..Instruction::default()
            },
            0x3A => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rhld,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x3B => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Sp,
                ..Instruction::default()
            },
            0x3C => Instruction {
                ins_type: InstructionType::Inc,
                addr_mode: AddressMode::R,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0x3D => Instruction {
                ins_type: InstructionType::Dec,
                addr_mode: AddressMode::R,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0x3E => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0x3F => Instruction {
                ins_type: InstructionType::Ccf,
                ..Instruction::default()
            },

            //0x4X
            0x40 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::B,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x41 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::B,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x42 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::B,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x43 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::B,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x44 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::B,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x45 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::B,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x46 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::B,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x47 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::B,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x48 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::C,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x49 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::C,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x4A => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::C,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x4B => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::C,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x4C => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::C,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x4D => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::C,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x4E => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::C,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x4F => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::C,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            //0x5X
            0x50 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::D,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x51 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::D,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x52 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::D,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x53 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::D,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x54 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::D,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x55 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::D,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x56 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::D,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x57 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::D,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x58 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::E,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x59 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::E,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x5A => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::E,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x5B => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::E,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x5C => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::E,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x5D => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::E,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x5E => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::E,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x5F => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::E,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            //0x6X
            0x60 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::H,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x61 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::H,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x62 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::H,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x63 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::H,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x64 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::H,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x65 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::H,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x66 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::H,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x67 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::H,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x68 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::L,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x69 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::L,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x6A => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::L,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x6B => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::L,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x6C => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::L,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x6D => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::L,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x6E => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::L,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x6F => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::L,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            //0x7X
            0x70 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x71 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x72 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x73 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x74 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x75 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x76 => Instruction {
                ins_type: InstructionType::Halt,
                ..Instruction::default()
            },
            0x77 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x78 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x79 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x7A => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x7B => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x7C => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x7D => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x7E => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x7F => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            // 0x8x
            0x80 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x81 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x82 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x83 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x84 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x85 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x86 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x87 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x88 => Instruction {
                ins_type: InstructionType::Adc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x89 => Instruction {
                ins_type: InstructionType::Adc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x8A => Instruction {
                ins_type: InstructionType::Adc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x8B => Instruction {
                ins_type: InstructionType::Adc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x8C => Instruction {
                ins_type: InstructionType::Adc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x8D => Instruction {
                ins_type: InstructionType::Adc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x8E => Instruction {
                ins_type: InstructionType::Adc,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x8F => Instruction {
                ins_type: InstructionType::Adc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            // 0x9x
            0x90 => Instruction {
                ins_type: InstructionType::Sub,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x91 => Instruction {
                ins_type: InstructionType::Sub,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x92 => Instruction {
                ins_type: InstructionType::Sub,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x93 => Instruction {
                ins_type: InstructionType::Sub,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x94 => Instruction {
                ins_type: InstructionType::Sub,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x95 => Instruction {
                ins_type: InstructionType::Sub,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x96 => Instruction {
                ins_type: InstructionType::Sub,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x97 => Instruction {
                ins_type: InstructionType::Sub,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0x98 => Instruction {
                ins_type: InstructionType::Sbc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0x99 => Instruction {
                ins_type: InstructionType::Sbc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0x9A => Instruction {
                ins_type: InstructionType::Sbc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0x9B => Instruction {
                ins_type: InstructionType::Sbc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0x9C => Instruction {
                ins_type: InstructionType::Sbc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0x9D => Instruction {
                ins_type: InstructionType::Sbc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0x9E => Instruction {
                ins_type: InstructionType::Sbc,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0x9F => Instruction {
                ins_type: InstructionType::Sbc,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            // 0xAx
            0xA0 => Instruction {
                ins_type: InstructionType::And,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0xA1 => Instruction {
                ins_type: InstructionType::And,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0xA2 => Instruction {
                ins_type: InstructionType::And,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0xA3 => Instruction {
                ins_type: InstructionType::And,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0xA4 => Instruction {
                ins_type: InstructionType::And,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0xA5 => Instruction {
                ins_type: InstructionType::And,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0xA6 => Instruction {
                ins_type: InstructionType::And,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0xA7 => Instruction {
                ins_type: InstructionType::And,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0xA8 => Instruction {
                ins_type: InstructionType::Xor,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0xA9 => Instruction {
                ins_type: InstructionType::Xor,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0xAA => Instruction {
                ins_type: InstructionType::Xor,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0xAB => Instruction {
                ins_type: InstructionType::Xor,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0xAC => Instruction {
                ins_type: InstructionType::Xor,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0xAD => Instruction {
                ins_type: InstructionType::Xor,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0xAE => Instruction {
                ins_type: InstructionType::Xor,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0xAF => Instruction {
                ins_type: InstructionType::Xor,
                addr_mode: AddressMode::R,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            // 0xBx
            0xB0 => Instruction {
                ins_type: InstructionType::Or,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0xB1 => Instruction {
                ins_type: InstructionType::Or,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0xB2 => Instruction {
                ins_type: InstructionType::Or,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0xB3 => Instruction {
                ins_type: InstructionType::Or,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0xB4 => Instruction {
                ins_type: InstructionType::Or,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0xB5 => Instruction {
                ins_type: InstructionType::Or,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0xB6 => Instruction {
                ins_type: InstructionType::Or,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0xB7 => Instruction {
                ins_type: InstructionType::Or,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0xB8 => Instruction {
                ins_type: InstructionType::Cp,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::B,
                ..Instruction::default()
            },
            0xB9 => Instruction {
                ins_type: InstructionType::Cp,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0xBA => Instruction {
                ins_type: InstructionType::Cp,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::D,
                ..Instruction::default()
            },
            0xBB => Instruction {
                ins_type: InstructionType::Cp,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::E,
                ..Instruction::default()
            },
            0xBC => Instruction {
                ins_type: InstructionType::Cp,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::H,
                ..Instruction::default()
            },
            0xBD => Instruction {
                ins_type: InstructionType::Cp,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::A,
                reg2: RegisterType::L,
                ..Instruction::default()
            },
            0xBE => Instruction {
                ins_type: InstructionType::Cp,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0xBF => Instruction {
                ins_type: InstructionType::Cp,
                addr_mode: AddressMode::R,
                reg1: RegisterType::A,
                reg2: RegisterType::A,
                ..Instruction::default()
            },

            // 0xCx
            0xC0 => Instruction {
                ins_type: InstructionType::Ret,
                cond_type: ConditionType::Nz,
                ..Instruction::default()
            },
            0xC1 => Instruction {
                ins_type: InstructionType::Pop,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Bc,
                ..Instruction::default()
            },
            0xC2 => Instruction {
                ins_type: InstructionType::Jp,
                addr_mode: AddressMode::D16,
                cond_type: ConditionType::Nz,
                ..Instruction::default()
            },
            0xC3 => Instruction {
                ins_type: InstructionType::Jp,
                addr_mode: AddressMode::D16,
                ..Instruction::default()
            },
            0xC4 => Instruction {
                ins_type: InstructionType::Call,
                addr_mode: AddressMode::D16,
                cond_type: ConditionType::Nz,
                ..Instruction::default()
            },
            0xC5 => Instruction {
                ins_type: InstructionType::Push,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Bc,
                ..Instruction::default()
            },
            0xC6 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xC7 => Instruction {
                ins_type: InstructionType::Rst,
                param: Some(0x00),
                ..Instruction::default()
            },
            0xC8 => Instruction {
                ins_type: InstructionType::Ret,
                cond_type: ConditionType::Z,
                ..Instruction::default()
            },
            0xC9 => Instruction {
                ins_type: InstructionType::Ret,
                ..Instruction::default()
            },
            0xCA => Instruction {
                ins_type: InstructionType::Jp,
                addr_mode: AddressMode::D16,
                cond_type: ConditionType::Z,
                ..Instruction::default()
            },
            0xCB => Instruction {
                ins_type: InstructionType::Cb,
                addr_mode: AddressMode::D8,
                ..Instruction::default()
            },
            0xCC => Instruction {
                ins_type: InstructionType::Call,
                addr_mode: AddressMode::D16,
                cond_type: ConditionType::Z,
                ..Instruction::default()
            },
            0xCD => Instruction {
                ins_type: InstructionType::Call,
                addr_mode: AddressMode::D16,
                ..Instruction::default()
            },
            0xCE => Instruction {
                ins_type: InstructionType::Adc,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xCF => Instruction {
                ins_type: InstructionType::Rst,
                param: Some(0x08),
                ..Instruction::default()
            },

            0xD0 => Instruction {
                ins_type: InstructionType::Ret,
                cond_type: ConditionType::Nc,
                ..Instruction::default()
            },
            0xD1 => Instruction {
                ins_type: InstructionType::Pop,
                addr_mode: AddressMode::R,
                reg1: RegisterType::De,
                ..Instruction::default()
            },
            0xD2 => Instruction {
                ins_type: InstructionType::Jp,
                addr_mode: AddressMode::D16,
                cond_type: ConditionType::Nc,
                ..Instruction::default()
            },
            0xD4 => Instruction {
                ins_type: InstructionType::Call,
                addr_mode: AddressMode::D16,
                cond_type: ConditionType::Nc,
                ..Instruction::default()
            },
            0xD5 => Instruction {
                ins_type: InstructionType::Push,
                addr_mode: AddressMode::R,
                reg1: RegisterType::De,
                ..Instruction::default()
            },
            0xD6 => Instruction {
                ins_type: InstructionType::Sub,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xD7 => Instruction {
                ins_type: InstructionType::Rst,
                param: Some(0x10),
                ..Instruction::default()
            },
            0xD8 => Instruction {
                ins_type: InstructionType::Ret,
                cond_type: ConditionType::C,
                ..Instruction::default()
            },
            0xD9 => Instruction {
                ins_type: InstructionType::Reti,
                ..Instruction::default()
            },
            0xDA => Instruction {
                ins_type: InstructionType::Jp,
                addr_mode: AddressMode::D16,
                cond_type: ConditionType::C,
                ..Instruction::default()
            },
            0xDC => Instruction {
                ins_type: InstructionType::Call,
                addr_mode: AddressMode::D16,
                cond_type: ConditionType::C,
                ..Instruction::default()
            },
            0xDE => Instruction {
                ins_type: InstructionType::Sbc,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xDF => Instruction {
                ins_type: InstructionType::Rst,
                param: Some(0x18),
                ..Instruction::default()
            },

            //0xEX
            0xE0 => Instruction {
                ins_type: InstructionType::Ldh,
                addr_mode: AddressMode::A8R,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0xE1 => Instruction {
                ins_type: InstructionType::Pop,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Hl,
                ..Instruction::default()
            },
            0xE2 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::MrR,
                reg1: RegisterType::C,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0xE5 => Instruction {
                ins_type: InstructionType::Push,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Hl,
                ..Instruction::default()
            },
            0xE6 => Instruction {
                ins_type: InstructionType::And,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xE7 => Instruction {
                ins_type: InstructionType::Rst,
                param: Some(0x20),
                ..Instruction::default()
            },
            0xE8 => Instruction {
                ins_type: InstructionType::Add,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::Sp,
                ..Instruction::default()
            },
            0xE9 => Instruction {
                ins_type: InstructionType::Jp,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Hl,
                ..Instruction::default()
            },
            0xEA => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::A16R,
                reg2: RegisterType::A,
                ..Instruction::default()
            },
            0xEE => Instruction {
                ins_type: InstructionType::Xor,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xEF => Instruction {
                ins_type: InstructionType::Rst,
                param: Some(0x28),
                ..Instruction::default()
            },

            //0xFX
            0xF0 => Instruction {
                ins_type: InstructionType::Ldh,
                addr_mode: AddressMode::Ra8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xF1 => Instruction {
                ins_type: InstructionType::Pop,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Af,
                ..Instruction::default()
            },
            0xF2 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::RmR,
                reg1: RegisterType::A,
                reg2: RegisterType::C,
                ..Instruction::default()
            },
            0xF3 => Instruction {
                ins_type: InstructionType::Di,
                ..Instruction::default()
            },
            0xF5 => Instruction {
                ins_type: InstructionType::Push,
                addr_mode: AddressMode::R,
                reg1: RegisterType::Af,
                ..Instruction::default()
            },
            0xF6 => Instruction {
                ins_type: InstructionType::Or,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xF7 => Instruction {
                ins_type: InstructionType::Rst,
                param: Some(0x30),
                ..Instruction::default()
            },
            0xF8 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::HlSpR,
                reg1: RegisterType::Hl,
                reg2: RegisterType::Sp,
                ..Instruction::default()
            },
            0xF9 => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Rr,
                reg1: RegisterType::Sp,
                reg2: RegisterType::Hl,
                ..Instruction::default()
            },
            0xFA => Instruction {
                ins_type: InstructionType::Ld,
                addr_mode: AddressMode::Ra16,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xFB => Instruction {
                ins_type: InstructionType::Ei,
                ..Instruction::default()
            },
            0xFE => Instruction {
                ins_type: InstructionType::Cp,
                addr_mode: AddressMode::Rd8,
                reg1: RegisterType::A,
                ..Instruction::default()
            },
            0xFF => Instruction {
                ins_type: InstructionType::Rst,
                param: Some(0x38),
                ..Instruction::default()
            },
            _ => Instruction {
                ins_type: InstructionType::Undefined,
                ..Instruction::default()
            },
        }
    }
}
