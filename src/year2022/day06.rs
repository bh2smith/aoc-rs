use std::collections::{HashSet};
use std::iter::FromIterator;


fn gen_solve(input: &str, n: usize) -> i64 {
    let input_chars: Vec<_> = input.chars().collect();
    for i in 0..input.len() {
        let curr: HashSet<_> = HashSet::from_iter(input_chars[i..i+n].iter());
        if curr.len() == n {
            return (i + n) as i64;
        }
    }
    panic!("Should not ever get here!")
}
pub fn puzzle1(input: &str) -> i64 {
    gen_solve(input, 4)
}

pub fn puzzle2(input: &str) -> i64 {
    gen_solve(input, 14)
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUTS_1: [&str; 4] = [
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    const SAMPLE_INPUTS_2: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUTS_1[0]), 5);
        assert_eq!(super::puzzle1(SAMPLE_INPUTS_1[1]), 6);
        assert_eq!(super::puzzle1(SAMPLE_INPUTS_1[2]), 10);
        assert_eq!(super::puzzle1(SAMPLE_INPUTS_1[3]), 11);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUTS_2[0]), 19);
        assert_eq!(super::puzzle2(SAMPLE_INPUTS_2[1]), 23);
        assert_eq!(super::puzzle2(SAMPLE_INPUTS_2[2]), 23);
        assert_eq!(super::puzzle2(SAMPLE_INPUTS_2[3]), 29);
        assert_eq!(super::puzzle2(SAMPLE_INPUTS_2[4]), 26);
    }
}
