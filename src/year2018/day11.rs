use std::collections::HashMap;
use std::mem;

fn get_power_level(x: usize, y: usize, serial_number: i32) -> i32 {
    let (x, y) = (x as i32, y as i32);

    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level -= 5;

    power_level
}

struct Grid {
    cells: [[i32; 300]; 300],
    powers: HashMap<(usize, usize, usize), i32>,
}

impl Grid {
    fn new(serial_number: i32) -> Grid {
        let mut grid = Grid {
            cells: [[0; 300]; 300],
            powers: HashMap::with_capacity(300 * 300 * 150),
        };

        for i in 0..300 {
            for j in 0..300 {
                grid.cells[i][j] = get_power_level(i + 1, j + 1, serial_number)
            }
        }

        grid
    }

    fn power2(&self, x: usize, y: usize) -> i32 {
        let (i, j) = (x - 1, y - 1);
        self.cells[i][j] + self.cells[i][j + 1] + self.cells[i + 1][j] + self.cells[i + 1][j + 1]
    }

    fn power3(&self, x: usize, y: usize) -> i32 {
        let (i, j) = (x - 1, y - 1);
        self.cells[i][j]
            + self.cells[i][j + 1]
            + self.cells[i][j + 2]
            + self.cells[i + 1][j]
            + self.cells[i + 1][j + 1]
            + self.cells[i + 1][j + 2]
            + self.cells[i + 2][j]
            + self.cells[i + 2][j + 1]
            + self.cells[i + 2][j + 2]
    }

    fn power(&mut self, x: usize, y: usize, s: usize) -> i32 {
        let (xx, yy) = (x - 1, y - 1);
        match s {
            0 => 0,
            1 => self.cells[xx][yy],
            2 => self.power2(x, y),
            3 => {
                if !self.powers.contains_key(&(x, y, s)) {
                    self.powers.insert((x, y, s), self.power3(x, y));
                }
                self.powers[&(x, y, s)]
            }
            _ => {
                if !self.powers.contains_key(&(x, y, s)) {
                    let mut p = self.power(x + 1, y + 1, s - 1);
                    for i in 0..s {
                        p += self.cells[xx + i][yy] + self.cells[xx][yy + i]
                    }
                    self.powers.insert((x, y, s), p);
                }
                self.powers[&(x, y, s)]
            }
        }
    }
}

pub fn puzzle1(input: &str) -> String {
    let serial_number = input.trim().parse().expect("invalid input");
    let grid = Grid::new(serial_number);

    let mut max = (i32::min_value(), (0, 0));
    for x in 1..=298 {
        for y in 1..=298 {
            let p = grid.power3(x, y);
            if max.0 < p {
                max = (p, (x, y));
            }
        }
    }

    format!("{},{}", (max.1).0, (max.1).1)
}

pub fn puzzle2(input: &str) -> String {
    let serial_number = input.trim().parse().expect("invalid input");
    let mut grid = Grid::new(serial_number);

    let mut max = (i32::min_value(), (0, 0, 2));
    for s in 2..=300 {
        let z = 300 - s + 1;
        for x in 1..=z {
            for y in 1..=z {
                let p = grid.power(x, y, s);
                if max.0 < p {
                    max = (p, (x, y, s));
                }
            }
        }
    }

    format!("{},{},{}", (max.1).0, (max.1).1, (max.1).2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_power_level() {
        assert_eq!(super::get_power_level(122, 79, 57), -5);
        assert_eq!(super::get_power_level(217, 196, 39), 0);
        assert_eq!(super::get_power_level(101, 153, 71), 4);
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("18"), "33,45");
        assert_eq!(super::puzzle1("42"), "21,61");
    }

    #[test]
    fn puzzle2() {
        // these take forever to run... uncomment when we spead things up
        //assert_eq!(super::puzzle2("18"), "90,269,16");
        //assert_eq!(super::puzzle2("42"), "232,251,12");
    }
}
