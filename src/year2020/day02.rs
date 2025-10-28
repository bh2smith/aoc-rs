use regex::Regex;
use std::str::FromStr;

lazy_static::lazy_static! {
    static ref RULE_PATTERN: Regex =
        Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
}

#[derive(Debug)]
struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl FromStr for PasswordRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RULE_PATTERN
            .captures(s)
            .ok_or_else(|| format!("Invalid rule: {}", s))?;

        Ok(Self {
            min: caps[1].parse().unwrap(),
            max: caps[2].parse().unwrap(),
            letter: caps[3].chars().next().unwrap(),
            password: caps[4].to_string(),
        })
    }
}

impl PasswordRule {
    fn is_valid_1(&self) -> bool {
        let count = self.password.chars().filter(|x| *x == self.letter).count();
        self.min <= count && count <= self.max
    }

    fn is_valid_2(&self) -> bool {
        let x = self.password.chars().nth(self.min - 1);
        let y = self.password.chars().nth(self.max - 1);
        match (x, y) {
            (Some(a), Some(b)) => {
                (a == self.letter || b == self.letter) && !(a == self.letter && b == self.letter)
            }
            _ => false,
        }
    }
}

pub fn puzzle1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter_map(|x| {
            let pass = PasswordRule::from_str(x).ok()?;
            pass.is_valid_1().then_some(pass)
        })
        // .map(|x| PasswordRule::from_str(x).unwrap())
        // .filter(|p| p.is_valid())
        .count()
}

pub fn puzzle2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter_map(|x| {
            let pass = PasswordRule::from_str(x).ok()?;
            pass.is_valid_2().then_some(pass)
        })
        .count()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 2);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 1);
    }
}
