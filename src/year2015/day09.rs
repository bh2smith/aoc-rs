use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type DistanceMap = HashMap<(String, String), u32>;

fn parse_input(input: &str) -> (DistanceMap, HashSet<String>) {
    let mut routes: DistanceMap = HashMap::new();
    let mut cities = HashSet::new();
    for line in input.trim().split('\n') {
        let split_line = line.split_whitespace().collect_vec();
        let city_1 = split_line[0].to_string();
        let city_2 = split_line[2].to_string();
        routes.insert(
            (city_1.clone(), city_2.clone()),
            split_line[4].parse::<u32>().unwrap(),
        );
        routes.insert(
            (city_2.clone(), city_1.clone()),
            split_line[4].parse::<u32>().unwrap(),
        );

        cities.insert(city_1);
        cities.insert(city_2);
    }

    (routes, cities)
}

pub fn puzzle1(input: &str) -> u32 {
    let (distances, cities) = parse_input(input);

    let mut best_distance = u32::MAX;
    for route in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for i in 0..cities.len() - 1 {
            distance += distances
                .get(&(route[i].clone(), route[i + 1].clone()))
                .unwrap();
        }
        if distance < best_distance {
            best_distance = distance
        }
    }
    best_distance
}

pub fn puzzle2(input: &str) -> u32 {
    let (distances, cities) = parse_input(input);

    let mut best_distance = u32::MIN;
    for route in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for i in 0..cities.len() - 1 {
            distance += distances
                .get(&(route[i].clone(), route[i + 1].clone()))
                .unwrap();
        }
        if distance > best_distance {
            best_distance = distance
        }
    }
    best_distance
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 605);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 982);
    }
}
