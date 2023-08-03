use crate::enums::address_mode::AddressMode;
use crate::enums::instruction_type::InstructionType;
use crate::modules::bus::Bus;
use crate::modules::emu::Emu;
use crate::modules::instruction::Instruction;
use crate::modules::registers::Registers;

pub struct CPU<'a> {
    pub registers: Registers,
    pub fetched_data: u16,
    pub mem_dest: u16,
    pub dest_is_mem: bool,
    pub opcode: u8,
    pub instruction: Instruction,

    pub halted: bool,
    pub stepping: bool,

    pub int_master_enabled: bool,
    pub ie_register: u8,

    pub bus: &'a mut Bus<'a>,
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
            ie_register: 0,

            bus: bus,
        }
    }

    pub fn init(&mut self) {
        self.registers.pc = 0x100;
        self.registers.a = 0x01;
    }

    fn fetch_instruction(&mut self) {
        self.opcode = Bus::read(&self, self.registers.pc);
        self.registers.pc += 1;
        self.instruction = Instruction::instruction_by_opcode(self.opcode);
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

    pub fn get_ie_register(&self) -> u8 {
        return self.ie_register;
    }

    pub fn set_ie_register(&mut self, value: u8) {
        self.ie_register = value;
    }
}
