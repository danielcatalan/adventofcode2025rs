pub struct Position{
    pub row: usize,
    pub col: usize,
}

impl Position{
    pub fn get_position(&self)->(usize,usize){
        (self.row,self.col)
    }
}