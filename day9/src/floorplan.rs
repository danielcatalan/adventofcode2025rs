
use crate::{straightline::StraightLine, tilematrix::{TileMatrix, TileType}, tiles::RedTile};



pub struct FloorPlan{
    matrix: TileMatrix,
}

impl FloorPlan {
    pub fn from_redtiles(tiles: &Vec<RedTile>) -> Self{

        // init Matrix
        let mut mat = TileMatrix::new(tiles);

        // create ranges for Vert/Horiz line
        let mut red_tile_iter = tiles.iter();
        let mut prev_tile = red_tile_iter.next().unwrap();
        let mut straight_lines = Vec::new();
        for tile in red_tile_iter{
            let line = StraightLine::forms(prev_tile, tile).unwrap();
            straight_lines.push(line);
            prev_tile = tile;
        }
        //draw matrix from lines of green
        for line in straight_lines{
            for point in line.points(){
                mat.set(point, TileType::GreenTile)
            }
        }

        todo!()
    }
    
    pub(crate) fn is_space(&self, point: &(usize, usize)) -> bool {
        let tile = self.matrix.get(point);
        if *tile == TileType::Space{
            return true;
        }
        false
    }
}