
use std::collections::HashSet;

use crate::util::{self, Grid};
use nalgebra::Point2;

pub fn puzzle1(input: &str) -> u32 {
    let engine = Engine {grid: Grid::from_layout(input)};
    engine.relevant_numbers().iter().sum()
}

pub fn puzzle2(_input: &str) -> u32 {
    0
}

struct Engine {
    grid: Grid
}
#[derive(Clone, Debug, PartialEq)]
struct NumberBuilder {
    number_str: String,
    row: usize,
    start: usize,
    end: usize,
    has_special_adjacents: bool
}

impl Engine {
    fn adjacent(&self, pos: Point2<usize>) -> impl '_  + Iterator<Item = u8> {
        util::adjacent8(pos).filter_map(move |p| self.grid.get(p))
    }

    fn relevant_numbers(&self) -> Vec<u32> {
        let mut res = vec![];
        let mut current_builder: Option<NumberBuilder> = None;
        let mut current_row;
        for (pos, val) in self.grid.iter() {
            current_row = pos[1];
            // When there is a number being built and row changes, push and reset the number builder
            if let Some(builder) = &current_builder {
                if current_row > builder.row {
                    if builder.has_special_adjacents {
                        res.push(builder.number_str.parse::<u32>().expect("is numba"));
                    }
                    current_builder = None;
                }
            }

            if val.is_ascii_digit() {
                // when a digit is 
                match current_builder {
                    Some(ref mut builder) => {
                        builder.number_str += &String::from_utf8(vec![val]).expect("is good");
                        builder.end = pos[0];
                        let special_chars = special_charset(self.adjacent(pos).collect());
                        builder.has_special_adjacents = builder.has_special_adjacents || !special_chars.is_empty();
                    },
                    None => {
                        current_builder = Some(NumberBuilder {
                            number_str: String::from_utf8(vec![val]).expect("is good"),
                            row: current_row,
                            start: pos[0],
                            end: pos[0],
                            has_special_adjacents: !special_charset(self.adjacent(pos).collect()).is_empty(),
                        })
                    }
                }
            } else {
                if let Some(builder) = &current_builder {
                    if builder.has_special_adjacents {
                        res.push(builder.number_str.parse::<u32>().expect("is numba"));
                    }
                    
                }
                current_builder = None;
            }
        }
        res
    }
}

fn special_charset(values: Vec<u8>) -> HashSet<char> {
    let mut res = HashSet::new();
    for v in values {
        res.insert(String::from_utf8(vec![v]).expect("is good").chars().next().expect("always one char"));
    }
    res.remove(&'.');
    for digit_char in ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'] {
        res.remove(&digit_char);
    }    
    res
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 4361);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }

}
