use crate::tiles::RedTile;

pub struct Theater {
    tiles: Vec<RedTile>,
}

impl Theater {
    pub fn new(tiles: Vec<RedTile>) -> Theater {
        Theater { tiles }
    }

    pub fn largest_area(&self) -> usize {
        todo!()
    }
}
