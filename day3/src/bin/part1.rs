use day3::solution;

use std::{fs::File, io::BufReader};

fn main() {
    println!("Calculating solution1...\n");
    let f = File::open("day3/input/input.txt").unwrap();

    let reader = BufReader::new(f);
    let solution = solution::solve_solution1(reader);
    println!("Solution1: {solution}");

    println!("\n...Done.")
}
