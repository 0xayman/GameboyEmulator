use crate::modules::bus::Bus;
use crate::modules::cart::Cart;
use crate::modules::common;
use crate::modules::cpu::CPU;
use crate::modules::io::IO;
use crate::modules::ram::RAM;
use crate::modules::timer::Timer;
use crate::modules::ui::UI;
use std::sync::mpsc;
use std::thread;

use super::dbg;

pub struct Emu {
    paused: bool,
    running: bool,
    pub die: bool,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            paused: false,
            running: true,
            die: false,
        }
    }

    pub fn run_cpu_thread(emu: &mut Emu, rom_path: &str, rx: mpsc::Receiver<String>) {
        let mut cpu = CPU::new();
        Cart::load(&mut cpu.bus.cart, rom_path);

        cpu.timer.init();
        cpu.init();

        emu.running = true;
        emu.paused = false;
        cpu.timer.ticks = 0;

        while emu.running {
            match rx.try_recv() {
                Ok(msg) => {
                    if msg == "die" {
                        emu.die = true;
                        break;
                    }
                }
                Err(_) => {}
            }

            if emu.paused {
                common::delay(10);
                continue;
            }

            if !cpu.step() {
                dbg!("CPU STOPED");
                return;
            }
        }
    }

    pub fn run(rom_path: String) {
        let mut emu = Emu::new();

        let (tx, rx) = mpsc::channel::<String>();

        // Create thread to run cpu
        let cpu_thread_handler =
            thread::spawn(move || Self::run_cpu_thread(&mut emu, &rom_path, rx));

        UI::init(tx);

        cpu_thread_handler.join().unwrap();

        return;
    }
}
