use std::io::BufRead;

use crate::parse::parse_homework;

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution1<R: BufRead>(reader: R) -> usize {
    let hw = parse_homework(reader);
    hw.grand_total()
}
pub fn solve_solution2<R: BufRead>(_reader: R) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn example_solution1() {
        let input = String::from_str(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution1(reader);
        assert_eq!(4277556, solution);
    }

    #[test]
    fn example_solution2() {
        let input = String::from_str(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution2(reader);
        assert_eq!(3263827, solution)
    }
}
