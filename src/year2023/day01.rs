use std::collections::HashMap;

use maplit::hashmap;

fn insert_at_position(original: &str, position: usize, substring: &str) -> String {
    let mut result = String::with_capacity(original.len() + substring.len());
    result.push_str(&original[..position]);
    result.push_str(substring);
    result.push_str(&original[position..]);

    result
}

fn find_all_word_positions(sentence: &str, word: &str) -> Vec<usize> {
    sentence
        .match_indices(word)
        .map(|(index, _)| index)
        .collect()
}

#[derive(Default, Debug)]
struct PostitionDigit {
    position: usize,
    digit: String,
}

pub fn solve(input: &str, only_digits: bool) -> u32 {
    let replacements: HashMap<&str, &str> = hashmap! {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "zero" => "0"
    };

    let split_input: Vec<&str> = input.trim().split('\n').collect();
    let mut cur_sum = 0;
    for row in split_input {
        let replaced_row = match only_digits {
            true => row.to_string(),
            false => {
                let mut left = PostitionDigit {
                    position: row.len(),
                    digit: "".to_string(),
                };
                let mut right = PostitionDigit::default();
                for (word, digit) in replacements.clone() {
                    let positions = find_all_word_positions(row, word);
                    for position in positions {
                        if position < left.position {
                            left.position = position;
                            left.digit = digit.to_string();
                        }
                        if position > right.position {
                            right.position = position;
                            right.digit = digit.to_string();
                        }
                    }
                }
                let mut updated = insert_at_position(row, left.position, &left.digit);
                updated = insert_at_position(&updated, right.position + 1, &right.digit);
                println!("Transformed {} --> {}", row, updated);
                updated
            }
        };
        let numbies: Vec<char> = replaced_row
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
        let next_num = 10 * numbies[0].to_digit(10).expect("left digit")
            + numbies[numbies.len() - 1]
                .to_digit(10)
                .expect("right digit");
        println!("Row Number {}", next_num);
        cur_sum += next_num;
    }
    cur_sum
}

pub fn puzzle1(input: &str) -> u32 {
    solve(input, true)
}

pub fn puzzle2(input: &str) -> u32 {
    solve(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 142);
    }

    #[test]
    fn puzzle2() {
        let sample_input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(super::puzzle2(sample_input), 281);
    }

    #[test]
    fn find_positions() {
        let my_sentence = "eightwothree";
        let my_word = "eight";

        let positions = find_all_word_positions(my_sentence, my_word);

        if positions.is_empty() {
            println!("The word '{}' is not found in the sentence.", my_word);
        } else {
            println!(
                "The word '{}' is found at positions: {:?}",
                my_word, positions
            );
        }
    }

    #[test]
    fn insert_position() {
        assert_eq!(insert_at_position("my_sentence", 0, "8"), "8my_sentence");
    }
}
