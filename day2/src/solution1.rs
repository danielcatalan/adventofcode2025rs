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

pub fn solve_solution<R: BufRead>(reader: R) -> u32 {
    let _x = reader
        .split(b',')
        .map(|v| String::from_utf8(v.unwrap()).unwrap());
    0
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
}
