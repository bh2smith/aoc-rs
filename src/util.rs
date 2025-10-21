use nalgebra::Point2;
use num::{One, Zero};
use std::fmt::{self, Debug, Formatter};
use std::ops::{Index, IndexMut, Neg};
use std::str::{self, FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T = i64> {
    pub x: T,
    pub y: T,
}

impl<T: Debug> Debug for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Empty name => prints as "(x, y)"
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Point<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Point<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            v => panic!("Unexpected direction character! {}", v),
        }
    }

    pub fn as_point<T: Zero + std::ops::Mul<Output = T> + Neg<Output = T> + One>(
        &self,
        magnitude: T,
    ) -> Point<T> {
        let one = T::one();
        let zero = T::zero();
        match self {
            Direction::Up => Point::new(zero, magnitude),
            Direction::Down => Point::new(zero, magnitude * -one),
            Direction::Left => Point::new(magnitude * -one, zero),
            Direction::Right => Point::new(magnitude, zero),
        }
    }
}

pub fn split(input: &str) -> impl '_ + Iterator<Item = &'_ str> {
    input.trim().split('\n').map(|s| s.trim())
}

pub fn parse_csv(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|s| i64::from_str(s.trim()).unwrap())
        .collect()
}

pub fn parse<'a, T>(input: &'a str) -> impl 'a + Iterator<Item = T>
where
    T: 'a + FromStr,
    T::Err: Debug,
{
    parse_with(input, |s| s.parse::<T>().unwrap())
}

pub fn parse_with<'a, T, F>(input: &'a str, f: F) -> impl 'a + Iterator<Item = T>
where
    T: 'a,
    F: 'a + Fn(&'a str) -> T,
{
    split(input).map(f)
}

#[derive(Clone)]
pub struct Grid {
    size: (usize, usize),
    squares: Vec<u8>,
}
#[allow(dead_code)]
impl Grid {
    pub fn new(w: usize, h: usize) -> Grid {
        let size = (w, h);
        let squares = vec![b'.'; w * h];

        Grid { size, squares }
    }

    pub fn from_layout(layout: &str) -> Grid {
        let (size, squares) = layout.trim().split('\n').map(|l| l.trim()).fold(
            ((0, 0), Vec::with_capacity(layout.len())),
            |((_, h), mut layout), line| {
                layout.extend_from_slice(line.as_bytes());
                ((line.len(), h + 1), layout)
            },
        );

        Grid { size, squares }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.squares
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    fn index_of(&self, pos: Point2<usize>) -> usize {
        let (x, y) = (pos[0], pos[1]);
        let (w, h) = self.size;
        if x < w && y < h {
            x + y * w
        } else {
            usize::MAX
        }
    }

    pub fn get<P: Into<Point2<usize>>>(&self, pos: P) -> Option<u8> {
        let i = self.index_of(pos.into());
        self.squares.get(i).cloned()
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item = (Point2<usize>, u8)> {
        let (w, _) = self.size;
        self.squares
            .iter()
            .cloned()
            .enumerate()
            .map(move |(i, v)| ([i % w, i / w].into(), v))
    }

    pub fn iter_mut(&mut self) -> impl '_ + Iterator<Item = (Point2<usize>, &'_ mut u8)> {
        let (w, _) = self.size;
        self.squares
            .iter_mut()
            .enumerate()
            .map(move |(i, v)| ([i % w, i / w].into(), v))
    }
}

impl<P: Into<Point2<usize>>> Index<P> for Grid {
    type Output = u8;
    fn index(&self, index: P) -> &Self::Output {
        let i = self.index_of(index.into());
        self.squares.index(i)
    }
}

impl<P: Into<Point2<usize>>> IndexMut<P> for Grid {
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        let i = self.index_of(index.into());
        self.squares.index_mut(i)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (w, h) = self.size;
        for j in 0..h {
            let slice = &self.squares[j * w..(j + 1) * w];
            writeln!(f, "{}", unsafe { str::from_utf8_unchecked(slice) })?;
        }
        Ok(())
    }
}
#[allow(dead_code)]
pub fn adjacent4(pos: Point2<usize>) -> impl Iterator<Item = Point2<usize>> {
    adjacent_helper(pos, &[(0, 1), (-1, 0), (1, 0), (0, -1)])
}
#[allow(dead_code)]
pub fn adjacent8(pos: Point2<usize>) -> impl Iterator<Item = Point2<usize>> {
    adjacent_helper(
        pos,
        &[
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ],
    )
}
#[allow(dead_code)]
pub fn adjacent_helper(
    pos: Point2<usize>,
    neighbours: &[(isize, isize)],
) -> impl Iterator<Item = Point2<usize>> + '_ {
    neighbours
        .iter()
        .map(move |(dx, dy)| ((pos[0] as isize) + *dx, (pos[1] as isize) + *dy))
        .map(|(x, y)| Point2::new(x as usize, y as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_as_point() {
        assert_eq!(Direction::Up.as_point(2), Point::new(0, 2));
        assert_eq!(Direction::Left.as_point(3), Point::new(-3, 0));
        assert_eq!(Direction::Down.as_point(4), Point::new(0, -4));
        assert_eq!(Direction::Right.as_point(5), Point::new(5, 0));
    }
}
