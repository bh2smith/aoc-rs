use crate::util::parse_csv;
use std::iter::Iterator;
use std::string::ToString;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Mul,
    InAt,
    OutFrom,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equal,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    op: Operation,
    modes: Vec<Mode>,
    op_type: u8,
}

pub fn parse_op(op_data: &str) -> Command {
    let op_type: u8;
    let op = match op_data.chars().last().unwrap() {
        '1' => {
            op_type = 3u8;
            Operation::Add
        }
        '2' => {
            op_type = 3u8;
            Operation::Mul
        }
        '3' => {
            op_type = 1u8;
            Operation::InAt
        }
        '4' => {
            op_type = 1u8;
            Operation::OutFrom
        }
        '5' => {
            op_type = 2u8;
            Operation::JumpTrue
        }
        '6' => {
            op_type = 2u8;
            Operation::JumpFalse
        }
        '7' => {
            op_type = 3u8;
            Operation::LessThan
        }
        '8' => {
            op_type = 3u8;
            Operation::Equal
        }
        _ => panic!("Unexpected operation character!"),
    };

    let mut modes: Vec<Mode> = vec![];
    let len = op_data.len();
    if len > 2 {
        modes = op_data[..len - 2]
            .chars()
            .map(|c| match c {
                '0' => Mode::Position,
                '1' => Mode::Immediate,
                _ => panic!("Unexpected mode character!"),
            })
            .rev()
            .collect();
    }
    while modes.len() < 3 {
        modes.push(Mode::Position)
    }
    Command { op, modes, op_type }
}

pub fn solve(program: &str, input: i64) -> i64 {
    let mut numbers: Vec<i64> = parse_csv(program);
    let mut i: usize = 0;
    let mut output: Option<i64> = None;

    while numbers[i] != 99 {
        let command = parse_op(&numbers[i].to_string());
        match command.op_type {
            1u8 => {
                let x = numbers[i + 1];
                match command.op {
                    Operation::InAt => numbers[x as usize] = input,
                    Operation::OutFrom => {
                        let a = if command.modes[0] == Mode::Position {
                            numbers[x as usize]
                        } else {
                            x
                        };
                        output = Some(a);
                    }
                    _ => panic!(""),
                };
                i += 2;
            }
            2u8 => {
                let x = numbers[i + 1];
                let y = numbers[i + 2];

                let a = if command.modes[0] == Mode::Position {
                    numbers[x as usize]
                } else {
                    x
                };
                let b = if command.modes[1] == Mode::Position {
                    numbers[y as usize]
                } else {
                    y
                };

                i = match command.op {
                    Operation::JumpTrue => {
                        if a > 0 {
                            b as usize
                        } else {
                            i + 3
                        }
                    }
                    Operation::JumpFalse => {
                        if a == 0 {
                            b as usize
                        } else {
                            i + 3
                        }
                    }
                    _ => panic!(""),
                }
            }
            3u8 => {
                let x = numbers[i + 1];
                let y = numbers[i + 2];
                let z = numbers[i + 3] as usize;

                let a = if command.modes[0] == Mode::Position {
                    numbers[x as usize]
                } else {
                    x
                };
                let b = if command.modes[1] == Mode::Position {
                    numbers[y as usize]
                } else {
                    y
                };

                numbers[z] = match command.op {
                    Operation::Add => a + b,
                    Operation::Mul => a * b,
                    Operation::LessThan => {
                        if a < b {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::Equal => {
                        if a == b {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Operation-type mismatch!"),
                };
                i += 4;
            }
            _ => panic!("Unexpected op-type!"),
        }
    }
    output.unwrap_or(0)
}

pub fn puzzle1(input: &str) -> i64 {
    solve(input, 1)
}

pub fn puzzle2(input: &str) -> i64 {
    solve(input, 5)
}

#[cfg(test)]
mod tests {
    use super::Command;
    use super::Mode::{Immediate, Position};
    use super::Operation;

    #[test]
    fn parse_test() {
        assert_eq!(
            super::parse_op("11101"),
            Command {
                op: Operation::Add,
                modes: vec![Immediate, Immediate, Immediate],
                op_type: 3u8,
            }
        );
        assert_eq!(
            super::parse_op("10102"),
            Command {
                op: Operation::Mul,
                modes: vec![Immediate, Position, Immediate],
                op_type: 3u8,
            }
        );
        assert_eq!(
            super::parse_op("1003"),
            Command {
                op: Operation::InAt,
                modes: vec![Position, Immediate, Position],
                op_type: 1u8,
            }
        );
        assert_eq!(
            super::parse_op("104"),
            Command {
                op: Operation::OutFrom,
                modes: vec![Immediate, Position, Position],
                op_type: 1u8,
            }
        );
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("1101,100,-1,4,0"), 0);
        assert_eq!(super::puzzle1("1002,4,3,4,33"), 0);
        assert_eq!(super::puzzle1("3,0,4,0,99"), 1);
    }

    #[test]
    fn comparison() {
        // Position Mode - Equal to 8
        assert_eq!(super::solve("3,9,8,9,10,9,4,9,99,-1,8", 1), 0);
        assert_eq!(super::solve("3,9,8,9,10,9,4,9,99,-1,8", 8), 1);

        // Position Mode - Less than 8
        assert_eq!(super::solve("3,9,7,9,10,9,4,9,99,-1,8", 1), 1);
        assert_eq!(super::solve("3,9,7,9,10,9,4,9,99,-1,8", 8), 0);

        // Immediate Mode - Equal to 8
        assert_eq!(super::solve("3,3,1108,-1,8,3,4,3,99", 8), 1);
        assert_eq!(super::solve("3,3,1108,-1,8,3,4,3,99", 7), 0);

        // Immediate Mode - Less than 8
        assert_eq!(super::solve("3,3,1107,-1,8,3,4,3,99", 5), 1);
        assert_eq!(super::solve("3,3,1107,-1,8,3,4,3,99", 9), 0);
    }

    #[test]
    fn jump() {
        // Position Mode - output 0 if input = 0 else 1
        assert_eq!(
            super::solve("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0),
            0
        );
        assert_eq!(
            super::solve("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 1),
            1
        );

        // Immediate Mode - output 0 if input = 0 else 1
        assert_eq!(super::solve("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0), 0);
        assert_eq!(super::solve("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1), 1);
    }

    #[test]
    fn large_example() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(super::solve(program, 7), 999);
        assert_eq!(super::solve(program, 8), 1000);
        assert_eq!(super::solve(program, 9), 1001);
    }
}
