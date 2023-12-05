use itertools::Itertools;

fn count_char(string: &str, character: char) -> usize {
    string.chars().filter(|&c| c == character).count()
}

pub fn puzzle1(input: &str) -> isize {
    let left = count_char(input, '(') as isize;
    let right = count_char(input, ')') as isize;
    left - right
}

pub fn puzzle2(input: &str) -> usize {
    let input = input.chars().collect_vec();
    let mut floor = 0;
    let mut i = 0;
    while i < input.len() && floor != -1 {
        match input[i] {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        i += 1;
        // println!("at floor {floor} after step {i}");
    }
    i
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "()())";
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), -1);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 5);
    }
}
