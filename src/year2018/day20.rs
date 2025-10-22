use bitflags::bitflags;
use nalgebra::{Point2, Vector2};
use std::cmp;
use std::collections::HashMap;
use std::isize;

#[derive(Clone, Copy, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn values() -> impl Iterator<Item = Dir> {
        static DIRS: [Dir; 4] = [Dir::N, Dir::S, Dir::S, Dir::W];
        DIRS.iter().cloned()
    }

    fn opposite(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }

    fn as_dirs(&self) -> Dirs {
        match self {
            Dir::N => Dirs::N,
            Dir::E => Dirs::E,
            Dir::S => Dirs::S,
            Dir::W => Dirs::W,
        }
    }

    fn to_vector(&self) -> Vector2<isize> {
        match self {
            Dir::N => -Vector2::y(),
            Dir::E => Vector2::x(),
            Dir::S => Vector2::y(),
            Dir::W => -Vector2::x(),
        }
    }
}

bitflags! {
    #[derive(Copy, Clone)]
    struct Dirs: u8 {
        const N = 0x01;
        const E = 0x02;
        const S = 0x04;
        const W = 0x08;
    }
}

impl Dirs {
    fn dirs(&self) -> impl Iterator<Item = Dir> + '_ {
        let ds = self;
        Dir::values().filter(move |d| ds.contains(d.as_dirs()))
    }
}

#[derive(Copy, Clone)]
struct Room {
    doors: Dirs,
    distance: usize,
}

fn puzzle(input: &str) -> HashMap<Point2<isize>, Room> {
    let steps = input.trim().as_bytes();
    if steps[0] != b'^' {
        panic!("unexpected '{}'", steps[0] as char);
    };

    let mut map = HashMap::new();
    let mut forks = Vec::new();
    let mut branches = Vec::new();
    let mut traversal = Vec::new();

    map.insert(
        Point2::<isize>::new(0, 0),
        Room {
            doors: Dirs::empty(),
            distance: 0,
        },
    );
    forks.push((1, Point2::<isize>::new(0, 0)));

    'outer: while let Some((mut i, mut pos)) = forks.pop() {
        loop {
            let step = steps[i];
            i += 1;

            let dir = match step {
                b'$' => {
                    continue 'outer;
                }
                b'N' => Dir::N,
                b'E' => Dir::E,
                b'S' => Dir::S,
                b'W' => Dir::W,
                b'(' => {
                    branches.push((forks.len(), pos));
                    continue;
                }
                b'|' => {
                    forks.push((i, pos));
                    pos = branches.last().expect("no branches").1;
                    continue;
                }
                b')' => {
                    let (n, _) = branches.pop().expect("no branches end");
                    for (ref mut j, _) in forks[n..].iter_mut() {
                        *j = i;
                    }
                    continue;
                }
                _ => unreachable!(),
            };

            let room = map.get_mut(&pos).expect("missing current room");
            room.doors.insert(dir.as_dirs());

            let distance = room.distance;
            traversal.push((pos, distance));

            pos += dir.to_vector();
            map.entry(pos).or_insert(Room {
                doors: dir.opposite().as_dirs(),
                distance: distance + 1,
            });

            while let Some((p, dist)) = traversal.pop() {
                let doors = map.get(&p).expect("missing current room").doors;
                for d in doors.dirs() {
                    let n = p + d.to_vector();
                    let r = map.get_mut(&n).expect("missing adjacent room");
                    if r.distance > dist + 1 {
                        r.distance = dist + 1;
                        traversal.push((n, dist + 1));
                    }
                }
            }
        }
    }

    map
}

pub fn puzzle1(input: &str) -> usize {
    puzzle(input)
        .values()
        .map(|r| r.distance)
        .max()
        .unwrap_or(0)
}

pub fn puzzle2(input: &str) -> usize {
    puzzle(input)
        .values()
        .filter(|r| r.distance >= 1000)
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("^ENWWW(NEEE|SSE(EE|N))$"), 10);
        assert_eq!(
            super::puzzle1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"),
            18
        );
        assert_eq!(
            super::puzzle1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"),
            31
        );
    }
}

#[allow(dead_code)]
fn debug_map(m: &HashMap<Point2<isize>, Room>) {
    let (xmin, xmax, ymin, ymax) = m.keys().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(xmin, xmax, ymin, ymax), p| {
            (
                cmp::min(xmin, p[0]),
                cmp::max(xmax, p[0]),
                cmp::min(ymin, p[1]),
                cmp::max(ymax, p[1]),
            )
        },
    );

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if m.contains_key(&Point2::new(x, y)) {
                let room = &m[&Point2::new(x, y)];
                if room.doors.contains(Dirs::N) {
                    print!("#--#");
                } else {
                    print!("####");
                }
            } else {
                print!("####");
            }
        }
        println!();

        for x in xmin..=xmax {
            if m.contains_key(&Point2::new(x, y)) {
                let room = &m[&Point2::new(x, y)];
                if room.doors.contains(Dirs::W) {
                    print!("|");
                } else {
                    print!("#");
                }
                print!("{:02}", room.distance);
                if room.doors.contains(Dirs::E) {
                    print!("|");
                } else {
                    print!("#");
                }
            } else {
                print!("###");
            }
        }
        println!();

        for x in xmin..=xmax {
            if m.contains_key(&Point2::new(x, y)) {
                let room = &m[&Point2::new(x, y)];
                if room.doors.contains(Dirs::S) {
                    print!("#--#");
                } else {
                    print!("####");
                }
            } else {
                print!("####");
            }
        }
        println!();
    }
}
