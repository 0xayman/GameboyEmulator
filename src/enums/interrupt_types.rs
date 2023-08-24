#[derive(Debug)]
#[allow(dead_code)]
pub enum InterruptType {
    Vblank,
    LcdStat,
    Timer,
    Serial,
    Joybad,
}
