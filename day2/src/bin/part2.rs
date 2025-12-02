use day2::solution2;

use std::{fs::File, io::BufReader};



fn main() {

    println!("Calculating solution2...\n");
    let f = File::open("day2/input/input.txt").unwrap();

    let reader = BufReader::new(f);
    let solution = solution2::solve_solution(reader);
    println!("Solution2: {solution}");

    println!("\n...Done.")
}