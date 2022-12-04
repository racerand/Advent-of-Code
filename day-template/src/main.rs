use day_template::{puzzle1, puzzle2};
use std::fs;

fn main() {
    let input = fs::read_to_string("YYYY/dayDD/input.txt").unwrap();
    println!("Puzzle 1 result: {}", puzzle1(&input));
    println!("Puzzle 2 result: {}", puzzle2(&input));
}
