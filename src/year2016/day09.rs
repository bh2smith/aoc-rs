fn decompress(input: &str, part2: bool) -> usize {
    let mut ans = 0;
    let mut i = 0;
    while i < input.len() {
        let c = input[i..].chars().next().unwrap();
        match c {
            '(' => {
                let close = input[i..].find(")").unwrap() + i;
                // Parsing AxB
                let (a, b) = input[i + 1..close].split_once('x').unwrap();
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                let start = close + 1;
                let stop = start + a;
                let chunk = &input[start..stop];

                if part2 {
                    // Recursion!
                    ans += b * decompress(chunk, part2);
                } else {
                    ans += b * chunk.len();
                }
                i = stop;
            }
            _ => {
                ans += 1;
                i += 1;
            }
        }
    }
    ans
}

pub fn puzzle1(input: &str) -> usize {
    decompress(input, false)
}

pub fn puzzle2(input: &str) -> usize {
    decompress(input, true)
}

#[cfg(test)]
mod tests {

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("A(1x5)BC"), 7);
        assert_eq!(super::puzzle1("(3x3)XYZ"), 9);
        assert_eq!(super::puzzle1("(6x1)(1x3)A"), 6);
        assert_eq!(super::puzzle1("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(super::puzzle1("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2("(3x3)XYZ"), 9);
        assert_eq!(super::puzzle2("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(super::puzzle2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(
            super::puzzle2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
