use std::{
    collections::{BTreeMap, HashMap},
    u32,
};

use itertools::Itertools;

fn parse_input(input: &str) -> BTreeMap<usize, HashMap<char, u32>> {
    let mut map: BTreeMap<usize, HashMap<char, u32>> = BTreeMap::new();
    input.trim().lines().for_each(|x| {
        for (index, value) in x.chars().enumerate() {
            let value = map
                .entry(index)
                .or_insert(HashMap::new())
                .entry(value)
                .or_default();
            *value += 1;
        }
    });
    map
}

struct Extrema {
    min: char,
    max: char,
}

fn key_extrema(map: &HashMap<char, u32>) -> Extrema {
    let mut max_val = 0u32;
    let mut min_val = u32::MAX;
    let mut max = '-';
    let mut min = '-';

    for (k, val) in map {
        if *val > max_val {
            max_val = *val;
            max = *k;
        }
        if *val < min_val {
            min_val = *val;
            min = *k;
        }
    }
    Extrema { min, max }
}

pub fn puzzle1(input: &str) -> String {
    let counter = parse_input(input);
    // BTreeMap has sorted keys!
    counter.values().map(|m| key_extrema(m).max).join("")
}

pub fn puzzle2(input: &str) -> String {
    let counter = parse_input(input);
    // BTreeMap has sorted keys!
    counter.values().map(|m| key_extrema(m).min).join("")
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "
    eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), "easter");
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}
