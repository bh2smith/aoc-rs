use anyhow::anyhow;
use std::{collections::HashSet, convert::TryFrom};

fn contains_abba(x: &str) -> bool {
    let v: Vec<char> = x.chars().collect();
    v.windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn get_abas(x: &str) -> HashSet<Aba> {
    let v: Vec<char> = x.chars().collect();
    v.windows(3).filter_map(|w| Aba::try_from(w).ok()).collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Aba {
    value: String,
    inverse: String,
}

impl TryFrom<&[char]> for Aba {
    type Error = anyhow::Error;

    fn try_from(w: &[char]) -> Result<Self, Self::Error> {
        match w.len() {
            3 => {
                if w[0] == w[2] && w[1] != w[0] {
                    let x = Self {
                        value: w.iter().collect(),
                        inverse: [w[1], w[0], w[1]].iter().collect(),
                    };
                    return Ok(x);
                }
                Err(anyhow!("not aba"))
            }
            _ => Err(anyhow!("Invalid length")),
        }
    }
}

#[derive(Debug)]

struct IPV7 {
    hypernets: Vec<String>,
    supernets: Vec<String>,
}

impl From<&str> for IPV7 {
    fn from(s: &str) -> Self {
        let mut hypernets = Vec::new();
        let mut supernets = Vec::new();

        let mut buf = String::new();
        let mut in_brackets = false;

        for ch in s.chars() {
            match ch {
                '[' => {
                    // flush the buffer to "others" if we were outside
                    if !in_brackets && !buf.is_empty() {
                        supernets.push(std::mem::take(&mut buf));
                    }
                    in_brackets = true;
                }
                ']' => {
                    // flush the buffer to "hypernets" if we were inside
                    if in_brackets && !buf.is_empty() {
                        hypernets.push(std::mem::take(&mut buf));
                    }
                    in_brackets = false;
                }
                _ => buf.push(ch),
            }
        }

        // flush whatever remains after the loop
        if !buf.is_empty() {
            if in_brackets {
                hypernets.push(buf);
            } else {
                supernets.push(buf);
            }
        }

        IPV7 {
            hypernets,
            supernets,
        }
    }
}

impl IPV7 {
    fn supports_tls(&self) -> bool {
        self.supernets.iter().any(|x| contains_abba(x))
            && !self.hypernets.iter().any(|x| contains_abba(x))
    }

    fn supports_ssl(&self) -> bool {
        let supenet_abas: HashSet<String> = self
            .supernets
            .iter()
            .map(|x| get_abas(x))
            .fold(HashSet::new(), |mut acc, set| {
                acc.extend(set);
                acc
            })
            .iter()
            .map(|x| x.value.clone())
            .collect();
        let hypernet_babs: HashSet<String> = self
            .hypernets
            .iter()
            .map(|x| get_abas(x))
            .fold(HashSet::new(), |mut acc, set| {
                acc.extend(set);
                acc
            })
            .iter()
            .map(|x| x.inverse.clone())
            .collect();
        // println!("{:?}", self);
        // println!("Supa {:?} - Hypa {:?}", supenet_abas, hypernet_babs);
        hypernet_babs.intersection(&supenet_abas).count() > 0
    }
}

fn parse_input(input: &str) -> Vec<IPV7> {
    input.trim().lines().map(IPV7::from).collect()
}

pub fn puzzle1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|x| x.supports_tls())
        .count()
}

pub fn puzzle2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|x| x.supports_ssl())
        .count()
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    const SAMPLE_INPUT: &str = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 2);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(
            super::puzzle2(
                "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb"
            ),
            3
        );
    }

    #[test]
    fn aba() {
        let res = super::Aba::try_from(vec!['a', 'b', 'a'].as_slice()).unwrap();
        assert_eq!(
            res,
            super::Aba {
                value: "aba".to_string(),
                inverse: "bab".to_string()
            }
        );
    }
}
