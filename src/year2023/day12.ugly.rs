use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
struct WordGame {
    word: Vec<char>,
    parts: Vec<usize>,
}

impl WordGame {
    fn from(input: &str) -> Self {
        let mut t = input.split_whitespace();
        let word = t.next().unwrap().trim_matches('.');

        WordGame {
            word: word.chars().collect_vec(),
            parts: t
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect_vec(),
        }
    }

    fn solve(&self) -> usize {
        solve_any(&self.word, &self.parts)
    }
}

fn parse_input(input: &str) -> Vec<WordGame> {
    input.trim().split('\n').map(WordGame::from).collect_vec()
}

fn free_solve(word: &[char], parts: &[usize]) -> usize {
    // // println!("  FreeSolve: word={:?}, parts={:?}", word, parts);
    if parts.is_empty() {
        // // println!("No parts!");
        match word.len() {
            0 => return 0,
            _ => return 1,
        }
    }
    let n = word.len();
    let k: usize = parts.iter().sum();
    if n == 0 || n < k + parts.len() - 1 {
        if n > 0 && parts.len() == 1 && n == k {
            // // println!("Returning {}", 1);
            return 1;
        }
        // // println!("Returning 0 cuz n < 2 * k - 1");
        return 0;
    }
    if n == k + parts.len() - 1 {
        // // println!("Returning 1 cuz n == k + parts.len() - 1");
        return 1;
    }
    assert!(word.iter().collect::<HashSet<_>>() == HashSet::from([&'?']));

    if parts.len() == 1 {
        // // println!("Returning {}", n - k + 1);
        return n - k + 1;
    }
    let v = parts[0];
    (0..(n - v - 1))
        .map(|i| {
            // // println!("i={i}, v={v}, sub_word={:?}", &word[v + 1 + i..]);
            free_solve(&word[v + 1 + i..], &parts[1..])
        })
        .sum()
}

fn longest_pound(word: &[char]) -> (usize, usize) {
    let mut best_length = 0;
    let mut index = 0;
    for i in 0..word.len() {
        if word[i] == '#' {
            let mut j = 0;
            let mut length = 1;
            while i + j + 1 < word.len() && word[i + j + 1] == '#' {
                length += 1;
                j += 1;
            }
            if length > best_length {
                best_length = length;
                index = i;
            }
        }
    }
    (index, best_length)
}

fn valid_parts(parts: &[usize], val: usize) -> HashSet<usize> {
    parts
        .iter()
        .enumerate()
        .filter_map(|(i, x)| if x >= &val { Some(i) } else { None })
        .collect::<HashSet<_>>()
}

fn solve_any(word: &[char], parts: &[usize]) -> usize {
    let word_str: String = word.iter().collect();
    let trimmed_word = word_str.trim_matches('.');
    let word = &trimmed_word.chars().collect_vec();
    let parts = parts.iter().cloned().filter(|x| x != &0).collect_vec();
    // println!("word={:?}, parts={:?}", word, parts);
    if word.is_empty() || parts.iter().sum::<usize>() > word.len() {
        if parts.is_empty() {
            // println!("  Return 1");
            return 1;
        } else {
            // println!("  Return 0");
            return 0;
        }
    }
    let word_chars = word.iter().collect::<HashSet<_>>();
    if word_chars == HashSet::from([&'?']) {
        let x = free_solve(word, parts.as_slice());
        // println!("   free_solve {x}");
        return x;
    }
    if word_chars == HashSet::from([&'.']) {
        return 0;
    }
    if word_chars == HashSet::from([&'#']) {
        // println!("Only #");
        // ONLY #
        if parts.len() == 1 && parts[0] == word.len() {
            // println!("  Return 1");
            return 1;
        }
        // println!("  Return 0");
        return 0;
    }
    if word.contains(&'.') {
        let split = word.iter().position(|x| x == &'.').unwrap();
        // println!("   Split . at {split}");
        return (0..parts.len())
            .map(|i| {
                let left = solve_any(&word[..split], &parts[..i]);
                let right = solve_any(&word[split + 1..], &parts[i..]);
                left * right
            })
            .sum();
    }

    // Word is only # and ? now (the harder part).
    if parts.len() == 1 && parts[0] <= word.len() {
        return word.len() - parts[0] + 1;
    }

    // w = w1 # w2
    let (index, length) = longest_pound(word);
    let valid_parts = valid_parts(&parts, length);
    if valid_parts.is_empty() {
        // There are no parts that can make this work!
        return 0;
    }
    let mut result = 0;
    let w1 = &word[..index];
    let w2 = &word[index + length..];
    // // println!("Case #: Operating on {word:?} as w1={w1:?} w2={w2:?}");
    for index in valid_parts {
        // We will use the special part in the longest pound word.
        let special_part = parts[index];

        let p_l = &parts[..index];
        let p_r = if index + 1 < parts.len() {
            &parts[index + 1..]
        } else {
            &[]
        };
        // println!("special {}: p_l= {:?}, p_r={:?}", special_part, &p_l, &p_r);

        if special_part == length {
            if !w1.is_empty() && !w2.is_empty() {
                // println!("left {w1:?} {p_l:?} right {w2:?} {p_r:?}");
                let x = solve_any(&w1[..w1.len() - 1], p_l) * solve_any(&w2[1..], p_r);
                // println!("Exact special found {} solutions", x);
                result += x;
            } else if w1.is_empty() && p_l.is_empty() {
                // println!("exact empty w1: left {w1:?} {p_l:?} right {w2:?} {p_r:?}");
                // println!("Solving {:?} {p_r:?}", &w2[1..]);
                let x = solve_any(&w2[1..], p_r);
                // println!("Exact special on empty w1 found {} solutions", x);
                result += x;
            } else if w2.is_empty() && p_r.is_empty() {
                // println!("exact empty w2: left {w1:?} {p_l:?} right {w2:?} {p_r:?}");
                // println!("Solving {:?} {p_l:?}", &w1[..w1.len() - 1]);
                let x = solve_any(&w1[..w1.len() - 1], p_l);
                // println!("Exact special on empty w2 found {} solutions", x);
                result += x;
            }
            // println!("No action: left {w1:?} {p_l:?} right {w2:?} {p_r:?}");
        } else {
            // special part needs moar SpAse!
            let diff = special_part - length;
            for num_left in 0..diff + 1 {
                let num_right = diff - num_left;
                if num_left > w1.len() || num_right > w2.len() {
                    continue;
                }
                // println!("Result before try num_l = {num_left}, num_r = {num_right}: {result}");
                if num_left == w1.len() && num_right == w2.len() {
                    result += 1;
                } else if num_left < w1.len() && num_right < w2.len() {
                    // println!("gibt enough space!");
                    let left = solve_any(&w1[..w1.len() - num_left - 1], p_l);
                    let right = solve_any(&w2[num_right + 1..], p_r);
                    // println!("      Found {left} * {right} = {}", left * right);
                    result += left * right;
                } else if w1.is_empty() && p_l.is_empty() && num_right <= w2.len() {
                    // println!("null Left! w2={:?} p_r={p_r:?}", &w2[num_right + 1..]);
                    result += solve_any(&w2[num_right + 1..], p_r);
                } else if w2.is_empty() && p_r.is_empty() && num_left <= w1.len() {
                    // println!(
                    //     "null Right! w1={:?} p_l={p_l:?}",
                    //     &w1[..w1.len() - num_left - 1]
                    // );
                    result += solve_any(&w1[..w1.len() - num_left - 1], p_l);
                }
                // println!("Result after {result}");
            }
        }
    }
    result
}

pub fn puzzle1(input: &str) -> usize {
    let games = parse_input(input);
    let mut answers = 0;
    for g in games {
        let next = g.solve();
        if next > 50 {
            println!("{:?}: {}", g, next);
        }

        answers += next;
    }

    answers
}

pub fn puzzle2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {

    use super::WordGame;

    const SAMPLE_INPUT: &str = "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 21);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }

    #[test]
    fn generic_solve() {
        assert_eq!(WordGame::from("???# 1,1").solve(), 2);
        assert_eq!(WordGame::from("#?#?#?#?#?#? 3,1,6").solve(), 1);
        assert_eq!(WordGame::from("?#?#?#?#?#?#?#? 1,3,1,6").solve(), 1);

        assert_eq!(WordGame::from("#??? 1,1").solve(), 2);

        assert_eq!(WordGame::from("#?????#### 1,1,5").solve(), 2);

        assert_eq!(WordGame::from("?###???????? 3,2,1").solve(), 10);
        assert_eq!(WordGame::from("?## 3").solve(), 1);
        assert_eq!(WordGame::from("#.#.### 1,1,3").solve(), 1);
        assert_eq!(WordGame::from("???.### 1,1,3").solve(), 1);
        assert_eq!(WordGame::from(".??..??...?##. 1,1,3").solve(), 4);
        assert_eq!(WordGame::from("????.#...#... 4,1,1").solve(), 1);
        assert_eq!(WordGame::from("????.######..#####. 1,6,5").solve(), 4);

        assert_eq!(
            super::solve_any(&['?', '?', '#', '?', '?', '?'], &[1, 2]),
            2
        );

        assert_eq!(
            super::solve_any(&['?', '?', '?', '?', '#', '?', '?', '?'], &[1, 1, 2]),
            4
        );

        assert_eq!(
            super::solve_any(&['?', '?', '?', '?', '#', '?', '?', '?', '?'], &[1, 1, 2]),
            7
        );
    }

    #[test]
    fn solve_without_pound() {
        assert_eq!(
            super::solve_any(&['.', '?', '?', '.', '.', '?', '?', '.', '.', '.'], &[1, 1]),
            4
        );
        assert_eq!(
            super::solve_any(&['?', '?', '.', '?', '?', '?'], &[1, 2]),
            4
        );

        assert_eq!(
            super::solve_any(&['?', '?', '?', '?', '.', '?', '?', '?'], &[1, 1, 2]),
            6
        );

        assert_eq!(
            super::solve_any(&['?', '?', '.', '?', '?', '.', '?', '?', '?'], &[1, 1, 2]),
            8
        );
    }

    #[test]
    fn longet_pound() {
        assert_eq!(super::longest_pound(&['.', '#', '.', '#', '#']), (3, 2));
        assert_eq!(
            super::longest_pound(&['.', '#', '.', '#', '#', '#', '.', '#']),
            (3, 3)
        );

        assert_eq!(
            super::longest_pound(&['#', '.', '#', '.', '#', '#', '#']),
            (4, 3)
        );
    }

    #[test]
    fn free_solve() {
        let game = WordGame::from("?????? 1,1");
        assert_eq!(
            super::free_solve(game.word.as_slice(), game.parts.as_slice()),
            10
        );
        assert_eq!(super::free_solve(&['?', '?', '?', '?'], &[1, 2]), 1);

        let game = WordGame::from("??????? 1,1,2");
        assert_eq!(
            super::free_solve(game.word.as_slice(), game.parts.as_slice()),
            4
        );

        assert_eq!(super::free_solve(&['?', '?', '?', '?'], &[]), 1);
        assert_eq!(super::free_solve(&[], &[]), 0);
        assert_eq!(super::free_solve(&[], &[1]), 0);
    }

    #[test]
    fn moar_free_solve() {
        assert_eq!(WordGame::from("????????#???????? 2,1,4,1,2").solve(), 56);
        assert_eq!(WordGame::from("???????????? 1,2,1,1").solve(), 70);
        assert_eq!(WordGame::from("?.????#????.?????? 1,1,3,1,3").solve(), 77);
        assert_eq!(
            WordGame::from("???#???????#??????? 1,1,1,1,5,1").solve(),
            52
        );
        assert_eq!(WordGame::from("???????????..?? 2,1,1,1,1").solve(), 70);
        assert_eq!(WordGame::from("#?##?.?.?????????? 1,2,3,1").solve(), 71);
        assert_eq!(WordGame::from("??.?##.????.#?? 1,2,1,1,1").solve(), 55);
        assert_eq!(
            WordGame::from("??????????.????#??? 1,2,1,2,1,1").solve(),
            165
        );
        assert_eq!(WordGame::from("?????????????? 3,1,2").solve(), 84);
        assert_eq!(WordGame::from("??????????.????? 1,1,2,2,1").solve(), 130);
        assert_eq!(WordGame::from("????#.???###.???? 1,1,3,1").solve(), 88);
        assert_eq!(WordGame::from("???????????????? 1,4,1,1,1").solve(), 126);
    }
}
