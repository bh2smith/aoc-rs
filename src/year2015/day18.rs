use nalgebra::Point2;
use std::collections::HashSet;

use crate::util::{adjacent8, Grid};

struct Lights {
    layout: Grid,
}
impl Lights {
    fn new(input: &str) -> Self {
        Self {
            layout: Grid::from_layout(input),
        }
    }

    fn step(&self, stuck_lights: HashSet<Point2<usize>>) -> Self {
        let size = self.layout.size();
        let mut next = Grid::new(size.0, size.1);
        for (point, val) in self.layout.iter() {
            if stuck_lights.contains(&point) {
                next[point] = b'#';
                continue;
            }
            let adjacent_on = adjacent8(point)
                .filter(|p| p[0] < size.0 && p[1] < size.1 && self.layout.get(*p).unwrap() == b'#')
                .count();

            next[point] = match val {
                b'#' => {
                    if adjacent_on == 2 || adjacent_on == 3 {
                        b'#'
                    } else {
                        b'.'
                    }
                }
                b'.' => {
                    if adjacent_on == 3 {
                        b'#'
                    } else {
                        b'.'
                    }
                }
                _ => unreachable!("not an option"),
            };
        }

        Self { layout: next }
    }

    fn step_n(&self, n: usize, stuck_lights: HashSet<Point2<usize>>) -> Self {
        println!("Initial State\n{:?}\n", self.layout);
        let mut next = self.step(stuck_lights.clone());
        for i in 1..n {
            next = next.step(stuck_lights.clone());
            println!("After {i} steps:\n{:?}\n", next.layout);
        }
        next
    }

    fn count_lights(&self) -> usize {
        self.layout.iter().filter(|(_, val)| *val == b'#').count()
    }
}

pub fn puzzle1(input: &str) -> usize {
    let state = Lights::new(input);
    state.step_n(100, HashSet::new()).count_lights()
}

pub fn puzzle2(input: &str) -> usize {
    let state = Lights::new(input);
    let size = state.layout.size();
    let stuck_points = [
        Point2::new(0, 0),
        Point2::new(size.0 - 1, 0),
        Point2::new(0, size.1 - 1),
        Point2::new(size.0 - 1, size.1 - 1),
    ];
    state
        .step_n(100, HashSet::from(stuck_points))
        .count_lights()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 4);
    }

    const SAMPLE_INPUT_2: &str = "##.#.#
...##.
#....#
..#...
#.#..#
####.#";
    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT_2), 17);
    }
}
