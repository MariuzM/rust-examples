#[derive(Debug)] // allow to print 'Data' object
pub struct RandomData {
    pub val1: f32,
    pub val2: i8,
}

impl RandomData {
    pub fn new() -> Self {
        Self { val1: 1.2, val2: 2 }
    }
}
