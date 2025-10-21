use std::collections::HashSet;

use itertools::Itertools;

use crate::util::{Direction, Point};

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: u32
}

struct Position {
    at: Point,
    facing: Direction,
    visited: HashSet<Point>,
    found: bool,
    teleport: bool,
}

impl Direction {
    fn left_of(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right_of(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Position {
    fn new(teleport: bool) -> Self {
        let at = Point::new(0, 0);
        let visited = HashSet::from([at]);
        Self {
            at,
            facing: Direction::Up,
            visited,
            found: false,
            teleport,
        }
    }
    fn turn(&mut self, d: Direction) {
        match d {
            Direction::Left => self.facing = self.facing.left_of(),
            Direction::Right => self.facing = self.facing.right_of(),
            _ => panic!("Unexpected Turn")
        }
    }

    fn teleport(&mut self, amount: u32) {
        self.at = self.at + self.facing.as_point(amount as i64) 
    }

    fn step(&mut self) {
        self.at = self.at + self.facing.as_point(1) 
    }

    pub fn apply_move(&mut self, mv: Move) {
        if self.found {
            return;
        }
        self.turn(mv.direction);
        if self.teleport {
            self.teleport(mv.amount);
        } else {
            for _ in 0..mv.amount {
                self.step();
                if self.visited.contains(&self.at) {
                    self.found = true;
                    break;
                }
                self.visited.insert(self.at);
            }
        }

        
    }

    fn distance_home(&self) -> i64 {
        self.at.x.abs() + self.at.y.abs()
    }
}

impl Move {
    fn from_str(s: &str) -> Self {
        let mut chars = s[..1].chars();
        Self {
            direction: Direction::from_char(chars.next().unwrap()),
            amount: s[1..].parse().expect("non-digit")
        }
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input.trim().split(", ").map(Move::from_str).collect_vec()
}

pub fn puzzle1(input: &str) -> i64 {
    println!("Input {}", input);
    let mut position = Position::new(true);
    for mv in parse_input(input) {
        position.apply_move(mv);
    }
    position.distance_home()
}

pub fn puzzle2(input: &str) -> i64 {
    let mut position = Position::new(false);
    for mv in parse_input(input) {
        position.apply_move(mv);
    }
    position.distance_home()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "R5, L5, R5, R3";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 12);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2("R8, R4, R4, R8"), 4);
    }
}
