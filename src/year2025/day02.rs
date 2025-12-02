use std::collections::HashSet;

use num::Integer;

fn chunk_set(s: &str, size: usize) -> HashSet<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(size)
        .map(|c| c.iter().collect())
        .collect()
}

fn is_invalid(n: u64) -> bool {
    let word = n.to_string();
    for i in 1..word.len() / 2 + 1 {
        if chunk_set(&word, i).len() == 1 {
            return true;
        }
    }
    false
}

fn is_invalid_a(n: u64) -> bool {
    let word = n.to_string();
    word.len().is_even() && chunk_set(&word, word.len() / 2).len() == 1
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .trim_start_matches('0')
        .split(',')
        .map(|c| {
            let (x, y) = c.split_once('-').expect("problem statement");
            (
                x.trim().parse().expect("x not number"),
                y.trim().parse().expect("y not number"),
            )
        })
        .collect()
}

pub fn puzzle1(input: &str) -> u64 {
    let tups = parse_input(input);
    let mut total = 0;
    for tup in tups {
        for num in tup.0..=tup.1 {
            if is_invalid_a(num) {
                total += num;
            }
        }
    }
    total
}

pub fn puzzle2(input: &str) -> u64 {
    let tups = parse_input(input);
    tups.iter()
        .map(|tup| {
            (tup.0..=tup.1)
                .map(|num| match is_invalid(num) {
                    true => num,
                    false => 0u64,
                })
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 4174379265);
    }

    #[test]
    fn test_is_valid() {
        // Base case < 100
        assert!(!super::is_invalid(15));
        assert!(super::is_invalid(55));
        assert!(super::is_invalid(55));
        assert!(super::is_invalid(66));

        // above 100
        assert!(super::is_invalid(1010));
    }
}
