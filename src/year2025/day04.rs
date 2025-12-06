// use std::{thread::sleep, time::Duration};

use crate::util::{adjacent8, Grid};

fn clear(g: &mut Grid) -> usize {
    let size = g.size();
    let mut to_clear = Vec::new();
    for (pos, v) in g.iter() {
        if v == b'@' {
            let adjacents = adjacent8(pos)
                .filter(|p| p[0] < size.0 && p[1] < size.1 && g.get(*p).unwrap() == b'@')
                .count();
            if adjacents < 4 {
                to_clear.push(pos);
            }
        }
    }
    for p in to_clear.iter() {
        g[*p] = b'.'
    }
    to_clear.len()
}

pub fn puzzle1(input: &str) -> usize {
    let mut g = Grid::from_layout(input);
    clear(&mut g)
}

pub fn puzzle2(input: &str) -> usize {
    let mut g = Grid::from_layout(input);
    std::iter::from_fn(|| {
        let n = clear(&mut g);
        // sleep(Duration::from_millis(100));
        // print!("\x1B[2J\x1B[1;1H");
        // println!("{g:?}");
        (n != 0).then_some(n)
    })
    .sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 13);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 43);
    }
}
