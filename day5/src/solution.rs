use std::io::BufRead;

use crate::fridge::parse_fridge;

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution1<R: BufRead>(reader: R) -> usize {
    let fridge = parse_fridge(reader);
    fridge.total_fresh_ids()
}
pub fn solve_solution2<R: BufRead>(reader: R) -> usize {
    let fridge = parse_fridge(reader);
    fridge.possible_fresh_ids()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn example_solution1() {
        let input = String::from_str(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution1(reader);
        assert_eq!(3, solution)
    }

    #[test]
    fn example_solution2() {
        let input = String::from_str(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution2(reader);
        assert_eq!(14, solution)
    }
}
