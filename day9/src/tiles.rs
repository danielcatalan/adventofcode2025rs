pub struct RedTile {
    pub position: (usize, usize),
}

impl RedTile {
    pub fn new(position: (usize, usize)) -> Self {
        Self { position }
    }
}
