use crate::position::Position;

pub struct RedTilePos {
    pub position: (usize, usize),
}

impl RedTilePos {
    pub fn new(position: (usize, usize)) -> Self {
        Self { position }
    }
    pub fn get_pos(&self) -> Position{
        Position{
            row: self.position.0,
            col: self.position.1,
        }
    }
}
