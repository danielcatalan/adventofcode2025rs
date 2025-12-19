use std::ops::Range;

use matrix::Matrix;
use crate::tiles::RedTile;

#[derive(PartialEq,Clone,Debug)]
pub enum GreenTileType{
    Top,
    Bottom,
    Left,
    Right,
    Fill,
}
#[derive(PartialEq, Clone,Debug)]
pub enum RedTileType{
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}


#[derive(PartialEq,Default,Clone, Debug)]
pub enum TileType{
    #[default] Unkown,
    GreenTile(GreenTileType),
    RedTile(RedTileType),
    Space
}

pub struct TileMatrix{
    mat: Matrix<TileType>,
    row_offset: usize,
    col_offset: usize,
}
impl TileMatrix {
    pub fn new(tiles: &Vec<RedTile>) -> Self {
        let mut row_min = usize::MAX;
        let mut row_max = usize::MIN;
        let mut col_min = usize::MAX;
        let mut col_max = usize::MIN;
        
        for tile in tiles{
            let row = tile.position.0;
            let col = tile.position.1;
            if row < row_min{
                row_min = row
            }
            else if row > row_max{
                row_max = row;
            }

            if col < col_min{
                col_min = col;
            }
            else if col > col_max{
                col_max = col;
            }
        }
        let row_len = (row_max - row_min) + 1;
        let col_len = (col_max - col_min) + 1;
        let mat = Matrix::new(row_len, col_len);

        Self{mat, row_offset:row_min, col_offset:col_min}
    }
    
    pub(crate) fn get(&self, row: usize, col: usize) -> Option<&TileType> {
        if (row < self.row_offset) || (col < self.col_offset){
            return None;
        }
        let true_row = row - self.row_offset;
        let true_col = col - self.col_offset;
        self.mat.get(true_row, true_col)
    }

    pub(crate) fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut TileType> {
        if (row < self.row_offset) || (col < self.col_offset){
            return None;
        }
        let true_row = row - self.row_offset;
        let true_col = col - self.col_offset;
        self.mat.get_mut(true_row, true_col)
    }
    
    pub(crate) fn row_range(&self) -> Range<usize> {
        let row_max = self.row_offset + self.mat.row_len();
        let row_offset = self.row_offset;

        row_offset..row_max
    }
    
    pub(crate) fn col_range(&self) -> Range<usize> {
        let col_max = self.col_offset + self.mat.col_len();
        let col_offset = self.col_offset;
        col_offset..col_max
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {


        let tile_matrix = example_mat();
        let inner_mat = &tile_matrix.mat;
        assert_eq!(3, inner_mat.row_len());
        assert_eq!(2, inner_mat.col_len());

        assert_eq!(TileType::Unkown, *tile_matrix.get(100,100).unwrap())
    }

    #[test]
    fn test_get_mut() {
        let mut tile_matrix = example_mat(); // 3x2 matrix
        assert_eq!(TileType::Unkown, *tile_matrix.get(100,100).unwrap());
        
        let item = tile_matrix.get_mut(100, 100).unwrap();
        *item = TileType::Space;

        assert_eq!(TileType::Space, *tile_matrix.get(100,100).unwrap());
        let inner_mat = tile_matrix.mat;
        assert_eq!(TileType::Space, *inner_mat.get(0,0).unwrap());
    }



    /// 3r x 2c matrix
    fn example_mat() -> TileMatrix{
        fn red_tile(r:usize,c:usize) -> RedTile{
            RedTile::new((r,c))
        }
        let red_tiles = vec![red_tile(100, 100), red_tile(100, 101),red_tile(102, 101),red_tile(102, 100)];
        TileMatrix::new(&red_tiles)
    }
}