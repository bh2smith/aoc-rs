use nalgebra::Point2;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::str::{self, FromStr};
use std::usize;

pub fn split<'a>(input: &'a str) -> impl 'a + Iterator<Item = &'a str> {
    input.trim().split('\n').map(|s| s.trim())
}

pub fn parse_csv(input: &str) -> Vec<i64> {
    input.split(',').map(|s| i64::from_str(s.trim()).unwrap()).collect()
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

    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (Point2<usize>, u8)> {
        let (w, _) = self.size;
        self.squares
            .iter()
            .cloned()
            .enumerate()
            .map(move |(i, v)| ([i % w, i / w].into(), v))
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl 'a + Iterator<Item = (Point2<usize>, &'a mut u8)> {
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

pub fn adjacent4(pos: Point2<usize>) -> impl Iterator<Item = Point2<usize>> {
    adjacent_helper(pos, &[(0, 1), (-1, 0), (1, 0), (0, -1)])
}

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

pub fn adjacent_helper(
    pos: Point2<usize>,
    neighbours: &[(isize, isize)],
) -> impl Iterator<Item = Point2<usize>> + '_ {
    neighbours
        .iter()
        .map(move |(dx, dy)| ((pos[0] as isize) + *dx, (pos[1] as isize) + *dy))
        .map(|(x, y)| Point2::new(x as usize, y as usize))
}
