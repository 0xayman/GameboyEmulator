use crate::modules::common;

#[derive(Clone, Copy)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn flag_z(&self) -> bool {
        common::bit(self.f, 7)
    }

    pub fn flag_n(&self) -> bool {
        common::bit(self.f, 6)
    }

    pub fn flag_h(&self) -> bool {
        common::bit(self.f, 5)
    }

    pub fn flag_c(&self) -> bool {
        common::bit(self.f, 4)
    }
}
