use std::borrow::BorrowMut;

use crate::enums::address_mode::AddressMode;
use crate::enums::condition_type::ConditionType;
use crate::enums::instruction_type::InstructionType;
use crate::enums::register_type::RegisterType;
use crate::modules::bus::Bus;
use crate::modules::common::set_bit;
use crate::modules::cpu::CPU;
use crate::modules::emu::Emu;
use crate::modules::stack::Stack;

impl<'a> CPU<'a> {
    fn process_none(&mut self) {
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
                Bus::write16(self, self.mem_dest, self.fetched_data);
            } else {
                Bus::write(self, self.mem_dest, self.fetched_data as u8);
            }
            Emu::cycles(1);
            return;
        }

        if self.instruction.addr_mode == AddressMode::HLSPR {
            let hflag: bool = (self.read_register(self.instruction.reg2) & 0xF)
                + (self.fetched_data & 0xF)
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

        self.set_register(self.instruction.reg1, self.fetched_data);
    }

    fn process_ldh(&mut self) {
        if self.instruction.reg1 == RegisterType::A {
            self.set_register(
                self.instruction.reg1,
                Bus::read(&self, (0xFF | self.fetched_data)) as u16,
            );
        } else {
            self::Bus::write(self, self.mem_dest, self.registers.a);
        }

        Emu::cycles(1);
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

    fn process_jr(&mut self) {
        let rel: u16 = self.fetched_data & 0xFF;
        let addr: u16 = self.registers.pc + rel;
        self.goto_addr(addr, false);
    }

    fn process_call(&mut self) {
        self.goto_addr(self.fetched_data, true);
    }

    fn process_rst(&mut self) {
        self.goto_addr(self.instruction.param.unwrap() as u16, true);
    }

    fn process_ret(&mut self) {
        if self.instruction.cond_type != ConditionType::NONE {
            Emu::cycles(1);
        }

        if self.check_condition() {
            let lo: u16 = Stack::pop(self) as u16;
            Emu::cycles(1);
            let hi: u16 = Stack::pop(self) as u16;
            Emu::cycles(1);

            let n: u16 = (hi << 8) | lo;
            self.registers.pc = n;

            Emu::cycles(1);
        }
    }

    fn process_reti(&mut self) {
        self.int_master_enabled = true;
        self.process_ret();
    }

    fn process_pop(&mut self) {
        let lo: u16 = Stack::pop(self) as u16;
        Emu::cycles(1);
        let hi: u16 = Stack::pop(self) as u16;
        Emu::cycles(1);

        let n: u16 = (hi << 8) | lo;

        self.set_register(self.instruction.reg1, n);

        if self.instruction.reg1 == RegisterType::AF {
            self.set_register(self.instruction.reg1, n & 0xFFF0);
        }
    }

    fn process_push(&mut self) {
        let hi = (self.read_register(self.instruction.reg1) >> 8) & 0xFF;
        Emu::cycles(1);
        Stack::push(self, hi as u8);

        let lo = self.read_register(self.instruction.reg1) & 0xFF;
        Emu::cycles(1);
        Stack::push(self, lo as u8);

        Emu::cycles(1);
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

    pub fn goto_addr(&mut self, addr: u16, pushpc: bool) {
        if self.check_condition() {
            if pushpc {
                Emu::cycles(2);
                Stack::push16(self, self.registers.pc);
            }

            self.registers.pc = addr;
            Emu::cycles(1);
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
        }
    }

    pub fn execute(&mut self) {
        return match &self.instruction.ins_type {
            InstructionType::NONE => self.process_none(),
            InstructionType::NOP => self.process_nop(),
            InstructionType::LD => self.process_ld(),
            InstructionType::LDH => self.process_ldh(),
            InstructionType::JP => self.process_jp(),
            InstructionType::DI => self.process_di(),
            InstructionType::XOR => self.process_xor(),
            InstructionType::POP => self.process_pop(),
            InstructionType::PUSH => self.process_push(),
            InstructionType::JR => self.process_jr(),
            InstructionType::CALL => self.process_call(),
            InstructionType::RET => self.process_ret(),
            InstructionType::RST => self.process_rst(),
            InstructionType::RETI => self.process_reti(),
            other => panic!(
                "Cannot Process instruction: {:#?} with opcode: {}",
                other, self.opcode
            ),
        };
    }
}
