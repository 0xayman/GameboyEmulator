use crate::{
    enums::{address_mode::AddressMode, register_type::RegisterType},
    modules::{bus::Bus, cpu::CPU, emu::Emu},
};

impl<'a> CPU<'a> {
    pub fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match &self.instruction.addr_mode {
            AddressMode::IMP => return,
            AddressMode::R => self.fetched_data = self.read_register(self.instruction.reg1),
            AddressMode::RR => self.fetched_data = self.read_register(self.instruction.reg2),
            AddressMode::RD8 => {
                self.fetched_data = Bus::read(&self, self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::RD16 | AddressMode::D16 => {
                let lo: u16 = Bus::read(&self, self.registers.pc) as u16;

                Emu::cycles(1);

                let hi: u16 = Bus::read(&self, self.registers.pc + 1) as u16;
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

                self.fetched_data = Bus::read(&self, addr) as u16;
                Emu::cycles(1);

                return;
            }
            AddressMode::RHLI => {
                self.fetched_data =
                    Bus::read(&self, self.read_register(self.instruction.reg2)) as u16;
                Emu::cycles(1);
                self.set_register(RegisterType::HL, self.read_register(RegisterType::HL) + 1);
                return;
            }
            AddressMode::RHLD => {
                self.fetched_data =
                    Bus::read(&self, self.read_register(self.instruction.reg2)) as u16;
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
                self.fetched_data = Bus::read(&self, self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::A8R => {
                self.mem_dest = Bus::read(&self, self.registers.pc) as u16 | 0xFF00;
                self.dest_is_mem = true;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::HLSPR => {
                self.fetched_data = Bus::read(&self, self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::D8 => {
                self.fetched_data = Bus::read(&self, self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::A16R | AddressMode::D16R => {
                let lo: u16 = Bus::read(&self, self.registers.pc) as u16;
                Emu::cycles(1);

                let hi: u16 = Bus::read(&self, self.registers.pc + 1) as u16;
                Emu::cycles(1);

                self.mem_dest = lo | (hi << 8);
                self.dest_is_mem = true;

                self.registers.pc += 2;
                self.fetched_data = self.read_register(self.instruction.reg2);
                return;
            }
            AddressMode::MRD8 => {
                self.fetched_data = Bus::read(&self, self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;
                return;
            }
            AddressMode::MR => {
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;
                self.fetched_data =
                    Bus::read(&self, self.read_register(self.instruction.reg1)) as u16;
                Emu::cycles(1);
                return;
            }
            AddressMode::RA16 => {
                let lo: u16 = Bus::read(&self, self.registers.pc) as u16;
                Emu::cycles(1);

                let hi: u16 = Bus::read(&self, self.registers.pc + 1) as u16;
                Emu::cycles(1);

                let addr: u16 = lo | (hi << 8);

                self.registers.pc += 1;
                self.fetched_data = Bus::read(&self, addr) as u16;
                Emu::cycles(1);
                return;
            }
            other => panic!("Address mode not implemented: {:#?}", other),
        }
    }
}
