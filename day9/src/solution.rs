use std::io::BufRead;

use crate::parser::parse_tiles;

pub fn solve_solution1<R: BufRead>(reader: R) -> usize {
    let theater = parse_tiles(reader);
    theater.largest_area()
}
pub fn solve_solution2<R: BufRead>(reader: R) -> usize {
    let theater = parse_tiles(reader);
    theater.largest_red_grean_area()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn example_solution1() {
        let input = String::from_str(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution1(reader);
        assert_eq!(50, solution)
    }

    #[test]
    fn example_solution2() {
        let input = String::from_str(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution2(reader);
        assert_eq!(24, solution)
    }
}
