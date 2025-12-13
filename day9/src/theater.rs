use crate::{area::Area, straightline::StraightLine, tiles::RedTile};

pub struct Theater {
    tiles: Vec<RedTile>,
}

impl Theater {
    pub fn new(tiles: Vec<RedTile>) -> Theater {
        Theater { tiles }
    }

    pub fn largest_area(&self) -> usize {
        let tile_len = self.tiles.len();
        let mut areas = Vec::new();
        for tile1_idx in 0..(tile_len - 1) {
            for tile2_idx in (tile1_idx + 1)..tile_len {
                let tile1 = &self.tiles[tile1_idx];
                let tile2 = &self.tiles[tile2_idx];
                let area = Area::new(tile1, tile2);
                areas.push(area);
            }
        }
        areas.sort_by(|a1, a2| a1.area.cmp(&a2.area));
        areas.reverse();
        areas[0].area
    }

    pub fn largest_red_grean_area(&self) -> usize {
        let tile_len = self.tiles.len();
        let mut areas = Vec::new();
        let mut straight_lines = Vec::new();
        for tile1_idx in 0..(tile_len - 1) {
            for tile2_idx in (tile1_idx + 1)..tile_len {
                let tile1 = &self.tiles[tile1_idx];
                let tile2 = &self.tiles[tile2_idx];
                if let Some(line) = StraightLine::forms(&tile1, &tile2) {
                    straight_lines.push(line);
                }
                let area = Area::new(tile1, tile2);
                areas.push(area);
            }
        }
        areas.sort_by(|a1, a2| a1.area.cmp(&a2.area));

        // Find largest area within red and green
        for area in areas.iter().rev() {
            let tile1 = area.tile1;
            let tile2 = area.tile2;
        }
        todo!()
    }
}
