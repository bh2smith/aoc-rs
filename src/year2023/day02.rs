use itertools::Itertools;

#[derive(PartialEq, Debug, Clone, Copy)]
struct ColourCount {
    red: u32,
    blue: u32,
    green: u32,
}

impl ColourCount {
    pub fn new(entry_data: &str) -> Self {
        let (mut red, mut blue, mut green) = (0, 0, 0);
        for counter in entry_data.split(',') {
            let num_colour: Vec<&str> = counter.trim().split(' ').collect();
            let number = num_colour[0].parse::<u32>().expect("parse number");
            match num_colour[1] {
                "red" => red = number,
                "blue" => blue = number,
                "green" => green = number,
                _ => panic!("unexpected colour"),
            }
        }
        Self { red, blue, green }
    }

    pub fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    round: u32,
    output: Vec<ColourCount>,
}

impl Game {
    pub fn new(input: &str) -> Self {
        let input: Vec<&str> = input.trim().split(':').collect();
        let round = input[0]
            .split(' ')
            .next_back()
            .expect("always something")
            .parse::<u32>()
            .expect("game number");
        let results = input[1].split(';');
        let mut output = vec![];
        for entry_data in results {
            output.push(ColourCount::new(entry_data));
        }
        Self { round, output }
    }

    pub fn is_valid(&self, red: u32, green: u32, blue: u32) -> bool {
        for shown in &self.output {
            if shown.red > red || shown.blue > blue || shown.green > green {
                return false;
            }
        }
        true
    }

    pub fn fewest(&self) -> ColourCount {
        let (mut red, mut blue, mut green) = (0, 0, 0);
        for shown in &self.output {
            if shown.red > red {
                red = shown.red;
            }
            if shown.blue > blue {
                blue = shown.blue;
            }
            if shown.green > green {
                green = shown.green;
            }
        }

        ColourCount { red, blue, green }
    }
}

pub fn puzzle1(input: &str) -> u32 {
    let valid_games: Vec<_> = input
        .trim()
        .split('\n')
        .map(Game::new)
        .filter(|game| game.is_valid(12, 13, 14))
        .collect_vec();
    valid_games.iter().map(|game| game.round).sum()
}

pub fn puzzle2(input: &str) -> u32 {
    let games: Vec<_> = input.trim().split('\n').map(Game::new).collect();
    games.iter().map(|game| game.fewest().power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 8);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 2286);
    }

    #[test]
    fn new_colour_count() {
        assert_eq!(
            ColourCount::new("3 blue, 4 red"),
            ColourCount {
                blue: 3,
                red: 4,
                green: 0,
            },
        )
    }

    #[test]
    fn new_game() {
        assert_eq!(
            Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                round: 1,
                output: vec![
                    ColourCount::new(" 3 blue, 4 red"),
                    ColourCount::new(" 1 red, 2 green, 6 blue"),
                    ColourCount::new("2 green")
                ]
            }
        )
    }
}
