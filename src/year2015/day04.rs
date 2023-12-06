use md5::{Digest, Md5};

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_str_with_int(key: &str, salt: usize, slice: usize) -> String {
    let salted_key = key.to_string() + &salt.to_string();
    hash_bytes(salted_key.as_bytes())[..slice].to_string()
}

pub fn puzzle1(key: &str) -> usize {
    let mut i = 0;
    while hash_str_with_int(key, i, 5) != "00000" {
        i += 1;
    }
    i
}

pub fn puzzle2(key: &str) -> usize {
    let mut i = 0;
    while hash_str_with_int(key, i, 6) != "000000" {
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "pqrstuv";
    const REAL_INPUT: &str = "iwrupvqb";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 1048970);
        assert_eq!(super::puzzle1(REAL_INPUT), 346386);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(REAL_INPUT), 9958218);
    }

    #[test]
    fn hasher() {
        assert_eq!(
            super::hash_bytes(b"abcdef609043"),
            "000001dbbfa3a5c83a2d506429c7b00e"
        );
    }
}
