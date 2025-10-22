use std::str::FromStr;

pub fn solve(input: &str, size: usize) -> i64 {
    let split_input: Vec<&str> = input.trim().split('\n').collect();

    let mut cur_sum = 0i64;
    let mut elf_sums = vec![];
    split_input.iter().for_each(|item| {
        if item == &"" {
            elf_sums.push(cur_sum);
            cur_sum = 0;
        } else {
            cur_sum += i64::from_str(item).unwrap();
        }
    });
    elf_sums.sort();
    elf_sums.iter().rev().take(size).sum()
}

pub fn puzzle1(input: &str) -> i64 {
    solve(input, 1)
}

pub fn puzzle2(input: &str) -> i64 {
    solve(input, 3)
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 24000);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 41000);
    }
}
