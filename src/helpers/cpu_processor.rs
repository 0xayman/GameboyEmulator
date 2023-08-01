use crate::enums::address_mode::AddressMode;
use crate::enums::condition_type::ConditionType;
use crate::enums::instruction_type::InstructionType;
use crate::enums::register_type::RegisterType;
use crate::modules::common::set_bit;
use crate::modules::cpu::CPU;
use crate::modules::emu::Emu;

impl<'a> CPU<'a> {
    fn process_none(&mut self) {
        println!("INVALID INSTRUCTION");
        return;
    }

    fn process_nop(&mut self) {}

    fn process_di(&mut self) {
        self.int_master_enabled = false;
    }

    fn process_ld(&mut self) {
        if self.dest_is_mem {
            if self.instruction.reg2 >= RegisterType::AF {
                Emu::cycles(1);
                self.bus.write16(self.mem_dest, self.fetched_data)
            } else {
                self.bus.write(self.mem_dest, self.fetched_data as u8)
            }
            return;
        }

        if self.instruction.addr_mode == AddressMode::HLSPR {
            let hflag: bool = (self.read_register(self.instruction.reg2) & 0xF) as u8
                + (self.fetched_data & 0xF) as u8
                >= 0x10;

            let cflag: bool = (self.read_register(self.instruction.reg2) & 0xFF)
                + (self.fetched_data & 0xFF)
                >= 0x100;

            self.set_flags(0, 0, hflag as i32, cflag as i32);
            self.set_register(
                self.instruction.reg1,
                self.read_register(self.instruction.reg2) + self.fetched_data,
            );

            return;
        }

        self.set_register(self.instruction.reg2, self.fetched_data);
    }

    fn process_xor(&mut self) {
        self.registers.a ^= self.fetched_data as u8 & 0xFF;
        self.set_flags((self.registers.a == 0) as i32, 0, 0, 0);
    }

    fn process_jp(&mut self) {
        if self.check_condition() {
            self.registers.pc = self.fetched_data;
            Emu::cycles(1);
        }
    }

    fn set_flags(&mut self, z: i32, n: i32, h: i32, c: i32) {
        if z != -1 {
            set_bit(self.registers.f, 7, z == 1);
        }
        if n != -1 {
            set_bit(self.registers.f, 6, n == 1);
        }
        if h != -1 {
            set_bit(self.registers.f, 5, h == 1);
        }
        if c != -1 {
            set_bit(self.registers.f, 4, c == 1);
        }
    }

    fn check_condition(&self) -> bool {
        let z: bool = self.registers.flag_z();
        let c: bool = self.registers.flag_c();

        match &self.instruction.cond_type {
            ConditionType::NONE => return true,
            ConditionType::C => return c,
            ConditionType::NC => return !c,
            ConditionType::Z => return z,
            ConditionType::NZ => return !z,
        };
    }

    pub fn execute(&mut self) {
        return match &self.instruction.ins_type {
            InstructionType::NONE => self.process_none(),
            InstructionType::NOP => self.process_nop(),
            InstructionType::LD => self.process_ld(),
            InstructionType::JP => self.process_jp(),
            InstructionType::DI => self.process_di(),
            InstructionType::XOR => self.process_xor(),
            other => panic!("Cannot Process this instruction: {:#?}", other),
        };
    }
}
