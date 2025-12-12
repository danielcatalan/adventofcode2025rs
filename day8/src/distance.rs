use crate::junction_box::JunctionBox;

pub struct Distance<'a,'b> {
    pub jb1: &'a JunctionBox,
    pub jb2: &'b JunctionBox,
    pub distance: f64
}

impl<'a,'b> Distance<'a,'b> {
    pub fn new(jb1: &'a JunctionBox, jb2: &'b JunctionBox) -> Self{
        let distance = calc_distance(jb1.get_postion(),jb2.get_postion());

        Distance{jb1,jb2,distance}
    }
}

fn calc_distance(pos1: &(usize,usize,usize), pos2: &(usize,usize,usize)) -> f64{
    let x1 = pos1.0 as f64;
    let y1 = pos1.1 as f64;
    let z1 = pos1.2 as f64;

    let x2 = pos2.0 as f64;
    let y2 = pos2.1 as f64;
    let z2 = pos2.2 as f64;

    let x = (x1-x2).powf(2.0);
    let y = (y1-y2).powf(2.0);
    let z = (z1-z2).powf(2.0);
    
    (x+y+z).sqrt()
}


