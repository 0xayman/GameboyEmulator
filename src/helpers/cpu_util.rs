use crate::enums::{address_mode::AddressMode, register_type::RegisterType};
use crate::modules::{cpu::CPU, emu::Emu};

impl<'a> CPU<'a> {
    pub fn read_register(&self, reg_type: RegisterType) -> u16 {
        match reg_type {
            RegisterType::A => return self.registers.a as u16,
            RegisterType::F => return self.registers.f as u16,
            RegisterType::B => return self.registers.b as u16,
            RegisterType::C => return self.registers.c as u16,
            RegisterType::D => return self.registers.d as u16,
            RegisterType::E => return self.registers.e as u16,
            RegisterType::H => return self.registers.h as u16,
            RegisterType::L => return self.registers.l as u16,

            RegisterType::AF => return (self.registers.a as u16) << 8 | self.registers.f as u16,
            RegisterType::BC => return (self.registers.b as u16) << 8 | self.registers.c as u16,
            RegisterType::DE => return (self.registers.d as u16) << 8 | self.registers.e as u16,
            RegisterType::HL => return (self.registers.h as u16) << 8 | self.registers.l as u16,

            RegisterType::SP => return self.registers.sp,
            RegisterType::PC => return self.registers.pc,
            RegisterType::NONE => return 0,
        }
    }

    pub fn set_register(&mut self, reg_type: RegisterType, data: u16) {
        match reg_type {
            RegisterType::A => self.registers.a = (data & 0xFF) as u8,
            RegisterType::F => self.registers.f = (data & 0xFF) as u8,
            RegisterType::B => self.registers.b = (data & 0xFF) as u8,
            RegisterType::C => self.registers.c = (data & 0xFF) as u8,
            RegisterType::D => self.registers.d = (data & 0xFF) as u8,
            RegisterType::E => self.registers.e = (data & 0xFF) as u8,
            RegisterType::H => self.registers.h = (data & 0xFF) as u8,
            RegisterType::L => self.registers.l = (data & 0xFF) as u8,

            RegisterType::AF => {
                self.registers.a = ((data >> 8) & 0xFF) as u8;
                self.registers.f = (data & 0xFF) as u8;
            }
            RegisterType::BC => {
                self.registers.b = ((data >> 8) & 0xFF) as u8;
                self.registers.c = (data & 0xFF) as u8;
            }
            RegisterType::DE => {
                self.registers.d = ((data >> 8) & 0xFF) as u8;
                self.registers.e = (data & 0xFF) as u8;
            }
            RegisterType::HL => {
                self.registers.h = ((data >> 8) & 0xFF) as u8;
                self.registers.l = (data & 0xFF) as u8;
            }

            RegisterType::SP => self.registers.sp = data,
            RegisterType::PC => self.registers.pc = data,
            RegisterType::NONE => return,
        }
    }

    pub fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match &self.instruction.addr_mode {
            AddressMode::IMP => return,
            AddressMode::R => self.fetched_data = self.read_register(self.instruction.reg1),
            AddressMode::RR => self.fetched_data = self.read_register(self.instruction.reg2),
            AddressMode::RD8 => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::RD16 | AddressMode::D16 => {
                let lo: u16 = self.bus.read(self.registers.pc) as u16;

                Emu::cycles(1);

                let hi: u16 = self.bus.read(self.registers.pc + 1) as u16;
                Emu::cycles(1);

                self.fetched_data = lo | (hi << 8);
                self.registers.pc += 2;
                return;
            }
            AddressMode::MRR => {
                self.fetched_data = self.read_register(self.instruction.reg2);
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;

                if self.instruction.reg1 == RegisterType::C {
                    self.mem_dest = 0xFF00 | self.mem_dest;
                }

                return;
            }
            AddressMode::RMR => {
                let mut addr: u16 = self.read_register(self.instruction.reg2);

                if self.instruction.reg2 == RegisterType::C {
                    addr |= 0xFF00;
                }

                self.fetched_data = self.bus.read(addr) as u16;
                Emu::cycles(1);

                return;
            }
            AddressMode::RHLI => {
                self.fetched_data = self.bus.read(self.read_register(self.instruction.reg2)) as u16;
                Emu::cycles(1);
                self.set_register(RegisterType::HL, self.read_register(RegisterType::HL) + 1);
                return;
            }
            AddressMode::RHLD => {
                self.fetched_data = self.bus.read(self.read_register(self.instruction.reg2)) as u16;
                Emu::cycles(1);
                self.set_register(RegisterType::HL, self.read_register(RegisterType::HL) - 1);
                return;
            }
            AddressMode::HLIR => {
                self.fetched_data = self.read_register(self.instruction.reg2);
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;
                self.set_register(RegisterType::HL, self.read_register(RegisterType::HL) + 1);
                return;
            }
            AddressMode::HLDR => {
                self.fetched_data = self.read_register(self.instruction.reg2);
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;
                self.set_register(RegisterType::HL, self.read_register(RegisterType::HL) - 1);
                return;
            }
            AddressMode::RA8 => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::A8R => {
                self.mem_dest = self.bus.read(self.registers.pc) as u16 | 0xFF00;
                self.dest_is_mem = true;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::HLSPR => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::D8 => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::A16R | AddressMode::D16R => {
                let lo: u16 = self.bus.read(self.registers.pc) as u16;
                Emu::cycles(1);

                let hi: u16 = self.bus.read(self.registers.pc + 1) as u16;
                Emu::cycles(1);

                self.mem_dest = lo | (hi << 8);
                self.dest_is_mem = true;

                self.registers.pc += 2;
                self.fetched_data = self.read_register(self.instruction.reg2);
                return;
            }
            AddressMode::MRD8 => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;
                return;
            }
            AddressMode::MR => {
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;
                self.fetched_data = self.bus.read(self.read_register(self.instruction.reg1)) as u16;
                Emu::cycles(1);
                return;
            }
            AddressMode::RA16 => {
                let lo: u16 = self.bus.read(self.registers.pc) as u16;
                Emu::cycles(1);

                let hi: u16 = self.bus.read(self.registers.pc + 1) as u16;
                Emu::cycles(1);

                let addr: u16 = lo | (hi << 8);

                self.registers.pc += 1;
                self.fetched_data = self.bus.read(addr) as u16;
                Emu::cycles(1);
                return;
            }
            other => panic!("Address mode not implemented: {:#?}", other),
        }
    }
}
