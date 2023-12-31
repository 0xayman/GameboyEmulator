use super::{bus::Bus, cpu::Cpu, ppu::Ppu};

pub struct Dma {
    active: bool,
    byte: u8,
    value: u8,
    start_delay: u8,
}

impl Dma {
    pub fn new() -> Self {
        Self {
            active: false,
            byte: 0,
            value: 0,
            start_delay: 0,
        }
    }

    pub fn start(cpu: &mut Cpu, start: u8) {
        cpu.dma.active = true;
        cpu.dma.byte = 0;
        cpu.dma.start_delay = 2;
        cpu.dma.value = start;
    }

    pub fn tick(cpu: &mut Cpu) {
        if !cpu.dma.active {
            return;
        }

        if cpu.dma.start_delay > 0 {
            cpu.dma.start_delay -= 1;
            return;
        }

        let value = Bus::read(cpu, cpu.dma.value as u16 * 0x100 + cpu.dma.byte as u16);

        Ppu::oam_write(&mut cpu.bus.ppu, cpu.dma.byte as u16, value);

        cpu.dma.byte += 1;
        cpu.dma.active = cpu.dma.byte < 0xA0;
    }

    pub fn is_trasferring(&self) -> bool {
        self.active
    }
}
