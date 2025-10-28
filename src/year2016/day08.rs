use itertools::Itertools;

use crate::util::Grid;

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

fn apply_instruction(grid: &mut Grid, instruction: Instruction) {
    match instruction {
        Instruction::Rect(a, b) => {
            for i in 0..a {
                for j in 0..b {
                    grid[[i, j]] = b'#';
                }
            }
        }
        Instruction::RotateRow(a, b) => {
            grid.rotate_row(a, b as isize);
        }
        Instruction::RotateCol(a, b) => {
            grid.rotate_column(a, b as isize);
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let items = value.split_whitespace().collect_vec();
        match items.len() {
            2 => {
                let nums = items[1]
                    .split("x")
                    .map(|x| x.trim().parse().unwrap())
                    .collect_vec();
                Instruction::Rect(nums[0], nums[1])
            }
            5 => {
                let b = items[4].parse().unwrap();
                let a = items[2][2..].parse().unwrap();
                match items[1] {
                    "column" => Instruction::RotateCol(a, b),
                    "row" => Instruction::RotateRow(a, b),
                    _ => panic!("invalid rotation instruction"),
                }
            }
            _ => panic!("Invalid instruction length {:?}", items),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

pub fn puzzle1(input: &str) -> usize {
    let mut grid = Grid::new(50, 6);
    for instruction in parse_input(input) {
        apply_instruction(&mut grid, instruction);
    }

    grid.as_bytes().iter().filter(|x| **x == b'#').count()
}

pub fn puzzle2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::util::Grid;

    const SAMPLE_INPUT: &str = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 6);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }

    #[test]
    fn test_grid_rotation() {
        let mut x = Grid::from_layout(".....x......");
        x.rotate_row(0, 4);
        assert_eq!(x.as_bytes(), Grid::from_layout(".........x..").as_bytes())
    }
}
