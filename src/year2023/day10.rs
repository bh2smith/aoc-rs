use colored::*;
use std::collections::{HashMap, HashSet};

use crate::util::Point;

fn openings(c: char) -> Vec<Point> {
    match c {
        '|' => vec![Point::new(0, -1), Point::new(0, 1)],
        '-' => vec![Point::new(-1, 0), Point::new(1, 0)],
        'L' => vec![Point::new(0, -1), Point::new(1, 0)],
        'J' => vec![Point::new(0, -1), Point::new(-1, 0)],
        '7' => vec![Point::new(0, 1), Point::new(-1, 0)],
        _ => vec![Point::new(0, 1), Point::new(1, 0)],
    }
}

fn find_loop(
    map: &HashMap<Point, char>,
    prev: Point,
    cur: Point,
    distance: usize,
    loop_points: &mut HashSet<Point>,
) -> usize {
    // Check to see if we've found the start.
    if map[&cur] == 'S' {
        let distance = distance + 1;
        match distance % 2 {
            0 => return distance / 2,
            _ => return distance / 2 + 1,
        }
    }

    // Each point on the map only has two connecting sides. If we know
    // where we came from, there can only be one other place to go.
    let nexts = openings(map[&cur]);
    let next = nexts.iter().find(|&&p| p != prev - cur).unwrap();
    loop_points.insert(cur + *next);
    find_loop(map, cur, cur + *next, distance + 1, loop_points)
}

fn display(c: char) -> &'static str {
    match c {
        'S' => "S",
        '|' => "│",
        '-' => "─",
        'L' => "└",
        'J' => "┘",
        '7' => "┐",
        'F' => "┌",
        _ => "·",
    }
}

fn in_loop_original(map: &HashMap<Point, char>, loop_points: &HashSet<Point>) -> HashSet<Point> {
    let dots = map
        .iter()
        .filter(|(p, &c)| c == '.' || !loop_points.contains(p))
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    let rights = HashSet::from(['-', 'L', 'F']);
    let lefts = HashSet::from(['-', 'J', '7']);
    let mut inside = HashSet::new();
    for dot in dots {
        let left = loop_points
            .iter()
            .filter(|p| p.x == dot.x && p.y < dot.y && lefts.contains(&map[p]))
            .count();
        let right = loop_points
            .iter()
            .filter(|p| p.x == dot.x && p.y < dot.y && rights.contains(&map[p]))
            .count();
        if left.min(right) % 2 == 1 {
            inside.insert(*dot);
        }
    }
    inside
}

fn in_loop_faster(map: &HashMap<Point, char>, loop_points: &HashSet<Point>) -> HashSet<Point> {
    let mut inside = HashSet::new();
    let max_x = map.keys().map(|p| p.x).max().unwrap();
    let max_y = map.keys().map(|p| p.y).max().unwrap();

    for x in 0..=max_x {
        let mut left = 0;
        let mut right = 0;
        for y in 0..=max_y {
            match loop_points.contains(&Point::new(x, y)) {
                true => match map[&Point::new(x, y)] {
                    '-' => {
                        left += 1;
                        right += 1
                    }
                    'L' => right += 1,
                    'F' => right += 1,
                    'J' => left += 1,
                    '7' => left += 1,
                    _ => (),
                },
                false => {
                    if left.min(right) % 2 == 1 {
                        inside.insert(Point::new(x, y));
                    }
                }
            }
        }
    }
    inside
}

fn parse_input(input: &str) -> HashMap<Point, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x as i64, y as i64), c))
        })
        .collect::<HashMap<Point, char>>()
}

pub fn puzzle1(input: &str) -> usize {
    let map = parse_input(input);
    // P1: Looking at S for the input, it's clear that the loop is
    // created above and below it.
    let start = map.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let next = *start + Point::new(0, 1);
    let mut loop_points = HashSet::from([*start, next]);
    find_loop(&map, *start, next, 0, &mut loop_points)
}

pub fn puzzle2(input: &str) -> usize {
    let map = parse_input(input);
    // P1: Looking at S for the input, it's clear that the loop is
    // created above and below it.
    let start = map.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let next = *start + Point::new(0, 1);
    let mut loop_points = HashSet::from([*start, next]);
    find_loop(&map, *start, next, 0, &mut loop_points);
    // P2: find all the non-loop point and see if they are inside the
    // loop. https://en.wikipedia.org/wiki/Point_in_polygon
    let now = std::time::Instant::now();
    let inside = in_loop_original(&map, &loop_points);
    println!("p2: {} ({:?})", inside.len(), now.elapsed());

    let now = std::time::Instant::now();
    let inside = in_loop_faster(&map, &loop_points);
    println!("p2+: {} ({:?})", inside.len(), now.elapsed());

    // Print the map to the screen.
    let max_x = map.keys().map(|p| p.x).max().unwrap();
    let max_y = map.keys().map(|p| p.y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = map[&Point::new(x, y)];
            let l = loop_points.contains(&Point::new(x, y));
            match l {
                true => print!("{}", display(c).red().bold()),
                false => match inside.contains(&Point::new(x, y)) {
                    true => print!("{}", ".".green().bold()),
                    false => print!("{}", ".".yellow().bold()),
                },
            };
        }
        println!();
    }
    inside.len()
}

#[cfg(test)]
mod tests {

    const BABY_INPUT: &str = "
.....
.S-7.
.|.|.
.L-J.
.....";

    const MEDIUM_INPUT: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const SAMPLE_INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(BABY_INPUT), 4);
        assert_eq!(super::puzzle1(MEDIUM_INPUT), 8);
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 8);
    }

    const SAMPLE_LOOP: &str = "
...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....";

    const SAMPLE_LOOP_2: &str = "
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";

    const SAMPLE_3: &str = "
OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO";

    const SAMPLE_4: &str = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn puzzle2() {
        // assert_eq!(super::puzzle2(SAMPLE_LOOP), 4);
        assert_eq!(super::puzzle2(SAMPLE_LOOP_2), 4);
        assert_eq!(super::puzzle2(SAMPLE_4), 10);
        assert_eq!(super::puzzle2(SAMPLE_3), 8);
    }
}
