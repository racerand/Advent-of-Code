pub fn process_part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut load_sums: Vec<u32> = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    // Sort largest to smallest
    load_sums.sort_by(|a, b| b.cmp(a));
    let sum: u32 = load_sums.iter().take(3).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_1_works() {
        assert_eq!("24000", process_part1(INPUT));
    }

    #[test]
    fn part_2_works() {
        assert_eq!("45000", process_part2(INPUT));
    }
}
