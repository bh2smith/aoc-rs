use std::collections::HashMap;

use itertools::Itertools;

// iterative cuz recursion wasn't going well for me.
fn power_set<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let n = items.len();
    let total_subsets = 1 << n; // 2^n

    for i in 0..total_subsets {
        let mut subset = Vec::new();

        for (j, item) in items.iter().enumerate().take(n) {
            if (i & (1 << j)) != 0 {
                subset.push(item.clone());
            }
        }

        result.push(subset);
    }

    result
}

pub fn puzzle1(input: &str) -> u32 {
    let containers = input
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect_vec();
    power_set(containers.as_slice())
        .into_iter()
        .filter(|subset| subset.iter().sum::<u32>() == 150)
        .collect_vec()
        .len() as u32
}

pub fn puzzle2(input: &str) -> u32 {
    let containers = input
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect_vec();
    let mut counter = HashMap::new();
    for subset in power_set(containers.as_slice()) {
        if subset.iter().sum::<u32>() == 150 {
            counter.entry(subset.len()).or_insert(vec![]).push(subset);
        }
    }

    counter[counter.keys().min().unwrap()].len() as u32
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "
20
15
10
5
5";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 4);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}
