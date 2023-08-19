use std::thread;
use std::time::Duration;

pub fn bit(value: u8, bit: u8) -> bool {
    (value & (1 << bit)) != 0
}

pub fn set_bit(value: u8, bit: u8, on: bool) -> u8 {
    if on {
        value | (1 << bit)
    } else {
        value & (!(1 << bit))
    }
}

pub fn delay(_: u32) {
    thread::sleep(Duration::from_millis(10));
}
