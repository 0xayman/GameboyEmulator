use crate::modules::bus::Bus;
use crate::modules::cart::Cart;
use crate::modules::common;
use crate::modules::cpu::CPU;
use crate::modules::ram::RAM;
use crate::modules::ui::UI;
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Copy)]
pub struct Emu {
    paused: bool,
    running: bool,
    pub die: bool,
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

    pub fn run_cpu_thread(emu: &mut Emu, cart: &mut Cart, rx: mpsc::Receiver<String>) {
        let mut ram = RAM::new();
        let mut bus = Bus::new(cart, &mut ram);
        let mut cpu = CPU::new(&mut bus);

        cpu.init();

        // try to recieve message from UI thread, if no message, run the while loop
        // if message, check if it's "die", if so, break the loop
        loop {
            match rx.try_recv() {
                Ok(msg) => {
                    if msg == "die" {
                        emu.die = true;
                        break;
                    }
                }
                Err(_) => {}
            }

            if emu.die {
                break;
            }

            if emu.paused {
                common::delay(10);
                continue;
            }

            if !cpu.step() {
                dbg!("CPU STOPPED");
                return;
            }

            emu.ticks += 1;

            print!("\nTICKS: {:0X}\t", emu.ticks);
        }
    }

    pub fn run(&mut self, rom_path: &str) {
        let mut emu = Emu::new();
        let mut cart = Cart::new();
        cart.load(rom_path);

        let (tx, rx) = mpsc::channel::<String>();

        // Create thread to run cpu
        let cpu_thread_handler =
            thread::spawn(move || Self::run_cpu_thread(&mut emu, &mut cart, rx));

        UI::init(tx);

        cpu_thread_handler.join().unwrap();

        return;
    }

    pub fn cycles(cycles: u32) {}
}
