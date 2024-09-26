use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Clone)]
struct NeighbourGraph {
    vertices: HashSet<String>,
    pairs: HashMap<(String, String), i64>,
}

impl NeighbourGraph {
    fn new(input: &str) -> Self {
        let mut pairs = HashMap::new();
        let mut vertices = HashSet::new();
        for row in input.trim().split('\n') {
            let line = row.split_whitespace().collect_vec();
            let name_b = line[line.len() - 1].to_string().replace('.', "");
            let weight = line[3].parse::<i64>().unwrap();
            vertices.insert(name_b.clone());
            pairs.insert(
                (line[0].to_string(), name_b),
                match line[2] {
                    "gain" => weight,
                    "lose" => -weight,
                    _ => unreachable!(""),
                },
            );
        }
        // println!("Vertices {vertices:?} Pairs {pairs:?}");
        Self { pairs, vertices }
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }
}

fn solve_graph(graph: NeighbourGraph) -> i64 {
    let size = graph.size();
    let mut best_result = i64::MIN;
    for order in graph.clone().vertices.into_iter().permutations(size) {
        let weight = (0..size)
            .map(|i| {
                let a = order[i].clone();
                let b = order[(i + 1) % size].clone();
                let a_b = graph.pairs.get(&(a.clone(), b.clone())).unwrap();
                let b_a = graph.pairs.get(&(b.clone(), a.clone())).unwrap();
                a_b + b_a
            })
            .sum();
        if weight > best_result {
            best_result = weight
        }
    }
    best_result
}

pub fn puzzle1(input: &str) -> i64 {
    let graph = NeighbourGraph::new(input);
    solve_graph(graph)
}

pub fn puzzle2(input: &str) -> i64 {
    let mut graph = NeighbourGraph::new(input);
    let myself = "bh2smith".to_string();
    for person in &graph.vertices {
        graph.pairs.insert((myself.clone(), person.clone()), 0);
        graph.pairs.insert((person.clone(), myself.clone()), 0);
    }
    graph.vertices.insert(myself);

    solve_graph(graph)
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 330);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 286);
    }
}
