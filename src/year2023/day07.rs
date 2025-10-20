use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct PokerHand {
    cards: [char; 5],
    bid: usize,
    wild_jack: bool,
}

pub fn card_value(ch: char, wild_jack: bool) -> usize {
    if wild_jack {
        return "J23456789TQKA".find(ch).unwrap();
    }
    "23456789TJQKA".find(ch).unwrap() + 2
}

impl PokerHand {
    pub fn from_row(row: &str, wild_jack: bool) -> Self {
        let split = row.split_whitespace().collect_vec();
        let chars = split[0].chars().collect_vec();
        Self {
            cards: [chars[0], chars[1], chars[2], chars[3], chars[4]],
            bid: split[1].parse().unwrap(),
            wild_jack,
        }
    }

    pub fn jack_count(&self) -> usize {
        *self.entry_count().get(&'J').unwrap_or(&0)
    }

    pub fn two_kind(&self) -> bool {
        if self.wild_jack && self.jack_count() > 0 {
            return true;
        }
        self.entry_count().values().contains(&2)
    }

    pub fn three_kind(&self) -> bool {
        let count = self.entry_count();
        if self.wild_jack && self.jack_count() > 0 {
            if self.jack_count() > 1 {
                return true;
            }
            let distinct_other_cards = count.keys().len() - 1;
            return distinct_other_cards <= 3;
        }
        count.values().contains(&3)
    }

    pub fn four_kind(&self) -> bool {
        let count = self.entry_count();
        let jacks = self.jack_count();
        if self.wild_jack && self.jack_count() > 0 {
            if jacks > 2 {
                return true;
            }
            let distinct_other_cards = count.keys().len() - 1;
            if jacks == 2 {
                return distinct_other_cards <= 2;
            }
            return count.values().contains(&3);
        }
        count.values().contains(&4)
    }

    pub fn five_kind(&self) -> bool {
        let count = self.entry_count();
        if self.wild_jack && self.jack_count() > 0 {
            let distinct_other_cards = count.keys().len() - 1;
            return distinct_other_cards <= 1;
        }
        count.values().contains(&5)
    }

    // pub fn n_of_a_kind(&self, n: usize) -> bool {
    //     // This is BROKEN for four of a kind!
    //     let count = self.entry_count();
    //     let jacks = self.jack_count();
    //     if self.wild_jack && jacks > 0 {
    //         if n <= 3 && jacks > 1 {
    //             return true;
    //         }
    //         // includes jack.
    //         let distinct_other_cards = count.keys().len() - 1;
    //         return match n {
    //             5 => distinct_other_cards <= 1,
    //             4 => distinct_other_cards <= 2,
    //             3 => distinct_other_cards <= 3,
    //             2 => distinct_other_cards <= 4,
    //             _ => panic!("shouldn't ask"),
    //         };
    //     }
    //     count.values().contains(&n)
    // }

    pub fn full_house(&self) -> bool {
        let count = self.entry_count();
        let distinct_other_cards = count.keys().len() - 1;
        let jacks = self.jack_count();
        if self.wild_jack && jacks > 0 {
            if jacks >= 3 {
                return true;
            }
            if jacks == 2 {
                return distinct_other_cards <= 2;
            }
            return distinct_other_cards <= 2 && distinct_other_cards > 1;
        }
        count.values().contains(&3) && count.values().contains(&2)
    }

    pub fn two_pair(&self) -> bool {
        let count = self.entry_count();
        let jacks = self.jack_count();
        if self.wild_jack && jacks > 0 {
            if jacks > 1 {
                return true;
            } else {
                let distinct_other_cards = count.keys().len() - 1;
                return distinct_other_cards < 4 && distinct_other_cards > 1;
            }
        }
        let distinct_cards = count.keys().len();
        distinct_cards <= 3 && distinct_cards > 1 && count.values().contains(&2)
    }

    pub fn entry_count(&self) -> HashMap<char, usize> {
        let mut char_count = HashMap::new();
        for &c in &self.cards {
            let count = char_count.entry(c).or_insert(0);
            *count += 1;
        }
        char_count
    }

    pub fn card_strengths(&self) -> (usize, usize, usize, usize, usize) {
        let mut x = self
            .cards
            .iter()
            .map(|ch| card_value(*ch, self.wild_jack))
            .take(5);
        (
            x.next().unwrap(),
            x.next().unwrap(),
            x.next().unwrap(),
            x.next().unwrap(),
            x.next().unwrap(),
        )
    }

    pub fn value(&self) -> (usize, (usize, usize, usize, usize, usize)) {
        let strength = self.card_strengths();

        if self.five_kind() {
            return (6, strength);
        }
        if self.four_kind() {
            return (5, strength);
        }
        if self.full_house() {
            return (4, strength);
        }
        if self.three_kind() {
            return (3, strength);
        }
        if self.two_pair() {
            return (2, strength);
        }
        if self.two_kind() {
            return (1, strength);
        }
        (0, strength)
    }
}

fn run_game(input: &str, wild_jack: bool) -> usize {
    let mut hands = input
        .trim()
        .split('\n')
        .map(|row| PokerHand::from_row(row, wild_jack))
        .collect_vec();
    hands.sort_by_key(|hand| hand.value());

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| {
            // println!("{:?} - value {:?}", hand, hand.alt_value());
            (rank + 1) * hand.bid
        })
        .sum()
}

pub fn puzzle1(input: &str) -> usize {
    run_game(input, false)
}

pub fn puzzle2(input: &str) -> usize {
    run_game(input, true)
}

#[cfg(test)]
mod tests {
    use super::PokerHand;

    const SAMPLE_INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn two_kind() {
        // No
        assert!(!PokerHand::from_row("23456 1", true).two_kind());
        // Yes
        assert!(PokerHand::from_row("2345J 1", true).two_kind());
        assert!(PokerHand::from_row("234JJ 1", true).two_kind());
        assert!(PokerHand::from_row("23JJJ 1", true).two_kind());
        assert!(PokerHand::from_row("2JJJJ 1", true).two_kind());
        assert!(PokerHand::from_row("2234J 1", true).two_kind());
    }

    #[test]
    fn three_kind() {
        // No
        assert!(!PokerHand::from_row("23456 1", true).three_kind());
        assert!(!PokerHand::from_row("2345J 1", true).three_kind());
        // Yes
        assert!(PokerHand::from_row("234JJ 1", true).three_kind());
        assert!(PokerHand::from_row("23JJJ 1", true).three_kind());
        assert!(PokerHand::from_row("2JJJJ 1", true).three_kind());
        assert!(PokerHand::from_row("2234J 1", true).three_kind());
        assert!(PokerHand::from_row("223JJ 1", true).three_kind());
    }

    #[test]
    fn four_kind() {
        // No
        assert!(!PokerHand::from_row("23456 1", true).four_kind());
        assert!(!PokerHand::from_row("2345J 1", true).four_kind());
        assert!(!PokerHand::from_row("234JJ 1", true).four_kind());
        assert!(!PokerHand::from_row("2234J 1", true).four_kind());
        // Yes
        assert!(PokerHand::from_row("23JJJ 1", true).four_kind());
        assert!(PokerHand::from_row("2JJJJ 1", true).four_kind());
        assert!(PokerHand::from_row("223JJ 1", true).four_kind());
        assert!(PokerHand::from_row("22JJJ 1", true).four_kind());
        assert!(PokerHand::from_row("2224J 1", true).four_kind());

        // TODO: The one that was BROKEN!
        // assert!(PokerHand::from_row("2244J 1", true).four_kind());
    }

    #[test]
    fn five_kind() {
        // No
        assert!(!PokerHand::from_row("23456 1", true).five_kind());
        assert!(!PokerHand::from_row("2345J 1", true).five_kind());
        assert!(!PokerHand::from_row("234JJ 1", true).five_kind());
        assert!(!PokerHand::from_row("23JJJ 1", true).five_kind());
        assert!(!PokerHand::from_row("2234J 1", true).five_kind());
        assert!(!PokerHand::from_row("223JJ 1", true).five_kind());
        assert!(!PokerHand::from_row("2224J 1", true).five_kind());
        // Yes
        assert!(PokerHand::from_row("2JJJJ 1", true).five_kind());
        assert!(PokerHand::from_row("2JJJJ 1", true).five_kind());
        assert!(PokerHand::from_row("222JJ 1", true).five_kind());
    }

    #[test]
    fn two_pair() {
        // No Jack:
        assert!(!PokerHand::from_row("23456 1", true).two_pair());
        assert!(!PokerHand::from_row("23455 1", true).two_pair());
        assert!(!PokerHand::from_row("23555 1", true).two_pair());
        assert!(PokerHand::from_row("22555 1", true).two_pair());
        assert!(PokerHand::from_row("22344 1", true).two_pair());
        // One Jack
        assert!(!PokerHand::from_row("2345J 1", true).two_pair());
        assert!(!PokerHand::from_row("2222J 1", true).two_pair());
        assert!(PokerHand::from_row("2223J 1", true).two_pair());
        assert!(PokerHand::from_row("2244J 1", true).two_pair());
        assert!(PokerHand::from_row("2244J 1", true).two_pair());

        // Yes
        assert!(PokerHand::from_row("234JJ 1", true).two_pair());
        assert!(PokerHand::from_row("23JJJ 1", true).two_pair());
        assert!(PokerHand::from_row("2JJJJ 1", true).two_pair());
        assert!(PokerHand::from_row("JJJJJ 1", true).two_pair());
        assert!(PokerHand::from_row("2234J 1", true).two_pair());
        assert!(PokerHand::from_row("2224J 1", true).two_pair());
    }

    #[test]
    fn full_house() {
        // No Jack:
        assert!(!PokerHand::from_row("23456 1", true).full_house());
        assert!(!PokerHand::from_row("23455 1", true).full_house());
        assert!(!PokerHand::from_row("23444 1", true).full_house());
        assert!(!PokerHand::from_row("23333 1", true).full_house());
        assert!(!PokerHand::from_row("22222 1", true).full_house());

        assert!(PokerHand::from_row("22333 1", true).full_house());
        assert!(!PokerHand::from_row("22334 1", true).full_house());
        assert!(!PokerHand::from_row("22334 1", true).full_house());
        // 1 Jack
        assert!(!PokerHand::from_row("2345J 1", true).full_house());
        assert!(!PokerHand::from_row("2344J 1", true).full_house());
        assert!(PokerHand::from_row("2333J 1", true).full_house());
        assert!(PokerHand::from_row("2233J 1", true).full_house());
        assert!(!PokerHand::from_row("2222J 1", true).full_house());
        // 2 jacks
        assert!(!PokerHand::from_row("234JJ 1", true).full_house());
        assert!(PokerHand::from_row("233JJ 1", true).full_house());
        assert!(PokerHand::from_row("222JJ 1", true).full_house());
        assert!(PokerHand::from_row("223JJ 1", true).full_house());
        assert!(!PokerHand::from_row("2222J 1", true).full_house());
        // 3 jacks
        assert!(PokerHand::from_row("23JJJ 1", true).full_house());
        assert!(PokerHand::from_row("22JJJ 1", true).full_house());
        assert!(PokerHand::from_row("2JJJJ 1", true).full_house());
        assert!(PokerHand::from_row("JJJJJ 1", true).full_house());
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 6440);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 5905);
    }
}
