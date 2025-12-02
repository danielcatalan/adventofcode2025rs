use crate::parse::IdRange;
use crate::parse::parse_to_range;
use std::io::BufRead;

pub fn solve_solution<R: BufRead>(reader: R) -> u64 {
    let invalid_ids = reader
        .split(b',')
        .map(|v| String::from_utf8(v.unwrap()).unwrap())
        .map(|s| parse_to_range(&s))
        .map(|range| find_invalid_ids(range))
        .sum();
    invalid_ids
}

fn find_invalid_ids(range: IdRange) -> u64 {
    let first = range.first;
    let last = range.last;

    let mut invalids = 0;
    for id in first..=last {
        if is_invalid(id) {
            invalids += id as u64;
        }
    }
    invalids
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn example1_solution() {
        let input = String::from_str(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution(reader);
        let expected = 1227775554;
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
    fn test_find_invalid_ids() {
        let range = IdRange {
            first: 11,
            last: 22,
        };
        assert_eq!(33, find_invalid_ids(range));

        let range = IdRange {
            first: 95,
            last: 115,
        };
        assert_eq!(99, find_invalid_ids(range));
    }
}
