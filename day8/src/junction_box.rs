use std::usize;

pub struct JunctionBox {
    position: (usize,usize,usize),
    circ_id: usize
}

impl JunctionBox {
    pub fn new(x:usize, y: usize, z:usize, circ_id:usize)->Self{
        Self { position: (x,y,z), circ_id}
    }

    pub fn get_postion(&self) -> &(usize,usize,usize){
        &self.position
    }
}