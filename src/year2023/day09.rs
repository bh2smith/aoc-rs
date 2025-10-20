use std::collections::HashSet;

use itertools::Itertools;
use maplit::hashset;

fn eval_diff(seq: &[isize]) -> Vec<isize> {
    seq.windows(2).map(|x| x[1] - x[0]).collect_vec()
}

fn first_or_last(seq: &[isize], first: bool) -> isize {
    *match first {
        true => seq.first().unwrap(),
        false => seq.last().unwrap(),
    }
}

fn solve(seq: &[isize], first: bool) -> Vec<isize> {
    let mut next_row = eval_diff(seq);
    let mut diffs = vec![first_or_last(seq, first)];
    while next_row.clone().iter().collect::<HashSet<_>>() != hashset! { &0 } {
        diffs.push(first_or_last(&next_row, first));
        next_row = eval_diff(&next_row);
    }
    diffs
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            row.split_whitespace()
                .map(|ch| ch.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn puzzle1(input: &str) -> isize {
    parse_input(input)
        .iter()
        .map(|seq| solve(seq, false).iter().sum::<isize>())
        .sum()
}

pub fn puzzle2(input: &str) -> isize {
    parse_input(input)
        .iter()
        .map(|seq| {
            let ends = solve(seq, true);
            ends.iter()
                .cloned()
                .enumerate()
                .map(|(i, v)| match i % 2 {
                    0 => v,
                    1 => -v,
                    _ => unreachable!(),
                })
                .sum::<isize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 114);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 2);
    }
}
