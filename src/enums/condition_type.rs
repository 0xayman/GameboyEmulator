#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]

pub enum ConditionType {
    NONE,
    NZ,
    Z,
    NC,
    C,
}
