use once_cell::sync::Lazy;
use regex::Regex;

pub enum Rotation {
    Left(i32),
    Right(i32),
}

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn parse_to_rot(line: &str) -> Rotation {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?P<rot>[LR])(?P<clicks>[0-9]{1,})").expect("RegEx Formula not valid")
    });

    let cap = RE.captures(line).expect("could not parse line");
    let rot: &str = cap["rot"].as_ref();
    let clicks: u32 = cap["clicks"].parse().expect("could not parse clicks");

    match rot {
        "L" => Rotation::Left(clicks as i32),
        "R" => Rotation::Right(clicks as i32),
        &_ => panic!("unexpected line being parsed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pasrsing() {
        let rotation = parse_to_rot("L68");

        if let Rotation::Left(clicks) = rotation {
            assert_eq!(68, clicks)
        } else if let Rotation::Right(clicks) = rotation {
            assert_eq!(68, clicks);
            panic!("parsed as right instead of left")
        } else {
            panic!("parsing is incorrect")
        }
    }
}
