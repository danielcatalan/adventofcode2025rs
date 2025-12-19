
use crate::{straightline::StraightLine, tilematrix::{GreenTileType, TileMatrix, TileType}, tiles::RedTile};
use crate::tilematrix::RedTileType::{
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight
};


pub struct FloorPlan{
    matrix: TileMatrix,
}

impl FloorPlan {
    pub fn from_redtiles(red_tiles: &Vec<RedTile>) -> Self{

        // init Matrix
        let mut mat = TileMatrix::new(red_tiles);

        // create ranges for Vert/Horiz line
        let mut red_tile_iter = red_tiles.iter();
        let mut prev_tile = red_tile_iter.next().unwrap(); // get First tile
        let mut straight_lines = Vec::new();
        for red_tile in red_tile_iter{
            let line = StraightLine::forms(prev_tile, red_tile).unwrap();
            straight_lines.push(line);
            prev_tile = red_tile;
        }

        //draw lines of green on matrix
        let _ = draw_green_lines(&mut mat, &straight_lines);


        // draw red corners
        for red_tile in red_tiles{
            Self::draw_red_tile(&mut mat, red_tile);
        }

        // Fill rest of elements in matrix
        Self::fill_matrix(&mut mat);

        Self { matrix: mat }
    }
    
    pub(crate) fn is_space(&self, point: &(usize, usize)) -> bool {
        let (r,c) = point;
        let tile = self.matrix.get(*r,*c);
        if let Some(TileType::Space) = tile{
            return true;
        }
        false
    }

    fn draw_red_tile(mat: &mut TileMatrix, red_tile: &RedTile){
        let (r,c) = red_tile.position;
        let top_tile = mat.get(r-1 ,c);
        let bottom_tile = mat.get(r+1, c);
        let left_tile = mat.get(r, c-1);
        let right_tile = mat.get(r, c+1);

        let tile_type = match (top_tile, bottom_tile, left_tile, right_tile) {
            (_, Some(GreenTile(Left)), _, Some(GreenTile(Top))) => TileType::RedTile(TopLeft),
            (Some(GreenTile(Left)), _, Some(GreenTile(Top)), _) => TileType::RedTile(TopLeft),
            (_, Some(GreenTile(Right)), Some(GreenTile(Top)), _) => TileType::RedTile(TopRight),
            (Some(GreenTile(Right)), _, _, Some(GreenTile(Top))) => TileType::RedTile(TopRight),
            (Some(GreenTile(Right)), _, Some(GreenTile(Bottom)), _) => TileType::RedTile(BottomRight),
            (_, Some(GreenTile(Right)), _, Some(GreenTile(Bottom))) => TileType::RedTile(BottomRight),
            (Some(GreenTile(Left)), _, _, Some(GreenTile(Bottom))) => TileType::RedTile(BottomLeft),
            (_, Some(GreenTile(Left)), Some(GreenTile(Bottom)), _) => TileType::RedTile(BottomLeft),

            _ => panic!("Unknown combination of tiles")
        };
        
        //mat.set(r,c, tile_type)
        let item = mat.get_mut(r, c).unwrap();
        *item = tile_type;
    }

    fn fill_matrix(mat: &mut TileMatrix){
        let row_range = mat.row_range();
        for row in row_range{
            let col_range = mat.col_range();
            for col in col_range{
                if let Some(TileType::Unkown) = mat.get(row,col){
                    //check left of this tile
                    let left_tile = mat.get(row,col-1);
                    let new_tile = match left_tile{
                        None => TileType::Space,
                        Some(TileType::GreenTile(Left)) => TileType::GreenTile(Fill),
                        Some(TileType::GreenTile(Fill)) => TileType::GreenTile(Fill),
                        Some(TileType::RedTile(TopLeft)) => TileType::GreenTile(Fill),
                        Some(TileType::RedTile(BottomLeft)) => TileType::GreenTile(Fill),
                        _ => TileType::Space
                    };
                    let item = mat.get_mut(row, col).unwrap();
                    *item = new_tile;
                }
            }
        }
    }
}

enum Orientation{
    Clockwise,
    CounterClockwise
}

use TileType::GreenTile;
use GreenTileType::{Top,Bottom,Left,Right,Fill};
impl Orientation{
    
    fn get_green_tile(&self,line: &StraightLine) -> TileType{
        match (self, line){
            (Orientation::Clockwise, StraightLine::HorizLeftLine(_, _)) => GreenTile(Bottom),
            (Orientation::Clockwise, StraightLine::HorizRightLine(_, _)) => GreenTile(Top),
            (Orientation::Clockwise, StraightLine::VertUpLine(_, _)) => GreenTile(Left),
            (Orientation::Clockwise, StraightLine::VertDownLine(_, _)) => GreenTile(Right),
            (Orientation::CounterClockwise, StraightLine::HorizLeftLine(_, _)) => GreenTile(Top),
            (Orientation::CounterClockwise, StraightLine::HorizRightLine(_, _)) => GreenTile(Bottom),
            (Orientation::CounterClockwise, StraightLine::VertUpLine(_, _)) => GreenTile(Right),
            (Orientation::CounterClockwise, StraightLine::VertDownLine(_, _)) => GreenTile(Left),
        }
    }
}

fn draw_green_lines(mat: &mut TileMatrix, straight_lines: &Vec<StraightLine>) -> Orientation{
    // Get only top most horizontal line 
    let top_line = straight_lines.iter()
        .filter(|line| match line{
            StraightLine::HorizLeftLine(_,_ ) => true,
            StraightLine::HorizRightLine(_,_) => true,
            _ => false
        })
        .min_by(|a,b|{
            let row_a = match a{
                StraightLine::HorizLeftLine(r, _) => r,
                StraightLine::HorizRightLine(r, _) => r,
                _ => panic!("expected only HorizLines")
            };
            let row_b = match b{
                StraightLine::HorizLeftLine(r, _) => r,
                StraightLine::HorizRightLine(r, _) => r,
                _ => panic!("expected only HorizLines")
            };
            row_a.cmp(row_b)
        }).expect("Should have gotten horizline");
    
    // get direction of line being drawn
    let orientation = match top_line {
        StraightLine::HorizLeftLine(_, _) => Orientation::Clockwise,
        StraightLine::HorizRightLine(_, _) => Orientation::CounterClockwise,
        _  => panic!("expected only HorizLines"),
    };
    // Now we can draw green lines
    for line in straight_lines{
        for point in line.points(){
            let green_tile = orientation.get_green_tile(line);
            let (r,c) = point;
            // mat.set(r,c, green_tile)
            let item = mat.get_mut(r, c).unwrap();
            *item = green_tile;
        }
    }
    orientation

}