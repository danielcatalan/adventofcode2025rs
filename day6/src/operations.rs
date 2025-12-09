pub enum Operation {
    Add,
    Mult,
}

impl Operation {
    pub fn get_operation(&self) -> impl Fn(usize, usize) -> usize {
        match self {
            Operation::Add => |a, b| a + b,
            Operation::Mult => |a, b| a * b,
        }
    }

    pub fn get_init(&self) -> usize {
        match self {
            Operation::Add => 0,
            Operation::Mult => 1,
        }
    }
}
