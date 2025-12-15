use day10::solution;

use std::{fs::File, io::BufReader};



fn main() {

    println!("Calculating solution2...\n");
    let f = File::open("day10/input/input.txt").unwrap();

    let reader = BufReader::new(f);
    let solution = solution::solve_solution2(reader);
    println!("Solution2: {solution}");

    println!("\n...Done.")
}