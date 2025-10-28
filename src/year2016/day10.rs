use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    static ref VALUE_PATTERN: Regex =
        Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    static ref GIVE_PATTERN: Regex =
        Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$").unwrap();
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Receiver {
    Bot(u32),
    Bin(u32),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Instruction {
    low: Receiver,
    high: Receiver,
}

impl Receiver {
    fn new(who: &str, num: &str) -> Self {
        let x = num.parse().unwrap();
        match who {
            "bot" => Self::Bot(x),
            "output" => Self::Bin(x),
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Default, Debug)]
struct Bot {
    id: u32,
    chips: HashSet<u32>,
    op: Option<Instruction>,
}

impl Bot {
    fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
    fn can_act(&self) -> bool {
        self.chips.len() > 1
    }
}

fn parse_input(input: &str) -> HashMap<u32, Bot> {
    let mut bots: HashMap<u32, Bot> = HashMap::new();
    for line in input.trim().lines() {
        if let Some(c) = VALUE_PATTERN.captures(line.trim()) {
            let b: u32 = c[2].parse().unwrap();
            let chip: u32 = c[1].parse().unwrap();
            bots.entry(b).or_insert(Bot::new(b)).chips.insert(chip);
        } else {
            let c = GIVE_PATTERN.captures(line.trim()).unwrap();
            let b: u32 = c[1].parse().unwrap();
            bots.entry(b).or_insert(Bot::new(b)).op = Some(Instruction {
                low: Receiver::new(&c[2], &c[3]),
                high: Receiver::new(&c[4], &c[5]),
            });
        }
    }
    bots
}

fn act_one(
    id: u32,
    bots: &mut HashMap<u32, Bot>,
    bins: &mut HashMap<u32, HashSet<u32>>,
    key: (u32, u32),
) -> Option<u32> {
    let mut me = match bots.remove(&id) {
        Some(b) => b,
        None => return None, // or panic if invariant should hold
    };
    if me.chips.len() < 2 {
        // put it back unchanged
        bots.insert(id, me);
        return None;
    }
    let low = *me.chips.iter().min().unwrap();
    let high = *me.chips.iter().max().unwrap();
    me.chips.clear();
    let instruction = me.op.as_ref().expect("bot requires instructions");

    match instruction.low {
        Receiver::Bin(bin) => bins.entry(bin).or_default().insert(low),
        Receiver::Bot(bot) => bots.get_mut(&bot).expect("bot expected").chips.insert(low),
    };
    match instruction.high {
        Receiver::Bin(bin) => bins.entry(bin).or_default().insert(high),
        Receiver::Bot(bot) => bots.get_mut(&bot).expect("bot expected").chips.insert(high),
    };
    let winner = ((low, high) == key).then_some(me.id);

    bots.insert(id, me);

    winner
}

fn solve(input: &str) -> (u32, u32) {
    // let key = (2, 5);
    let key = (17, 61);
    let mut bots = parse_input(input);
    let mut bins = HashMap::new();
    let mut winner = None;
    // keep stepping while someone can act
    while bots.values().any(|b| b.can_act()) {
        // collect ready IDs first (avoid holding an immutable borrow while mutating)
        let ready: Vec<u32> = bots
            .iter()
            .filter_map(|(&id, b)| b.can_act().then_some(id))
            .collect();

        if ready.is_empty() {
            break;
        }

        for id in ready {
            if let Some(w) = act_one(id, &mut bots, &mut bins, key) {
                winner = Some(w);
            }
        }
    }
    // Assumes all have one entry.
    let x = *bins.get(&0).unwrap().iter().max().unwrap();
    let y = *bins.get(&1).unwrap().iter().max().unwrap();
    let z = *bins.get(&2).unwrap().iter().max().unwrap();
    (winner.expect("must have a winner"), x * y * z)
}

pub fn puzzle1(input: &str) -> u32 {
    solve(input).0
}

pub fn puzzle2(input: &str) -> u32 {
    solve(input).1
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 2);
    }
}
