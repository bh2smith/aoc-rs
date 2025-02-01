use crate::util::Direction;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryInto;
use std::str::FromStr;

type Pair = (i32, i32);
#[derive(Debug, Clone, PartialEq)]
struct Head {
    position: Pair,
}

#[derive(Debug, Clone, PartialEq)]
struct Tail {
    position: Pair,
    visited: HashSet<Pair>,
}

#[derive(Debug, Clone, PartialEq)]
struct Configuration {
    head: Head,
    tail: Tail,
}

impl Configuration {
    fn new() -> Self {
        let position = (0, 0);
        Self {
            head: Head { position },
            tail: Tail {
                position,
                visited: HashSet::from([position]),
            },
        }
    }

    fn adjust_tail(&mut self) -> Option<Pair> {
        let x = self.head.position.0 - self.tail.position.0;
        let y = self.head.position.1 - self.tail.position.1;
        if x.abs().max(y.abs()) > 1 {
            let pair = match (x, y) {
                (2, t) => match t.cmp(&0) {
                    Ordering::Less => (1, -1),
                    Ordering::Equal => (1, 0),
                    Ordering::Greater => (1, 1),
                },
                (t, 2) => match t.cmp(&0) {
                    Ordering::Less => (-1, 1),
                    Ordering::Equal => (0, 1),
                    Ordering::Greater => (1, 1),
                },
                (-2, t) => match t.cmp(&0) {
                    Ordering::Less => (-1, -1),
                    Ordering::Equal => (-1, 0),
                    Ordering::Greater => (-1, 1),
                },
                (t, -2) => match t.cmp(&0) {
                    Ordering::Less => (-1, -1),
                    Ordering::Equal => (0, -1),
                    Ordering::Greater => (1, -1),
                },
                (_, _) => panic!("This can't happen because of inequality!"),
            };
            self.move_tail_by(&pair);
            return Some(pair);
        }
        None
    }

    fn move_tail_by(&mut self, pair: &Pair) {
        self.tail.position.0 += pair.0;
        self.tail.position.1 += pair.1;
        self.tail.visited.insert(self.tail.position);
    }

    fn move_head_by(&mut self, pair: Pair) -> Option<Pair> {
        self.head.position.0 += pair.0;
        self.head.position.1 += pair.1;
        self.adjust_tail()
    }

    fn move_head(&mut self, mv: &Move) -> Pair {
        let move_pair = match mv.direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let mut tail_move = (0, 0);
        for _ in 0..mv.amount {
            if let Some(next) = self.move_head_by(move_pair) {
                tail_move.0 += next.0;
                tail_move.1 += next.1;
            }
        }
        tail_move
    }
}

#[derive(Debug, PartialEq)]
struct ConfigChain {
    data: Vec<Configuration>,
}

impl ConfigChain {
    fn new(n: usize) -> Self {
        Self {
            data: vec![Configuration::new(); n - 1],
        }
    }

    fn apply_move(&mut self, mv: Move) {
        // This is kinda ugly, I would have preferred a single move to "trickle down".
        let mut next_moves = vec![mv];
        self.data = self
            .data
            .clone()
            .into_iter()
            .enumerate()
            .map(|(_index, mut config)| {
                // println!(
                //     "Index {}: Applying Move(s) {:?} on Config {:?}",
                //     index, next_moves, config
                // );
                let mut upcoming_moves = vec![];
                while let Some(next_move) = next_moves.pop() {
                    let move_pair = config.move_head(&next_move);
                    // println!("Move Pair {:?}, {:?}", move_pair, Move::from(move_pair));
                    if let Some(mut new_moves) = Move::from(move_pair) {
                        upcoming_moves.append(&mut new_moves);
                    }
                }
                next_moves = upcoming_moves;
                config
            })
            .collect();
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    direction: Direction,
    amount: usize,
}

impl Move {
    fn new(row: &str) -> Self {
        let x: Vec<&str> = row.trim().split(' ').collect();
        Self {
            direction: Direction::from_char(x[0].chars().next().unwrap()),
            amount: usize::from_str(x[1]).unwrap(),
        }
    }

    fn from(pair: Pair) -> Option<Vec<Self>> {
        if pair == (0, 0) {
            return None;
        }
        let (x, y) = pair;

        let move_1 = match y.cmp(&0) {
            Ordering::Less => Some(Self {
                direction: Direction::Down,
                amount: y.unsigned_abs() as usize,
            }),
            Ordering::Equal => None,
            Ordering::Greater => Some(Self {
                direction: Direction::Up,
                amount: y.unsigned_abs() as usize,
            }),
        };
        let move_2 = match x.cmp(&0) {
            Ordering::Less => Some(Self {
                direction: Direction::Left,
                amount: x.unsigned_abs() as usize,
            }),
            Ordering::Equal => None,
            Ordering::Greater => Some(Self {
                direction: Direction::Right,
                amount: x as usize,
            }),
        };

        let mut result = vec![];
        if let Some(mv) = move_1 {
            result.push(mv);
        }
        if let Some(mv) = move_2 {
            result.push(mv);
        }
        if result.is_empty() {
            return Some(result);
        }
        None
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input.trim().split('\n').map(Move::new).collect()
}

pub fn puzzle1(input: &str) -> i64 {
    let mut config = Configuration::new();
    for mv in parse_input(input) {
        config.move_head(&mv);
    }
    config.tail.visited.len().try_into().unwrap()
}

pub fn puzzle2(input: &str) -> i64 {
    let mut config_chain = ConfigChain::new(10);
    for mv in parse_input(input) {
        config_chain.apply_move(mv);
    }
    // println!("{:?}", config_chain);
    config_chain
        .data
        .pop()
        .unwrap()
        .tail
        .visited
        .len()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const SAMPLE_INPUT_2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn parse_input() {
        assert_eq!(
            super::parse_input(SAMPLE_INPUT),
            vec![
                Move::new("R 4"),
                Move::new("U 4"),
                Move::new("L 3"),
                Move::new("D 1"),
                Move::new("R 4"),
                Move::new("D 1"),
                Move::new("L 5"),
                Move::new("R 2")
            ]
        );
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 13);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT_2), 1);
    }

    #[test]
    fn apply_move() {
        let mut chain = ConfigChain::new(3);
        chain.apply_move(Move::new("U 7"));
        // println!("{:?}", chain.data.last().unwrap().tail);
        assert_eq!(
            chain.data.last().unwrap().tail.visited.len(),
            HashSet::from([(0, 5), (0, 4), (0, 3), (0, 2), (0, 1), (0, 0)]).len(),
        );
    }
}
