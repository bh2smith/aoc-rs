use itertools::Itertools;
use nalgebra::Point2;

type P = Point2<i64>;

fn cross(o: &P, a: &P, b: &P) -> i64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

fn point_on_segment(a: &P, b: &P, q: &P) -> bool {
    // collinear?
    if cross(a, b, q) != 0 {
        return false;
    }
    // within bounding box?
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);

    q.x >= min_x && q.x <= max_x && q.y >= min_y && q.y <= max_y
}

fn polygon_edges(poly: &[P]) -> Vec<(P, P)> {
    let n = poly.len();
    (0..n).map(|i| (poly[i], poly[(i + 1) % n])).collect()
}

fn segments_properly_intersect(p1: &P, p2: &P, q1: &P, q2: &P) -> bool {
    let c1 = cross(p1, p2, q1);
    let c2 = cross(p1, p2, q2);
    let c3 = cross(q1, q2, p1);
    let c4 = cross(q1, q2, p2);

    // Proper intersection: strictly opposite sides on both segments
    if c1 == 0 && point_on_segment(p1, p2, q1) {
        return false;
    }
    if c2 == 0 && point_on_segment(p1, p2, q2) {
        return false;
    }
    if c3 == 0 && point_on_segment(q1, q2, p1) {
        return false;
    }
    if c4 == 0 && point_on_segment(q1, q2, p2) {
        return false;
    }

    (c1 > 0 && c2 < 0 || c1 < 0 && c2 > 0) && (c3 > 0 && c4 < 0 || c3 < 0 && c4 > 0)
}

struct Box {
    bottom_left: P,
    length: u64,
    width: u64,
}

impl Box {
    fn new(a: P, b: P) -> Self {
        let x0 = a[0].min(b[0]);
        let y0 = a[1].min(b[1]);
        let x1 = a[0].max(b[0]);
        let y1 = a[1].max(b[1]);

        let width = (x1 - x0 + 1) as u64;
        let length = (y1 - y0 + 1) as u64;

        Self {
            bottom_left: P::from([x0, y0]),
            length,
            width,
        }
    }

    fn area(&self) -> u64 {
        self.length * self.width
    }

    fn corners(&self) -> Vec<P> {
        vec![
            self.bottom_left,
            P::from([
                self.bottom_left[0] + self.width as i64 - 1,
                self.bottom_left[1],
            ]),
            P::from([
                self.bottom_left[0],
                self.bottom_left[1] + self.length as i64 - 1,
            ]),
            P::from([
                self.bottom_left[0] + self.width as i64 - 1,
                self.bottom_left[1] + self.length as i64 - 1,
            ]),
        ]
    }

    fn edges(&self) -> [(P, P); 4] {
        let c = self.corners();
        // c[0] = bottom-left
        // c[1] = bottom-right
        // c[2] = top-left
        // c[3] = top-right
        [
            (c[0], c[1]), // bottom
            (c[0], c[2]), // left
            (c[2], c[3]), // top
            (c[1], c[3]), // right
        ]
    }

    fn in_range(&self, poly: &[P]) -> bool {
        // 1. Early exit if corners are not all contained.
        if !self.corners().iter().all(|c| point_in_polygon(poly, *c)) {
            return false;
        }

        // 2. None of the 4 edges may cross the polygon boundary
        let box_edges = self.edges();
        let poly_edges = polygon_edges(poly);

        for (b1, b2) in box_edges.iter() {
            for (p1, p2) in poly_edges.iter() {
                if segments_properly_intersect(b1, b2, p1, p2) {
                    return false;
                }
            }
        }

        true
    }
}

fn point_in_polygon(poly: &[P], q: P) -> bool {
    let n = poly.len();
    if n == 0 {
        return false;
    }
    if n == 1 {
        return q == poly[0];
    }

    // First, check if q lies exactly on any polygon edge
    for i in 0..n {
        let a = &poly[i];
        let b = &poly[(i + 1) % n];
        if point_on_segment(a, b, &q) {
            return true; // on boundary = inside
        }
    }

    // Ray casting to the right
    let mut inside = false;
    for i in 0..n {
        let a = &poly[i];
        let b = &poly[(i + 1) % n];

        // Ensure a.y <= b.y for this edge in logic
        let (ay, by) = (a.y, b.y);
        let (ax, bx) = (a.x, b.x);

        // Check if edge straddles horizontal ray at q.y
        let intersects = ((ay > q.y) != (by > q.y))
            && (q.x < (bx - ax) * (q.y - ay) / (by - ay) + ax);

        if intersects {
            inside = !inside;
        }
    }

    inside
}

fn parse_input(input: &str) -> Vec<P> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            P::from([x.parse().unwrap(), y.parse().unwrap()])
        })
        .collect()
}

pub fn puzzle1(input: &str) -> isize {
    let points: Vec<P> = parse_input(input);
    points
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let l = (a[0] as isize - b[0] as isize).abs() + 1;
            let w = (a[1] as isize - b[1] as isize).abs() + 1;
            // println!("a={}, b={} : area = {}", a, b, l* w);
            l * w
        })
        .max()
        .unwrap()
}

pub fn puzzle2(input: &str) -> u64 {
    let points: Vec<P> = parse_input(input);

    points
        .clone()
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let b = Box::new(a, b);
            if b.in_range(&points) {
                b.area()
            } else {
                0
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    const SAMPLE_INPUT: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 50);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 24);
    }

    #[test]
    fn box_test() {
        let hull = [[2, 3], [7, 1], [11, 1], [11, 7], [9, 7], [2, 5]]
            .iter()
            .map(|&x| super::P::from(x))
            .collect_vec();
        let b = super::Box::new(super::P::from([9, 5]), super::P::from([2, 3]));
        assert!(b.in_range(&hull));
    }
}
