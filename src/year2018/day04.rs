use crate::util;
use chrono::{DateTime, TimeZone, Timelike, Utc};
use failure::{self, Error};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

struct Record {
    timestamp: DateTime<Utc>,
    kind: RecordKind,
}

enum RecordKind {
    Shift(i64),
    Sleep,
    Wake,
}

lazy_static! {
    static ref RECORD_PATTERN: Regex =
        Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] ((Guard #(\d+) begins shift)|(falls asleep)|(wakes up))").unwrap();
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = RECORD_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match record pattern"))?;

        let y = c[1].parse()?;
        let m = c[2].parse()?;
        let d = c[3].parse()?;
        let h = c[4].parse()?;
        let mm = c[5].parse()?;

        Ok(Record {
            timestamp: Utc.ymd(y, m, d).and_hms(h, mm, 0),
            kind: match &c[6] {
                "falls asleep" => RecordKind::Sleep,
                "wakes up" => RecordKind::Wake,
                _ => RecordKind::Shift(c[8].parse()?),
            },
        })
    }
}

fn parse_sleep_times<'a>(input: &'a str) -> impl 'a + Iterator<Item = (i64, u32, u32)> {
    let mut records = util::parse::<Record>(input).collect::<Vec<_>>();
    records.sort_unstable_by_key(|r| r.timestamp);

    let mut current_id = None;
    let mut sleep_start = None;
    records.into_iter().filter_map(move |r| match &r.kind {
        RecordKind::Shift(id) => {
            current_id = Some(*id);
            None
        }
        RecordKind::Sleep => {
            sleep_start = Some(r.timestamp.minute());
            None
        }
        RecordKind::Wake => {
            let sleep_end = r.timestamp.minute();
            Some((current_id.unwrap(), sleep_start.unwrap(), sleep_end))
        }
    })
}

pub fn puzzle1(input: &str) -> i64 {
    let sleep_times = parse_sleep_times(input).collect::<Vec<_>>();

    let (_, (max_id, _)) = sleep_times.iter().cloned().fold(
        (HashMap::new(), (0, 0)),
        |(mut totals, max), (id, start, end)| {
            let total = {
                let total = totals.entry(id).or_insert(0u32);
                *total += end - start;
                *total
            };

            if total > max.1 {
                (totals, (id, total))
            } else {
                (totals, max)
            }
        },
    );

    let (_, (max_min, _)) = sleep_times
        .iter()
        .cloned()
        .filter(|(id, _, _)| *id == max_id)
        .fold(
            ([0u8; 60], (0, 0)),
            |(mut counts, mut max), (_, start, end)| {
                for i in start..end {
                    let count = &mut counts[i as usize];

                    *count += 1;
                    max = if *count > max.1 { (i, *count) } else { max }
                }

                (counts, max)
            },
        );

    max_id * (max_min as i64)
}

pub fn puzzle2(input: &str) -> i64 {
    let (_, (max_id, max_min, _)) = parse_sleep_times(input).fold(
        (HashMap::new(), (0, 0, 0)),
        |(mut counts_map, mut max), (id, start, end)| {
            {
                let counts = counts_map.entry(id).or_insert([0u8; 60]);
                for i in start..end {
                    let count = &mut counts[i as usize];

                    *count += 1;
                    max = if *count > max.2 { (id, i, *count) } else { max }
                }
            }
            (counts_map, max)
        },
    );

    max_id * (max_min as i64)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        [1518-11-01 00:00] Guard #10 begins shift
        [1518-11-01 00:05] falls asleep
        [1518-11-01 00:25] wakes up
        [1518-11-01 00:30] falls asleep
        [1518-11-01 00:55] wakes up
        [1518-11-01 23:58] Guard #99 begins shift
        [1518-11-02 00:40] falls asleep
        [1518-11-02 00:50] wakes up
        [1518-11-03 00:05] Guard #10 begins shift
        [1518-11-03 00:24] falls asleep
        [1518-11-03 00:29] wakes up
        [1518-11-04 00:02] Guard #99 begins shift
        [1518-11-04 00:36] falls asleep
        [1518-11-04 00:46] wakes up
        [1518-11-05 00:03] Guard #99 begins shift
        [1518-11-05 00:45] falls asleep
        [1518-11-05 00:55] wakes up
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(INPUT), 240);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(INPUT), 4455);
    }
}
