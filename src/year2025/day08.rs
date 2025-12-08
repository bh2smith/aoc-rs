use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nalgebra::Point3;

type P = Point3<i64>;

fn parse_input(input: &str) -> Vec<Point3<i64>> {
    input
        .trim()
        .lines()
        .map(|x| {
            let arr = x
                .split(",")
                .map(|y| y.parse::<i64>().unwrap())
                .collect_vec();
            Point3::from_slice(&arr)
        })
        .collect()
}

fn euclidean_distance(a: Point3<i64>, b: Point3<i64>) -> f64 {
    let c = a - b;
    let s = (c[0].pow(2) + c[1].pow(2) + c[2].pow(2)) as f64;
    s.sqrt()
}

// This will evaluate all.
fn reduce(arr: Vec<Point3<i64>>) -> Vec<(Point3<i64>, Point3<i64>)> {
    arr.into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, euclidean_distance(a, b)))
        .sorted_by(|x, y| x.2.partial_cmp(&y.2).unwrap())
        .map(|(x, y, _)| (x, y))
        .collect()
}

// Assumes ordered by shortest
fn connected_components(edges: &[(P, P)], stop: Option<usize>) -> Vec<HashSet<P>> {
    let mut components: Vec<HashSet<P>> = Vec::new();
    let mut index: HashMap<P, usize> = HashMap::new();
    let mut handled = 0;
    for (a, b) in edges.iter().cloned() {
        let xa = index.get(&a).copied();
        let xb = index.get(&b).copied();

        match (xa, xb) {
            (None, None) => {
                // Neither vertex in a component yet. but in same.
                let set = HashSet::from([a, b]);
                let i = components.len();
                components.push(set);
                index.insert(a, i);
                index.insert(b, i);
            }
            (Some(i), None) => {
                // b in component a
                components[i].insert(b);
                index.insert(b, i);
            }
            (None, Some(j)) => {
                components[j].insert(a);
                index.insert(a, j);
            }
            (Some(i), Some(j)) => {
                // Merge components.
                let (l, r) = if i < j { (i, j) } else { (j, i) };
                let moved: Vec<_> = components[r].drain().collect();
                for point in moved {
                    components[l].insert(point);
                    index.insert(point, l);
                }
            }
        }
        handled += 1;
        if let Some(n) = stop {
            if handled == n {
                break;
            }
        }
    }

    components.into_iter().filter(|c| !c.is_empty()).collect()
}

fn first_edge_that_connects_all(points: &[P], edges: &[(P, P)]) -> (P, P) {
    let mut components: Vec<HashSet<P>> = Vec::new();
    let mut index: HashMap<P, usize> = HashMap::new();

    // each point starts as own component.
    for p in points {
        let i = components.len();
        components.push(HashSet::from([*p]));
        index.insert(*p, i);
    }

    let mut component_count = components.len(); // start with N singleton components

    for (a, b) in edges.iter().cloned() {
        let ca = index.get(&a).copied().unwrap();
        let cb = index.get(&b).copied().unwrap();

        if ca == cb {
            // Same component already, nothing changes
        } else {
            // Merge components ca and cb
            let (l, r) = if ca < cb { (ca, cb) } else { (cb, ca) };
            let moved: Vec<_> = components[r].drain().collect();
            for point in moved {
                components[l].insert(point);
                index.insert(point, l);
            }
            component_count -= 1;
        }

        if component_count == 1 {
            // First edge connecting the graph.
            return (a, b);
        }
    }

    panic!("Graph never became fully connected");
}

pub fn puzzle1(input: &str) -> usize {
    let points = parse_input(input);
    let edges = reduce(points);
    let mut components = connected_components(&edges, Some(1000));
    components.sort_by_key(|c| c.len());
    components.reverse();
    components.iter().take(3).map(|c| c.len()).product()
}

pub fn puzzle2(input: &str) -> i64 {
    let points = parse_input(input);
    let edges = reduce(points.clone());
    let last_edge = first_edge_that_connects_all(&points, &edges);
    println!("Last Edge {last_edge:?}");
    last_edge.0[0] * last_edge.1[0]
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 40);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 25272);
    }
}
