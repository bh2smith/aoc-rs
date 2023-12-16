use itertools::Itertools;
use num_integer::Integer;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Direction {
    right: String,
    left: String,
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, Direction>) {
    // Parse input
    let mut input = input.trim().split('\n');
    let pattern = input.next().unwrap().chars().collect_vec();
    input.next();
    let mut map = HashMap::new();
    for next in input {
        let line = next.split_whitespace().collect_vec();
        map.insert(
            line[0],
            Direction {
                left: line[2][1..4].to_string(),
                right: line[3][..3].to_string(),
            },
        );
    }
    (pattern, map)
}

fn starting_points(places: Vec<&str>) -> Vec<&str> {
    places
        .into_iter()
        .filter(|p| p.chars().nth(2).unwrap() == 'A')
        .collect()
}

fn single_run(
    start: &str,
    end_char: char,
    pattern: Vec<char>,
    map: HashMap<&str, Direction>,
) -> usize {
    let mut pos = start;
    let mut step = 0;
    while pos.chars().nth(2) != Some(end_char) {
        pos = match pattern[step % pattern.len()] {
            'L' => &map[pos].left,
            'R' => &map[pos].right,
            _ => unreachable!("no other directions"),
        };
        step += 1;
    }
    step
}

pub fn puzzle1(input: &str) -> usize {
    let (pattern, map) = parse_input(input);
    single_run("AAA", 'Z', pattern, map)
}

pub fn puzzle2(input: &str) -> usize {
    let (pattern, map) = parse_input(input);
    let positions = starting_points(map.keys().cloned().collect_vec());
    let mut results = vec![];
    for pos in positions {
        let res = single_run(pos, 'Z', pattern.clone(), map.clone());
        // println!("Single run for {pos}: {res}");
        results.push(res);
    }
    // LCM of seprate results (CRT)
    results.iter().cloned().fold(1, |acc, x| acc.lcm(&x))
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 6);
    }

    const SAMPLE_INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT_2), 6);
    }
}
