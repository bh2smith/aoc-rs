use crate::util::Grid;

struct Hill {
    pos: (usize, usize),
    grid: Grid,
}

fn parse_input(input: &str) -> Hill {
    Hill {
        pos: (0, 0),
        grid: Grid::from_layout(input.trim()),
    }
}

impl Hill {
    fn step(&mut self, dx: usize, dy: usize) -> Option<bool> {
        let (w, h) = self.grid.size();

        if self.pos.1 + dy >= h {
            return None;
        }
        self.pos.0 = (self.pos.0 + dx) % w;
        self.pos.1 += dy;

        Some(self.grid[[self.pos.0, self.pos.1]] == b'#')
    }

    fn slide_by(&mut self, x: usize, y: usize) -> u32 {
        let mut hits = 0;
        while let Some(is_tree) = self.step(x, y) {
            if is_tree {
                hits += 1;
            }
        }
        hits
    }
}

pub fn puzzle1(input: &str) -> u32 {
    let mut hill = parse_input(input);
    hill.slide_by(3, 1)
}

pub fn puzzle2(input: &str) -> u32 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .map(|(x, y)| parse_input(input).slide_by(x, y))
        .iter()
        .product()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 7);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 336);
    }
}
