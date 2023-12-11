use std::collections::HashMap;

use itertools::Itertools;
use maplit::hashmap;

pub fn solve(input: &str, part_2: bool) -> u32 {
    let target = hashmap! {
      "children" => 3,
      "cats" => 7,
      "samoyeds" => 2,
      "pomeranians" => 3,
      "akitas" => 0,
      "vizslas" => 0,
      "goldfish" => 5,
      "trees" => 3,
      "cars" => 2,
      "perfumes" => 1,
    };
    for row in input.trim().split('\n') {
        let items = row.split_whitespace().collect_vec();
        let mut map = HashMap::new();
        for i in 0..(items.len() - 2) / 2 {
            let pair = items[2 * (i + 1)..2 * (i + 2)].to_vec();
            map.insert(
                pair[0].replace(':', ""),
                pair[1].replace(',', "").parse::<u32>().unwrap(),
            );
        }
        if target.keys().all(|key| {
            if let Some(val) = map.get(*key) {
                if part_2 {
                    return match *key {
                        "cats" | "trees" => &target[key] < val,
                        "pomeranians" | "goldfish" => &target[key] > val,
                        _ => &target[key] == val,
                    };
                }

                return &target[key] == val;
            }
            true
        }) {
            return items[1].replace(':', "").parse().unwrap();
        }
    }
    panic!("No Sue found")
}

pub fn puzzle1(input: &str) -> u32 {
    solve(input, false)
}

pub fn puzzle2(input: &str) -> u32 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "Sue 1: cars: 9, akitas: 3, goldfish: 0
    Sue 2: akitas: 9, children: 3, samoyeds: 9
    Sue 3: trees: 6, cars: 6, children: 4
    Sue 4: trees: 4, vizslas: 4, goldfish: 9
    Sue 5: akitas: 9, vizslas: 7, cars: 5
    Sue 6: vizslas: 6, goldfish: 6, akitas: 3";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}
