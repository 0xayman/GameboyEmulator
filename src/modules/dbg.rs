use crate::modules::bus::Bus;
use crate::modules::cpu::CPU;

pub struct DBG {
    msg: [u8; 1024],
    msg_size: usize,
}

impl Default for DBG {
    fn default() -> Self {
        Self {
            msg: [0; 1024],
            msg_size: 0,
        }
    }
}

impl DBG {
    pub fn update(cpu: &mut CPU) {
        if Bus::read(cpu, 0xFF02) == 0x81 {
            let c = Bus::read(cpu, 0xFF01);

            cpu.dbg.msg[cpu.dbg.msg_size] = c;
            cpu.dbg.msg_size += 1;

            Bus::write(cpu, 0xFF02, 0);
        }
    }

    pub fn print(cpu: &CPU) {
        if cpu.dbg.msg[0] != 0 {
            let dbg_str = std::str::from_utf8(&cpu.dbg.msg[..cpu.dbg.msg_size]).unwrap();
            println!("DBG {}", dbg_str);
        }
    }
}
