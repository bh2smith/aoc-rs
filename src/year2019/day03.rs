use std::collections::HashSet;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point2 {
    x: i64,
    y: i64,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Segment2 {
    left: Point2,
    right: Point2,
}

impl Point2 {
    fn norm(self) -> i64 {
        self.manhattan(Point2 { x: 0, y: 0 })
    }
    fn manhattan(self, other: Point2) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    fn walk(self, direction: Point2) -> Point2 {
        Point2 {
            x: self.x + direction.x,
            y: self.y + direction.y,
        }
    }
}

impl Segment2 {
    fn new(p1: Point2, p2: Point2) -> Self {
        Segment2 {
            left: p1,
            right: p2,
        }
    }
    fn horizontal(self) -> bool {
        self.left.y == self.right.y
    }
    fn vertical(self) -> bool {
        self.left.x == self.right.x
    }
    fn perpendicular(self, other: Segment2) -> bool {
        self.horizontal() ^ other.horizontal()
    }
    fn reversed(self) -> Self {
        Segment2 {
            left: self.right,
            right: self.left,
        }
    }
    fn sorted(self) -> Self {
        if self.horizontal() && self.left.x > self.right.x
            || self.vertical() && self.left.y > self.right.y
        {
            self.reversed()
        } else {
            self
        }
    }
    fn crosses(self, other: Segment2) -> Option<Point2> {
        if self.perpendicular(other) {
            let (h, v) = if self.horizontal() {
                (self.sorted(), other.sorted())
            } else {
                (other.sorted(), self.sorted())
            };
            if h.left.x <= v.left.x
                && v.left.x <= h.right.x
                && v.left.y <= h.left.y
                && h.left.y <= v.right.y
            {
                Some(Point2 {
                    x: v.left.x,
                    y: h.left.y,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn parse_line(input: &str) -> Vec<(char, i64)> {
    input
        .split(',')
        .map(|s| (s.chars().next().unwrap(), i64::from_str(&s[1..]).unwrap()))
        .collect()
}

pub fn parse_input(input: &str) -> Vec<Vec<(char, i64)>> {
    input
        .trim()
        .split('\n')
        .map(|line| parse_line(line.trim()))
        .collect()
}

pub fn construct_points(travel_logs: Vec<Vec<(char, i64)>>) -> Vec<Vec<Point2>> {
    let mut points = vec![vec![], vec![]];
    for (i, path) in travel_logs.iter().enumerate() {
        let mut pos = Point2 { x: 0, y: 0 };
        for (d, num) in path.iter() {
            pos = match d {
                'U' => pos.walk(Point2 { x: 0, y: *num }),
                'D' => pos.walk(Point2 { x: 0, y: -*num }),
                'L' => pos.walk(Point2 { x: -*num, y: 0 }),
                'R' => pos.walk(Point2 { x: *num, y: 0 }),
                _ => panic!("Received unexpected direction!"),
            };
            points[i].push(Point2 { x: pos.x, y: pos.y });
        }
    }
    points
}

pub fn puzzle1(input: &str) -> i64 {
    let points = construct_points(parse_input(input));

    let mut intersection = HashSet::new();
    for i in 1..points[0].len() {
        let pair = Segment2::new(points[0][i - 1], points[0][i]);
        for j in 1..points[1].len() {
            let other_pair = Segment2::new(points[1][j - 1], points[1][j]);
            if let Some(crossing) = pair.crosses(other_pair) {
                intersection.insert(crossing);
            }
        }
    }
    intersection.iter().map(|point| point.norm()).min().unwrap()
}

pub fn puzzle2(input: &str) -> i64 {
    let points = construct_points(parse_input(input));

    let mut crossing_steps = HashSet::new();

    let mut first_steps = points[0][0].norm();
    let mut second_steps;
    for i in 1..points[0].len() {
        let first_segment = Segment2::new(points[0][i - 1], points[0][i]);
        second_steps = points[1][0].norm();
        for j in 1..points[1].len() {
            let other_segment = Segment2::new(points[1][j - 1], points[1][j]);
            if let Some(crossing) = first_segment.crosses(other_segment) {
                crossing_steps.insert(
                    first_steps
                        + second_steps
                        + crossing.manhattan(first_segment.left)
                        + crossing.manhattan(other_segment.left),
                );
            }
            second_steps += other_segment.left.manhattan(other_segment.right);
        }
        first_steps += first_segment.left.manhattan(first_segment.right);
    }
    *crossing_steps.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn input_parser() {
        assert_eq!(
            super::parse_line("R1000,U371,R195"),
            vec![('R', 1000), ('U', 371), ('R', 195)]
        );
        let input = "R75,D30,R83,U83,L12,D49,R71\nU62,R66,U55,R34,D71";
        let expected = vec![
            vec![
                ('R', 75),
                ('D', 30),
                ('R', 83),
                ('U', 83),
                ('L', 12),
                ('D', 49),
                ('R', 71),
            ],
            vec![('U', 62), ('R', 66), ('U', 55), ('R', 34), ('D', 71)],
        ];
        assert_eq!(super::parse_input(input), expected);
    }

    #[test]
    fn puzzle1() {
        assert_eq!(
            super::puzzle1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
        assert_eq!(
            super::puzzle1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn puzzle2() {
        assert_eq!(
            super::puzzle2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );
        assert_eq!(
            super::puzzle2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
