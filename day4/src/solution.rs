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

pub fn solve_solution1<R: BufRead>(_reader: R) -> u32 {
    0
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
            "some
lines
of
text",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let _solution = solve_solution1(reader);
        todo!("write an assertion")
    }

    #[test]
    fn example_solution2() {
        let input = String::from_str(
            "some
lines
of
text",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let _solution = solve_solution2(reader);
        todo!("write an assertion")
    }
}
