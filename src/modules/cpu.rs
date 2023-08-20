use std::fs::OpenOptions;
use std::io::Write;

use crate::modules::bus::Bus;
use crate::modules::dbg::DBG;
use crate::modules::instruction::Instruction;
use crate::modules::interrupts::interrupt;
use crate::modules::registers::Registers;
use crate::modules::timer::Timer;

use super::dma::Dma;

pub struct CPU {
    pub registers: Registers,
    pub fetched_data: u16,
    pub mem_dest: u16,
    pub dest_is_mem: bool,
    pub opcode: u8,
    pub instruction: Instruction,

    pub halted: bool,
    pub stepping: bool,

    pub int_master_enabled: bool,
    pub enabling_ime: bool,
    pub ie_register: u8,
    pub interrupt_flags: u8,

    pub bus: Bus,
    pub dbg: DBG,
    pub timer: Timer,
    pub dma: Dma,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            opcode: 0,
            instruction: Instruction::default(),
            halted: false,
            stepping: false,

            int_master_enabled: false,
            enabling_ime: false,
            ie_register: 0,
            interrupt_flags: 0,

            bus: Bus::new(),
            dbg: DBG::default(),
            timer: Timer::default(),
            dma: Dma::new(),
        }
    }

    pub fn init(&mut self) {
        self.registers.pc = 0x0100;
        self.registers.sp = 0xFFFE;
        self.registers.a = 0x01;
        self.registers.f = 0xB0;
        self.registers.b = 0x00;
        self.registers.c = 0x13;
        self.registers.d = 0x00;
        self.registers.e = 0xD8;
        self.registers.h = 0x01;
        self.registers.l = 0x4D;
        self.ie_register = 0;
        self.interrupt_flags = 0;
        self.int_master_enabled = true;
        self.enabling_ime = true;

        self.timer.div = 0xABCC;
    }

    fn fetch_instruction(&mut self) {
        self.opcode = Bus::read(&self, self.registers.pc);
        self.registers.pc += 1;
        self.instruction = Instruction::instruction_by_opcode(self.opcode);
    }

    pub fn step(&mut self) -> bool {
        // self.log();

        if !self.halted {
            let pc: u16 = self.registers.pc;

            self.fetch_instruction();
            Timer::cycles(self, 1);
            self.fetch_data();

            let mut flags: [char; 4] = [' '; 4];
            let f: &u8 = &self.registers.f;
            flags[0] = if f & (1 << 7) != 0 { 'Z' } else { '-' };
            flags[1] = if f & (1 << 6) != 0 { 'N' } else { '-' };
            flags[2] = if f & (1 << 5) != 0 { 'H' } else { '-' };
            flags[3] = if f & (1 << 4) != 0 { 'C' } else { '-' };

            println!(
                "TICKS: {:04X} | PC: {:#06X} | {:#?} | OPCODE: ({:02X})({:2X})({:2X}) | A: {:02X} | F: {} | BC: {:02X}{:02X} | DE: {:02X}{:02X} | HL: {:02X}{:02X} | Mode: {:#?}",
                self.timer.ticks ,pc, self.instruction.ins_type, self.opcode, Bus::read(self, pc + 1), Bus::read(self, pc + 2), self.registers.a, flags.iter().collect::<String>() ,self.registers.b, self.registers.c, self.registers.d, self.registers.e, self.registers.h, self.registers.l, self.instruction.addr_mode
            );

            DBG::update(self);
            DBG::print(self);

            self.execute();
        } else {
            // CPU IS HALTED
            Timer::cycles(self, 1);

            if self.interrupt_flags != 0 {
                self.halted = false;
            }
        }

        if self.int_master_enabled {
            interrupt::handle(self);
            self.enabling_ime = false;
        }

        if self.enabling_ime {
            self.int_master_enabled = true;
        }
        return true;
    }

    pub fn get_ie_register(&self) -> u8 {
        return self.ie_register;
    }

    pub fn set_ie_register(&mut self, value: u8) {
        self.ie_register = value;
    }

    fn log(&self) {
        println!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            self.registers.a, self.registers.f, self.registers.b, self.registers.c, self.registers.d, self.registers.e, self.registers.h, self.registers.l,
            self.registers.sp, self.registers.pc, Bus::read(self, self.registers.pc), Bus::read(self, self.registers.pc + 1), Bus::read(self, self.registers.pc + 2), Bus::read(self, self.registers.pc + 3)
        );

        // Write the above print content to file log.txt
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("logs.txt")
            .unwrap();

        file.write_all(format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
            self.registers.a, self.registers.f, self.registers.b, self.registers.c, self.registers.d, self.registers.e, self.registers.h, self.registers.l,
            self.registers.sp, self.registers.pc, Bus::read(self, self.registers.pc), Bus::read(self, self.registers.pc + 1), Bus::read(self, self.registers.pc + 2), Bus::read(self, self.registers.pc + 3)
        ).as_bytes()).unwrap();
    }
}
