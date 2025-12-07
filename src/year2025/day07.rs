use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Config {
    splitters: HashMap<usize, HashSet<usize>>,
    beams: HashMap<usize, usize>, // col -> multiplicity
    row: usize,
    height: usize,
}

impl Config {
    fn run_one(&mut self) -> Option<usize> {
        // Returns the number of splits in a run.
        // None if we've reached the end
        if self.row == self.height {
            return None;
        }
        self.row += 1;

        let mut new_beams: HashMap<usize, usize> = HashMap::new();
        let mut split_count = 0usize;

        // All splitter columns on this row (if any)
        let splitter_cols = self.splitters.get(&self.row);

        for (&col, &count) in self.beams.iter() {
            if splitter_cols.is_some_and(|set| set.contains(&col)) {
                // A split occurs at this column.
                // If you want to count unique split positions, do += 1.
                // If you want to count beams, do += count.
                split_count += 1;

                *new_beams.entry(col - 1).or_default() += count;
                *new_beams.entry(col + 1).or_default() += count;
            } else {
                // Beam continues straight down
                *new_beams.entry(col).or_default() += count;
            }
        }

        self.beams = new_beams;

        Some(split_count)
    }
}
fn parse_input(input: &str) -> Config {
    let mut splitters: HashMap<_, HashSet<_>> = HashMap::new();
    let mut beams = HashMap::new();
    for (row, line) in input.trim().lines().enumerate() {
        for (col, val) in line.chars().enumerate() {
            match val {
                'S' => {
                    beams.insert(col, 1);
                }
                '^' => {
                    splitters
                        .entry(row)
                        .or_insert_with(HashSet::new)
                        .insert(col);
                }
                _ => (),
            };
        }
    }
    let height = *splitters.keys().max().expect("board is populated");
    Config {
        splitters,
        beams,
        row: 0,
        height,
    }
}

pub fn puzzle1(input: &str) -> usize {
    let mut config = parse_input(input);
    let mut res = 0;
    while let Some(amt) = config.run_one() {
        res += amt
    }
    res
}

pub fn puzzle2(input: &str) -> usize {
    let mut config = parse_input(input);
    loop {
        if config.run_one().is_none() {
            break;
        }
    }
    config.beams.into_values().sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 21);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 40);
    }
}
