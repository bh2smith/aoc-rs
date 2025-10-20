use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: HashSet<u32>,
    actual_numbers: HashSet<u32>,
    copies: u32,
}

fn comma_separated_as_u32<T: std::iter::FromIterator<u32>>(input: &str) -> T {
    // There were some double spaces (making the number tables look better)
    let input = input.replace("  ", " ");
    input
        .trim()
        .split(' ')
        .map(|t| t.parse::<u32>().expect("invalid number"))
        .collect::<T>()
}

impl Card {
    pub fn new(input: &str) -> Self {
        let input: Vec<&str> = input.trim().split('|').collect();
        Self {
            winning_numbers: comma_separated_as_u32::<HashSet<u32>>(input[0]),
            actual_numbers: comma_separated_as_u32::<HashSet<u32>>(input[1]),
            copies: 1,
        }
    }

    pub fn overlap_count(&self) -> usize {
        let overlap: HashSet<_> = self
            .winning_numbers
            .intersection(&self.actual_numbers)
            .collect();
        overlap.len()
    }

    pub fn increment_copies(&mut self, amount: &u32) {
        self.copies += amount;
    }
}

pub fn puzzle1(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .map(|row| {
            // Split Card number out
            let input: Vec<&str> = row.trim().split(':').collect();
            let card = Card::new(input[1]);
            let overlap = card.overlap_count();
            if overlap == 0 {
                return 0;
            }
            2u32.pow(overlap as u32 - 1)
        })
        .sum()
}

pub fn puzzle2(input: &str) -> u32 {
    // We use BtreeMap to keep the keys ordered!
    let mut cards = BTreeMap::new();
    for row in input.trim().split('\n') {
        // Split Card number out
        let input: Vec<&str> = row.trim().split(':').collect();
        let id: usize = input[0]
            .split(' ')
            .next_back()
            .expect("element exists")
            .parse()
            .expect("is number");
        let card = Card::new(input[1]);
        cards.insert(id, card);
    }
    for id in 1..(cards.len() + 1) {
        let card = cards.get(&id).expect("exists by construction");
        let card_copies = card.copies;
        for i in (id + 1)..(id + card.overlap_count() + 1) {
            let increment_card = cards.get_mut(&i).expect("exists by problem statement");
            increment_card.increment_copies(&card_copies);
        }
    }
    cards.into_values().map(|card| card.copies).sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 13);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 30);
    }
}
