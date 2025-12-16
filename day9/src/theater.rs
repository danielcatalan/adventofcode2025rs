use crate::{area::Area, floorplan::FloorPlan, tiles::RedTile};

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
        // Build Floor-plan
        let floorplan = FloorPlan::from_redtiles(&self.tiles);
        
        // Find Areas
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
        areas.sort_by(|a1, a2| a1.area.cmp(&a2.area)); // min --> max
        areas.reverse(); // max --> min
        
        'area_loop: for area in areas {
            let perimeter = area.get_perimeter();
            for point in perimeter{
                if floorplan.is_space(&point){
                    continue 'area_loop;
                }
            }
            // no spaces in this area
            return area.area
        }
        panic!("Could not find area")
    }
}
