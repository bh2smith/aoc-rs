use md5::{Digest, Md5};

fn hash_str(str: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(str);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn puzzle1(input: &str) -> String {
    let input = input.trim();
    let mut password = "".to_string();
    let mut i = 0;
    while password.len() < 8 {
        let word = format!("{input}{i}");
        let hash = hash_str(&word);
        if hash.starts_with("00000") {
            // println!("Found word {word} with hash {hash}");
            password.push(hash.chars().nth(5).unwrap());
        }
        i += 1;
    }
    password
}

pub fn puzzle2(input: &str) -> String {
    let input = input.trim();
    let mut password: [Option<char>; 8] = [None; 8];
    let mut i = 0;
    while !password.iter().all(|x| x.is_some()) {
        let word = format!("{input}{i}");
        let hash = hash_str(&word);
        if hash.starts_with("00000") {
            // println!("Found word {word} with hash {hash}");
            let mut chars = hash[5..7].chars(); // ASCII hex
            let index = chars.next().unwrap().to_digit(16).unwrap() as usize;
            let value = chars.next().unwrap();
            if index < 8 && password[index].is_none() {
                // println!("Insert {value} at index {index}");
                password[index] = Some(value);
            }
        }

        i += 1;
    }
    password.iter().map(|x| x.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::year2016::day05::hash_str;

    const SAMPLE_INPUT: &str = "abc";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), "18f47a30");
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), "05ace8e3");
    }

    #[test]
    fn hasher() {
        assert_eq!(hash_str("abc3231929"), "00000155f8105dff7f56ee10fa9b9abd");
    }
}
