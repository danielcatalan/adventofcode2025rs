use std::io::BufRead;


use crate::parser::parse_christmas_decoration;

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution1<R: BufRead>(reader: R, loop_limit:usize) -> usize {
    let christ_dec = parse_christmas_decoration(reader);
    christ_dec.eval_connections(loop_limit)
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
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution1(reader,10);
        assert_eq!(40, solution);
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
