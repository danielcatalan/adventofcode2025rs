use matrix::Matrix;
use crate::tiles::RedTile;

#[derive(PartialEq)]
pub enum GreenTileType{
    Top,
    Bottom,
    Left,
    Right,
}
#[derive(PartialEq)]
pub enum RedTileType{
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}


#[derive(PartialEq)]
pub enum TileType{
    Unkown,
    GreenTile(GreenTileType),
    RedTile(RedTileType),
    Space
}

pub struct TileMatrix{
    mat: Matrix<TileType>
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

        }
        todo!()
    }
    
    pub(crate) fn get(&self, point: &(usize, usize)) -> &TileType {
        todo!()
    }
    
    pub(crate) fn set(&mut self, point: (usize, usize), green_tile: TileType) {
        todo!()
    }
}