use crate::junction_box::JunctionBox;

pub struct ChristmasDecoration{
    junction_boxs: Vec<JunctionBox>
}

impl ChristmasDecoration {
    pub fn new(jbs: Vec<JunctionBox>) -> Self {
        Self { junction_boxs: jbs}
    }
}