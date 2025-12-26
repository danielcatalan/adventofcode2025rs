use core::panic;
use std::{collections::HashMap, ops::Range};

use crate::tiles::RedTilePos;
use matrix::Matrix;

#[derive(PartialEq, Clone, Debug)]
pub enum GreenTileType {
    Top,
    Bottom,
    Left,
    Right,
    Fill,
}
#[derive(PartialEq, Clone, Debug)]
pub enum RedTileType {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(PartialEq, Default, Clone, Debug)]
pub enum TileType {
    #[default]
    Unkown,
    GreenTile(GreenTileType),
    RedTile(RedTileType),
}

pub struct TileMatrix {
    pub map: HashMap<(usize, usize), TileType>,
    row_min: usize,
    row_max: usize,
    col_min: usize,
    col_max: usize,
}
impl TileMatrix {
    pub fn new(tiles: &Vec<RedTilePos>) -> Self {
        let mut row_min = usize::MAX;
        let mut row_max = usize::MIN;
        let mut col_min = usize::MAX;
        let mut col_max = usize::MIN;

        for tile in tiles {
            let row = tile.position.0;
            let col = tile.position.1;
            if row < row_min {
                row_min = row
            } else if row > row_max {
                row_max = row;
            }

            if col < col_min {
                col_min = col;
            } else if col > col_max {
                col_max = col;
            }
        }
        let map = HashMap::new();

        Self {
            map,
            row_min,
            row_max,
            col_min,
            col_max,
        }
    }

    /// Returns tile-type if within bounds. Returns `None` if
    /// out of bounds
    pub(crate) fn get(&self, row: usize, col: usize) -> Option<&TileType> {
        if (row < self.row_min)
            || (row > self.row_max)
            || (col < self.col_min)
            || (col > self.col_max)
        {
            return None;
        }

        // self.map.get(true_row, true_col)
        if let Some(tile) = self.map.get(&(row, col)) {
            return Some(tile);
        } else {
            Some(&TileType::Unkown)
        }
    }
    fn check_bound(&self, row: usize, col: usize) -> bool {
        if (row < self.row_min)
            || (row > self.row_max)
            || (col < self.col_min)
            || (col > self.col_max)
        {
            return false;
        }
        true
    }

    pub fn set(&mut self, row: usize, col: usize, tile_type: TileType) {
        if !self.check_bound(row, col) {
            panic!("Out of bounds");
        }

        self.map.insert((row, col), tile_type);
    }

    pub(crate) fn row_range(&self) -> Range<usize> {
        let row_max = self.row_max + 1;
        let row_offset = self.row_min;

        row_offset..row_max
    }

    pub(crate) fn col_range(&self) -> Range<usize> {
        let col_max = self.col_max + 1;
        let col_offset = self.col_min;
        col_offset..col_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let tile_matrix = example_mat();
        assert_eq!(TileType::Unkown, *tile_matrix.get(100, 100).unwrap())
    }

    #[test]
    fn test_get_mut() {
        let mut tile_matrix = example_mat(); // 3x2 matrix
        assert_eq!(TileType::Unkown, *tile_matrix.get(100, 100).unwrap());

        // let item = tile_matrix.get_mut(100, 100).unwrap();
        // *item = TileType::Space;
        tile_matrix.set(100, 100, TileType::GreenTile(GreenTileType::Top));
        assert_eq!(
            TileType::GreenTile(GreenTileType::Top),
            *tile_matrix.get(100, 100).unwrap()
        );
    }

    // 3r x 2c matrix
    fn example_mat() -> TileMatrix {
        fn red_tile(r: usize, c: usize) -> RedTilePos {
            RedTilePos::new((r, c))
        }
        let red_tiles = vec![
            red_tile(100, 100),
            red_tile(100, 101),
            red_tile(102, 101),
            red_tile(102, 100),
        ];
        TileMatrix::new(&red_tiles)
    }
}
