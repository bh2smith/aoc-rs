type Bank = Vec<u64>;

fn solve(jolts: &Bank, n: usize) -> u64 {
    if n == 1 {
        return *jolts.iter().max().expect("numbers are ordered");
    }
    // lowest array index containing the slice's max value.
    let left = jolts[..jolts.len() - n + 1]
        .iter()
        .enumerate()
        .rev() // without rev this would return the largest index
        .max_by_key(|&(_, v)| v)
        .map(|(i, _v)| i)
        .expect("max must exist");
    let (_, right) = jolts.split_at(left + 1);
    // Recursion
    10u64.pow(n as u32 - 1) * jolts[left] + solve(&right.to_vec(), n - 1)
}

fn parse_input(input: &str) -> Vec<Bank> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("non-digit found") as u64)
                .collect()
        })
        .collect()
}

pub fn puzzle1(input: &str) -> u64 {
    parse_input(input).iter().map(|b| solve(b, 2)).sum()
}

pub fn puzzle2(input: &str) -> u64 {
    parse_input(input).iter().map(|b| solve(b, 12)).sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 357);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 3121910778619);
    }
}
