use std::ops::RangeInclusive;

use crate::tiles::RedTile;

pub enum StraightLine {
    HorizLine(RangeInclusive<usize>),
    VertLine(RangeInclusive<usize>),
}

impl StraightLine {
    pub fn forms(tile1: &RedTile, tile2: &RedTile) -> Option<Self> {
        let position1 = &tile1.position;
        let position2 = &tile2.position;

        if position1.0 == position2.0 {
            let min = position1.1.min(position2.1);
            let max = position1.1.max(position2.1);
            return Some(Self::HorizLine(min..=max));
        } else if position1.1 == position2.1 {
            let min = position1.0.min(position2.0);
            let max = position1.0.max(position2.0);
            return Some(Self::VertLine(min..=max));
        }
        return None;
    }
}
