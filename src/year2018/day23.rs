use crate::util;
use failure::Error;
use lazy_static::lazy_static;
use nalgebra::{zero, Point3, Vector3};
use regex::Regex;
use std::cmp;
use std::i64;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Nanobot(Point3<i64>, i64);

impl Nanobot {
    #[inline]
    fn is_in_range(&self, other: &Nanobot) -> bool {
        self.contains(other.0)
    }

    #[inline]
    fn contains(&self, p: Point3<i64>) -> bool {
        manhattan_distance(self.0, p) <= self.1
    }
}

lazy_static! {
    static ref NANOBOT_PATTERN: Regex =
        Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
}

impl FromStr for Nanobot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = NANOBOT_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match nanobot pattern"))?;

        Ok(Nanobot(
            Point3::new(c[1].parse()?, c[2].parse()?, c[3].parse()?),
            c[4].parse()?,
        ))
    }
}

#[inline]
fn manhattan_distance(a: Point3<i64>, b: Point3<i64>) -> i64 {
    let c = a - b;
    c[0].abs() + c[1].abs() + c[2].abs()
}

pub fn puzzle1(input: &str) -> usize {
    let nanobots = util::parse::<Nanobot>(input).collect::<Vec<_>>();
    let strongest = nanobots.iter().max_by_key(|n| n.1).expect("no nanobots");

    nanobots.iter().filter(|n| strongest.is_in_range(n)).count()
}

pub fn puzzle2(input: &str) -> i64 {
    let nanobots = util::parse::<Nanobot>(input).collect::<Vec<_>>();
    let (mut min, mut max) = nanobots[1..].iter().fold(
        (nanobots[0].0, nanobots[0].0),
        |(min, max), Nanobot(p, _)| {
            (
                [
                    cmp::min(min[0], p[0]),
                    cmp::min(min[1], p[1]),
                    cmp::min(min[2], p[2]),
                ]
                .into(),
                [
                    cmp::max(max[0], p[0]),
                    cmp::max(max[1], p[1]),
                    cmp::max(max[2], p[2]),
                ]
                .into(),
            )
        },
    );

    let mut scale = 1;
    while (max.coords - min.coords) / 2 != nalgebra::zero::<Vector3<i64>>() {
        min /= 2;
        max /= 2;
        scale *= 2;
    }

    let origin: Point3<i64> = [0, 0, 0].into();
    let mut scaled = nanobots.clone();
    let mut best = (0usize, 0i64, origin);
    while scale > 0 {
        for (s, n) in scaled.iter_mut().zip(nanobots.iter()) {
            *s = Nanobot(n.0 / scale, n.1 / scale);
        }

        best = (0, i64::MAX, origin);
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let p = Point3::new(x, y, z);
                    let n = scaled.iter().filter(|n| n.contains(p)).count();
                    let m = manhattan_distance(p, origin);
                    if n > best.0 || (n == best.0 && m < best.1) {
                        best = (n, m, p);
                    }
                }
            }
        }

        min = (best.2 - Vector3::new(1, 1, 1)) * 2;
        max = (best.2 + Vector3::new(1, 1, 1)) * 2;
        scale /= 2;
    }

    best.1
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        const INPUT: &str = r"
            pos=<0,0,0>, r=4
            pos=<1,0,0>, r=1
            pos=<4,0,0>, r=3
            pos=<0,2,0>, r=1
            pos=<0,5,0>, r=3
            pos=<0,0,3>, r=1
            pos=<1,1,1>, r=1
            pos=<1,1,2>, r=1
            pos=<1,3,1>, r=1
        ";

        assert_eq!(super::puzzle1(INPUT), 7);
    }

    #[test]
    fn puzzle2() {
        const INPUT: &str = r"
            pos=<10,12,12>, r=2
            pos=<12,14,12>, r=2
            pos=<16,12,12>, r=4
            pos=<14,14,14>, r=6
            pos=<50,50,50>, r=200
            pos=<10,10,10>, r=5
        ";

        assert_eq!(super::puzzle2(INPUT), 36);
    }
}
