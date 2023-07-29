use std::collections::HashSet;
use crate::util;
use itertools::Itertools;

const CHAR_VALUES_BY_INDEX: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(ch: char) -> i64 {
    CHAR_VALUES_BY_INDEX.find(ch).expect("We were told all rows would have overlap") as i64
}

pub fn puzzle1(input: &str) -> i64 {
    util::parse::<String>(input).map(|item| {
        // lengths are always even.
        let (left, right) = item.split_at(item.len()/2);
        let left_set: HashSet<_> = left.chars().collect();
        let right_set: HashSet<_> = right.chars().collect();
        let overlap: Vec<_> = left_set.intersection(&right_set).collect();
        assert_eq!(overlap.len(), 1);
        priority(*overlap[0])
        // CHAR_VALUES_BY_INDEX.find(*overlap[0]).expect("We were told all rows would have overlap")
    }).sum()
}

pub fn puzzle2(input: &str) -> i64 {
    let mut result = 0i64;
    for group in &util::parse::<String>(input).chunks(3) {
        let items: Vec<_> = group.collect();

        let a: HashSet<_> = items[0].chars().collect();
        let b: HashSet<_> = items[1].chars().collect();
        let c: HashSet<_> = items[2].chars().collect();
        let ab: HashSet<_> = a.intersection(&b).collect();
        let ac: HashSet<_> = a.intersection(&c).collect();
        let overlap: Vec<_> = ab.intersection(&ac).collect();
        assert_eq!(overlap.len(), 1);
        result += priority(**overlap[0])
        // let overlap: HashSet<_> = items[0].chars().collect();
        // for item in items[1..].into_iter() {
        //     let item_set: HashSet<_> = item.as_str().chars().collect();
        //     let x: HashSet<_> = item_set.intersection(&overlap).collect();
        // }
    }
    result
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";



    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 157);
        // assert_eq!(super::puzzle1(REAL_INPUT), 7795);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 70);
        // assert_eq!(super::puzzle2(REAL_INPUT), 2703);
    }
}
