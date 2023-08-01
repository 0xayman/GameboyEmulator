use crate::modules::bus::Bus;
use crate::modules::cart::Cart;
use crate::modules::common;
use crate::modules::cpu::CPU;

pub struct Emu {
    paused: bool,
    running: bool,
    ticks: u64,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            paused: false,
            running: true,
            ticks: 0,
        }
    }

    pub fn run(&mut self, rom_path: &str) {
        let mut cart = Cart::new();
        cart.load(rom_path);

        let mut bus = Bus::new(&mut cart);
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
        }

        return;
    }

    pub fn cycles(cycles: u32) {}
}
