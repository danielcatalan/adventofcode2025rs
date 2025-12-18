
use crate::{straightline::{self, StraightLine}, tilematrix::{GreenTileType, TileMatrix, TileType}, tiles::RedTile};



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

        //draw matrix from lines of green
        let orientation = draw_green_lines(&mut mat, &straight_lines);


        // draw red corners
        for red_tile in red_tiles{
            mat.set(red_tile.position, TileType::RedTile)
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

enum Orientation{
    Clockwise,
    CounterClockwise
}

use TileType::GreenTile;
use GreenTileType::{Top,Bottom,Left,Right};
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
            mat.set(point, green_tile)
        }
    }
    orientation

}