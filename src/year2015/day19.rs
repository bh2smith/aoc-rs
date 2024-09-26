use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse_input(input: &str) -> (HashMap<&str, HashSet<&str>>, &str) {
    let mut lines = input.trim().split('\n').collect_vec();
    lines.reverse();
    let mut map = HashMap::new();
    while let Some(line) = lines.pop() {
        if line.is_empty() {
            break;
        }
        let items = line.split_whitespace().collect_vec();
        map.entry(items[0])
            .or_insert(HashSet::new())
            .insert(items[2]);
    }

    (map, lines.pop().unwrap())
}

fn find_all_positions(word: &str, subword: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;

    while let Some(index) = word[start..].find(subword) {
        let absolute_index = start + index;
        positions.push(absolute_index);
        start = absolute_index + 1;
    }

    positions
}

pub fn puzzle1(input: &str) -> usize {
    let (map, word) = parse_input(input);

    println!("{word} - {map:?}");
    let mut combos = HashSet::new();
    for (k, s) in map.into_iter() {
        println!("On item {k} {s:?}");
        for i in find_all_positions(word, k) {
            println!("replace position {i}");
            for choice in &s {
                let new_word = format!("{}{}{}", &word[..i], choice, &word[i + k.len()..]);
                println!("insert word {new_word}");
                combos.insert(new_word);
            }
        }
    }
    println!("Combos {:?}", combos);
    combos.len()
}

pub fn puzzle2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    const EASY_SAMPLE: &str = "H => HO
H => OH
O => HH

HOH";
    const SAMPLE_INPUT: &str = "H => HO
H => OH
O => HH

HOHOHO";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(EASY_SAMPLE), 4);
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 7);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }

    #[test]
    fn find_in_word() {
        let word = "abracadabra";
        let subword = "ra";
        let positions = super::find_all_positions(word, subword);
        assert_eq!(positions, vec![2, 9]);
    }
}
