use std::collections::HashSet;

use itertools::Itertools;
use maplit::hashset;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn next_letter(letter: char) -> char {
    let index = ALPHABET.chars().position(|x| x == letter).unwrap();
    ALPHABET.chars().nth(index + 1).unwrap_or('a')
}

fn increment_word(word: &str) -> String {
    let mut chars = word.chars().rev();
    let mut next_word = "".to_string();
    for curr in chars.by_ref() {
        let next_letter = next_letter(curr);
        next_word.push(next_letter);
        if next_letter != 'a' {
            break;
        }
    }
    for curr in chars {
        next_word.push(curr);
    }
    next_word.chars().rev().map(|c| c.to_string()).join("")
}

fn consecutive(word: &str, n: u32) -> bool {
    if n == 1 {
        return true;
    }
    let mut count = 0u32;
    let mut characters = word.chars();
    let mut curr = characters.next().expect("empty word");
    for next in characters {
        if ALPHABET.find(curr).unwrap() + 1 == ALPHABET.find(next).unwrap() {
            count += 1;
        } else {
            count = 0;
        }
        if count >= n - 1 {
            return true;
        }
        curr = next;
    }
    false
}

fn contains_any(word: &str, chars: HashSet<char>) -> bool {
    word.chars().any(|c| chars.contains(&c))
}

fn distinct_pairs(word: &str, n: usize) -> bool {
    let mut pairs = vec![];
    let mut characters = word.chars();
    let mut curr = characters.next().unwrap();
    while let Some(next) = characters.next() {
        if curr == next {
            pairs.push(curr);
            match characters.next() {
                Some(x) => curr = x,
                None => break,
            }
        } else {
            curr = next;
        }
    }
    pairs.len() >= n
}

fn is_valid(password: &str) -> bool {
    let conditions = [
        // must include one increasing straight of at least three letters
        consecutive(password, 3),
        // may not contain the letters i, o, or l
        !contains_any(password, hashset! {'i', 'o', 'l'}),
        // must contain at least two different, non-overlapping pairs of letters
        distinct_pairs(password, 2),
    ];
    // println!("conditions {:?}", conditions);
    conditions.iter().all(|x| *x)
}

pub fn puzzle1(input: &str) -> String {
    let mut password = input.to_string();

    while !is_valid(&password) {
        // println!("Password {password}");
        password = increment_word(&password);
    }
    password
}

pub fn puzzle2(input: &str) -> String {
    puzzle1(&increment_word(&puzzle1(input)))
}

#[cfg(test)]
mod tests {
    use maplit::hashset;

    const SAMPLE_INPUT: &str = "vzbxkghb";

    #[test]
    fn puzzle1() {
        // assert_eq!(super::puzzle1("abcdefgh"), "abcdffaa");
        // assert_eq!(super::puzzle1("ghijklmn"), "ghjaabcc");

        // Real input
        assert_eq!(super::puzzle1(SAMPLE_INPUT), "vzbxxyzz");
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), "vzcaabcc");
    }

    #[test]
    fn increment_word() {
        assert_eq!(super::increment_word("abcd"), "abce");
        assert_eq!(super::increment_word("abcz"), "abda");
        assert_eq!(super::increment_word("abzz"), "acaa");
    }

    #[test]
    fn consecutive() {
        assert!(super::consecutive("abcd", 1));
        assert!(super::consecutive("abcd", 2));
        assert!(super::consecutive("abcd", 3));
        assert!(super::consecutive("abcd", 4));
        assert!(!super::consecutive("abcd", 5));

        assert!(super::consecutive("ba", 1));
        assert!(!super::consecutive("ba", 2));
        assert!(!super::consecutive("ba", 3));

        assert!(super::consecutive("hellothereabcdefoneto", 6));
        assert!(!super::consecutive("hellothereabcdefoneto", 7));
    }

    #[test]
    fn contains_any() {
        assert!(super::contains_any("abcd", hashset! {'a'}));
        assert!(super::contains_any("abcd", hashset! {'b'}));
        assert!(super::contains_any("abcd", hashset! {'c', 'd'}));
        assert!(!super::contains_any("abcd", hashset! {'x', 'y'}));
    }

    #[test]
    fn distinct_pairs() {
        assert!(super::distinct_pairs("ghjaabcc", 2));

        assert!(!super::distinct_pairs("abcd", 1));
        assert!(!super::distinct_pairs("abcd", 2));
        assert!(super::distinct_pairs("aabb", 2));
        assert!(super::distinct_pairs("aabb", 1));

        assert!(!super::distinct_pairs("aabb", 3));

        assert!(!super::distinct_pairs("aaa", 2));
        assert!(super::distinct_pairs("aaaa", 2));
    }

    #[test]
    fn is_valid() {
        assert!(!super::is_valid("hijklmmn"));
        assert!(!super::is_valid("abbceffg"));
        assert!(!super::is_valid("abbcegjk"));

        assert!(super::is_valid("ghjaabcc"));
    }
}
