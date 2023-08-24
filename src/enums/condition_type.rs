#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]

pub enum ConditionType {
    None,
    Nz,
    Z,
    Nc,
    C,
}
