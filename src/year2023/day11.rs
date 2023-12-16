use std::collections::HashSet;

use itertools::Itertools;

use crate::util::Point;

#[derive(Clone)]
struct Universe {
    galaxies: Vec<Point>,
    size: Point,
}

impl Universe {
    fn new(input: &str) -> Self {
        let mut galaxies = Vec::new();
        let rows = input.trim().split('\n').collect::<Vec<_>>();
        let size = Point::new(rows[0].len() as i64, rows.len() as i64);
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.trim().char_indices() {
                if ch == '#' {
                    galaxies.push(Point::new(x as i64, y as i64));
                }
            }
        }
        Self { galaxies, size }
    }

    fn empty_rows_cols(&self) -> (HashSet<i64>, HashSet<i64>) {
        let Point {
            x: num_rows,
            y: num_cols,
        } = self.size;
        let mut rows: HashSet<i64> = (0..num_rows).collect();
        let mut cols: HashSet<i64> = (0..num_cols).collect();
        for g in &self.galaxies {
            rows.remove(&g.x);
            cols.remove(&g.y);
        }
        (rows, cols)
    }

    fn solve(&self, expansion_factor: i64) -> u64 {
        let (empty_rows, empty_cols) = self.empty_rows_cols();
        let mut res = 0;
        for i in 0..self.galaxies.len() - 1 {
            for j in i + 1..self.galaxies.len() {
                let g_i = self.galaxies[i];
                let g_j = self.galaxies[j];
                let diff = g_i - g_j;

                let empty_row_crossings = (g_i.x.min(g_j.x)..(g_i.x.min(g_j.x) + diff.x.abs()))
                    .collect::<HashSet<_>>()
                    .intersection(&empty_rows)
                    .collect_vec()
                    .len() as i64;
                let empty_col_crossings = (g_i.y.min(g_j.y)..(g_i.y.min(g_j.y) + diff.y.abs()))
                    .collect::<HashSet<_>>()
                    .intersection(&empty_cols)
                    .collect_vec()
                    .len() as i64;
                let dist = diff.x.abs()
                    + diff.y.abs()
                    + (empty_col_crossings + empty_row_crossings) * (expansion_factor - 1);
                res += dist as u64;
            }
        }
        res
    }
}

pub fn puzzle1(input: &str) -> u64 {
    Universe::new(input).solve(2)
}

pub fn puzzle2(input: &str) -> u64 {
    Universe::new(input).solve(1000000)
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn both() {
        let universe = super::Universe::new(SAMPLE_INPUT);
        assert_eq!(universe.solve(2), 374);
        assert_eq!(universe.solve(10), 1030);
        assert_eq!(universe.solve(100), 8410);
    }
}
