use itertools::Itertools;

#[derive(Debug)]
struct RaceParams {
    time: usize,
    record: usize,
}

impl RaceParams {
    /// Naively counts the winning strategies.
    fn winning_strategies(&self) -> usize {
        (0..(self.time + 1))
            .filter(|hold_time| (self.time - hold_time) * hold_time > self.record)
            .count()
    }
}

pub fn puzzle1(input: &str) -> u32 {
    // Parse input
    let input = input.trim().split('\n').collect_vec();
    let times = input[0]
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect_vec();
    let distances = input[1]
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect_vec();
    let params = times
        .into_iter()
        .zip(distances)
        .map(|(time, record)| RaceParams { time, record })
        .collect_vec();

    // Actual solution
    params
        .iter()
        .map(|p| p.winning_strategies())
        .product::<usize>() as u32
}

pub fn puzzle2(input: &str) -> u32 {
    let input = input.trim().split('\n').collect_vec();
    let param = RaceParams {
        time: input[0][5..].replace(' ', "").parse().unwrap(),
        record: input[1][10..].replace(' ', "").parse().unwrap(),
    };

    param.winning_strategies() as u32
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 288);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 71503);
    }
}
