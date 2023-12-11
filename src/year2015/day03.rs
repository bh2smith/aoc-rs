use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Point(i32, i32);

impl Point {
    pub fn add(&self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Direction {
    pub fn from_char(ch: char) -> Self {
        match ch {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Left,
            '<' => Direction::Right,
            v => panic!("Unexpected direction character! {}", v),
        }
    }

    pub fn delta(&self) -> Point {
        let coord = match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        Point(coord.0, coord.1)
    }
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.chars().map(Direction::from_char).collect_vec()
}

pub fn puzzle1(input: &str) -> usize {
    let steps = parse_input(input);

    let mut visited = HashSet::new();
    let mut position = Point(0, 0);
    for step in steps {
        visited.insert(position);
        position = position.add(step.delta());
    }
    visited.len()
}

pub fn puzzle2(input: &str) -> usize {
    let steps = parse_input(input);
    let mut santa = HashMap::new();
    let mut robot = HashMap::new();

    let mut santa_pos = Point(0, 0);
    let mut robot_pos = Point(0, 0);

    santa.insert(santa_pos, 1);
    robot.insert(santa_pos, 1);

    for (index, step) in steps.iter().enumerate() {
        if index % 2 == 0 {
            robot_pos = robot_pos.add(step.delta());
            *robot.entry(robot_pos).or_insert(0) += 1;
        } else {
            santa_pos = santa_pos.add(step.delta());
            *santa.entry(santa_pos).or_insert(0) += 1;
        }
    }
    let santa_keys = santa.keys().cloned().collect::<HashSet<_>>();
    let robot_keys = robot.keys().cloned().collect::<HashSet<_>>();

    santa_keys
        .union(&robot_keys)
        .cloned()
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"^>v<";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 4);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 3);
    }
}
