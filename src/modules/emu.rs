use crate::modules::bus::Bus;
use crate::modules::cart::Cart;
use crate::modules::common;
use crate::modules::cpu::CPU;
use crate::modules::ram::RAM;

pub struct Emu {
    paused: bool,
    running: bool,
    die: bool,
    ticks: u64,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            paused: false,
            running: true,
            die: false,
            ticks: 0,
        }
    }

    pub fn run(&mut self, rom_path: &str) {
        let mut cart = Cart::new();
        cart.load(rom_path);

        let mut ram = RAM::new();
        let mut bus = Bus::new(&mut cart, &mut ram);
        let mut cpu = CPU::new(&mut bus);

        cpu.init();

        while self.running {
            if self.paused {
                common::delay(10);
                continue;
            }

            if !cpu.step() {
                dbg!("CPU STOPPED");
                return;
            }

            self.ticks += 1;

            print!("\nTICKS: {:0X}\t", self.ticks);
        }

        return;
    }

    pub fn cycles(cycles: u32) {}
}
