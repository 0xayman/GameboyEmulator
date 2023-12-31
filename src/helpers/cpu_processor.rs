use crate::enums::address_mode::AddressMode;
use crate::enums::condition_type::ConditionType;
use crate::enums::instruction_type::InstructionType;
use crate::enums::register_type::RegisterType;
use crate::modules::bus::Bus;
use crate::modules::common::set_bit;
use crate::modules::cpu::Cpu;
use crate::modules::stack::Stack;
use crate::modules::timer::Timer;

impl Cpu {
    fn process_none(&mut self) {}

    fn process_nop(&mut self) {}

    fn process_di(&mut self) {
        self.int_master_enabled = false;
    }

    fn process_ld(&mut self) {
        if self.dest_is_mem {
            if Self::is_16bit(self.instruction.reg2) {
                Timer::cycles(self, 1);
                Bus::write16(self, self.mem_dest, self.fetched_data);
            } else {
                Bus::write(self, self.mem_dest, self.fetched_data as u8);
            }

            Timer::cycles(self, 1);

            return;
        }

        if self.instruction.addr_mode == AddressMode::HlSpR {
            let hflag: bool = (self.read_register(self.instruction.reg2) & 0xF)
                + (self.fetched_data & 0xF)
                >= 0x10;

            let cflag: bool = (self.read_register(self.instruction.reg2) & 0xFF)
                + (self.fetched_data & 0xFF)
                >= 0x100;

            self.set_flags(Some(false), Some(false), Some(hflag), Some(cflag));

            self.set_register(
                self.instruction.reg1,
                self.read_register(self.instruction.reg2)
                    .wrapping_add_signed(self.fetched_data as i8 as i16),
            );
            return;
        }

        self.set_register(self.instruction.reg1, self.fetched_data);
    }

    fn process_ldh(&mut self) {
        if self.instruction.reg1 == RegisterType::A {
            self.set_register(
                self.instruction.reg1,
                Bus::read(self, 0xFF00 | self.fetched_data) as u16,
            );
        } else {
            self::Bus::write(self, self.mem_dest, self.registers.a);
        }

        Timer::cycles(self, 1);
    }

    fn process_jp(&mut self) {
        self.goto_addr(self.fetched_data, false)
    }

    fn process_jr(&mut self) {
        let rel: i8 = (self.fetched_data & 0xFF) as i8;
        let addr: u16 = self.registers.pc.wrapping_add(rel as u16);
        self.goto_addr(addr, false);
    }

    fn process_call(&mut self) {
        self.goto_addr(self.fetched_data, true);
    }

    fn process_rst(&mut self) {
        self.goto_addr(self.instruction.param.unwrap() as u16, true);
    }

    fn process_ret(&mut self) {
        if self.instruction.cond_type != ConditionType::None {
            Timer::cycles(self, 1);
        }

        if self.check_condition() {
            let lo: u16 = Stack::pop(self) as u16;
            Timer::cycles(self, 1);
            let hi: u16 = Stack::pop(self) as u16;
            Timer::cycles(self, 1);

            let n: u16 = (hi << 8) | lo;
            self.registers.pc = n;

            Timer::cycles(self, 1);
        }
    }

    fn process_reti(&mut self) {
        self.int_master_enabled = true;
        self.process_ret();
    }

    fn process_pop(&mut self) {
        let lo: u16 = Stack::pop(self) as u16;
        Timer::cycles(self, 1);
        let hi: u16 = Stack::pop(self) as u16;
        Timer::cycles(self, 1);

        let n: u16 = (hi << 8) | lo;

        self.set_register(self.instruction.reg1, n);

        if self.instruction.reg1 == RegisterType::Af {
            self.set_register(self.instruction.reg1, n & 0xFFF0);
        }
    }

    fn process_push(&mut self) {
        let hi = (self.read_register(self.instruction.reg1) >> 8) & 0xFF;
        Timer::cycles(self, 1);
        Stack::push(self, hi as u8);

        let lo = self.read_register(self.instruction.reg1) & 0xFF;
        Timer::cycles(self, 1);
        Stack::push(self, lo as u8);

        Timer::cycles(self, 1);
    }

    fn process_inc(&mut self) {
        let mut val: u16 = self.read_register(self.instruction.reg1).wrapping_add(1);

        if Self::is_16bit(self.instruction.reg1) {
            Timer::cycles(self, 1);
        }

        if self.instruction.reg1 == RegisterType::Hl
            && self.instruction.addr_mode == AddressMode::Mr
        {
            val = (Bus::read(self, self.read_register(RegisterType::Hl)) as u16).wrapping_add(1);
            val &= 0xFF;
            Bus::write(self, self.read_register(RegisterType::Hl), val as u8);
        } else {
            self.set_register(self.instruction.reg1, val);
            val = self.read_register(self.instruction.reg1);
        }

        if (self.opcode & 0x03) == 0x03 {
            return;
        }

        self.set_flags(Some(val == 0), Some(false), Some((val & 0x0F) == 0), None);
    }

    fn process_dec(&mut self) {
        let mut val: u16 = self.read_register(self.instruction.reg1).wrapping_sub(1);

        if Self::is_16bit(self.instruction.reg1) {
            Timer::cycles(self, 1);
        }

        if self.instruction.reg1 == RegisterType::Hl
            && self.instruction.addr_mode == AddressMode::Mr
        {
            val = (Bus::read(self, self.read_register(RegisterType::Hl)) as u16).wrapping_sub(1);
            Bus::write(self, self.read_register(RegisterType::Hl), val as u8);
        } else {
            self.set_register(self.instruction.reg1, val);
            val = self.read_register(self.instruction.reg1);
        }

        if (self.opcode & 0x0B) == 0x0B {
            return;
        }

        self.set_flags(Some(val == 0), Some(true), Some((val & 0x0F) == 0x0F), None);
    }

    fn process_add(&mut self) {
        let mut val: u32 = (self.read_register(self.instruction.reg1) as u32)
            .wrapping_add(self.fetched_data as u32);

        if Self::is_16bit(self.instruction.reg1) {
            Timer::cycles(self, 1);
        }

        if self.instruction.reg1 == RegisterType::Sp {
            val = self
                .read_register(self.instruction.reg1)
                .wrapping_add_signed(self.fetched_data as i8 as i16) as u32;
        }

        let mut z: Option<bool> = Some((val & 0xFF) == 0);
        let mut h: Option<bool> = Some(
            (self.read_register(self.instruction.reg1) & 0xF) + (self.fetched_data & 0xF) >= 0x10,
        );
        let mut c: Option<bool> = Some(
            (self.read_register(self.instruction.reg1) as i16 & 0xFF)
                + (self.fetched_data as i16 & 0xFF)
                >= 0x100,
        );

        if Self::is_16bit(self.instruction.reg1) {
            z = None;
            h = Some(
                (self.read_register(self.instruction.reg1) & 0xFFF) + (self.fetched_data & 0xFFF)
                    >= 0x1000,
            );
            c = Some(
                (self.read_register(self.instruction.reg1) as u32) + (self.fetched_data as u32)
                    >= 0x10000,
            );
        }

        if self.instruction.reg1 == RegisterType::Sp {
            z = Some(false);
            h = Some(
                (self.read_register(self.instruction.reg1) & 0xF) + (self.fetched_data & 0xF)
                    >= 0x10,
            );
            c = Some(
                (self.read_register(self.instruction.reg1) as i16 & 0xFF)
                    + (self.fetched_data as i16 & 0xFF)
                    >= 0x100,
            );
        }

        self.set_register(self.instruction.reg1, val as u16);
        self.set_flags(z, Some(false), h, c);
    }

    fn process_adc(&mut self) {
        let u: u16 = self.fetched_data;
        let a: u16 = self.registers.a as u16;
        let c: u16 = self.registers.flag_c() as u16;

        self.registers.a = (a + u + c) as u8;

        self.set_flags(
            Some(self.registers.a == 0),
            Some(false),
            Some((a & 0xF) + ((u & 0xF) + c) > 0xF),
            Some((a + u + c) > 0xFF),
        )
    }

    fn process_sub(&mut self) {
        let val: u16 = self
            .read_register(self.instruction.reg1)
            .wrapping_sub(self.fetched_data);

        let z: Option<bool> = Some(val == 0);

        let h: Option<bool> = Some(
            ((self.read_register(self.instruction.reg1) as i32) & 0xF)
                .wrapping_sub((self.fetched_data as i32) & 0xF)
                < 0,
        );

        let c: Option<bool> = Some(
            (self.read_register(self.instruction.reg1) as i32)
                .wrapping_sub(self.fetched_data as i32)
                < 0,
        );

        self.set_register(self.instruction.reg1, val);
        self.set_flags(z, Some(true), h, c);
    }

    fn process_sbc(&mut self) {
        let val: u8 = self
            .fetched_data
            .wrapping_add(self.registers.flag_c() as u16) as u8;

        let reg_1_val = self.read_register(self.instruction.reg1);
        let z: Option<bool> = Some(reg_1_val.wrapping_sub(val.into()) == 0);

        let h: Option<bool> = Some(
            ((reg_1_val & 0xF) as i32
                - (self.fetched_data & 0xF) as i32
                - self.registers.flag_c() as i32)
                < 0,
        );

        let c: Option<bool> = Some(
            (reg_1_val as i32 - self.fetched_data as i32 - self.registers.flag_c() as i32) < 0,
        );

        self.set_register(self.instruction.reg1, reg_1_val.wrapping_sub(val.into()));
        self.set_flags(z, Some(true), h, c);
    }

    fn process_or(&mut self) {
        self.registers.a |= self.fetched_data as u8;
        self.set_flags(
            Some(self.registers.a == 0),
            Some(false),
            Some(false),
            Some(false),
        );
    }

    fn process_and(&mut self) {
        self.registers.a &= self.fetched_data as u8;
        self.set_flags(
            Some(self.registers.a == 0),
            Some(false),
            Some(true),
            Some(false),
        );
    }

    fn process_xor(&mut self) {
        self.registers.a ^= (self.fetched_data & 0xFF) as u8;
        self.set_flags(
            Some(self.registers.a == 0),
            Some(false),
            Some(false),
            Some(false),
        );
    }

    fn process_cp(&mut self) {
        let z = (self.registers.a as i32).wrapping_sub(self.fetched_data as i32);

        let hflag = (((self.registers.a as i32) & 0x0F) - ((self.fetched_data as i32) & 0x0F)) < 0;

        self.set_flags(Some(z == 0), Some(true), Some(hflag), Some(z < 0));
    }

    fn process_cb(&mut self) {
        let op: u8 = self.fetched_data as u8;
        let reg: RegisterType = Self::decode_reg(op & 0b111);
        let bit: u8 = (op >> 3) & 0b111;
        let bit_op: u8 = (op >> 6) & 0b11;
        let mut reg_val: u8 = self.read_register_8bits(reg);

        Timer::cycles(self, 1);

        if reg == RegisterType::Hl {
            Timer::cycles(self, 2);
        }

        match bit_op {
            1 => {
                // BIT
                self.set_flags(
                    Some(reg_val & (1 << bit) == 0),
                    Some(false),
                    Some(true),
                    None,
                );
                return;
            }
            2 => {
                // RST
                reg_val &= !(1 << bit);
                self.set_register_8bits(reg, reg_val);
                return;
            }
            3 => {
                // SET
                reg_val |= 1 << bit;
                self.set_register_8bits(reg, reg_val);
                return;
            }
            _other => {}
        }

        let flag_c: bool = self.registers.flag_c();

        match bit {
            0 => {
                // RLC
                let mut set_c: bool = false;
                let mut res: u8 = reg_val << 1;

                if reg_val & (1 << 7) != 0 {
                    res |= 1;
                    set_c = true;
                }

                self.set_register_8bits(reg, res);
                self.set_flags(Some(res == 0), Some(false), Some(false), Some(set_c));
                return;
            }
            1 => {
                // RRC
                let old: u8 = reg_val;
                reg_val >>= 1;
                reg_val |= old << 7;

                self.set_register_8bits(reg, reg_val);
                self.set_flags(
                    Some(reg_val == 0),
                    Some(false),
                    Some(false),
                    Some((old & 1) != 0),
                );
                return;
            }
            2 => {
                // RL
                let old: u8 = reg_val;
                reg_val <<= 1;
                reg_val |= flag_c as u8;

                self.set_register_8bits(reg, reg_val);
                self.set_flags(
                    Some(reg_val == 0),
                    Some(false),
                    Some(false),
                    Some(old & 0x80 == 0),
                );
                return;
            }
            3 => {
                // RR
                let old: u8 = reg_val;
                reg_val >>= 1;
                reg_val |= (flag_c as u8) << 7;

                self.set_register_8bits(reg, reg_val);
                self.set_flags(
                    Some(reg_val == 0),
                    Some(false),
                    Some(false),
                    Some(old & 1 != 0),
                );
                return;
            }
            4 => {
                // SLA
                let old: u8 = reg_val;
                reg_val <<= 1;

                self.set_register_8bits(reg, reg_val);
                self.set_flags(
                    Some(reg_val == 0),
                    Some(false),
                    Some(false),
                    Some(old & 0x80 != 0),
                );
                return;
            }
            5 => {
                // SRA
                let u: u8 = (reg_val as i8 >> 1) as u8;
                self.set_register_8bits(reg, u);
                self.set_flags(
                    Some(u == 0),
                    Some(false),
                    Some(false),
                    Some((reg_val & 1) != 0),
                );
                return;
            }
            6 => {
                // SWAP
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);
                self.set_register_8bits(reg, reg_val);
                self.set_flags(Some(reg_val == 0), Some(false), Some(false), Some(false));
                return;
            }
            7 => {
                // SRL
                let u: u8 = reg_val >> 1;
                self.set_register_8bits(reg, u);
                self.set_flags(
                    Some(u == 0),
                    Some(false),
                    Some(false),
                    Some(reg_val & 1 != 0),
                );
                return;
            }
            _other => {}
        }

        panic!("INVALID OP: {:02x}", op);
    }

    fn process_rrca(&mut self) {
        let b: u8 = self.registers.a & 1;
        self.registers.a >>= 1;
        self.registers.a |= b << 7;

        self.set_flags(Some(false), Some(false), Some(false), Some(b != 0));
    }

    fn process_rlca(&mut self) {
        let mut u: u8 = self.registers.a;
        let c: u8 = (u >> 7) & 1;

        u = (u << 1) | c;
        self.registers.a = u;

        self.set_flags(Some(false), Some(false), Some(false), Some(c != 0));
    }

    fn process_rra(&mut self) {
        let carry: u8 = self.registers.flag_c() as u8;
        let new_c: u8 = self.registers.a & 1;

        self.registers.a >>= 1;
        self.registers.a |= carry << 7;

        self.set_flags(Some(false), Some(false), Some(false), Some(new_c != 0))
    }

    fn process_rla(&mut self) {
        let u: u8 = self.registers.a;
        let cf: u8 = self.registers.flag_c() as u8;
        let c: u8 = (u >> 7) & 1;

        self.registers.a = (u << 1) | cf;
        self.set_flags(Some(false), Some(false), Some(false), Some(c != 0));
    }

    fn process_stop(&mut self) {
        panic!("STOP FUNCTION NOT IMPLEMENTED");
    }

    fn process_halt(&mut self) {
        self.halted = true;
    }

    fn process_daa(&mut self) {
        let mut u: i8 = 0;
        let mut fc: i8 = 0;

        if self.registers.flag_h() || (!self.registers.flag_n() && (self.registers.a & 0xF) > 9) {
            u = 6;
        }

        if self.registers.flag_c() || (!self.registers.flag_n() && self.registers.a > 0x99) {
            u |= 0x60;
            fc = 1;
        }

        if self.registers.flag_n() {
            self.registers.a = self.registers.a.wrapping_add_signed(-u);
        } else {
            self.registers.a = self.registers.a.wrapping_add(u as u8);
        }

        self.set_flags(
            Some(self.registers.a == 0),
            None,
            Some(false),
            Some(fc != 0),
        );
    }

    fn process_cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.set_flags(None, Some(true), Some(true), None)
    }

    fn process_csf(&mut self) {
        self.set_flags(None, Some(false), Some(false), Some(true));
    }

    fn process_ccf(&mut self) {
        self.set_flags(
            None,
            Some(false),
            Some(false),
            Some((self.registers.flag_c() as u8) ^ 1 != 0),
        );
    }

    fn process_ei(&mut self) {
        self.enabling_ime = true;
    }

    fn decode_reg(reg: u8) -> RegisterType {
        if reg > 0b111 {
            return RegisterType::None;
        }

        Self::register_lookup(reg as usize)
    }

    fn register_lookup(index: usize) -> RegisterType {
        let registers: [RegisterType; 8] = [
            RegisterType::B,
            RegisterType::C,
            RegisterType::D,
            RegisterType::E,
            RegisterType::H,
            RegisterType::L,
            RegisterType::Hl,
            RegisterType::A,
        ];

        registers[index]
    }

    fn is_16bit(reg_type: RegisterType) -> bool {
        reg_type >= RegisterType::Af
    }

    fn set_flags(&mut self, z: Option<bool>, n: Option<bool>, h: Option<bool>, c: Option<bool>) {
        if let Some(z) = z {
            self.registers.f = set_bit(self.registers.f, 7, z);
        }
        if let Some(n) = n {
            self.registers.f = set_bit(self.registers.f, 6, n);
        }
        if let Some(h) = h {
            self.registers.f = set_bit(self.registers.f, 5, h);
        }
        if let Some(c) = c {
            self.registers.f = set_bit(self.registers.f, 4, c);
        }
    }

    pub fn goto_addr(&mut self, addr: u16, pushpc: bool) {
        if self.check_condition() {
            if pushpc {
                Timer::cycles(self, 2);
                Stack::push16(self, self.registers.pc);
            }

            self.registers.pc = addr;
            Timer::cycles(self, 1);
        }
    }

    fn check_condition(&self) -> bool {
        let z: bool = self.registers.flag_z();
        let c: bool = self.registers.flag_c();

        match &self.instruction.cond_type {
            ConditionType::None => true,
            ConditionType::C => c,
            ConditionType::Nc => !c,
            ConditionType::Z => z,
            ConditionType::Nz => !z,
        }
    }

    pub fn execute(&mut self) {
        match &self.instruction.ins_type {
            InstructionType::None => self.process_none(),
            InstructionType::Nop => self.process_nop(),
            InstructionType::Ld => self.process_ld(),
            InstructionType::Ldh => self.process_ldh(),
            InstructionType::Jp => self.process_jp(),
            InstructionType::Di => self.process_di(),
            InstructionType::Pop => self.process_pop(),
            InstructionType::Push => self.process_push(),
            InstructionType::Jr => self.process_jr(),
            InstructionType::Call => self.process_call(),
            InstructionType::Ret => self.process_ret(),
            InstructionType::Rst => self.process_rst(),
            InstructionType::Reti => self.process_reti(),
            InstructionType::Inc => self.process_inc(),
            InstructionType::Dec => self.process_dec(),
            InstructionType::Add => self.process_add(),
            InstructionType::Adc => self.process_adc(),
            InstructionType::Sub => self.process_sub(),
            InstructionType::Sbc => self.process_sbc(),
            InstructionType::Or => self.process_or(),
            InstructionType::And => self.process_and(),
            InstructionType::Xor => self.process_xor(),
            InstructionType::Cp => self.process_cp(),
            InstructionType::Cb => self.process_cb(),
            InstructionType::Rrca => self.process_rrca(),
            InstructionType::Rlca => self.process_rlca(),
            InstructionType::Rra => self.process_rra(),
            InstructionType::Rla => self.process_rla(),
            InstructionType::Stop => self.process_stop(),
            InstructionType::Halt => self.process_halt(),
            InstructionType::Daa => self.process_daa(),
            InstructionType::Cpl => self.process_cpl(),
            InstructionType::Scf => self.process_csf(),
            InstructionType::Ccf => self.process_ccf(),
            InstructionType::Ei => self.process_ei(),
            other => panic!(
                "Cannot Process instruction: {:#?} with opcode: {:X}",
                other, self.opcode
            ),
        }
    }
}
