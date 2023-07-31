use crate::enums::address_mode::AddressMode;
use crate::enums::instruction_type::InstructionType;
use crate::helpers::cpu_processor::processor;
use crate::helpers::cpu_util::util;
use crate::modules::bus::Bus;
use crate::modules::emu::Emu;
use crate::modules::instruction::Instruction;
use crate::modules::registers::Registers;

pub struct CPU<'a> {
    pub registers: Registers,
    pub fetched_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    opcode: u8,
    instruction: Instruction,

    halted: bool,
    stepping: bool,

    pub int_master_enabled: bool,
    bus: &'a mut Bus<'a>,
}

impl<'a> CPU<'a> {
    pub fn new(bus: &'a mut Bus<'a>) -> Self {
        Self {
            registers: Registers::new(),
            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            opcode: 0,
            instruction: Instruction::new(),
            halted: false,
            stepping: false,
            int_master_enabled: false,
            bus: bus,
        }
    }

    pub fn init(&mut self) {
        self.registers.pc = 0x100;
        self.registers.a = 0x01;
    }

    fn fetch_instruction(&mut self) {
        self.opcode = self.bus.read(self.registers.pc);
        self.registers.pc += 1;
        self.instruction = Instruction::instruction_by_opcode(self.opcode);
    }

    fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match &self.instruction.addr_mode {
            AddressMode::IMP => return,
            AddressMode::R => self.fetched_data = util::read_register(self, &self.instruction.reg1),
            AddressMode::RD8 => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                Emu::cycles(1);
                self.registers.pc += 1;
                return;
            }
            AddressMode::D16 => {
                let lo: u16 = self.bus.read(self.registers.pc) as u16;

                Emu::cycles(1);

                let hi: u16 = self.bus.read(self.registers.pc + 1) as u16;
                Emu::cycles(1);

                self.fetched_data = lo | (hi << 8);
                self.registers.pc += 2;
                return;
            }
            other => panic!("Address mode not implemented: {:#?}", other),
        }
    }

    fn execute(&mut self) {
        let processor = processor::get_processor_by_instruction_type(&self.instruction.ins_type);

        processor(self);
    }

    pub fn step(&mut self) -> bool {
        if !self.halted {
            let pc: u16 = self.registers.pc;

            self.fetch_instruction();
            self.fetch_data();

            println!(
                "PC: {:#06X} | OPCODE: {:#04X} | INSTRUCTION: {:#?} | ADDRESS MODE: {:#?}",
                pc, self.opcode, self.instruction.ins_type, self.instruction.addr_mode
            );

            self.execute();
        }
        return true;
    }
}
