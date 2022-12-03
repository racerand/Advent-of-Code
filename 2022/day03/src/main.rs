use day03;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Puzzle 1 result: {}", day03::puzzle1(&input));
    println!("Puzzle 2 result: {}", day03::puzzle2(&input));
}
