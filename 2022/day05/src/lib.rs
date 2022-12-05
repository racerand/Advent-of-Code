#![feature(iter_collect_into)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, anychar, char, digit1, line_ending, newline, not_line_ending,
    },
    multi::{count, many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

pub fn puzzle1(input: &str) -> String {
    let (_, (mut stacks, moves)) = crates(input).unwrap();

    for Move { amount, from, to } in moves {
        let length = stacks[from as usize].len();
        stacks[from]
            .drain(length - amount as usize..)
            .rev()
            .collect::<Vec<char>>()
            .iter()
            .collect_into(&mut stacks[to]);
    }
    collect_result(stacks)
}

fn collect_result(mut stacks: Vec<Vec<char>>) -> String {
    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .fold("".to_string(), |result, top_crate| {
            result + &top_crate.to_string()
        })
}

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, c) = alt((
        delimited(char(' '), char(' '), char(' ')),
        delimited(char('['), anychar, char(']')),
    ))(input)?;
    let result = match c {
        ' ' => None,
        c => Some(c),
    };
    Ok((input, result))
}

fn line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(char(' '), parse_crate)(input)
}

fn stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, inverted_stacks) = separated_list1(line_ending, line)(input)?;
    let result = (0..inverted_stacks[0].len())
        .map(|stack_idx| {
            (0..inverted_stacks.len())
                .rev()
                .filter_map(|row_idx| inverted_stacks[row_idx][stack_idx])
                .collect()
        })
        .collect();
    Ok((input, result))
}

#[derive(Debug)]
struct Move {
    amount: u16,
    from: usize,
    to: usize,
}

impl Move {
    fn new(amount: u16, from: usize, to: usize) -> Self {
        Self { amount, from, to }
    }
}

fn move_crate(input: &str) -> IResult<&str, Move> {
    let (input, amount) = preceded(tag("move "), complete::u16)(input)?;
    let (input, from) = preceded(tag(" from "), complete::u16)(input)?;
    let (input, to) = preceded(tag(" to "), complete::u16)(input)?;
    Ok((input, Move::new(amount, from as usize - 1, to as usize - 1)))
}

fn crates(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Move>)> {
    let (input, stacks) = stacks(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = terminated(not_line_ending, line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, moves) = separated_list1(line_ending, move_crate)(input)?;
    Ok((input, (stacks, moves)))
}

pub fn puzzle2(input: &str) -> String {
    let (_, (mut stacks, moves)) = crates(input).unwrap();

    for Move { amount, from, to } in moves {
        let length = stacks[from as usize].len();
        stacks[from]
            .drain(length - amount as usize..)
            .collect::<Vec<char>>()
            .iter()
            .collect_into(&mut stacks[to]);
    }
    collect_result(stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn puzzle1_works() {
        assert_eq!(puzzle1(INPUT), "CMZ");
    }

    #[test]
    fn puzzle2_works() {
        assert_eq!(puzzle2(INPUT), "MCD");
    }
}
