use matrix::Matrix;
use crate::tiles::RedTile;

#[derive(PartialEq,Clone,Debug)]
pub enum GreenTileType{
    Top,
    Bottom,
    Left,
    Right,
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

        Self{mat,row_offset:row_min, col_offset:col_min}
    }
    
    pub(crate) fn get(&self, row: usize, col: usize) -> &TileType {
        todo!()
    }
    
    pub(crate) fn set(&mut self, row: usize, col: usize, green_tile: TileType) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilematrix() {
        fn red_tile(r:usize,c:usize) -> RedTile{
            RedTile::new((r,c))
        }

        let red_tiles = vec![red_tile(100, 100), red_tile(100, 101),red_tile(102, 101),red_tile(102, 100)];
        let tile_matrix = TileMatrix::new(&red_tiles);
        let inner_mat = &tile_matrix.mat;
        assert_eq!(3, inner_mat.row_len());
        assert_eq!(2, inner_mat.col_len());

        assert_eq!(TileType::Unkown, *tile_matrix.get(100,100))
    }
}