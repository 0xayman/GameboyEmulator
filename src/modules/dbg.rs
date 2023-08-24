use crate::modules::bus::Bus;
use crate::modules::cpu::Cpu;

pub struct Dbg {
    msg: [char; 1024],
    msg_size: usize,
}

impl Default for Dbg {
    fn default() -> Self {
        Self {
            msg: [' '; 1024],
            msg_size: 0,
        }
    }
}

impl Dbg {
    pub fn update(cpu: &mut Cpu) {
        if Bus::read(cpu, 0xFF02) == 0x81 {
            let c = Bus::read(cpu, 0xFF01) as char;

            cpu.dbg.msg[cpu.dbg.msg_size] = c;
            cpu.dbg.msg_size += 1;

            Bus::write(cpu, 0xFF02, 0);
        }
    }

    pub fn print(cpu: &Cpu) {
        if cpu.dbg.msg[0] != ' ' {
            let dbg_str: String = cpu.dbg.msg[..cpu.dbg.msg_size].iter().collect();
            println!("DBG {}", dbg_str);
        }
    }
}
