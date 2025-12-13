pub struct RedTile {
    position: (usize, usize),
}

impl RedTile {
    pub fn new(position: (usize, usize)) -> Self {
        Self { position }
    }
}
