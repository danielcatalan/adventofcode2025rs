use crate::{
    homework::{HomeWork, HomeWork2},
    operations::Operation,
};
use regex::Regex;
use std::{io::BufRead, sync::LazyLock};

pub fn parse_homework<R: BufRead>(reader: R) -> HomeWork {
    let lines = reader.lines();
    let mut numbers: Vec<Vec<usize>> = Vec::new();
    let mut last_line = String::new();

    for line in lines {
        let line = line.unwrap();
        if let Some(nums) = parse_numbers(&line) {
            numbers.push(nums);
            continue;
        } else {
            last_line = line;
            break;
        }
    }
    let operations = parse_operators(&last_line);
    HomeWork::new(numbers, operations)
}

pub fn parse_homework2<R: BufRead>(reader: R) -> HomeWork2 {
    let content = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| parse_line2(&s))
        .collect();
    HomeWork2::new(content)
}

pub fn parse_line2(line: &str) -> Vec<u8> {
    line.as_bytes().to_vec()
}

fn parse_numbers(line: &str) -> Option<Vec<usize>> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
    let nums: Vec<_> = RE
        .find_iter(line)
        .map(|s| s.as_str().parse::<usize>())
        .map(|n| n.unwrap())
        .collect();
    if nums.len() == 0 {
        return None;
    }
    return Some(nums);
}

fn parse_operators(line: &str) -> Vec<Operation> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[*+]").unwrap());
    let ops: Vec<_> = RE
        .find_iter(line)
        .map(|s| match s.as_str() {
            "*" => Operation::Mult,
            "+" => Operation::Add,
            &_ => panic!("Unexpected operator"),
        })
        .collect();
    ops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let line = "123 328  51 64";
        let numbers = parse_numbers(line);
        assert_eq!(Some(vec![123, 328, 51, 64]), numbers);

        let line = "*   +   *   +  ";
        let numbers = parse_numbers(line);
        assert_eq!(None, numbers);
    }
}
