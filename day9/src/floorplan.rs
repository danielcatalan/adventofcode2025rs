use std::usize;

use crate::area::{Area, Perimeter};
use crate::edge::Edge;
use crate::position::Position;
use crate::tilematrix::RedTileType::{BottomLeft, BottomRight, TopLeft, TopRight};
use crate::{
    straightline::StraightLine,
    tilematrix::{GreenTileType, TileMatrix, TileType},
    tiles::RedTilePos,
};

pub struct FloorPlan {
    edges: Vec<Edge>,
}

impl FloorPlan {
    pub fn from_redtiles(red_tiles: &Vec<RedTilePos>) -> Self {
        // create ranges for Edge line
        println!("Create lines from redtiles...");
        let mut red_tile_iter = red_tiles.iter();
        let mut prev_tile = red_tile_iter.next().unwrap(); // get First tile
        let mut edge_lines = Vec::new();
        for red_tile in red_tile_iter {
            let pos1 = prev_tile.get_pos();
            let pos2 = red_tile.get_pos();
            let edge = Edge{
                start: pos1,
                end: pos2,
            };
            edge_lines.push(edge);
            prev_tile = red_tile;
        }
        // do last connection
        let first_tile = &red_tiles[0];
        let pos1 = prev_tile.get_pos();
        let pos2 = first_tile.get_pos();
        let edge = Edge{
            start: pos1,
            end: pos2,
        };
        
        edge_lines.push(edge);


        Self { edges:  edge_lines}
    }

    fn draw_red_tile(mat: &mut TileMatrix, red_tile: &RedTilePos) {
        let (r, c) = red_tile.position;
        let top_tile = mat.get(r - 1, c);
        let bottom_tile = mat.get(r + 1, c);
        let left_tile = mat.get(r, c - 1);
        let right_tile = mat.get(r, c + 1);

        let tile_type = match (top_tile, bottom_tile, left_tile, right_tile) {
            (_, Some(GreenTile(Left)), _, Some(GreenTile(Top))) => TileType::RedTile(TopLeft),
            (Some(GreenTile(Left)), _, Some(GreenTile(Top)), _) => TileType::RedTile(TopLeft),
            (_, Some(GreenTile(Right)), Some(GreenTile(Top)), _) => TileType::RedTile(TopRight),
            (Some(GreenTile(Right)), _, _, Some(GreenTile(Top))) => TileType::RedTile(TopRight),
            (Some(GreenTile(Right)), _, Some(GreenTile(Bottom)), _) => {
                TileType::RedTile(BottomRight)
            }
            (_, Some(GreenTile(Right)), _, Some(GreenTile(Bottom))) => {
                TileType::RedTile(BottomRight)
            }
            (Some(GreenTile(Left)), _, _, Some(GreenTile(Bottom))) => TileType::RedTile(BottomLeft),
            (_, Some(GreenTile(Left)), Some(GreenTile(Bottom)), _) => TileType::RedTile(BottomLeft),

            _ => panic!("Unknown combination of tiles"),
        };

        // let item = mat.get_mut(r, c).unwrap();
        // *item = tile_type;
        mat.set(r, c, tile_type);
    }

    pub(crate) fn is_perimeter_in_bounds(&self, perimeter: &Perimeter) -> bool {
        // check Top
        for (row, col) in &perimeter.top {
            let tile = self.matrix.get(*row, *col).unwrap();
            if let TileType::Unkown = tile {
                if !self.walk2((row, col), WalkDir::Up) {
                    return false; // Found invalid tile walking up
                }
                // Found valid tile. move on to next tile
            }
        }

        // check Bottom
        for (row, col) in &perimeter.bottom {
            let tile = self.matrix.get(*row, *col).unwrap();
            if let TileType::Unkown = tile {
                if !self.walk2((row, col), WalkDir::Down) {
                    return false; // Found invalid tile walking down
                }
                // Found valid tile. move on to next tile
            }
        }
        // check Left
        for (row, col) in &perimeter.left {
            let tile = self.matrix.get(*row, *col).unwrap();
            if let TileType::Unkown = tile {
                if !self.walk2((row, col), WalkDir::Left) {
                    return false; // Found invalid tile walking left
                }
                // Found valid tile. move on to next tile
            }
        }

        // check Right
        for (row, col) in &perimeter.right {
            let tile = self.matrix.get(*row, *col).unwrap();
            if let TileType::Unkown = tile {
                if !self.walk2((row, col), WalkDir::Right) {
                    return false; // Found invalid tile walking right
                }
                // Found valid tile. move on to next tile
            }
        }

        true
    }

    fn walk(&self, pos: (&usize, &usize), dir: WalkDir) -> bool {
        let mut row = *pos.0;
        let mut col = *pos.1;

        loop {
            (row, col) = dir.next_pos(row, col);
            let tile = self.matrix.get(row, col);
            if let None = tile {
                return false; // out of bounds
            } else if let Some(tile) = tile {
                if let TileType::Unkown = tile {
                    continue;
                }
                if dir.is_accetible_tile(tile) {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }

    fn walk2(&self, pos: (&usize, &usize), dir: WalkDir) -> bool {
        let row = *pos.0;
        let col = *pos.1;

        let predicate: Box<dyn Fn(usize, usize) -> bool> = match dir {
            WalkDir::Up => Box::new(|r, c| (r < row) && (c == col)),
            WalkDir::Down => Box::new(|r, c| (r > row) && (c == col)),
            WalkDir::Left => Box::new(|r, c| (r == row) && (c < col)),
            WalkDir::Right => Box::new(|r, c| (r == row) && (c > col)),
        };

        let map = &self.matrix.map;
        let tiles_in_line: Vec<_> = map.iter().filter(|((r, c), _)| predicate(*r, *c)).collect();
        let len = tiles_in_line.len();
        if len == 1 {
            return true;
        } else if len == 0 {
            return false;
        } else if len == 586 {
            println!("{}", tiles_in_line[0].0.0);
            panic!();
        } else {
            panic!("oh ooh! did not expect {} tiles in line.", len);
        }
    }

    pub(crate) fn contains_rg_tiles(&self, r: usize, c: usize) -> bool {
        let tile = self.matrix.map.get(&(r, c));
        return true;
        // match tile {
        //     Some(_) => true,
        //     None => false,
        // }
    }
    
    pub(crate) fn check_collition(&self, area: &Area) -> bool {
        todo!()
    }
}

enum WalkDir {
    Up,
    Down,
    Left,
    Right,
}
impl WalkDir {
    fn next_pos(&self, row: usize, col: usize) -> (usize, usize) {
        match self {
            WalkDir::Up => (row - 1, col),
            WalkDir::Down => (row + 1, col),
            WalkDir::Left => (row, col - 1),
            WalkDir::Right => (row, col + 1),
        }
    }

    fn is_accetible_tile(&self, tile: &TileType) -> bool {
        match self {
            WalkDir::Up => match tile {
                GreenTile(Top) | RedTile(TopLeft) | RedTile(TopRight) => true,
                _ => false,
            },
            WalkDir::Down => match tile {
                GreenTile(Bottom) | RedTile(BottomLeft) | RedTile(BottomRight) => true,
                _ => false,
            },
            WalkDir::Left => match tile {
                GreenTile(Left) | RedTile(TopLeft | BottomLeft) => true,
                _ => false,
            },
            WalkDir::Right => match tile {
                GreenTile(Right) | RedTile(TopRight | BottomRight) => true,
                _ => false,
            },
        }
    }
}

enum Orientation {
    Clockwise,
    CounterClockwise,
}

use GreenTileType::{Bottom, Fill, Left, Right, Top};
use TileType::{GreenTile, RedTile};
impl Orientation {
    fn get_green_tile(&self, line: &StraightLine) -> TileType {
        match (self, line) {
            (Orientation::Clockwise, StraightLine::HorizLeftLine(_, _)) => GreenTile(Bottom),
            (Orientation::Clockwise, StraightLine::HorizRightLine(_, _)) => GreenTile(Top),
            (Orientation::Clockwise, StraightLine::VertUpLine(_, _)) => GreenTile(Left),
            (Orientation::Clockwise, StraightLine::VertDownLine(_, _)) => GreenTile(Right),
            (Orientation::CounterClockwise, StraightLine::HorizLeftLine(_, _)) => GreenTile(Top),
            (Orientation::CounterClockwise, StraightLine::HorizRightLine(_, _)) => {
                GreenTile(Bottom)
            }
            (Orientation::CounterClockwise, StraightLine::VertUpLine(_, _)) => GreenTile(Right),
            (Orientation::CounterClockwise, StraightLine::VertDownLine(_, _)) => GreenTile(Left),
        }
    }
}

fn draw_green_lines(mat: &mut TileMatrix, straight_lines: &Vec<StraightLine>) -> Orientation {
    // Get only top most horizontal line
    let top_line = straight_lines
        .iter()
        .filter(|line| match line {
            StraightLine::HorizLeftLine(_, _) => true,
            StraightLine::HorizRightLine(_, _) => true,
            _ => false,
        })
        .min_by(|a, b| {
            let row_a = match a {
                StraightLine::HorizLeftLine(r, _) => r,
                StraightLine::HorizRightLine(r, _) => r,
                _ => panic!("expected only HorizLines"),
            };
            let row_b = match b {
                StraightLine::HorizLeftLine(r, _) => r,
                StraightLine::HorizRightLine(r, _) => r,
                _ => panic!("expected only HorizLines"),
            };
            row_a.cmp(row_b)
        })
        .expect("Should have gotten horizline");

    // get direction of line being drawn
    let orientation = match top_line {
        StraightLine::HorizLeftLine(_, _) => Orientation::CounterClockwise,
        StraightLine::HorizRightLine(_, _) => Orientation::Clockwise,
        _ => panic!("expected only HorizLines"),
    };
    // Now we can draw green lines
    for line in straight_lines {
        for point in line.points() {
            let green_tile = orientation.get_green_tile(line);
            let (r, c) = point;

            // let item = mat.get_mut(r, c).unwrap();
            // *item = green_tile;
            mat.set(r, c, green_tile);
        }
    }
    orientation
}
