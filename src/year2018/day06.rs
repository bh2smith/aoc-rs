use crate::util;
use failure::{self, Error};
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vec2(i16, i16);

impl Vec2 {
    fn cmap<F>(a: Vec2, b: Vec2, f: F) -> Vec2
    where
        F: Fn(i16, i16) -> i16,
    {
        Vec2(f(a.0, b.0), f(a.1, b.1))
    }

    fn manhattan_dist(&self, other: Vec2) -> usize {
        let abs = Vec2::cmap(*self, other, |a, b| (a - b).abs());
        (abs.0 + abs.1) as _
    }

    fn span_out<F>(&self, mut f: F)
    where
        F: FnMut(Vec2) -> bool,
    {
        if !f(*self) {
            return;
        }

        let Vec2(x, y) = *self;
        let (mut xmin, mut xmax) = (x, x);

        macro_rules! check {
            ($i:expr, $j:expr) => {
                let vi = Vec2($i, $j);
                if !f(vi) {
                    break;
                }
            };
        }

        for i in (i16::min_value()..x).rev() {
            check!(i, y);
            xmin = i;
        }
        for i in (x + 1).. {
            check!(i, y);
            xmax = i;
        }

        for i in xmin..=xmax {
            for j in (i16::min_value()..y).rev() {
                check!(i, j);
            }
            for j in (y + 1).. {
                check!(i, j);
            }
        }
    }

    fn sprawl_out<F>(&self, mut f: F)
    where
        F: FnMut(Vec2) -> bool,
    {
        let mut visitied = HashSet::new();
        let mut remaining = vec![*self];

        while let Some(v) = remaining.pop() {
            if !visitied.insert(v) {
                continue;
            }

            if f(v) {
                remaining.push(v + Vec2(0, 1));
                remaining.push(v + Vec2(1, 0));
                remaining.push(v + Vec2(0, -1));
                remaining.push(v + Vec2(-1, 0));
            }
        }
    }
}

impl FromStr for Vec2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ',').map(|p| p.trim());
        match (parts.next(), parts.next()) {
            (Some(x), Some(y)) => Ok(Vec2(x.parse()?, y.parse()?)),
            _ => Err(failure::err_msg("expected 'x, y' format")),
        }
    }
}

impl Into<(usize, usize)> for Vec2 {
    fn into(self) -> (usize, usize) {
        (self.0 as _, self.1 as _)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

struct Bounds {
    min: Vec2,
    max: Vec2,
}

impl Bounds {
    fn empty() -> Bounds {
        Bounds {
            min: Vec2(0, 0),
            max: Vec2(0, 0),
        }
    }

    fn containing<I: Iterator<Item = Vec2>>(mut vs: I) -> Bounds {
        let v = match vs.next() {
            Some(v) => v,
            None => return Bounds::empty(),
        };
        let (min, max) = vs.fold((v, v), |(min, max), v| {
            (Vec2::cmap(min, v, cmp::min), Vec2::cmap(max, v, cmp::max))
        });

        Bounds { min, max }
    }

    fn dim(&self) -> (usize, usize) {
        (self.max - self.min + Vec2(1, 1)).into()
    }

    fn contains(&self, v: Vec2) -> bool {
        v.0 >= self.min.0 && v.0 <= self.max.0 && v.1 >= self.min.1 && v.1 <= self.max.1
    }
}

struct Grid<T> {
    bounds: Bounds,
    squares: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    fn new(bounds: Bounds) -> Grid<T> {
        let (w, h) = bounds.dim();
        Grid {
            bounds,
            squares: vec![T::default(); w * h],
        }
    }
}

impl<T> Grid<T> {
    fn len(&self) -> usize {
        self.squares.len()
    }

    fn index_of(&self, i: Vec2) -> usize {
        let (x, y) = (i - self.bounds.min).into();
        let (w, _) = self.bounds.dim();

        x + y * w
    }

    fn get(&self, i: Vec2) -> &T {
        let i = self.index_of(i);
        &self.squares[i]
    }

    fn get_mut(&mut self, i: Vec2) -> &mut T {
        let i = self.index_of(i);
        &mut self.squares[i]
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (Vec2, &mut T)> {
        let (xmin, ymin) = self.bounds.min.into();
        let (w, _) = self.bounds.dim();

        self.squares.iter_mut().enumerate().map(move |(i, s)| {
            let x = xmin + (i % w);
            let y = ymin + (i / w);

            (Vec2(x as _, y as _), s)
        })
    }

    fn borders(&self) -> impl Iterator<Item = (Vec2, &T)> {
        let Vec2(xmin, ymin) = self.bounds.min;
        let Vec2(xmax, ymax) = self.bounds.max;

        let left = (ymin + 1..ymax).map(move |y| Vec2(xmin, y));
        let top = (xmin..=xmax).map(move |x| Vec2(x, ymin));
        let right = (ymin + 1..ymax).map(move |y| Vec2(xmax, y));
        let bottom = (xmin..=xmax).map(move |x| Vec2(x, ymax));

        left.chain(top)
            .chain(right)
            .chain(bottom)
            .map(move |v| (v, self.get(v)))
    }

    fn span_out_mut<F>(&mut self, v: Vec2, mut f: F)
    where
        F: FnMut(Vec2, &mut T) -> bool,
    {
        v.span_out(move |v| {
            if !self.bounds.contains(v) {
                return false;
            }
            f(v, self.get_mut(v))
        });
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (w, h) = self.bounds.dim();

        let mut set = f.debug_set();
        for y in 0..h {
            let xs = &self.squares[y * w..(y + 1) * w];
            set.entry(&format_args!("{:?}", xs));
        }
        set.finish()
    }
}

pub fn puzzle1(input: &str) -> usize {
    let coords = util::parse::<Vec2>(input)
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .collect::<Vec<_>>();

    let bounds = Bounds::containing(coords.iter().cloned().map(|(_, v)| v));
    let mut grid = Grid::new(bounds);
    let mut counts = HashMap::new();

    for (i, s) in grid.iter_mut() {
        let (id, v) = coords[0];
        *s = (id, v.manhattan_dist(i));
    }
    counts.insert(coords[0].0, grid.len());

    for (id, v) in coords[1..].iter().cloned() {
        grid.span_out_mut(v, |vi, s| {
            let dist = v.manhattan_dist(vi);
            if dist < s.1 {
                if s.0 != 0 {
                    *counts.entry(s.0).or_insert_with(|| panic!()) -= 1;
                }
                *counts.entry(id).or_insert(0) += 1;
                *s = (id, dist);
                true
            } else if dist > s.1 {
                false
            } else {
                if s.0 != 0 {
                    *counts.entry(s.0).or_insert_with(|| panic!()) -= 1;
                    s.0 = 0;
                }
                true
            }
        })
    }

    for (_, s) in grid.borders() {
        counts.remove(&s.0);
    }

    counts.values().cloned().max().unwrap()
}

pub fn puzzle2(input: &str) -> usize {
    puzzle2_with_size(input, 10000)
}

fn puzzle2_with_size(input: &str, size: usize) -> usize {
    let coords = util::parse::<Vec2>(input).collect::<Vec<_>>();

    let (xtotal, ytotal) = coords[1..]
        .iter()
        .fold(coords[0].into(), |(xtotal, ytotal), v| {
            let (x, y) = (*v).into();
            (xtotal + x, ytotal + y)
        });
    let center = Vec2((xtotal / coords.len()) as _, (ytotal / coords.len()) as _);

    let mut count = 0;
    let (mut min, mut vmin) = (usize::max_value(), Vec2(0, 0));
    center.sprawl_out(|i| {
        let dist: usize = coords.iter().map(|v| v.manhattan_dist(i)).sum();
        let safe = dist < size;
        if safe {
            count += 1;
        }
        if dist < min {
            min = dist;
            vmin = i;
        }

        safe
    });

    count
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        1, 1
        1, 6
        8, 3
        3, 4
        5, 5
        8, 9
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(INPUT), 17);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2_with_size(INPUT, 32), 16);
    }
}
