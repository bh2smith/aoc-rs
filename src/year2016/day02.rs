fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split_terminator("\n").collect_vec()
}

use itertools::Itertools;
use nalgebra::Point2;

use crate::util::{Grid, Point};

#[derive(Debug)]
struct Keypad {
    at: Point<isize>,
    grid: Grid,
}

impl Keypad {
    fn new(start: Point<isize>, layout: &str) -> Self {
        let grid = Grid::from_layout(layout);
        Self { at: start, grid }
    }

    fn get_value(&self) -> char {
        self.get_index(self.at) as char
    }

    fn get_index(&self, Point { x, y }: Point<isize>) -> u8 {
        self.grid
            .get(Point2::new(x as usize, y as usize))
            .expect("we stay within bounds")
    }

    fn move_finger(&mut self, ch: char) {
        let next = self.at
            + match ch {
                'L' => Point::new(-1, 0),
                'R' => Point::new(1, 0),
                'U' => Point::new(0, -1),
                'D' => Point::new(0, 1),
                _ => panic!("Unexpected Direction"),
            };
        if self.out_of_bounds(next) {
            return;
        }
        self.at = next;
    }

    fn out_of_bounds(&self, pt: Point<isize>) -> bool {
        self.get_index(pt) == b'x'
    }
}

fn solve(input: &str, mut keypad: Keypad) -> String {
    let mut password = vec![];
    for mv in parse_input(input) {
        mv.chars().for_each(|ch| keypad.move_finger(ch));
        password.push(keypad.get_value());
    }

    password.into_iter().join("")
}

pub fn puzzle1(input: &str) -> String {
    let keypad = Keypad::new(Point::new(2, 2), "xxxxx\nx123x\nx456x\nx789x\nxxxxx");
    solve(input, keypad)
}

pub fn puzzle2(input: &str) -> String {
    let keypad = Keypad::new(
        Point::new(1, 3),
        "xxxxxxx\nxxx1xxx\nxx234xx\nx56789x\nxxABCxx\nxxxDxxx\nxxxxxxx",
    );
    solve(input, keypad)
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), "1985");
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), "5DB3");
    }
}
