use std::str::FromStr;

use itertools::Itertools;

use crate::util::Grid;

struct Region {
    length: usize,
    width: usize,
    requirements: Vec<usize>,
}

impl Region {
    fn area(&self) -> usize {
        self.length * self.width
    }

    fn area_check(&self, presents: &[Present]) -> bool {
        let required_size: usize = self
            .requirements
            .iter()
            .enumerate()
            .map(|(i, v)| presents[i].area() * v)
            .sum();

        required_size <= self.area()
    }
}

impl std::str::FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dim, reqs) = s.split_once(": ").ok_or("Missing ': ' separator")?;
        let (l, w) = dim.split_once("x").ok_or("Missing 'x' separator")?;
        let requirements = reqs
            .split_whitespace()
            .map(|n| {
                n.parse::<usize>()
                    .map_err(|_| format!("Invalid number {n}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Region {
            length: l.parse().map_err(|_| format!("Invalid length {l}"))?,
            width: w.parse().map_err(|_| format!("Invalid width {w}"))?,
            requirements,
        })
    }
}
struct Present {
    layout: Grid,
}

impl Present {
    pub fn area(&self) -> usize {
        self.layout
            .as_bytes()
            .iter()
            .filter(|&&c| c == b'#')
            .count()
    }
}

impl std::str::FromStr for Present {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Skip the first line:
        lines.next().ok_or("Empty present string")?;

        // Collect the remaining lines into the grid layout
        let layout_str = lines.collect::<Vec<_>>().join("\n");

        if layout_str.is_empty() {
            return Err("Missing grid layout".into());
        }

        Ok(Present {
            layout: Grid::from_layout(&layout_str),
        })
    }
}

struct Problem {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

impl Problem {}

fn parse_input(input: &str) -> Problem {
    let mut items = input.trim().split("\n\n").collect_vec();
    let region_data = items.pop().unwrap();
    let regions = region_data
        .lines()
        .map(|s| Region::from_str(s).unwrap())
        .collect();
    let presents = items
        .iter()
        .map(|s| Present::from_str(s).unwrap())
        .collect();

    Problem { presents, regions }
}

pub fn puzzle1(input: &str) -> usize {
    let Problem { presents, regions } = parse_input(input);
    // Filter out all that won't fit (by area check)
    let might_fit: Vec<Region> = regions
        .into_iter()
        .filter(|r| r.area_check(&presents))
        .collect();

    might_fit.len()
}

pub fn puzzle2(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 2);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}
