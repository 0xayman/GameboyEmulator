#![allow(warnings)]
use std::env;

mod modules {
    pub mod bus;
    pub mod cart;
    pub mod common;
    pub mod cpu;
    pub mod emu;
    pub mod instruction;
    pub mod registers;
}

mod enums {
    pub mod address_mode;
    pub mod condition_type;
    pub mod instruction_type;
    pub mod register_type;
}

mod helpers {
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

    let mut emu = Emu::new();
    emu.run(rom_path);
}
