use std::collections::{HashMap, HashSet};

struct Graph {
    adj: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn from_input(input: &str) -> Self {
        let mut adj = HashMap::new();
        for x in input.trim().lines() {
            let (id, right) = x.split_once(": ").unwrap();
            let deps = adj.entry(id.to_string()).or_insert(HashSet::new());
            for id in right.split(" ") {
                deps.insert(id.to_string());
            }
        }

        Graph { adj }
    }

    // Memoized DFS
    fn count_paths_through_dp(&self, start: &str, end: &str, a: &str, b: &str) -> usize {
        #[allow(clippy::too_many_arguments)]
        fn dfs(
            g: &Graph,
            node: &str,
            end: &str,
            a: &str,
            b: &str,
            seen_a: bool,
            seen_b: bool,
            memo: &mut HashMap<(String, bool, bool), usize>,
        ) -> usize {
            let key = (node.to_string(), seen_a, seen_b);
            if let Some(&v) = memo.get(&key) {
                return v;
            }

            // update flags based on *current* node
            let seen_a = seen_a || node == a;
            let seen_b = seen_b || node == b;

            let res = if node == end {
                if seen_a && seen_b {
                    1
                } else {
                    0
                }
            } else {
                let mut tot = 0;
                if let Some(neighbours) = g.adj.get(node) {
                    for neighbour in neighbours {
                        tot += dfs(g, neighbour, end, a, b, seen_a, seen_b, memo);
                    }
                }
                tot
            };

            memo.insert(key, res);
            res
        }

        let mut memo = HashMap::new();
        dfs(self, start, end, a, b, start == a, start == b, &mut memo)
    }
}

pub fn puzzle1(input: &str) -> usize {
    Graph::from_input(input).count_paths_through_dp("you", "out", "you", "out")
}

pub fn puzzle2(input: &str) -> usize {
    Graph::from_input(input).count_paths_through_dp("svr", "out", "dac", "fft")
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT_A: &str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const SAMPLE_INPUT_B: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT_A), 5);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT_B), 2);
    }
}
