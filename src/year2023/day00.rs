pub fn parse_input(input: &str) {
    println!("Got input of length {}", input.len());
}

pub fn puzzle1(input: &str) -> u32 {
    parse_input(input);
    0
}

pub fn puzzle2(input: &str) -> u32 {
    parse_input(input);
    0
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}
