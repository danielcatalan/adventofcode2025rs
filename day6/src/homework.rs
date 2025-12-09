use crate::operations::Operation;

pub struct HomeWork {
    numbers: Vec<Vec<usize>>,
    operations: Vec<Operation>,
}

impl HomeWork {
    pub fn grand_total(&self) -> usize {
        if self.numbers[0].len() != self.operations.len() {
            panic!("missmatch between size of numbers and operations")
        }
        let col_len = self.numbers[0].len();
        let mut grand_total = 0;
        for col in 0..col_len {
            let op = &self.operations[col];
            let result = self
                .numbers
                .iter()
                .map(|r| r[col])
                .fold(op.get_init(), op.get_operation());
            grand_total += result;
        }
        grand_total
    }

    pub fn new(numbers: Vec<Vec<usize>>, operations: Vec<Operation>) -> Self {
        Self {
            numbers,
            operations,
        }
    }
}
