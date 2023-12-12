/// After way too long: I gave up and found this elegant solution:
/// https://github.com/Philippe-Cholet/rusty-aoc/blob/main/aoc2023/day12/src/lib.rs
use std::convert::{TryFrom, TryInto};

use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Record {
    Operational,
    Damaged,
    Unknown,
}

/// Hot Springs
pub fn solver(input: &str, part_2: bool) -> Result<usize> {
    input
        .lines()
        .map(|line| {
            let (records, nbs) = line.split_once(' ').context("No space")?;
            let mut records: Vec<Record> = records.chars().map(TryInto::try_into).try_collect()?;
            let mut nbs: Vec<u8> = nbs.split(',').map(str::parse).try_collect()?;
            if part_2 {
                let len = records.len();
                records.reserve((len * 5 + 4).saturating_sub(records.capacity()));
                for _ in 0..4 {
                    records.push(Record::Unknown);
                    for i in 0..len {
                        records.push(records[i]);
                    }
                }
                nbs = itertools::repeat_n(&nbs, 5).flatten().copied().collect();
            }
            Ok((records, nbs))
        })
        .process_results(|it| {
            it.map(|(records, nums)| multiple_damaged_groups(&records, &nums))
                .sum()
        })
}

impl TryFrom<char> for Record {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Wrong record: {}", value),
        })
    }
}

fn one_damaged_group(records: &[Record], num: usize) -> usize {
    let mut dam_pos = records.iter().positions(|rec| rec == &Record::Damaged);
    match dam_pos.next() {
        // No damaged, look for at least `num` successive unknown.
        None => records
            .split(|rec| rec == &Record::Operational)
            .filter(|unknowns| unknowns.len() >= num)
            .map(|unknowns| unknowns.len() - num + 1)
            .sum(),
        Some(first) => {
            let last = dam_pos.next_back().unwrap_or(first);
            if last - first >= num {
                return 0; // damaged records are too far apart
            }
            records
                .split(|rec| rec == &Record::Operational)
                .filter(|window| window.contains(&Record::Damaged))
                .exactly_one()
                .map_or(0, |window| {
                    // I should have some explicit formula for this...
                    window
                        .windows(num)
                        .enumerate()
                        .filter(|(i, _)| {
                            !window[..*i].contains(&Record::Damaged)
                                && !window[*i + num..].contains(&Record::Damaged)
                        })
                        .count()
                })
        }
    }
}

// Some divide-and-conquer algorithm
fn multiple_damaged_groups(records: &[Record], nums: &[u8]) -> usize {
    let n = nums.len();
    if n == 0 {
        return (!records.contains(&Record::Damaged)).into();
    }
    if n == 1 {
        return one_damaged_group(records, nums[0] as usize);
    }
    let num_idx = n / 2;
    let num = nums[num_idx] as usize;
    // Find all possible places for `num`, then split the task in two smaller tasks.
    records
        .windows(num)
        .positions(|window| !window.contains(&Record::Operational))
        .filter(|&pos| {
            // Positions around must not be damaged or the window would be too large!
            pos.checked_sub(1)
                .map_or(true, |i| records[i] != Record::Damaged)
                && (pos + num < records.len())
                    .then_some(pos + num)
                    .map_or(true, |i| records[i] != Record::Damaged)
        })
        .map(|pos| {
            let c0 = (num_idx != 0).then(|| {
                multiple_damaged_groups(&records[..pos.saturating_sub(1)], &nums[..num_idx])
            });
            let c1 = (c0 != Some(0)).then(|| {
                multiple_damaged_groups(
                    &records[(pos + num + 1).min(records.len())..],
                    &nums[num_idx + 1..],
                )
            });
            match (c0, c1) {
                (None, None) => 0,
                (Some(c), None) | (None, Some(c)) => c,
                (Some(c0), Some(c1)) => c0 * c1,
            }
        })
        .sum()
}

pub fn puzzle1(input: &str) -> usize {
    solver(input, false).unwrap()
}

pub fn puzzle2(input: &str) -> usize {
    solver(input, true).unwrap()
}
