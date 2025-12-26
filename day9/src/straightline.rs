use std::ops::Range;

use crate::tiles::RedTilePos;

pub enum StraightLine {
    HorizLeftLine(usize, Range<usize>),
    HorizRightLine(usize, Range<usize>),
    VertUpLine(Range<usize>, usize),
    VertDownLine(Range<usize>, usize),
}

impl StraightLine {
    pub fn forms(tile1: &RedTilePos, tile2: &RedTilePos) -> Option<Self> {
        let position1 = &tile1.position;
        let position2 = &tile2.position;

        if position1.0 == position2.0 {
            // Horizontal line
            let r = position1.0;
            let c1 = position1.1;
            let c2 = position2.1;
            if c1 < c2 {
                // going Right
                return Some(Self::HorizRightLine(r, (c1 + 1)..c2));
            } else {
                // assume going Left
                return Some(Self::HorizLeftLine(r, (c2 + 1)..c1));
            }
        } else if position1.1 == position2.1 {
            // Vertical Line
            let c = position1.1;
            let r1 = position1.0;
            let r2 = position2.0;
            if r1 < r2 {
                // Going Down
                return Some(Self::VertDownLine((r1 + 1)..r2, c));
            } else {
                // Assume Going up
                return Some(Self::VertUpLine((r2 + 1)..r1, c));
            }
        }
        return None;
    }

    pub fn points(&self) -> Vec<(usize, usize)> {
        match self {
            StraightLine::HorizLeftLine(row, cols) => points_from_horiz(row, cols),
            StraightLine::HorizRightLine(row, cols) => points_from_horiz(row, cols),

            StraightLine::VertUpLine(rows, col) => points_from_vert(rows, col),
            StraightLine::VertDownLine(rows, col) => points_from_vert(rows, col),
        }
    }
}

fn points_from_horiz(row: &usize, cols: &Range<usize>) -> Vec<(usize, usize)> {
    let x = cols.to_owned().map(|col| (*row, col));
    x.collect()
}

fn points_from_vert(rows: &Range<usize>, col: &usize) -> Vec<(usize, usize)> {
    let x = rows.to_owned().map(|row| (row, *col));
    x.collect()
}
