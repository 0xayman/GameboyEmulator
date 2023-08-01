#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[allow(dead_code)]
pub enum RegisterType {
    NONE,
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}
