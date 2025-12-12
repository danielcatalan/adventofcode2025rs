use std::{collections::HashMap, hash::Hash};
use crate::junction_box::JunctionBox;

#[derive(Eq,PartialEq,Hash,Clone, Copy)]
pub struct Circuit(usize);

pub struct CircuitAssignment<'a>{
    circuits_assigned: HashMap<&'a JunctionBox, Circuit>,
    bj_assigned: HashMap<Circuit, Vec<&'a JunctionBox>>
}

impl<'a> CircuitAssignment<'a>{
    pub fn new<It>(iter: It) -> Self
    where It: Iterator<Item = &'a JunctionBox>
    {
        let circuits_assigned_iter = iter
            .enumerate()
            .map(|(i,jb)| (jb,Circuit(i)));
        let circuits_assigned: HashMap<&JunctionBox, Circuit> = HashMap::from_iter(circuits_assigned_iter);
        
        let x = circuits_assigned.iter()
            .map(|(jb,circ)|(*circ,vec![*jb]) );
        let bj_assigned = HashMap::from_iter(x);
        Self { circuits_assigned, bj_assigned }
    }

    pub fn make_connections(&mut self, jb1: &JunctionBox, jb2: &JunctionBox) -> bool{
        let circuit1 = self.circuits_assigned.get(jb1).unwrap().clone();
        let circuit2 = self.circuits_assigned.get(jb2).unwrap().clone();
        
        if circuit1 == circuit2{
            return false; // same circuit, do nothing
        }

        // reassign all JBs of Circuit2 to Circuit1
        let mut vec_jb2 = self.bj_assigned.remove(&circuit2).unwrap();
        for jb in vec_jb2.iter(){
            let circuit_ref = self.circuits_assigned.get_mut(jb).unwrap();
            *circuit_ref = circuit1;
        }
        // move vec of circuit2 to circuit1
        let vec_jb1 = self.bj_assigned.get_mut(&circuit1).unwrap();
        vec_jb1.append(&mut vec_jb2);
/********************************************************* */
        // // assigned jb2's circuit to jb1's
        // let circuit2_ref = self.circuits_assigned.get_mut(jb2).unwrap();
        // *circuit2_ref = circuit1;

        // // move vec of circuit2 to circuit1
        // let mut vec_jb2 = self.bj_assigned.remove(&circuit2).unwrap();
        // let vec_jb1 = self.bj_assigned.get_mut(&circuit1).unwrap();
        // vec_jb1.append(&mut vec_jb2);
/********************************************************************** */
        return true;
    }
    
    pub fn get_groupings(&self) -> Vec<Vec<&'a JunctionBox>> {
        let x = self.bj_assigned.iter()
        .map(|(_,jb_vec)|  jb_vec.clone())
        .collect();
        x
    }
    
    pub fn number_of_circuits(&self) -> usize {
        let x = self.bj_assigned.len();
        x
    }
}

