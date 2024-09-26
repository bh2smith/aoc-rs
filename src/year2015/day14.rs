use std::collections::HashMap;

use itertools::Itertools;

const SECONDS: u32 = 2503;
#[derive(Default, PartialEq, Eq, Hash, Debug)]
struct Reindeer {
    name: String,
    fly: u32,
    rest: u32,
    speed: u32,
}
impl Reindeer {
    fn from(row: &str) -> Self {
        let items = row.split_whitespace().collect_vec();

        Self {
            name: items[0].to_string(),
            speed: items[3].parse().unwrap(),
            fly: items[6].parse().unwrap(),
            rest: items[13].parse().unwrap(),
        }
    }
    fn position_at_second(&self, second: u32) -> u32 {
        let Reindeer {
            name: _,
            fly,
            rest,
            speed,
        } = self;
        if second <= *fly {
            return speed * second;
        }
        let k = second / (fly + rest);
        let mut extra = 0;
        if k * (fly + rest) < second {
            let bonus_time = (second - (k * (fly + rest))).min(*fly);
            extra = bonus_time * speed;
        }
        k * (fly * speed) + extra
    }
}

pub fn puzzle1(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .map(|row| {
            let reindeer = Reindeer::from(row);
            reindeer.position_at_second(SECONDS)
        })
        .max()
        .unwrap()
}

pub fn puzzle2(input: &str) -> u32 {
    let reindeer = input.trim().split('\n').map(Reindeer::from);
    let mut points = HashMap::new();
    for second in 1..SECONDS + 1 {
        let leading_position = reindeer
            .clone()
            .map(|r| r.position_at_second(second))
            .max()
            .unwrap();
        for r in reindeer.clone() {
            if r.position_at_second(second) == leading_position {
                *points.entry(r).or_insert(0) += 1;
            }
        }
    }
    points.into_values().max().unwrap()
}
