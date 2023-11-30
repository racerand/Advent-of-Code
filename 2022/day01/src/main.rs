use day01::part1;
use day01::part2;

fn main() {
    let input = include_str!("../input.txt");
    println!("Result of part 1 {}", part1::process(&input));
    println!("Result of part 2 {}", part2::process(&input));
}
