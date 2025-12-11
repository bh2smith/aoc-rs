// use std::collections::HashSet;

use itertools::Itertools;

use bitgauss::BitMatrix;

use crate::util::transpose;

#[derive(Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<bool>>,
    // jolts: HashSet<usize>,
}

impl Machine {
    fn solve(&self) {
        let a = BitMatrix::from_bool_vec(&transpose(self.buttons.clone()));
        let b = BitMatrix::from_bool_vec(&transpose(vec![self.target.clone()]));

        // Build augmented matrix [A | b]
        let aug = a.hstack(&b);

        let mut aug_red = aug.clone();
        aug_red.gauss(true);

        println!("Row echelon form: \n{}", aug_red);
        // let mut solution = vec![false; self.buttons.len()];
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
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
                buttons.push(button);
            }

            Machine {
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

pub fn puzzle1(input: &str) -> u64 {
    let machines = parse_input(input);
    for m in machines.iter() {
        m.solve();
    }
    1
}

pub fn puzzle2(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}
