use crate::parse::{Rotation, parse_to_rot};
#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::io::BufRead;

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution<R: BufRead>(reader: R) -> u32 {
    let rots = reader.lines().map(|l| parse_to_rot(l.unwrap().as_ref()));

    let mut pos = 50; // start at 50 clicks
    let mut zero_counter = 0;
    for rot in rots {
        zero_counter += turn(&mut pos, rot);
    }

    zero_counter
}

fn turn(pos: &mut i32, rot: Rotation) -> u32 {
    let mut zero_counter = 0;

    match rot {
        Rotation::Left(clicks) => {
            let new_pos = (*pos as i32 - clicks).rem_euclid(100);
            let full_turns = clicks / 100;
            if full_turns > 0 {
                zero_counter += full_turns;
            }

            if (new_pos > *pos) && (*pos != 0) {
                zero_counter += 1;
            } else if new_pos == 0 {
                zero_counter += 1;
            }
            *pos = new_pos;
        }
        Rotation::Right(clicks) => {
            let new_pos = (*pos + clicks).rem_euclid(100);
            let full_turns = clicks / 100;
            if full_turns > 0 {
                zero_counter += full_turns;
            }

            if new_pos < *pos {
                zero_counter += 1;
            }
            *pos = new_pos;
        }
    };

    zero_counter as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn test_solve() {
        let input = String::from_str(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution(reader);
        assert_eq!(6, solution)
    }

    #[test]
    fn test_sub_solve() {
        let input = String::from_str(
            "L68
L30
R48
L5",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution(reader);
        assert_eq!(2, solution)
    }

    #[test]
    fn remainder() {
        let x = 0i32.rem_euclid(100);
        assert_eq!(0, x);

        let x = (-1_i32).rem_euclid(100);
        assert_eq!(99, x);
        let x = (-2_i32).rem_euclid(100);
        assert_eq!(98, x);
        let x = (-101i32).rem_euclid(100);
        assert_eq!(99, x);
    }
}
