use std::ops::{Range, RangeInclusive};

use crate::tiles::RedTilePos;

struct Position{
    row: usize,
    col: usize,
}

impl Position{
    fn get_position(&self)->(usize,usize){
        (self.row,self.col)
    }
}

pub struct Area {
    pub tile1: Position,
    pub tile2: Position,
    pub area: usize,
    left_pos: usize,
    right_pos: usize,
    top_pos: usize,
    bottom_pos:usize
}

impl Area {
    pub fn new(tile1: &RedTilePos, tile2: & RedTilePos) -> Self {
        let area = calc_area(&tile1.position, &tile2.position);
        let position1 = Position{
            row: tile1.position.0,
            col: tile1.position.1,
        };
        let position2 = Position{
            row: tile2.position.0,
            col: tile2.position.1,
        };
        // left right top bottom
        let left_pos =  tile1.position.1.min(tile2.position.1);
        let right_pos = tile1.position.1.max(tile2.position.1);
        let top_pos = tile1.position.0.min(tile2.position.0);
        let bottom_pos = tile1.position.0.max(tile2.position.0);
        Area { 
            tile1: position1, 
            tile2: position2, 
            area,left_pos,right_pos,top_pos,bottom_pos }
    }

    pub fn get_inner_perimeter(&self) -> Vec<(usize, usize)> {
        let tile1_pos = self.tile1.get_position();
        let tile2_pos = self.tile2.get_position();
        let row_min = tile1_pos.0.min(tile2_pos.0) + 1;
        let row_max = tile1_pos.0.max(tile2_pos.0) - 1;
        let col_min = tile1_pos.1.min(tile2_pos.1) + 1;
        let col_max = tile1_pos.1.max(tile2_pos.1) - 1;

        let mut perimeter: Vec<(usize, usize)> = Vec::new();
        // Top of area
        let tb = col_min..=col_max;
        let mut top = tb.to_owned().map(|col| (row_min, col)).collect();
        perimeter.append(&mut top);

        let mut bottom = tb.to_owned().map(|col| (row_max, col)).collect();
        perimeter.append(&mut bottom);

        let lr = row_min..=row_max;
        let mut left = lr.to_owned().map(|row| (row, col_min)).collect();
        perimeter.append(&mut left);
        let mut right = lr.to_owned().map(|row| (row, col_max)).collect();
        perimeter.append(&mut right);

        return perimeter;
    }

    pub fn get_perimeter(&self) -> Perimeter {
        let tile1_pos = self.tile1.get_position();
        let tile2_pos = self.tile2.get_position();
        let row_min = tile1_pos.0.min(tile2_pos.0);
        let row_max = tile1_pos.0.max(tile2_pos.0);
        let col_min = tile1_pos.1.min(tile2_pos.1);
        let col_max = tile1_pos.1.max(tile2_pos.1);

        // Top of area
        let tb = col_min..=col_max;
        let top = tb.to_owned().map(|col| (row_min, col)).collect();

        let bottom = tb.to_owned().map(|col| (row_max, col)).collect();

        let lr = row_min..=row_max;
        let left = lr.to_owned().map(|row| (row, col_min)).collect();
        let right = lr.to_owned().map(|row| (row, col_max)).collect();

        return Perimeter {
            top,
            bottom,
            left,
            right,
        };
    }
}

pub struct Perimeter {
    pub top: Vec<(usize, usize)>,
    pub bottom: Vec<(usize, usize)>,
    pub left: Vec<(usize, usize)>,
    pub right: Vec<(usize, usize)>,
}

fn calc_area(pos1: &(usize, usize), pos2: &(usize, usize)) -> usize {
    let x1 = pos1.0 as isize;
    let y1 = pos1.1 as isize;

    let x2 = pos2.0 as isize;
    let y2 = pos2.1 as isize;

    let x = (x1 - x2).abs() + 1;
    let y = (y1 - y2).abs() + 1;

    let area = x * y;
    area as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let pos1 = (2, 5);
        let pos2 = (11, 1);
        let area = calc_area(&pos2, &pos1);
        assert_eq!(50, area)
    }
}
