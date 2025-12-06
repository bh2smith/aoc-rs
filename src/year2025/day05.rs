#[derive(Debug)]
struct Puzzle {
    range: Vec<(u64, u64)>,
    items: Vec<u64>,
}

impl Puzzle {
    fn is_fresh(&self, item: &u64) -> bool {
        self.range.iter().any(|(l, r)| item >= l && item <= r)
    }

    fn merge_ranges(&mut self) {
        self.range = merge_tuples(self.range.clone());
    }
}

fn merge_tuples(mut intervals: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if intervals.is_empty() {
        return intervals;
    }

    intervals.sort_by_key(|(start, _)| *start);

    let mut merged = Vec::with_capacity(intervals.len());
    let mut current = intervals[0];

    for (start, end) in intervals.into_iter().skip(1) {
        if start <= current.1 {
            if end > current.1 {
                current.1 = end;
            }
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

fn parse(input: &str) -> Puzzle {
    let mut range = vec![];
    let mut items = vec![];
    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some((x, y)) = line.split_once('-') {
            range.push((x.parse().unwrap(), y.parse().unwrap()));
        } else {
            items.push(line.parse().expect("expected item"));
        }
    }
    Puzzle { range, items }
}

pub fn puzzle1(input: &str) -> usize {
    let puz = parse(input);
    puz.items.iter().filter(|x| puz.is_fresh(x)).count()
}

pub fn puzzle2(input: &str) -> u64 {
    let mut puz = parse(input);
    puz.merge_ranges();
    puz.range.iter().map(|(l, r)| r - l + 1).sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 3);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 14);
    }
}
