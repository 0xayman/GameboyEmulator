#[derive(Debug)]
#[allow(dead_code)]
pub enum InterruptType {
    VBLANK,
    LCDSTAT,
    TIMER,
    SERIAL,
    JOYPAD,
}
