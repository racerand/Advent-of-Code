use day01::process_part2;
use std::fs;

fn main() {
    let input = fs::read_to_string("2022/day01/input.txt").unwrap();
    println!("{}", process_part2(&input));
}
