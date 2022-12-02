pub fn puzzle1(input: &str) -> String {
    total_score(input, knows_move_strategy).to_string()
}

fn total_score <'a, F> (input: &str, strategy_parser: F) -> u32 
    where F: FnMut((&str, &str)) -> u32
{
    input.lines()
        //.filter(|line| *line != "")
        .map(|line| line.split_once(" ").unwrap())
        .map(strategy_parser)
        .sum::<u32>()
}

fn knows_move_strategy(strategy: (&str, &str)) -> u32 {
    match strategy {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        _ => panic!("Unknown strategy: {:?}", strategy),
    }
}

pub fn puzzle2(input: &str) -> String {
    total_score(input, knows_result_strategy).to_string()
}

fn knows_result_strategy(strategy: (&str, &str)) -> u32 {
    match strategy {
        ("A", "X") => 3, // lose with scissor
        ("A", "Y") => 4, // draw with stone
        ("A", "Z") => 8, // win with paper
        ("B", "X") => 1, // lose with stone
        ("B", "Y") => 5, // draw with paper
        ("B", "Z") => 9, // win with scissor
        ("C", "X") => 2, // lose with paper
        ("C", "Y") => 6, // draw with scissor
        ("C", "Z") => 7, // win with rock
        _ => panic!("Unknown strategy: {:?}", strategy),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn puzzle1_works() {
        assert_eq!(puzzle1(INPUT), "15");
    }
    
    #[test]
    fn puzzle2_works() {
        assert_eq!(puzzle2(INPUT), "12");
    }
}
