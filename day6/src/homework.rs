
use crate::operations::Operation;

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
        let column_len = self.content[0].len();
        let mut grand_total = 0;
        let mut num_buffer = Vec::new();
        for c in (0..column_len).rev(){
            let mut num_str: String = self.content
                .iter()
                .map(|row| row[c] as char)
                .collect();

            let op_or_space = num_str.pop().unwrap();
            num_str.retain(|c| matches!(c, '0'..='9'));
            let num = num_str.parse();
            if let Err(_) = num{
                continue;
            }
            num_buffer.push(num.unwrap());

            match op_or_space {
                '+' => {
                    let sum = num_buffer.iter().fold(0, |acc,x| acc+x);
                    grand_total += sum;
                    num_buffer.clear();
                },
                '*' => {
                    let mult = num_buffer.iter().fold(1, |acc,x| acc*x);
                    grand_total += mult;
                    num_buffer.clear();
                },
                ' ' => {/*Do nothing */}, 
                _ => panic!()
            }

        }
        grand_total
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn sanity_check() {
        let mut s = "  4  ".to_string();
        s.retain(|c| matches!(c, '0'..='9'));
        assert_eq!(4, s.parse().unwrap());
    }
}
