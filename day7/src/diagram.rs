use std::collections::HashSet;


pub struct TachyonManifold{
    content: Vec<Vec<Cell>>
}

impl TachyonManifold  {
    pub fn from_vec(content: Vec<Vec<Cell>>)-> Self{
        TachyonManifold { content }
    }
    
    pub fn find_split_count(&self) -> usize {
        let mut beam_log = self.beam_start();
        let row_len = self.row_length();

        let mut split_counter = 0;

        // Iterate through Row numbers
        for r in 1..row_len{
            let mut new_beam_log = HashSet::new();
            for (_brow, bcol) in beam_log.iter(){

                let pos = (r, *bcol);
                let cell = self.get_cell(&pos);
                match cell {
                    Cell::Start => panic!("Did not expect Start Cell"),
                    Cell::EmptySpace => {
                        new_beam_log.insert(pos);
                    },
                    Cell::Splitter => {
                        let pos1 = (pos.0, pos.1-1);
                        let pos2 = (pos.0, pos.1+1);
                        new_beam_log.insert(pos1);
                        new_beam_log.insert(pos2);
                        split_counter +=1;
                    },
                }
                
            }
            beam_log = new_beam_log;
        }
        split_counter
    }

    fn get_cell(&self, pos: &(usize,usize)) -> &Cell{
        &self.content[pos.0][pos.1]
    }

    fn row_length(&self) -> usize{
        self.content.len()
    }
    
    fn beam_start(&self) -> HashSet<(usize,usize)> {
        let start_pos = self.content[0].iter().position(|cell| *cell == Cell::Start).expect("Could not find start");
        let beam_pos = (0,start_pos);
        let mut beam_log = HashSet::new();
        let _ = beam_log.insert(beam_pos);
        beam_log
    }
}

#[derive(PartialEq)]
pub enum Cell{
    Start,
    EmptySpace,
    Splitter
}

impl Cell {
    pub fn from_byte(b: &u8) -> Self{
        match b {
            b'S' => Cell::Start,
            b'.' => Cell::EmptySpace,
            b'^' => Cell::Splitter,
            x => panic!("Unkown Charactor \'{x}\'")
        }
    }
}