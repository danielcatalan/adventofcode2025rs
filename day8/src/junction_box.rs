use std::usize;

#[derive(Eq,PartialEq,Hash)]
pub struct JunctionBox {
    position: (usize,usize,usize),
}

impl JunctionBox {
    pub fn new(x:usize, y: usize, z:usize)->Self{
        Self { position: (x,y,z),}
    }

    pub fn get_postion(&self) -> &(usize,usize,usize){
        &self.position
    }
}