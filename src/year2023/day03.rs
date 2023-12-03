
use std::collections::{HashSet, HashMap};
use maplit::hashset;

use crate::util::{self, Grid};
use nalgebra::Point2;

pub fn puzzle1(input: &str) -> u32 {
    let engine = Engine {grid: Grid::from_layout(input)};
    engine.relevant_numbers(None).iter().map(|builder| builder.value()).sum()
}


pub fn puzzle2(input: &str) -> u32 {
    let engine = Engine {grid: Grid::from_layout(input)};
    let numbers = engine.relevant_numbers(Some(hashset!{'*'}));
    let mut gears = HashMap::new();
    let stars = engine.star_positions();
    for star_position in stars {
        for number in numbers.clone() {
            if number.is_adjacent(star_position) {
                let entry = gears.entry(star_position).or_insert(vec![]);
                entry.push(number.value())
            }
        }
    }
    // println!("Gears {:?}", gears);
    gears.iter().filter_map(|(_, numbers)| {if numbers.len() == 2 {
        return Some(numbers[0] * numbers[1]);
    } None}).sum::<u32>()
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
    special_adjacents: HashSet<char>
}

impl NumberBuilder {
    pub fn has_special_adjacents(&self) -> bool {
        !self.special_adjacents.is_empty()
    }

    pub fn is_adjacent(&self, pos: Point2<usize>) -> bool {
        let col = pos[0];
        let row = pos[1];
        for i in self.start..self.end + 1 {
            let row_diff = self.row.abs_diff(row);
            let col_diff = col.abs_diff(i);
            if (row_diff + col_diff <= 1) || (row_diff == 1 && col_diff == 1) {
                return true;
            }
        }
        false
        // pos[1] == self.row && (self.start <= pos[0] && pos[0] <= self.end)
    }

    pub fn value(&self) -> u32 {
        self.number_str.parse::<u32>().expect("is numba")
    }
}

impl Engine {
    fn adjacent(&self, pos: Point2<usize>) -> impl '_  + Iterator<Item = u8> {
        util::adjacent8(pos).filter_map(move |p| self.grid.get(p))
    }

    fn relevant_numbers(&self, explicitly_special: Option<HashSet<char>>) -> Vec<NumberBuilder> {
        let mut res = vec![];
        let mut current_builder: Option<NumberBuilder> = None;
        let mut current_row;
        for (pos, val) in self.grid.iter() {
            current_row = pos[1];
            // When there is a number being built and row changes, push and reset the number builder
            if let Some(builder) = &current_builder {
                if current_row > builder.row {
                    if builder.has_special_adjacents() {
                        res.push(builder.clone());
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
                        let special_chars = special_charset(self.adjacent(pos).collect(), explicitly_special.clone());
                        builder.special_adjacents.extend(special_chars);
                    },
                    None => {
                        current_builder = Some(NumberBuilder {
                            number_str: String::from_utf8(vec![val]).expect("is good"),
                            row: current_row,
                            start: pos[0],
                            end: pos[0],
                            special_adjacents: special_charset(self.adjacent(pos).collect(), explicitly_special.clone()),
                        })
                    }
                }
            } else {
                if let Some(builder) = &current_builder {
                    if builder.has_special_adjacents() {
                        res.push(builder.clone());
                    }
                    
                }
                current_builder = None;
            }
        }
        res
    }

    fn star_positions(&self) -> HashSet<Point2<usize>> {
        let mut res = HashSet::new();
        for (pos, val) in self.grid.iter() {
            if val == b'*' {
                res.insert(pos);
            }
        }
        res
    }
}

fn special_charset(values: Vec<u8>, explicitly_special: Option<HashSet<char>>) -> HashSet<char> {
    let mut res = HashSet::new();
    for v in values {
        res.insert(String::from_utf8(vec![v]).expect("is good").chars().next().expect("always one char"));
    }
    // When explicitly special characters are provided we keep only those, otherwise, we remove all digits and .
    if let Some(special_chars) = explicitly_special {
        res.retain(|char| special_chars.contains(char))
    } else {
        for digit_char in ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'] {
            res.remove(&digit_char);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::NumberBuilder;


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
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 467835);
    }

    #[test]
    fn is_adjacent() {
        // ****x
        // 123*x
        // ****x
        let num = NumberBuilder {
            number_str: "123".to_string(),
            row: 1,
            start: 0,
            end: 2,
            special_adjacents: HashSet::new(),
        };

        assert!(num.is_adjacent([0, 0].into()));
        assert!(num.is_adjacent([1, 0].into()));
        assert!(num.is_adjacent([2, 0].into()));
        assert!(num.is_adjacent([3, 0].into()));
        assert!(!num.is_adjacent([4, 0].into()));
        assert!(num.is_adjacent([3, 1].into()));
        assert!(!num.is_adjacent([4, 1].into()));
        assert!(num.is_adjacent([0, 2].into()));
        assert!(num.is_adjacent([1, 2].into()));
        assert!(num.is_adjacent([2, 2].into()));
        assert!(num.is_adjacent([3, 2].into()));
        assert!(!num.is_adjacent([4, 2].into()));
    }

}
