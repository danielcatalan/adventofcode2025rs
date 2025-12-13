#[allow(unused_imports)]
use regex::Regex;
use std::io::BufRead;
#[allow(unused_imports)]
use std::sync::LazyLock;

use crate::{theater::Theater, tiles::RedTile};

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn parse_tiles<R: BufRead>(reader: R) -> Theater {
    let tiles: Vec<RedTile> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_one_tile(&line))
        .collect();
    Theater::new(tiles)
}

fn parse_one_tile(line: &str) -> RedTile {
    let positions: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();

    RedTile::new((positions[0], positions[1]))
}
