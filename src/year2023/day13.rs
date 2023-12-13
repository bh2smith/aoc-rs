fn find_reflection(pattern: &Vec<Vec<char>>) -> u32 {
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();
    for split_col in 1..num_cols {
        let left_range = (0..split_col).rev();
        let right_range = split_col..num_cols;
        if left_range.zip(right_range).all(|column_pair| {
            let left = pattern
                .iter()
                .map(|row| row[column_pair.0])
                .collect::<Vec<char>>();
            let right = pattern
                .iter()
                .map(|row| row[column_pair.1])
                .collect::<Vec<char>>();
            left == right
        }) {
            return split_col as u32;
        }
    }
    for split_row in 1..num_rows {
        let top_range = (0..split_row).rev();
        let bottom_range = split_row..num_rows;
        if top_range.zip(bottom_range).all(|row_pair| {
            let top = &pattern[row_pair.0];
            let bottom = &pattern[row_pair.1];
            top == bottom
        }) {
            return (split_row as u32) * 100;
        }
    }
    0
}

fn count_diff(s1: &[char], s2: &[char]) -> u32 {
    let mut count = 0;
    for (ch1, ch2) in s1.iter().zip(s2.iter()) {
        if ch1 != ch2 {
            count += 1;
        }
    }
    count
}

fn find_reflection2(pattern: &Vec<Vec<char>>) -> u32 {
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();
    for split_col in 1..num_cols {
        let left_range = (0..split_col).rev();
        let right_range = split_col..num_cols;
        let mut diff_count = 0;
        for column_pair in left_range.zip(right_range) {
            let left = pattern
                .iter()
                .map(|row| row[column_pair.0])
                .collect::<Vec<char>>();
            let right = pattern
                .iter()
                .map(|row| row[column_pair.1])
                .collect::<Vec<char>>();
            diff_count += count_diff(&left, &right);
        }
        if diff_count == 1 {
            return split_col as u32;
        }
    }
    for split_row in 1..num_rows {
        let top_range = (0..split_row).rev();
        let bottom_range = split_row..num_rows;
        let mut diff_count = 0;
        for row_pair in top_range.zip(bottom_range) {
            let top = &pattern[row_pair.0];
            let bottom = &pattern[row_pair.1];
            diff_count += count_diff(top, bottom);
        }
        if diff_count == 1 {
            return (split_row as u32) * 100;
        }
    }
    0
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut patterns: Vec<Vec<Vec<char>>> = vec![];
    let mut current_pattern: Vec<Vec<char>> = vec![];
    for line in input.trim().split('\n') {
        if line.is_empty() {
            patterns.push(current_pattern);
            current_pattern = vec![];
        } else {
            let row: Vec<char> = line.chars().collect();
            current_pattern.push(row);
        }
    }
    patterns.push(current_pattern);
    patterns
}

pub fn puzzle1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let patterns = parse_input(input);
    for pattern in patterns {
        sum += find_reflection(&pattern);
    }
    sum
}

pub fn puzzle2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let patterns = parse_input(input);
    for pattern in patterns {
        sum += find_reflection2(&pattern);
    }
    sum
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 405);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 400);
    }
}
