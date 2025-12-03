use crate::{factorsdb::FactorsDB, parse::parse_to_range};
use std::{io::BufRead, sync::LazyLock};

pub fn solve_solution1<R: BufRead>(reader: R) -> u64 {
    solution(reader, is_invalid)
}

pub fn solve_solution2<R: BufRead>(reader: R) -> u64 {
    solution(reader, is_invalid2)
}

fn solution<R: BufRead, P>(reader: R, invalid_pred: P) -> u64
where
    P: Fn(u32) -> bool,
{
    let invalid_ids = reader
        .split(b',') // split by ','
        .map(|v| String::from_utf8(v.unwrap()).unwrap()) // work on strings instead of vec
        .map(|s| parse_to_range(&s)) // parse to IdRange
        .map(|r| r.iter()) // IdRange to Range-iter
        .flatten() // get IDs
        .filter(|n| invalid_pred(*n)) // filter IDs
        .map(|n| n as u64)
        .sum();
    invalid_ids
}

fn is_invalid(id: u32) -> bool {
    let id_str = id.to_string().into_bytes();
    if (id_str.len() % 2) != 0 {
        return false;
    }
    let len = id_str.len();
    let mid = len / 2;

    let x = &id_str[0..mid];
    let y = &id_str[mid..];
    if *x == *y {
        return true;
    } else {
        return false;
    }
}

fn is_invalid2(id: u32) -> bool {
    static DB: LazyLock<FactorsDB> = LazyLock::new(|| FactorsDB::new());

    let id_str = id.to_string();
    let len = id_str.len();
    let factors = DB.factors_of(len); // factors of length of id_str

    for f1 in factors.iter().rev() {
        // idea is that f1xf2 = length(id_str)
        let f2 = len / f1;
        let sub = &id_str[0..*f1];
        let comp = sub.repeat(f2);
        if comp == id_str {
            // println!("Found!!");
            return true;
        }
    }
    // println!("Not Found");
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn example_solution1() {
        let input = String::from_str(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution1(reader);
        let expected = 1227775554;
        assert_eq!(expected, solution);
    }

    #[test]
    fn example_solution2() {
        let input = String::from_str(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution2(reader);
        let expected = 4174379265;
        assert_eq!(expected, solution);
    }

    #[test]
    fn test_is_invalid() {
        assert_eq!(false, is_invalid(10));
        assert_eq!(true, is_invalid(11));
        assert_eq!(true, is_invalid(99));
        assert_eq!(true, is_invalid(1010));
        assert_eq!(false, is_invalid(1011));
    }

    #[test]
    fn test_is_invalid2() {
        assert_eq!(false, is_invalid2(10));
        assert_eq!(true, is_invalid2(11));
        assert_eq!(true, is_invalid2(111));
        assert_eq!(true, is_invalid2(99));
        assert_eq!(true, is_invalid2(1010));
        assert_eq!(false, is_invalid2(1011));
        assert_eq!(false, is_invalid2(123123122));
        assert_eq!(true, is_invalid2(123123123));
        assert_eq!(true, is_invalid2(565656));
        assert_eq!(true, is_invalid2(2121212121));
    }
}
