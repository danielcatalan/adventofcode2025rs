use crate::parse::parse_bank;
#[allow(unused_imports)]
use regex::Regex;
use std::io::BufRead;
#[allow(unused_imports)]
use std::sync::LazyLock;

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution1<R: BufRead>(reader: R) -> usize {
    let x = reader
        .lines()
        .map(|l| parse_bank(l.unwrap()))
        .map(|bank| bank.largest_jolt())
        .sum();
    x
}
pub fn solve_solution2<R: BufRead>(reader: R) -> usize {
    let x = reader
        .lines()
        .map(|l| parse_bank(l.unwrap()))
        .map(|bank| bank.largest_jolt2())
        .sum();
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn example_solution1() {
        let input = String::from_str(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution1(reader);
        assert_eq!(357, solution);
    }

    #[test]
    fn example_solution2() {
        let input = String::from_str(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution2(reader);
        assert_eq!(3121910778619, solution);
    }
}
