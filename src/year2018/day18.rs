use crate::util::{self, Grid};
use nalgebra::Point2;
use std::collections::HashMap;
use std::mem;
use std::str;

struct Field {
    current: Grid,
    next: Grid,
}

impl Field {
    fn new(layout: &str) -> Field {
        let current = Grid::from_layout(layout);
        let (w, h) = current.size();
        let next = Grid::new(w, h);

        Field { current, next }
    }

    fn adjacent<'a>(&'a self, pos: Point2<usize>) -> impl 'a + Iterator<Item = u8> {
        util::adjacent8(pos).filter_map(move |p| self.current.get(p))
    }

    fn generation(&mut self) {
        for (pos, acre) in self.current.iter() {
            let (_, tree, yard) = self
                .adjacent(pos)
                .fold((0, 0, 0), |(empty, tree, yard), a| match a {
                    b'.' => (empty + 1, tree, yard),
                    b'|' => (empty, tree + 1, yard),
                    b'#' => (empty, tree, yard + 1),
                    _ => (empty, tree, yard),
                });

            self.next[pos] = match acre {
                b'.' => {
                    if tree >= 3 {
                        b'|'
                    } else {
                        b'.'
                    }
                }
                b'|' => {
                    if yard >= 3 {
                        b'#'
                    } else {
                        b'|'
                    }
                }
                b'#' => {
                    if tree > 0 && yard > 0 {
                        b'#'
                    } else {
                        b'.'
                    }
                }
                _ => unreachable!(),
            }
        }

        mem::swap(&mut self.current, &mut self.next);
    }

    fn count(&self) -> usize {
        let (tree, yard) = self
            .current
            .iter()
            .fold((0, 0), |(tree, yard), (_, acre)| match acre {
                b'|' => (tree + 1, yard),
                b'#' => (tree, yard + 1),
                _ => (tree, yard),
            });
        tree * yard
    }
}

pub fn puzzle1(input: &str) -> usize {
    let mut field = Field::new(input);

    for _ in 0..10 {
        field.generation();
    }

    field.count()
}

pub fn puzzle2(input: &str) -> usize {
    const GENERATIONS: usize = 1_000_000_000;

    let mut field = Field::new(input);
    let mut values = HashMap::new();
    let mut counts = Vec::new();

    for i in 0.. {
        counts.push(field.count());
        if let Some(prev) = values.insert(field.current.as_bytes().to_owned(), i) {
            let period = i - prev;
            let result = i - period + ((GENERATIONS - i) % period);

            return counts[result];
        }
        field.generation();
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        .#.#...|#.
        .....#|##|
        .|..|...#.
        ..|#.....#
        #.#|||#|#|
        ...#.||...
        .|....|...
        ||...#|.#|
        |.||||..|.
        ...#.|..|.
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(INPUT), 1147);
    }
}
