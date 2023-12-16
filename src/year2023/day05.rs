use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct RangeFunction {
    from_start: u64,
    to_start: u64,
    length: u64,
}

impl RangeFunction {
    fn contains(&self, val: &u64) -> bool {
        val >= &self.from_start && val < &(self.from_start + self.length)
    }

    fn eval(&self, val: &u64) -> u64 {
        assert!(self.contains(val), "Not contained!");
        (val - self.from_start) + self.to_start
    }
}

#[derive(Debug, Clone)]
struct CompressedRange {
    data: HashSet<RangeFunction>,
}

impl CompressedRange {
    fn new(input: Vec<&str>) -> Self {
        let mut data = HashSet::new();
        for row in input.into_iter() {
            let numbers = row
                .split_whitespace()
                .map(|x| x.parse::<u64>().expect("number"))
                .collect_vec();
            data.insert(RangeFunction {
                from_start: numbers[1],
                to_start: numbers[0],
                length: numbers[2],
            });
        }
        Self { data }
    }

    fn eval(&self, key: u64) -> u64 {
        for range in self.data.clone() {
            if range.contains(&key) {
                return range.eval(&key);
            }
        }
        key
    }
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<CompressedRange>) {
    let lines = input.trim().split('\n').collect_vec();
    let seeds = lines[0].split(' ').collect_vec()[1..]
        .iter()
        .map(|s| s.parse::<u64>().expect("number"))
        .collect_vec();
    let map_defs: Vec<Vec<&str>> = lines[2..]
        .split(|&item| item.is_empty())
        .filter(|&chunk| !chunk.is_empty())
        .map(|chunk| chunk.to_vec())
        .collect();
    let mut ranges = vec![];
    for map_def in map_defs {
        // No need to parse source-dest because they come in order.
        let next = CompressedRange::new(map_def[1..].to_vec());
        ranges.push(next);
    }
    (seeds, ranges)
}

fn composed_eval(ranges: Vec<CompressedRange>, key: u64) -> u64 {
    let mut val = key;
    for range in ranges {
        val = range.eval(val);
    }
    val
}

/// (Recursive) Binary search to break up ranges into piecewise linear functions.
fn build_linear_ranges(left: u64, right: u64, ranges: Vec<CompressedRange>) -> Vec<(u64, u64)> {
    let start = composed_eval(ranges.clone(), left);
    let end = composed_eval(ranges.clone(), right);
    if start <= end && end - start == right - left {
        return vec![(start, end)];
    }
    let mid = (right - left) / 2;
    let mut res_left = build_linear_ranges(left, left + mid, ranges.clone());
    let res_right = build_linear_ranges(left + mid + 1, right, ranges.clone());
    res_left.extend(res_right);
    res_left
}

pub fn puzzle1(input: &str) -> u64 {
    let (seeds, ranges) = parse_input(input);
    let mut results = vec![];
    for seed in seeds {
        let location = composed_eval(ranges.clone(), seed);
        // println!("Seed {seed} Location {location}");
        results.push(location);
    }
    results.into_iter().min().expect("exists")
}

pub fn puzzle2(input: &str) -> u64 {
    let (seeds, ranges) = parse_input(input);
    let mut min_results = vec![];
    for chunk in seeds.chunks(2) {
        match chunk {
            &[seed, length] => {
                let left = seed;
                let right = seed + length - 1;

                let results = build_linear_ranges(left, right, ranges.clone());
                min_results.push(results.iter().map(|tuple| tuple.0).min().expect("exists"));
            }
            _ => panic!("Invalid pair"),
        }
    }
    min_results.into_iter().min().expect("exists")
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 35);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 46);
    }
}
