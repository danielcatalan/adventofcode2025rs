use crate::machine::Machine;

pub struct Factory {
    machines: Vec<Machine>,
}

impl Factory {
    pub fn new(machines: Vec<Machine>) -> Self {
        Factory { machines }
    }

    pub fn get_fewest_button_presses(&self) -> usize {
        todo!()
    }
}
