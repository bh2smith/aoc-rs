use itertools::Itertools;

fn transform(x: &str) -> String {
    let mut characters = x.chars().collect_vec();
    characters.reverse();
    let mut curr = characters.pop().unwrap();
    let mut count = 1;
    let mut result = vec![];
    while let Some(next) = characters.pop() {
        if next == curr {
            count += 1;
        } else {
            result.push(count.to_string() + &curr.to_string());
            count = 1;
        }
        curr = next;
    }
    result.push(count.to_string() + &curr.to_string());
    result.join("")
}

fn multiple_transform(input: &str, n: usize) -> String {
    let mut result = input.to_string();
    for _ in 0..n {
        result = transform(&result);
    }
    result
}

pub fn puzzle1(input: &str) -> usize {
    multiple_transform(input, 40).len()
}

pub fn puzzle2(input: &str) -> usize {
    multiple_transform(input, 50).len()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "1321131112";

    #[test]
    fn transform() {
        assert_eq!(super::transform("1"), "11");
        assert_eq!(super::transform("11"), "21");
        assert_eq!(super::transform("21"), "1211");
        assert_eq!(super::transform("1211"), "111221");
        assert_eq!(super::transform("111221"), "312211");
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 492982);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 6989950);
    }
}
