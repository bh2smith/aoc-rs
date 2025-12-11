use itertools::Itertools;

use crate::util::transpose;

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
}

struct Problem {
    nums: Vec<u64>,
    op: Op,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.op {
            Op::Add => self.nums.iter().copied().sum(),
            Op::Mul => self.nums.iter().copied().product(),
        }
    }
}

fn parse_1<'a, I>(input: I) -> Vec<Vec<u64>>
where
    I: IntoIterator<Item = &'a str>,
{
    input
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|t| t.parse::<u64>().ok()) // ignore parse errors
                .collect::<Vec<u64>>()
        })
        .collect()
}

fn parse_2(input: Vec<&str>) -> Vec<Vec<u64>> {
    let char_vec = input.into_iter().map(|x| x.chars().collect()).collect();
    let x: Vec<String> = transpose(char_vec)
        .iter()
        .map(|x| x.iter().collect())
        .collect();
    parse_1(x.iter().map(|s| s.as_str()))
}

fn group_by_empty(rows: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut groups = Vec::new();
    let mut current = Vec::new();

    for row in rows {
        if row.is_empty() {
            if !current.is_empty() {
                groups.push(current);
                current = Vec::new();
            }
        } else {
            current.extend(row); // append digits or numbers
        }
    }

    if !current.is_empty() {
        groups.push(current);
    }

    groups
}

fn parse(input: &str, part_2: bool) -> Vec<Problem> {
    let mut lines: Vec<&str> = input.lines().collect();

    // Take the last line separately
    let last_line = lines.pop().unwrap_or("");
    let ops = last_line
        .split_whitespace()
        .map(|x| match x {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("Invalid operation input"),
        })
        .collect_vec();

    // Parse non-op lines
    let columns = if part_2 {
        group_by_empty(parse_2(lines))
    } else {
        transpose(parse_1(lines))
    };

    assert_eq!(columns.len(), ops.len());
    (0..columns.len())
        .map(|i| Problem {
            nums: columns[i].clone(),
            op: ops[i],
        })
        .collect()
}

pub fn puzzle1(input: &str) -> u64 {
    parse(input, false).iter().map(|p| p.solve()).sum()
}

pub fn puzzle2(input: &str) -> u64 {
    parse(input, true).iter().map(|p| p.solve()).sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 4277556);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 3263827);
    }
}
