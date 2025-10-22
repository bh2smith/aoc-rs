use crate::util;
use std::str::FromStr;

type Pair = (i32, i32);

fn unpack_as_tuple(val: &str) -> Pair {
    let res: Vec<_> = val
        .split('-')
        .map(|s| i32::from_str(s.trim()).unwrap())
        .collect();
    (res[0], res[1])
}

fn left_in_right(left: &Pair, right: &Pair) -> bool {
    left.0 >= right.0 && left.1 <= right.1
}

fn has_overlap(a: &Pair, b: &Pair) -> bool {
    !(a.1 < b.0 || a.0 > b.1)
}

fn parse_input(input: &str) -> Vec<(Pair, Pair)> {
    // TODO - return iterator (so not to have to collect all the time).
    util::split(input)
        .map(|row| {
            let x: Vec<_> = row.split(',').collect();
            let [left, right] = x[..2] else {
                panic!("Unexpected pattern {:?}", x)
            };
            let left_tup = unpack_as_tuple(left);
            let right_tup = unpack_as_tuple(right);
            (left_tup, right_tup)
        })
        .collect()
}

pub fn puzzle1(input: &str) -> i64 {
    parse_input(input)
        .iter()
        .map(|(left, right)| (left_in_right(left, right) || left_in_right(right, left)) as i64)
        .sum()
}

pub fn puzzle2(input: &str) -> i64 {
    parse_input(input)
        .iter()
        .map(|(left, right)| has_overlap(right, left) as i64)
        .sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    const CRAFTED_SAMPLE: &str = r"1-2,1-2
1-3,2-3
1-2,1-3
2-3,1-3
1-3,1-2";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 2);
        assert_eq!(super::puzzle1(CRAFTED_SAMPLE), 5);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 4);
    }
}
