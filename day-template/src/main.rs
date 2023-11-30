use day_template::part1;
use day_template::part2;

fn main() {
    let input = include_str!("../input.txt");
    println!("Result of part 2 {}", part1::process(&input));
    println!("Result of part 2 {}", part2::process(&input));
}
