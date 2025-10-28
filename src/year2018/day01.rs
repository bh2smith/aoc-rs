use crate::util;
use std::collections::HashSet;

pub fn puzzle1(input: &str) -> i64 {
    util::parse::<i64>(input).sum()
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum = 0;
    let mut numbers = HashSet::new();

    numbers.insert(0);
    loop {
        for n in util::parse::<i64>(input) {
            sum += n;
            if !numbers.insert(sum) {
                return sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("+1 \n +1 \n +1"), 3);
        assert_eq!(super::puzzle1("+1 \n +1 \n -2"), 0);
        assert_eq!(super::puzzle1("-1 \n -2 \n -3"), -6);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2("+1 \n -1"), 0);
        assert_eq!(super::puzzle2("+3 \n +3 \n +4 \n -2 \n -4"), 10);
        assert_eq!(super::puzzle2("-6 \n +3 \n +8 \n +5 \n -6"), 5);
        assert_eq!(super::puzzle2("+7 \n +7 \n -2 \n -7 \n -4"), 14);
    }
}
