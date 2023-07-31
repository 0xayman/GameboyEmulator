pub struct CPU {}

impl CPU {
    pub fn new() -> Self {
        Self {}
    }

    pub fn step(&self) -> bool {
        println!("CPU Not Implemented");
        return false;
    }
}
