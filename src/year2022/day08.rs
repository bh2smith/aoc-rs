use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

type TreeHeights = Vec<Vec<u32>>;
type Visibility = HashMap<Pair, HashSet<Direction>>;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Pair = (usize, usize);

fn outer_visibility_for_pair(
    heights: &TreeHeights,
    pair: &Pair,
    visibility: &mut Visibility,
) -> bool {
    // Check Up:
    let (i, j) = *pair;
    let h = heights[i][j];
    if (heights[i - 1][j] < h && visibility[&(i - 1, j)].contains(&Direction::Up))
        || h > heights[..i].iter().map(|row| row[j]).max().unwrap()
    {
        visibility.get_mut(pair).unwrap().insert(Direction::Up);
    }

    // Check Left:
    if heights[i][j - 1] < h && visibility[&(i, j - 1)].contains(&Direction::Left)
        || h > *heights[i][..j].iter().max().unwrap()
    {
        visibility.get_mut(pair).unwrap().insert(Direction::Left);
    }

    // Check Right:
    if (visibility[&(i, j + 1)].contains(&Direction::Right) && heights[i][j + 1] < heights[i][j])
        || h > *heights[i][j + 1..heights.len()].iter().max().unwrap()
    {
        visibility.get_mut(pair).unwrap().insert(Direction::Right);
    }
    // Check Down:
    if (visibility[&(i + 1, j)].contains(&Direction::Down) && heights[i + 1][j] < heights[i][j])
        || h > heights[i + 1..].iter().map(|row| row[j]).max().unwrap()
    {
        visibility.get_mut(pair).unwrap().insert(Direction::Down);
    }
    !visibility.get(pair).unwrap().is_empty()
}

fn count_directional_visibility(val: u32, mut slice: Vec<u32>) -> u32 {
    let mut count = 1;
    if val > *slice.iter().max().unwrap() {
        return slice.len() as u32;
    }
    while let Some(next) = slice.pop() {
        if next < val {
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn inner_visibility_for_pair(heights: &TreeHeights, pair: &Pair) -> u32 {
    let (i, j) = *pair;
    let h = heights[i][j];
    // Check Up:
    let up_slice = heights[..i].iter().map(|row| row[j]).collect();
    let up = count_directional_visibility(h, up_slice);

    // Check Left:
    let left_slice = heights[i][..j].to_vec();
    let left = count_directional_visibility(h, left_slice);

    // Check Right:
    let right_slice = heights[i][j + 1..heights.len()]
        .iter()
        .copied()
        .rev()
        .collect();
    let right = count_directional_visibility(h, right_slice);

    // Check Down:
    let down_slice = heights[i + 1..].iter().map(|row| row[j]).rev().collect();
    let down = count_directional_visibility(h, down_slice);
    // println!(
    //     "Pair {}, {}: UDLR {}, {}, {}, {}",
    //     i, j, up, down, left, right
    // );
    up * down * left * right
}

fn populate_outer_visibility(visibility: &mut Visibility, heights: &TreeHeights) {
    let n = heights.len();
    let m = heights[0].len();
    for i in 0..n {
        visibility.get_mut(&(i, 0)).unwrap().insert(Direction::Left);
        visibility
            .get_mut(&(i, m - 1))
            .unwrap()
            .insert(Direction::Right);
    }
    for j in 0..m {
        visibility.get_mut(&(0, j)).unwrap().insert(Direction::Up);
        visibility
            .get_mut(&(n - 1, j))
            .unwrap()
            .insert(Direction::Down);
    }
}
pub fn puzzle1(input: &str) -> i64 {
    let heights = parse_input(input);
    let dim = heights.len();
    let mut visibility: HashMap<Pair, HashSet<Direction>> = HashMap::new();
    for i in 0..dim {
        for j in 0..dim {
            visibility.insert((i, j), HashSet::new());
        }
    }
    populate_outer_visibility(&mut visibility, &heights);
    let mut answer = 4 * heights.len() - 4; // Outer Edges.
    for i in 1..dim - 1 {
        for j in 1..dim - 1 {
            if outer_visibility_for_pair(&heights, &(i, j), &mut visibility) {
                answer += 1;
                println!("Pair ({}, {}) is visible", i, j);
            }
        }
    }
    answer.try_into().unwrap()
}

pub fn puzzle2(input: &str) -> i64 {
    let heights = parse_input(input);
    let dim = heights.len();
    let mut answer = 0;
    for i in 1..dim - 1 {
        for j in 1..dim - 1 {
            let iv_pair = inner_visibility_for_pair(&heights, &(i, j));
            println!("Pair ({},{}): {}", i, j, iv_pair);
            if iv_pair > answer {
                answer = iv_pair;
            }
        }
    }
    answer.into()
}

fn parse_input(input: &str) -> TreeHeights {
    input
        .trim()
        .split('\n')
        .map(|row| {
            row.trim()
                .chars()
                .map(|st| st.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"30373
    25512
    65332
    33549
    35390";

    #[test]
    fn parse_input() {
        assert_eq!(
            super::parse_input(SAMPLE_INPUT),
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0]
            ]
        );
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 21);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 8);
    }
}
