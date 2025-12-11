use std::io::BufRead;

use crate::diagram::{Cell, TachyonManifold};


pub fn parse_tachyon_manifold<R: BufRead>(reader: R) -> TachyonManifold{
    let content = reader.lines()
        .map(|row| row.unwrap())
        .map(|row| parse_row(row))
        .collect();
    TachyonManifold::from_vec(content)
}

fn parse_row(row: String) -> Vec<Cell>{
    row.as_bytes().iter()
        .map(|b|Cell::from_byte(b))
        .collect()
}