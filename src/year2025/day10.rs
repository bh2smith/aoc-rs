// use crate::util::transpose;
use itertools::Itertools;

use std::{iter::Sum, ops::Add};

#[derive(Clone, Debug)]
pub struct Button(Vec<bool>);

impl Add for Button {
    type Output = Button;

    fn add(self, rhs: Button) -> Button {
        assert_eq!(self.0.len(), rhs.0.len(), "Vectors must have same length");
        Button(self.0.into_iter().zip(rhs.0).map(|(a, b)| a ^ b).collect())
    }
}

impl Sum<Button> for Button {
    fn sum<I>(iter: I) -> Button
    where
        I: Iterator<Item = Button>,
    {
        let mut iter = iter.into_iter();
        if let Some(first) = iter.next() {
            iter.fold(first, |acc, b| acc + b)
        } else {
            // identity element for XOR is all-false; but if we don't know length,
            // we can return empty and let the caller decide if empty is valid.
            Button(Vec::new())
        }
    }
}

#[derive(Debug)]
struct MachineA {
    target: Vec<bool>,
    buttons: Vec<Button>,
    // jolts: Vec<i64>,
}

#[derive(Debug)]
struct MachineB {
    buttons: Vec<Vec<i64>>,
    jolts: Vec<i64>,
}

fn parse_1(input: &str) -> Vec<MachineA> {
    input
        .trim()
        .lines()
        .map(|line| {
            let items = line.split(" ").collect_vec();
            let mut target = vec![];

            for ch in items[0].chars() {
                match ch {
                    '.' => target.push(false),
                    '#' => target.push(true),
                    _ => (),
                }
            }
            let mut buttons = vec![];
            for &item in &items[1..items.len() - 1] {
                let mut button = vec![false; target.len()];
                for x in item.trim_matches(|c| c == '(' || c == ')').split(",") {
                    let i = x.parse::<usize>().unwrap();
                    button[i] = true;
                }
                buttons.push(Button(button));
            }

            MachineA {
                target,
                buttons,
                // jolts: items[items.len() - 1]
                //     .trim_matches(|c| c == '{' || c == '}')
                //     .split(",")
                //     .map(|x| x.parse().unwrap())
                //     .collect(),
            }
        })
        .collect()
}

fn parse_2(input: &str) -> Vec<MachineB> {
    input
        .trim()
        .lines()
        .map(|line| {
            let items = line.split(" ").collect_vec();
            let mut target = vec![];

            for ch in items[0].chars() {
                match ch {
                    '.' => target.push(false),
                    '#' => target.push(true),
                    _ => (),
                }
            }
            let mut buttons = vec![];
            for &item in &items[1..items.len() - 1] {
                let mut button = vec![0; target.len()];
                for x in item.trim_matches(|c| c == '(' || c == ')').split(",") {
                    let i = x.parse::<usize>().unwrap();
                    button[i] = 1;
                }
                buttons.push(button);
            }

            MachineB {
                buttons,
                jolts: items[items.len() - 1]
                    .trim_matches(|c| c == '{' || c == '}')
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

pub fn puzzle1(input: &str) -> usize {
    parse_1(input)
        .iter()
        .map(|m| {
            // It will take at most the number of lights switches to solve
            // This is an overdetermined, commutative linear system of idempotents.
            for i in 0..m.target.len() {
                for subset in m.buttons.clone().into_iter().combinations(i) {
                    if subset.into_iter().sum::<Button>().0 == m.target {
                        return i;
                    }
                }
            }
            panic!("failed biatch")
        })
        .sum()
}

pub fn puzzle2(input: &str) -> u64 {
    for m in parse_2(input).iter() {
        // m.inspect();
        // let x = m.solve();
        println!("Buttons {:?}", m.buttons);
        println!("Jolts {:?}", m.jolts);
    }
    0
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 7);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 33);
    }
}
