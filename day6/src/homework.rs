use crate::{content::Content, operations::Operation};

pub struct HomeWork {
    numbers: Vec<Vec<usize>>,
    operations: Vec<Operation>,
}

impl HomeWork {
    pub fn new(numbers: Vec<Vec<usize>>, operations: Vec<Operation>) -> Self {
        Self {
            numbers,
            operations,
        }
    }

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
}

pub struct HomeWork2 {
    content: Vec<Vec<u8>>,
}

impl HomeWork2 {
    pub fn new(content: Vec<Vec<u8>>) -> Self {
        HomeWork2 { content }
    }

    pub(crate) fn grand_total(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let mut s = "  4  ".to_string();
        s.retain(|c| matches!(c, '0'..='9'));
        assert_eq!(4, s.parse().unwrap());
    }
}
