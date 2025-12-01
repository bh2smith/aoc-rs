use std::str::FromStr;
struct Clock {
    pub at: u32,
    hours: u32,
}

impl Clock {
    fn new(start: u32, hours: u32) -> Self {
        Self { at: start, hours }
    }

    fn tick_left(&mut self) {
        if self.at == 0 {
            self.at = self.hours - 1;
        } else {
            self.at -= 1;
        }
    }

    fn tick_right(&mut self) {
        if self.at == self.hours - 1 {
            self.at = 0;
        } else {
            self.at += 1;
        }
    }

    // Returns num times crossed zero.
    fn turn_left(&mut self, by: u32) -> u32 {
        let mut around = by / self.hours;
        let mut remainder = by.rem_euclid(self.hours);
        while remainder > 0 {
            self.tick_left();
            if self.at == 0 {
                around += 1
            }
            remainder -= 1;
        }
        around
    }

    fn turn_right(&mut self, by: u32) -> u32 {
        let mut around = by / self.hours;
        let mut remainder = by.rem_euclid(self.hours);
        while remainder > 0 {
            self.tick_right();
            if self.at == 0 {
                around += 1
            }
            remainder -= 1;
        }
        around
    }

    fn turn(&mut self, turn: &Turn) -> u32 {
        match turn.dir {
            Dir::Left => self.turn_left(turn.by),
            Dir::Right => self.turn_right(turn.by),
        }
    }
}

enum Dir {
    Left,
    Right,
}
struct Turn {
    dir: Dir,
    by: u32,
}

fn parse_input(input: &str) -> Vec<Turn> {
    input
        .trim()
        .split('\n')
        .map(|line| Turn {
            dir: match line.chars().next().unwrap() {
                'L' => Dir::Left,
                'R' => Dir::Right,
                _ => panic!("invalid input"),
            },
            by: u32::from_str(&line[1..]).unwrap(),
        })
        .collect()
}

pub fn puzzle1(input: &str) -> usize {
    let mut clock = Clock::new(50, 100);
    parse_input(input)
        .iter()
        .filter(|by| {
            clock.turn(by);
            clock.at == 0
        })
        .count()
}

pub fn puzzle2(input: &str) -> u32 {
    let mut clock = Clock::new(50, 100);
    parse_input(input).iter().map(|by| clock.turn(by)).sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 3);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 6);
    }
}
