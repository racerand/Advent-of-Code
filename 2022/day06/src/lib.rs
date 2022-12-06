use std::collections::VecDeque;

pub fn puzzle1(input: &str) -> String {
    find_unique_chars_of_length(input, 4).unwrap().to_string()
}

fn find_unique_chars_of_length(input: &str, length: u16) -> Option<usize> {
    let mut marker_so_far: VecDeque<char> = VecDeque::new();
    for (idx, c) in input.chars().enumerate() {
        while marker_so_far.contains(&c) {
            marker_so_far.pop_front().unwrap();
        }
        marker_so_far.push_back(c);
        if marker_so_far.len() == length as usize {
            return Some(idx + 1);
        }
    };
    None
}

pub fn puzzle2(input: &str) -> String {
    find_unique_chars_of_length(input, 14).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_INPUT1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const P1_INPUT2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const P1_INPUT3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const P1_INPUT4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn puzzle1_input1() {
        assert_eq!(puzzle1(P1_INPUT1), "5");
    }
    
    #[test]
    fn puzzle1_input2() {
        assert_eq!(puzzle1(P1_INPUT2), "6");
    }
    
    #[test]
    fn puzzle1_input3() {
        assert_eq!(puzzle1(P1_INPUT3), "10");
    }
    
    #[test]
    fn puzzle1_input4() {
        assert_eq!(puzzle1(P1_INPUT4), "11");
    }

    const P2_INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const P2_INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const P2_INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const P2_INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const P2_INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    
    #[test]
    fn puzzle2_input1() {
        assert_eq!(puzzle2(P2_INPUT1), "19");
    }
    
    #[test]
    fn puzzle2_input2() {
        assert_eq!(puzzle2(P2_INPUT2), "23");
    }
    
    #[test]
    fn puzzle2_input3() {
        assert_eq!(puzzle2(P2_INPUT3), "23");
    }
    
    #[test]
    fn puzzle2_input4() {
        assert_eq!(puzzle2(P2_INPUT4), "29");
    }
    
    #[test]
    fn puzzle2_input5() {
        assert_eq!(puzzle2(P2_INPUT5), "26");
    }
}
