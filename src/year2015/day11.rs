use itertools::Itertools;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

fn next_letter(letter: char) -> Option<char> {
    let index = LETTERS.chars().position(|x| x == letter).unwrap();
    LETTERS.chars().nth(index + 1)
}

fn increment_word(word: &str) -> String {
    let mut characters = word.chars().collect_vec();
    characters.reverse();
    for mut item in characters.iter_mut() {
        match next_letter(*item) {
            Some(mut letter) => {
                item = &mut letter;
                break;
            }
            None => item = &mut 'a',
        }
    }
    characters.iter().map(|c| c.to_string()).join("")
}

fn is_valid(_password: &str) -> bool {
    // must include one increasing straight of at least three letters
    // may not contain the letters i, o, or l
    // must contain at least two different, non-overlapping pairs of letters
    false
}

pub fn puzzle1(input: &str) -> String {
    let mut password = input.to_string();

    while !is_valid(&password) {
        password = increment_word(&password);
    }
    password
}

pub fn puzzle2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "vzbxkghb";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), "492982");
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 6989950);
    }
}
