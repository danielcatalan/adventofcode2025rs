use regex::Regex;
use std::{io::BufRead, sync::LazyLock};



pub fn parse_homework<R: BufRead>(reader: R) -> HomeWork{
    let lines = reader.lines();
    let mut numbers:Vec<Vec<usize>> = Vec::new();
    let mut last_line = String::new();
    
    for line in lines{
        let line = line.unwrap();
        if let Some(nums) = parse_numbers(&line){
            numbers.push(nums);
            continue;
        }
        else{
            last_line = line;
            break;
        }
    };
    let operations = parse_operators(&last_line);
    HomeWork { numbers, operations }
}

fn parse_numbers(line: &str) -> Option<Vec<usize>>{
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
    let nums: Vec<_> = RE
        .find_iter(line)
        .map(|s|s.as_str().parse::<usize>())
        .map(|n| n.unwrap())
        .collect();
    if nums.len() == 0{
        return None;
    }
    return Some(nums);
}

fn parse_operators(line: &str) -> Vec<Operation>{
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[*+]").unwrap());
    let ops: Vec<_> = RE
        .find_iter(line)
        .map(|s|{
            match s.as_str() {
                "*" => Operation::Mult,
                "+" => Operation::Add,
                &_ => panic!("Unexpected operator"),
            }
        })
        .collect();
    ops
}

pub struct HomeWork{
    numbers: Vec<Vec<usize>>,
    operations:Vec<Operation>
}

impl HomeWork{
    pub fn grand_total(&self) ->usize{
        if self.numbers[0].len() != self.operations.len(){
            panic!("missmatch between size of numbers and operations")
        }
        let col_len = self.numbers[0].len();
        let mut grand_total = 0;
        for col in 0..col_len{
            let op = &self.operations[col];
            let result = self.numbers.iter()
                .map(|r| r[col])
                .fold(op.get_init(), op.get_operation());
            grand_total += result;
        }
        grand_total
    }
}

pub enum Operation {
    Add,
    Mult,
}

impl Operation {
    fn get_operation(&self) -> impl Fn(usize,usize)->usize{
        match self {
            Operation::Add => |a,b| a+b,
            Operation::Mult => |a,b| a*b,
        }
    }

    fn get_init(&self) -> usize{
                match self {
            Operation::Add => 0,
            Operation::Mult => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let line = "123 328  51 64";
        let numbers = parse_numbers(line);
        assert_eq!(Some(vec![123,328,51,64]), numbers);

        let line = "*   +   *   +  ";
        let numbers = parse_numbers(line);
        assert_eq!(None, numbers);
    }
}