use crate::util;

pub fn fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

pub fn puzzle1(input: &str) -> i64 {
    let mut sum = 0;

    for n in util::parse::<i64>(input) {
        sum += fuel(n);
    }
    sum
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum = 0;

    for n in util::parse::<i64>(input) {
        let mut res = fuel(n);
        while res > 0 {
            sum += res;
            res = fuel(res);
        }
    }
    sum
}

#[cfg(test)]
mod tests {

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("12"), 2);
        assert_eq!(super::puzzle1("14"), 2);
        assert_eq!(super::puzzle1("1969"), 654);
        assert_eq!(super::puzzle1("100756"), 33583);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2("14"), 2);
        assert_eq!(super::puzzle2("1969"), 966);
        assert_eq!(super::puzzle2("100756"), 50346);
    }
}
