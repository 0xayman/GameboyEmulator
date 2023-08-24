use crate::{
    enums::{address_mode::AddressMode, register_type::RegisterType},
    modules::{bus::Bus, cpu::Cpu, timer::Timer},
};

impl Cpu {
    pub fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match &self.instruction.addr_mode {
            AddressMode::Imp => (),

            AddressMode::R => self.fetched_data = self.read_register(self.instruction.reg1),

            AddressMode::Rr => self.fetched_data = self.read_register(self.instruction.reg2),

            AddressMode::Rd8 => {
                self.fetched_data = Bus::read(self, self.registers.pc) as u16;
                Timer::cycles(self, 1);
                self.registers.pc += 1
            }

            AddressMode::Rd16 | AddressMode::D16 => {
                let lo: u16 = Bus::read(self, self.registers.pc) as u16;
                Timer::cycles(self, 1);

                let hi: u16 = Bus::read(self, self.registers.pc + 1) as u16;
                Timer::cycles(self, 1);

                self.fetched_data = lo | (hi << 8);
                self.registers.pc += 2
            }

            AddressMode::MrR => {
                self.fetched_data = self.read_register(self.instruction.reg2);
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;

                if self.instruction.reg1 == RegisterType::C {
                    self.mem_dest |= 0xFF00;
                }
            }

            AddressMode::RmR => {
                let mut addr: u16 = self.read_register(self.instruction.reg2);

                if self.instruction.reg2 == RegisterType::C {
                    addr |= 0xFF00;
                }

                self.fetched_data = Bus::read(self, addr) as u16;
                Timer::cycles(self, 1)
            }

            AddressMode::Rhli => {
                let addr: u16 = self.read_register(self.instruction.reg2);
                self.fetched_data = Bus::read(self, addr) as u16;

                Timer::cycles(self, 1);

                let data: u16 = self.read_register(RegisterType::Hl).wrapping_add(1);
                self.set_register(RegisterType::Hl, data)
            }

            AddressMode::Rhld => {
                let addr: u16 = self.read_register(self.instruction.reg2);
                self.fetched_data = Bus::read(self, addr) as u16;

                Timer::cycles(self, 1);

                let data: u16 = self.read_register(RegisterType::Hl).wrapping_sub(1);
                self.set_register(RegisterType::Hl, data)
            }

            AddressMode::HliR => {
                self.fetched_data = self.read_register(self.instruction.reg2);
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;

                let data: u16 = self.read_register(RegisterType::Hl).wrapping_add(1);
                self.set_register(RegisterType::Hl, data)
            }

            AddressMode::HldR => {
                self.fetched_data = self.read_register(self.instruction.reg2);
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;

                let data: u16 = self.read_register(RegisterType::Hl).wrapping_sub(1);
                self.set_register(RegisterType::Hl, data)
            }

            AddressMode::Ra8 => {
                self.fetched_data = Bus::read(self, self.registers.pc) as u16;
                Timer::cycles(self, 1);
                self.registers.pc += 1
            }

            AddressMode::A8R => {
                let bus_data: u16 = Bus::read(self, self.registers.pc) as u16;
                self.mem_dest = bus_data | 0xFF00;
                self.dest_is_mem = true;
                Timer::cycles(self, 1);
                self.registers.pc += 1
            }

            AddressMode::HlSpR => {
                self.fetched_data = Bus::read(self, self.registers.pc) as u16;
                Timer::cycles(self, 1);
                self.registers.pc += 1
            }

            AddressMode::D8 => {
                self.fetched_data = Bus::read(self, self.registers.pc) as u16;
                Timer::cycles(self, 1);
                self.registers.pc += 1
            }

            AddressMode::A16R | AddressMode::D16R => {
                let lo: u16 = Bus::read(self, self.registers.pc) as u16;
                Timer::cycles(self, 1);

                let hi: u16 = Bus::read(self, self.registers.pc + 1) as u16;
                Timer::cycles(self, 1);

                self.mem_dest = lo | (hi << 8);
                self.dest_is_mem = true;

                self.registers.pc += 2;
                self.fetched_data = self.read_register(self.instruction.reg2)
            }

            AddressMode::MrD8 => {
                self.fetched_data = Bus::read(self, self.registers.pc) as u16;
                Timer::cycles(self, 1);

                self.registers.pc += 1;
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true
            }

            AddressMode::Mr => {
                self.mem_dest = self.read_register(self.instruction.reg1);
                self.dest_is_mem = true;

                let addr: u16 = self.read_register(self.instruction.reg1);
                self.fetched_data = Bus::read(self, addr) as u16;

                Timer::cycles(self, 1)
            }

            AddressMode::Ra16 => {
                let lo: u16 = Bus::read(self, self.registers.pc) as u16;
                Timer::cycles(self, 1);

                let hi: u16 = Bus::read(self, self.registers.pc + 1) as u16;
                Timer::cycles(self, 1);

                let addr: u16 = lo | (hi << 8);

                self.registers.pc += 2;
                self.fetched_data = Bus::read(self, addr) as u16;
                Timer::cycles(self, 1)
            }
        }
    }
}
