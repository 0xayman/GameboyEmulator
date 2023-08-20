#![allow(dead_code)]
use std::env;

mod modules {
    pub mod bus;
    pub mod cart;
    pub mod common;
    pub mod cpu;
    pub mod dbg;
    pub mod dma;
    pub mod emu;
    pub mod instruction;
    pub mod interrupts;
    pub mod io;
    pub mod ppu;
    pub mod ram;
    pub mod registers;
    pub mod stack;
    pub mod timer;
}

mod enums {
    pub mod address_mode;
    pub mod condition_type;
    pub mod instruction_type;
    pub mod interrupt_types;
    pub mod register_type;
}

mod helpers {
    pub mod cpu_fetch_data;
    pub mod cpu_processor;
    pub mod cpu_util;
}

pub mod constants;

use modules::emu::Emu;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No ROM path provided!")
    }

    let rom_path = &args[1];

    println!("ROM PATH: {}", rom_path);

    Emu::run(rom_path.clone());
}
