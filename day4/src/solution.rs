use crate::grid::parse_grid;
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
    let grid = parse_grid(reader);

    grid.total_accessible_rolls().unwrap().len()
}

pub fn solve_solution2<R: BufRead>(reader: R) -> usize {
    let mut grid = parse_grid(reader);

    grid.total_rolls_removed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn example_solution1() {
        let input = String::from_str(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution1(reader);
        assert_eq!(13, solution)
    }

    #[test]
    fn example_solution2() {
        let input = String::from_str(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution2(reader);
        assert_eq!(43, solution)
    }
}
