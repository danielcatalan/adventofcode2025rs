use crate::{area::Area, floorplan::FloorPlan, tiles::RedTilePos};

pub struct Theater {
    tiles: Vec<RedTilePos>,
}

impl Theater {
    pub fn new(tiles: Vec<RedTilePos>) -> Theater {
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
        println!("**Build Floor-plan**");
        let floorplan = FloorPlan::from_redtiles(&self.tiles);

        // Find all Areas
        println!("**Find all possible Areas**");
        let areas = find_areas(&self.tiles);

        println!("**Get largest Area that fits**");
        // println!("Area count: {}", areas.len());
        'arealoop: for area in areas.iter() {
            // println!("Try area{}", i);
            let range = area.get_inner_perimeter();
            for (r, c) in range {
                if floorplan.contains_rg_tiles(r, c) {
                    continue 'arealoop;
                }
            }
            return area.area;
        }
        panic!("Could not find area")
    }
}

fn find_areas(tiles: &Vec<RedTilePos>) -> Vec<Area<'_>> {
    let tile_len = tiles.len();
    let mut areas = Vec::new();
    for tile1_idx in 0..(tile_len - 1) {
        for tile2_idx in (tile1_idx + 1)..tile_len {
            let tile1 = &tiles[tile1_idx];
            let tile2 = &tiles[tile2_idx];
            let area = Area::new(tile1, tile2);
            areas.push(area);
        }
    }
    areas.sort_by(|a1, a2| a1.area.cmp(&a2.area)); // min --> max
    areas.reverse(); // max --> min
    areas
}
