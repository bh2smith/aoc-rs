use crate::util;
use std::collections::HashMap;

pub fn puzzle1(input: &str) -> i64 {
    let mut counts = HashMap::new();

    let mut twos = 0;
    let mut threes = 0;

    for id in util::split(input) {
        counts.clear();
        let mut has_two = 0;
        let mut has_three = 0;

        for c in id.chars() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
            match *count {
                2 => has_two += 1,
                3 => {
                    has_two -= 1;
                    has_three += 1;
                }
                4 => has_three -= 1,
                _ => {}
            }
        }

        if has_two > 0 {
            twos += 1;
        }

        if has_three > 0 {
            threes += 1;
        }
    }

    twos * threes
}

pub fn puzzle2(input: &str) -> String {
    let ids = util::split(input).collect::<Vec<_>>();
    let mut diff = String::new();

    for (i, id) in ids.iter().cloned().enumerate() {
        for other in ids[(i + 1)..].iter().cloned() {
            diff.clear();
            diff.reserve_exact(id.len());
            for (a, b) in id.chars().zip(other.chars()) {
                if a == b {
                    diff.push(a);
                }
            }

            if diff.len() + 1 == id.len() {
                return diff;
            }
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(
            super::puzzle1(
                r"
                    abcdef
                    bababc
                    abbcde
                    abcccd
                    aabcdd
                    abcdee
                    ababab
                "
            ),
            12
        );
    }

    #[test]
    fn puzzle2() {
        assert_eq!(
            super::puzzle2(
                r"
                    abcde
                    fghij
                    klmno
                    pqrst
                    fguij
                    axcye
                    wvxyz
                "
            ),
            "fgij".to_string()
        );
    }
}
