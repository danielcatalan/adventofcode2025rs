/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").unwrap());
 *
 */
use regex::Regex;
use std::{ops::RangeInclusive, sync::LazyLock};

pub struct IdRange {
    pub first: u32,
    pub last: u32,
}

impl IdRange {
    pub fn iter(&self) -> RangeInclusive<u32> {
        return self.first ..= self.last;
    }
}

pub fn parse_to_range(s: &str) -> IdRange {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?P<first>[0-9]{1,})-(?P<last>[0-9]{1,})").unwrap());
    let caps = RE.captures(s).expect("captures should have found range");
    let first = caps["first"].parse().expect("First Id could not be parsed");
    let last = caps["last"].parse().expect("last Id could not be parsed");
    IdRange { first, last }
}
