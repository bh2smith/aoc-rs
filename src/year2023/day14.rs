enum Direction {
    North,
    West,
    South,
    East,
}

struct Platform {
    grid: Vec<Vec<char>>,
}

impl Platform {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self { grid }
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                for i in 0..self.grid.len() {
                    for j in 0..self.grid[i].len() {
                        match self.grid[i][j] == 'O' {
                            true => {
                                let mut k = i;

                                while k > 0 && self.grid[k - 1][j] == '.' {
                                    k -= 1;
                                }

                                self.grid[i][j] = '.';
                                self.grid[k][j] = 'O';
                            }
                            false => (),
                        }
                    }
                }
            }
            Direction::West => {
                for i in 0..self.grid.len() {
                    for j in 0..self.grid[i].len() {
                        match self.grid[i][j] == 'O' {
                            true => {
                                let mut k = j;

                                while k > 0 && self.grid[i][k - 1] == '.' {
                                    k -= 1;
                                }

                                self.grid[i][j] = '.';
                                self.grid[i][k] = 'O';
                            }
                            false => (),
                        }
                    }
                }
            }
            Direction::South => {
                for i in (0..self.grid.len()).rev() {
                    for j in 0..self.grid[i].len() {
                        match self.grid[i][j] == 'O' {
                            true => {
                                let mut k = i;

                                while k < self.grid.len() - 1 && self.grid[k + 1][j] == '.' {
                                    k += 1;
                                }

                                self.grid[i][j] = '.';
                                self.grid[k][j] = 'O';
                            }
                            false => (),
                        }
                    }
                }
            }
            Direction::East => {
                for i in 0..self.grid.len() {
                    for j in (0..self.grid[i].len()).rev() {
                        match self.grid[i][j] == 'O' {
                            true => {
                                let mut k = j;

                                while k < self.grid[0].len() - 1 && self.grid[i][k + 1] == '.' {
                                    k += 1;
                                }

                                self.grid[i][j] = '.';
                                self.grid[i][k] = 'O';
                            }
                            false => (),
                        }
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn load(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let rounded = row
                    .iter()
                    .copied()
                    .filter(|c| *c == 'O')
                    .collect::<Vec<char>>()
                    .len();

                rounded * (self.grid.len() - i)
            })
            .sum()
    }
}

pub fn puzzle1(input: &str) -> usize {
    let mut platform = Platform::from(input);
    platform.tilt(Direction::North);

    platform.load()
}

pub fn puzzle2(input: &str) -> usize {
    let mut platform = Platform::from(input);

    for _ in 0..1000 {
        platform.cycle();
    }

    platform.load()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 136);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 64);
    }
}
