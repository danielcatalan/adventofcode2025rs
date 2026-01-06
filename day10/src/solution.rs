
use std::io::BufRead;


use crate::parse::parse_factory;


pub fn solve_solution1<R: BufRead>(reader: R) -> usize {
    let factory = parse_factory(reader);
    factory.get_fewest_button_presses()
}
pub fn solve_solution2<R: BufRead>(_reader: R) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn example_solution1() {
        let input = String::from_str(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution1(reader);
        assert_eq!(7, solution)
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
