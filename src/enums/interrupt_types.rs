#[derive(Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum InterruptType {
    VBLANK = 1,
    LCDSTAT = 2,
    TIMER = 4,
    SERIAL = 8,
    JOYPAD = 16,
}
