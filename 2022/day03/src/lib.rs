#![feature(iter_array_chunks)]

pub fn puzzle1(input: &str) -> String {
    input
        .lines()
        .map(|backpacks| backpacks.split_at(backpacks.len() / 2))
        .map(|(pack1, pack2)| {
            pack1
                .chars()
                .find_map(|item| {
                    if pack2.contains(item) {
                        Some(item)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .map(char_to_num)
        .sum::<u16>()
        .to_string()
}

pub fn puzzle2(input: &str) -> String {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[bag1, bag2, bag3]| {
            bag1.chars()
                .find_map(|item| {
                    if bag2.contains(item) && bag3.contains(item) {
                        Some(item)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .map(char_to_num)
        .sum::<u16>()
        .to_string()
}

fn char_to_num(c: char) -> u16 {
    match c.is_uppercase() {
        true => c as u16 - 38,
        false => c as u16 - 96,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn puzzle1_works() {
        assert_eq!(puzzle1(INPUT), "157");
    }

    #[test]
    fn puzzle2_works() {
        assert_eq!(puzzle2(INPUT), "70");
    }

    #[test]
    fn a_is_1() {
        assert_eq!(1, char_to_num('a'));
    }

    #[test]
    fn z_is_26() {
        assert_eq!(26, char_to_num('z'));
    }

    #[test]
    fn a_upper_is_27() {
        assert_eq!(27, char_to_num('A'));
    }

    #[test]
    fn z_upper_is_52() {
        assert_eq!(52, char_to_num('Z'));
    }
}
