use day02;
use std::fs;

fn main() {
    let input = fs::read_to_string("2022/day02/input.txt").unwrap();
    println!("Puzzle 1 result: {}", day02::puzzle1(&input));
    println!("Puzzle 2 result: {}", day02::puzzle2(&input));
}
