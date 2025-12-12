use std::collections::HashMap;

use crate::{circuits::CircuitAssignment, distance::Distance, junction_box::{self, JunctionBox}};

pub struct ChristmasDecoration{
    junction_boxs: Vec<JunctionBox>
}

impl ChristmasDecoration {
    pub fn new(jbs: Vec<JunctionBox>) -> Self {
        Self { junction_boxs: jbs}
    }

    pub fn eval_connections(&self, loop_limit:Option<usize>) -> usize{
        
        // Assign JBs to individual circuits
        // let circuits_assigned_iter = self.junction_boxs.iter()
        //     .enumerate()
        //     .map(|(i,jb)| (jb,i));
        // let mut circuits_assigned: HashMap<&JunctionBox, usize> = HashMap::from_iter(circuits_assigned_iter);
        let mut circuits_assigned = CircuitAssignment::new(self.junction_boxs.iter());

        // Find shortes distances
        let mut distances: Vec<Distance> = Vec::new(); 
        let jb_length = self.junction_boxs.len();
        for item1_idx in 0..(jb_length-1){
            for item2_idx in (item1_idx+1)..jb_length{
                let jb1 = &self.junction_boxs[item1_idx];
                let jb2 = &self.junction_boxs[item2_idx];
                if jb1 == jb2{ panic!("should not have compared same JBs");}
                let distance = Distance::new(jb1, jb2);
                distances.push(distance);
            }
        }
        distances.sort_by(|d1,d2| d1.distance.partial_cmp(&d2.distance).unwrap());

        // Make connections
        let mut loop_counter = 0;
        for distance in distances.iter(){

            
            let jb1 = distance.jb1;
            let jb2 = distance.jb2;
            circuits_assigned.make_connections(jb1, jb2);
            
            if let Some(loop_limit) = loop_limit{
                loop_counter+=1;
                if !(loop_counter < loop_limit){
                    break;
                }
            }
            else{
                if circuits_assigned.number_of_circuits() == 1{
                    let pos1 = jb1.get_postion();
                    let pos2 = jb2.get_postion();
                    return pos1.0 * pos2.0;
                }
            }

        }
        // Multiply size of 3 largest groups
        let mut groups = circuits_assigned.get_groupings();
        groups.sort_by(|v1,v2| v1.len().cmp(&v2.len()));
        groups.reverse();
        let size1 = groups[0].len();
        let size2 = groups[1].len();
        let size3 = groups[2].len();
        return size1*size2*size3
    }
}

