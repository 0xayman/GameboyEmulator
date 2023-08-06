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

            self.set_flags(0, 0, hflag as i8, cflag as i8);
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
                Bus::read(self, (0xFF00 | self.fetched_data)) as u16,
            );
        } else {
            self::Bus::write(self, self.mem_dest, self.registers.a);
        }

        Emu::cycles(1);
    }

    fn process_jp(&mut self) {
        if self.check_condition() {
            self.registers.pc = self.fetched_data;
            Emu::cycles(1);
        }
    }

    fn process_jr(&mut self) {
        let rel: i8 = (self.fetched_data as i8);
        let addr: u16 = self.registers.pc.wrapping_add_signed(rel as i16);
        self.goto_addr(addr, false);
        // panic!("STH WENT WRONG");
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

    fn process_inc(&mut self) {
        let mut val: u16 = self.read_register(self.instruction.reg1).wrapping_add(1);

        if Self::is_16bit(self.instruction.reg1) {
            Emu::cycles(1);
        }

        if self.instruction.reg1 == RegisterType::HL
            && self.instruction.addr_mode == AddressMode::MR
        {
            val = (Bus::read(self, self.read_register(RegisterType::HL)) as u16).wrapping_add(1);
            val &= 0xFF;
            Bus::write(self, self.read_register(RegisterType::HL), val as u8);
        } else {
            self.set_register(self.instruction.reg1, val);
            val = self.read_register(self.instruction.reg1);
        }

        if (self.opcode & 0x03) == 0x03 {
            return;
        }

        self.set_flags((val == 0) as i8, 0, ((val & 0x0F) == 0) as i8, -1);
    }

    fn process_dec(&mut self) {
        let mut val: u16 = self.read_register(self.instruction.reg1).wrapping_sub(1);

        if Self::is_16bit(self.instruction.reg1) {
            Emu::cycles(1);
        }

        if self.instruction.reg1 == RegisterType::HL
            && self.instruction.addr_mode == AddressMode::MR
        {
            val = (Bus::read(self, self.read_register(RegisterType::HL)) as u16).wrapping_sub(1);
            Bus::write(self, self.read_register(RegisterType::HL), val as u8);
        } else {
            self.set_register(self.instruction.reg1, val);
            val = self.read_register(self.instruction.reg1);
        }

        if (self.opcode & 0x0B) == 0x0B {
            return;
        }

        self.set_flags((val == 0) as i8, 1, ((val & 0x0F) == 0x0F) as i8, -1);
    }

    fn process_add(&mut self) {
        let mut val: u32 = (self.read_register(self.instruction.reg1) as u32)
            .wrapping_add(self.fetched_data as u32);

        if Self::is_16bit(self.instruction.reg1) {
            Emu::cycles(1);
        }

        if self.instruction.reg1 == RegisterType::SP {
            val = self
                .read_register(self.instruction.reg1)
                .wrapping_add_signed(self.fetched_data as i8 as i16) as u32;
        }

        let mut z: i8 = ((val & 0xFF) == 0) as i8;
        let mut h: i8 = ((self.read_register(self.instruction.reg1) & 0xF)
            + (self.fetched_data & 0xF)
            >= 0x10) as i8;
        let mut c: i8 = ((self.read_register(self.instruction.reg1) as i16 & 0xFF)
            + (self.fetched_data as i16 & 0xFF)
            >= 0x100) as i8;

        if Self::is_16bit(self.instruction.reg1) {
            z = -1;
            h = ((self.read_register(self.instruction.reg1) & 0xFFF) + (self.fetched_data & 0xFFF)
                >= 0x1000) as i8;
            c = ((self.read_register(self.instruction.reg1) as u32) + (self.fetched_data as u32)
                >= 0x10000) as i8;
        }

        if self.instruction.reg1 == RegisterType::SP {
            z = 0;
            h = ((self.read_register(self.instruction.reg1) & 0xF) + (self.fetched_data & 0xF)
                >= 0x10) as i8;
            c = ((self.read_register(self.instruction.reg1) as i16 & 0xFF)
                + (self.fetched_data as i16 & 0xFF)
                >= 0x100) as i8;
        }

        self.set_register(self.instruction.reg1, (val as u16 & 0xFFFF));
        self.set_flags(z, 0, h, c);
    }

    fn process_adc(&mut self) {
        let u: u16 = self.fetched_data;
        let a: u16 = self.registers.a as u16;
        let c: u16 = self.registers.flag_c() as u16;

        self.registers.a = (a + u + c) as u8;

        self.set_flags(
            (self.registers.a == 0) as i8,
            0,
            ((a & 0xF) + ((u & 0xF) + c) > 0xF) as i8,
            ((a + u + c) > 0xFF) as i8,
        )
    }

    fn process_sub(&mut self) {
        let val: u16 = self
            .read_register(self.instruction.reg1)
            .wrapping_sub(self.fetched_data);

        let z: i8 = (val == 0) as i8;
        let h: i8 = (((self.read_register(self.instruction.reg1) as i8) & 0xF)
            - ((self.fetched_data as i8) & 0xF)
            < 0) as i8;

        let c: i8 = ((self.read_register(self.instruction.reg1) as i8) - (self.fetched_data as i8)
            < 0) as i8;

        self.set_register(self.instruction.reg1, val);
        self.set_flags(z, 1, h, c);
    }

    fn process_sbc(&mut self) {
        let val: u8 = (self.fetched_data as u16 + self.registers.flag_c() as u16) as u8;

        let z: i8 = (self.read_register(self.instruction.reg1) - (val as u16) == 0) as i8;

        let h: i8 = (((self.read_register(self.instruction.reg1)) as i8)
            & 0xF - ((self.fetched_data as i8) & 0xF) - (self.registers.flag_c() as i8)
            < 0) as i8;

        let c: i8 = ((self.read_register(self.instruction.reg1) as i8)
            - (self.fetched_data as i8)
            - (self.registers.flag_c() as i8)
            < 0) as i8;

        self.set_register(
            self.instruction.reg1,
            self.read_register(self.instruction.reg1) - val as u16,
        );
        self.set_flags(z, 1, h, c);
    }

    fn process_or(&mut self) {
        self.registers.a |= self.fetched_data as u8;
        self.set_flags((self.registers.a == 0) as i8, 0, 0, 0);
    }

    fn process_and(&mut self) {
        self.registers.a &= self.fetched_data as u8;
        self.set_flags((self.registers.a == 0) as i8, 0, 1, 0);
    }

    fn process_xor(&mut self) {
        self.registers.a ^= self.fetched_data as u8;
        self.set_flags((self.registers.a == 0) as i8, 0, 0, 0);
    }

    fn process_cp(&mut self) {
        let z: i16 = (self.registers.a as i16).wrapping_sub(self.fetched_data as i16);
        self.set_flags(
            (z == 0) as i8,
            1,
            (((self.registers.a as i16) & 0x0F - (self.fetched_data as i16) & 0x0F) < 0) as i8,
            (z < 0) as i8,
        );
    }

    fn process_cb(&mut self) {
        let op: u8 = self.fetched_data as u8;
        let reg: RegisterType = Self::decode_reg(op & 0b111);
        let bit: u8 = (op >> 3) & 0b111;
        let bit_op: u8 = op.wrapping_shr(8) & 0b11;
        let mut reg_val: u8 = self.read_register_8bits(reg);

        Emu::cycles(1);

        if (reg == RegisterType::HL) {
            Emu::cycles(2);
        }

        match bit_op {
            1 => self.set_flags((reg_val & (1 << bit)) as i8, 0, 1, -1),
            2 => {
                reg_val &= !(1 << bit);
                self.set_register_8bits(reg, reg_val);
            }
            3 => {
                reg_val |= (1 << bit);
                self.set_register_8bits(reg, reg_val);
            }
            _other => panic!("UNKNOWN BIT OP"),
        }

        let flag_c: bool = self.registers.flag_c();

        match bit {
            0 => {
                let mut set_c: bool = false;
                let mut res: u8 = (reg_val << 1) & 0xFF;

                if (reg_val & (1 << 7) != 0) {
                    res |= 1;
                    set_c = true;
                }

                self.set_register_8bits(reg, res);
                self.set_flags((res == 0) as i8, 0, 0, set_c as i8);
            }
            1 => {
                let old: u8 = reg_val;
                reg_val >>= 1;
                reg_val |= (old << 7);

                self.set_register_8bits(reg, reg_val);
                self.set_flags(!reg_val as i8, 0, 0, (old & 1) as i8);
            }
            2 => {
                let old: u8 = reg_val;
                reg_val <<= 1;
                reg_val |= flag_c as u8;

                self.set_register_8bits(reg, reg_val);
                self.set_flags(!reg_val as i8, 0, 0, !!(old & 0x80) as i8);
            }
            3 => {
                let old: u8 = reg_val;
                reg_val >>= 1;
                reg_val |= ((flag_c as u8) << 7);

                self.set_register_8bits(reg, reg_val);
                self.set_flags(!reg_val as i8, 0, 0, (old & 1) as i8);
            }
            4 => {
                let old: u8 = reg_val;
                reg_val << 1;

                self.set_register_8bits(reg, reg_val);
                self.set_flags(!reg_val as i8, 0, 0, !!(old & 0x80) as i8);
            }
            5 => {
                let u: u8 = (reg_val as i8 as u8) >> 1;
                self.set_register_8bits(reg, u);
                self.set_flags(!u as i8, 0, 0, (reg_val & 1) as i8);
            }
            6 => {
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);
                self.set_register_8bits(reg, reg_val);
                self.set_flags((reg_val == 0) as i8, 0, 0, 0);
            }
            7 => {
                let u: u8 = reg_val >> 1;
                self.set_register_8bits(reg, u);
                self.set_flags(!u as i8, 0, 0, (reg_val & 1) as i8);
            }
            _other => panic!("UNKNOWN BIT OP"),
        }

        panic!("INVALID CB: {:02x}", op);
    }

    fn decode_reg(reg: u8) -> RegisterType {
        if (reg > 0b111) {
            return RegisterType::NONE;
        }

        return Self::register_lookup(reg as usize);
    }

    fn register_lookup(index: usize) -> RegisterType {
        let registers: [RegisterType; 8] = [
            RegisterType::B,
            RegisterType::C,
            RegisterType::D,
            RegisterType::E,
            RegisterType::H,
            RegisterType::L,
            RegisterType::HL,
            RegisterType::A,
        ];

        registers[index]
    }

    fn is_16bit(reg_type: RegisterType) -> bool {
        return reg_type >= RegisterType::AF;
    }

    fn set_flags(&mut self, z: i8, n: i8, h: i8, c: i8) {
        println!("Z: {} | N: {} | H: {} | C: {}", z, n, h, c);
        if z != -1 {
            self.registers.f = set_bit(self.registers.f, 7, z == 1);
        }
        if n != -1 {
            self.registers.f = set_bit(self.registers.f, 6, n == 1);
        }
        if h != -1 {
            self.registers.f = set_bit(self.registers.f, 5, h == 1);
        }
        if c != -1 {
            self.registers.f = set_bit(self.registers.f, 4, c == 1);
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
            InstructionType::POP => self.process_pop(),
            InstructionType::PUSH => self.process_push(),
            InstructionType::JR => self.process_jr(),
            InstructionType::CALL => self.process_call(),
            InstructionType::RET => self.process_ret(),
            InstructionType::RST => self.process_rst(),
            InstructionType::RETI => self.process_reti(),
            InstructionType::INC => self.process_inc(),
            InstructionType::DEC => self.process_dec(),
            InstructionType::ADD => self.process_add(),
            InstructionType::ADC => self.process_adc(),
            InstructionType::SUB => self.process_sub(),
            InstructionType::SBC => self.process_sbc(),
            InstructionType::OR => self.process_or(),
            InstructionType::AND => self.process_and(),
            InstructionType::XOR => self.process_xor(),
            InstructionType::CP => self.process_cp(),
            InstructionType::CB => self.process_cb(),
            other => panic!(
                "Cannot Process instruction: {:#?} with opcode: {:X}",
                other, self.opcode
            ),
        };
    }
}
