pub fn bit(value: u8, bit: u8) -> bool {
    (value & (1 << bit)) != 0
}

pub fn set_bit(value: u8, bit: u8, on: bool) -> u8 {
    if on {
        value | (1 << bit)
    } else {
        value & !(1 << bit)
    }
}

pub fn between(value: u8, min: u8, max: u8) -> bool {
    value >= min && value <= max
}

fn delay(ms: u32) {}
