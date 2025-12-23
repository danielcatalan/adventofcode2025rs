pub struct RedTilePos {
    pub position: (usize, usize),
}

impl RedTilePos {
    pub fn new(position: (usize, usize)) -> Self {
        Self { position }
    }
}
