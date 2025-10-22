use crate::util;
use failure::Error;
use lazy_static::lazy_static;
use nalgebra::Vector2;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::str::FromStr;

type Scalar = i32;

#[derive(Debug)]
struct Point {
    pos: Vector2<Scalar>,
    v: Vector2<Scalar>,
}

impl Point {
    fn step(&mut self) {
        self.pos += self.v;
    }
}

lazy_static! {
    static ref POINT_PATTERN: Regex =
        Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = POINT_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match point pattern"))?;

        let pos = Vector2::new(c[1].parse()?, c[2].parse()?);
        let v = Vector2::new(c[3].parse()?, c[4].parse()?);

        Ok(Point { pos, v })
    }
}

struct TextRecognizer {
    points: HashSet<Vector2<Scalar>>,
    segment_len: usize,
}

impl TextRecognizer {
    fn new() -> TextRecognizer {
        TextRecognizer {
            points: HashSet::new(),
            segment_len: 8, // some arbitrary length
        }
    }

    fn has_text(&mut self, points: &[Point]) -> bool {
        // very naive text recognizer algorithm that checks to see all points
        // are in segments that are at least `segment_len` long and without any
        // duplicate coordinates

        self.points.clear();
        self.points.extend(points.iter().map(|p| p.pos));

        let mut segment_len;
        let mut segment = Vec::new();
        let dirs = vec![
            Vector2::new(0, -1),
            Vector2::new(1, -1),
            Vector2::new(1, 0),
            Vector2::new(1, 1),
            Vector2::new(0, 1),
            Vector2::new(-1, 1),
            Vector2::new(-1, 0),
            Vector2::new(-1, -1),
        ];

        for point in points.iter().map(|p| p.pos) {
            if !self.points.remove(&point) {
                continue;
            }

            segment_len = 1;
            segment.push(point);
            while let Some(current) = segment.pop() {
                for dir in dirs.iter() {
                    let next = current + dir;
                    if self.points.remove(&next) {
                        segment_len += 1;
                        segment.push(next);
                    }
                }
            }

            if segment_len < self.segment_len {
                return false;
            }
        }

        true
    }
}

fn points_to_string(mut points: Vec<Point>) -> String {
    let (minx, miny, maxx, maxy) = points.iter().fold(
        (
            Scalar::max_value(),
            Scalar::max_value(),
            Scalar::min_value(),
            Scalar::min_value(),
        ),
        |o, p| {
            (
                cmp::min(o.0, p.pos.x),
                cmp::min(o.1, p.pos.y),
                cmp::max(o.2, p.pos.x),
                cmp::max(o.3, p.pos.y),
            )
        },
    );

    let origin = Vector2::new(minx, miny);
    let max = Vector2::new(maxx, maxy) - origin;

    points.sort_unstable_by_key(|p| (p.pos.y, p.pos.x));

    let mut result = String::with_capacity(1 + (2 + max.x * max.y) as usize);
    result.push('\n');

    let (mut i, mut j) = (0, 0);
    for (x, y) in points
        .into_iter()
        .map(|p| p.pos - origin)
        .map(|p| (p.x, p.y))
    {
        while j < y {
            for _ in i..=max.x {
                result.push('.');
            }
            i = 0;

            result.push('\n');
            j += 1;
        }
        while i < x {
            result.push('.');
            i += 1;
        }

        if i == x {
            result.push('#');
            i += 1;
        }
    }
    for _ in i..=max.x {
        result.push('.');
    }

    result
}

fn puzzle(input: &str) -> (usize, String) {
    let mut points = util::parse::<Point>(input).collect::<Vec<_>>();
    let mut recognizer = TextRecognizer::new();

    let mut steps = 0;
    while !recognizer.has_text(&points) {
        for p in points.iter_mut() {
            p.step();
        }
        steps += 1;
    }

    (steps, points_to_string(points))
}

pub fn puzzle1(input: &str) -> String {
    let (_, result) = puzzle(input);
    result
}

pub fn puzzle2(input: &str) -> usize {
    let (result, _) = puzzle(input);
    result
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        position=< 9,  1> velocity=< 0,  2>
        position=< 7,  0> velocity=<-1,  0>
        position=< 3, -2> velocity=<-1,  1>
        position=< 6, 10> velocity=<-2, -1>
        position=< 2, -4> velocity=< 2,  2>
        position=<-6, 10> velocity=< 2, -2>
        position=< 1,  8> velocity=< 1, -1>
        position=< 1,  7> velocity=< 1,  0>
        position=<-3, 11> velocity=< 1, -2>
        position=< 7,  6> velocity=<-1, -1>
        position=<-2,  3> velocity=< 1,  0>
        position=<-4,  3> velocity=< 2,  0>
        position=<10, -3> velocity=<-1,  1>
        position=< 5, 11> velocity=< 1, -2>
        position=< 4,  7> velocity=< 0, -1>
        position=< 8, -2> velocity=< 0,  1>
        position=<15,  0> velocity=<-2,  0>
        position=< 1,  6> velocity=< 1,  0>
        position=< 8,  9> velocity=< 0, -1>
        position=< 3,  3> velocity=<-1,  1>
        position=< 0,  5> velocity=< 0, -1>
        position=<-2,  2> velocity=< 2,  0>
        position=< 5, -2> velocity=< 1,  2>
        position=< 1,  4> velocity=< 2,  1>
        position=<-2,  7> velocity=< 2, -2>
        position=< 3,  6> velocity=<-1, -1>
        position=< 5,  0> velocity=< 1,  0>
        position=<-6,  0> velocity=< 2,  0>
        position=< 5,  9> velocity=< 1, -2>
        position=<14,  7> velocity=<-2,  0>
        position=<-3,  6> velocity=< 2, -1>
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(
            &super::puzzle1(INPUT),
            r"
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###"
        );
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(INPUT), 3);
    }
}
