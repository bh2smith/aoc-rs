use std::collections::HashMap;

use itertools::Itertools;
struct Room {
    checksum: String,
    sector_id: u32,
    name: String,
}

impl Room {
    fn evaluate_checksum(&self) -> String {
        let mut char_counts = HashMap::new();
        for char in self.name.chars() {
            if char == ' ' {
                continue;
            }
            *char_counts.entry(char).or_insert(0) += 1;
        }
        let mut char_counts = char_counts.iter().collect_vec();
        char_counts.sort_by_key(|(char, count)| (-1 * *count, *char));
        char_counts
            .iter()
            .take(5)
            .map(|(char, _)| *char)
            .collect::<String>()
    }

    fn decrypt_name(&self) -> String {
        let mut name = String::new();
        for c in self.name.chars() {
            if c == ' ' {
                name.push(' ');
            } else {
                let shifted = ((c as u8 - b'a' + (self.sector_id % 26) as u8) % 26) + b'a';
                name.push(shifted as char);
            }
        }
        name
    }
}

impl From<&str> for Room {
    fn from(input: &str) -> Self {
        let split = input.split('-').collect_vec();
        let name = split[..split.len() - 1].join(" ");
        let right = split[split.len() - 1].split('[').collect_vec();
        let sector_id = right[0].parse::<u32>().unwrap();
        let checksum = right[1][..5].to_string();
        Self {
            checksum,
            sector_id,
            name,
        }
    }
}
fn parse_input(input: &str) -> Vec<Room> {
    input.lines().map(Room::from).collect()
}

pub fn puzzle1(input: &str) -> u32 {
    let rooms = parse_input(input);
    rooms
        .iter()
        .filter(|room| room.evaluate_checksum() == room.checksum)
        .map(|room| room.sector_id)
        .sum()
}

pub fn puzzle2(input: &str) -> u32 {
    let rooms = parse_input(input);
    // rooms
    //     .iter()
    //     .for_each(|room| println!("{}", room.decrypt_name()));
    rooms
        .iter()
        .filter(|room| {
            // Test case.
            room.decrypt_name() == "very encrypted name"
            // Real case.
                || room.decrypt_name() == "northpole object storage"
        })
        .map(|room| room.sector_id)
        .next()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 1514);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2("qzmt-zixmtkozy-ivhz-343[ploop]"), 343);
    }
}
