use day04::{puzzle1, puzzle2};
use std::fs;

fn main() {
    let input = fs::read_to_string("2022/day04/input.txt").unwrap();
    println!("Puzzle 1 result: {}", puzzle1(&input));
    println!("Puzzle 2 result: {}", puzzle2(&input));
}
