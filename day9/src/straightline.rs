use std::{iter::Map, ops::RangeInclusive};

use crate::tiles::RedTile;

pub enum StraightLine {
    HorizLine(usize,RangeInclusive<usize>),
    VertLine(RangeInclusive<usize>,usize),
}

impl StraightLine {
    pub fn forms(tile1: &RedTile, tile2: &RedTile) -> Option<Self> {
        let position1 = &tile1.position;
        let position2 = &tile2.position;

        if position1.0 == position2.0 {
            let min = position1.1.min(position2.1);
            let max = position1.1.max(position2.1);
            return Some(Self::HorizLine(position1.0, min..=max));
        } else if position1.1 == position2.1 {
            let min = position1.0.min(position2.0);
            let max = position1.0.max(position2.0);
            return Some(Self::VertLine(min..=max, position1.1));
        }
        return None;
    }

    pub fn points(&self) -> Vec<(usize,usize)>{
        match self {
            StraightLine::HorizLine(row, cols) =>  {
                let x = cols.to_owned()
                    .map(|col| (*row,col));
                x.collect()
            },
            StraightLine::VertLine(rows, col) => {
                let x = rows.to_owned()
                    .map(|row| (row, *col));
                x.collect()
            }
        }
    }
}
