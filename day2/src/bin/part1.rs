use day2::solution1;

use std::{fs::File, io::BufReader};



fn main() {

    println!("Calculating solution1...\n");
    let f = File::open("day2/input/input.txt").unwrap();

    let reader = BufReader::new(f);
    let solution = solution1::solve_solution(reader);
    println!("Solution1: {solution}");

    println!("\n...Done.")
}

