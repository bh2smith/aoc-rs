// Rock-Paper-Scissors
// https://adventofcode.com/2022/day/2

#[derive(Debug, PartialEq)]
pub enum Play {
    Rock,
    Paper,
    Scissors
}

impl Play {
    fn from_char(play_char: char) -> Play {
        // For Puzzle 1: X, Y, Z represent rock, paper scissor respectively
        match play_char {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => panic!("Invalid play char {}", play_char)
        }
    }

    fn from_pair_str(left: char, right: char) -> (Play, Play) {
        // For Puzzle 2: X, Y, Z represent lose, draw, win respectively
        let left_move = Play::from_char(left);
        let shift = match right {
            'X' => 2,
            'Y' => 0,
            'Z' => 1,
            _ => panic!("Invalid play char {}", right)
        };
        let move_points = (left_move.points() + shift).rem_euclid(3);
        let right_move = Play::from_points(move_points);
        (left_move, right_move)
    }

    fn from_points(pts: i64) -> Play {
        match pts.rem_euclid(3) {
            1 => Play::Rock,
            2 => Play::Paper,
            0 => Play::Scissors,
            _ => panic!("Invalid point constructor {}", pts)
        }
    }

    fn points(&self) -> i64 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn compete(&self, other: &Play) -> i64 {
        let result = match (other.points() - self.points()).rem_euclid(3) {
            0 => 3, // draw
            1 => 6, // other wins
            _ => 0  // other loses
        };
        result + other.points()
    }
}


pub fn parse_input(input: &str) -> Vec<(char, char)> {
    let parsed_input: Vec<&str> = input.trim().split('\n').collect();
    // Each entry looks like 'A B' 
    parsed_input.iter().map(|gp| {
        let chars: Vec<char> = gp.chars().collect();
        (chars[0], chars[2])
    }).collect()
}


pub fn puzzle1(input: &str) -> i64 {
    let game_chars = parse_input(input);
    game_chars.into_iter().map(|(them, me)| Play::from_char(them).compete(&Play::from_char(me))).sum()
}

pub fn puzzle2(input: &str) -> i64 {
    let game_chars = parse_input(input);
    game_chars.into_iter().map(|(them, me)| {
        let (left, right) = Play::from_pair_str(them, me);
        left.compete(&right)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::Play;
    const SAMPLE_INPUT: &str =r"A Y
B X
C Z";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 15);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 12);
    }

    #[test]
    fn test_play_from_char() {
        assert_eq!(Play::from_char('X'), Play::Rock);
        assert_eq!(Play::from_char('A'), Play::Rock);

        assert_eq!(Play::from_char('B'), Play::Paper);
        assert_eq!(Play::from_char('Y'), Play::Paper);

        assert_eq!(Play::from_char('C'), Play::Scissors);
        assert_eq!(Play::from_char('Z'), Play::Scissors);
    }

    #[test]
    fn test_play_struct() {
        let rock = Play::Rock;
        let paper = Play::Paper;
        let scissors = Play::Scissors;

        assert_eq!(rock.compete(&rock), 3 + 1);
        assert_eq!(rock.compete(&paper), 6 + 2);
        assert_eq!(rock.compete(&scissors), 0 + 3);
        
        assert_eq!(paper.compete(&rock), 0 + 1);
        assert_eq!(paper.compete(&paper), 3 + 2);
        assert_eq!(paper.compete(&scissors), 6 + 3);

        assert_eq!(scissors.compete(&rock), 6 + 1);
        assert_eq!(scissors.compete(&paper), 0 + 2);
        assert_eq!(scissors.compete(&scissors), 3 + 3);
    }

}
