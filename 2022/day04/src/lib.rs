pub fn puzzle1(input: &str) -> String {
    input
        .lines()
        .map(|pair_info| pair_info.split_once(',').unwrap())
        .map(|(left, right)| {
            (
                left.split_once('-').unwrap().into(),
                right.split_once('-').unwrap().into(),
            )
        })
        .filter(|(left_range, right_range): &(SectionRange, SectionRange)| {
            left_range.fully_contains(right_range) || right_range.fully_contains(left_range)
        })
        .count()
        .to_string()
}

struct SectionRange {
    min: usize,
    max: usize,
}

impl SectionRange {
    fn new(min: usize, max: usize) -> Self {
        SectionRange {
            min: (min),
            max: (max),
        }
    }
    fn fully_contains(&self, other: &SectionRange) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        (self.min <= other.min && self.max >= other.min)
            || (self.max >= other.max && self.min <= other.max)
            || other.fully_contains(self)
    }
}

impl From<(&str, &str)> for SectionRange {
    fn from(item: (&str, &str)) -> Self {
        SectionRange {
            min: (item.0.parse().unwrap()),
            max: (item.1.parse().unwrap()),
        }
    }
}

pub fn puzzle2(input: &str) -> String {
    input
        .lines()
        .map(|pair_info| pair_info.split_once(',').unwrap())
        .map(|(left, right)| {
            (
                left.split_once('-').unwrap().into(),
                right.split_once('-').unwrap().into(),
            )
        })
        .filter(|(left_range, right_range): &(SectionRange, SectionRange)| {
            left_range.overlaps(right_range)
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn puzzle1_works() {
        assert_eq!(puzzle1(INPUT), "2");
    }

    #[test]
    fn puzzle2_works() {
        assert_eq!(puzzle2(INPUT), "4");
    }

    #[test]
    fn puzzle2_t1() {
        let input = "51-69,50-62";
        assert_eq!(puzzle2(input), "1");
    }

    #[test]
    fn test_overlap() {
        let range1 = SectionRange::new(51, 69);
        let range2 = SectionRange::new(50, 62);
        assert!(range1.overlaps(&range2));
    }
}
