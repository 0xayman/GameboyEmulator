use std::env;

mod modules {
    pub mod cart;
    pub mod cpu;
    pub mod emu;
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
