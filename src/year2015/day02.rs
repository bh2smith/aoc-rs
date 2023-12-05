use itertools::Itertools;

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(input: &str) -> Self {
        let coords = input.trim().split('x').collect_vec();
        Self {
            length: coords[0].parse().expect("exists"),
            width: coords[1].parse().expect("exists"),
            height: coords[2].parse().expect("exists"),
        }
    }

    fn surface_area(&self) -> u32 {
        let Self {
            length: l,
            width: w,
            height: h,
        } = self;
        2 * (l * w + w * h + h * l)
    }

    fn slack(&self) -> u32 {
        let Self {
            length: l,
            width: w,
            height: h,
        } = self;
        *[l * w, l * h, h * w].iter().min().expect("exists")
    }

    fn smallest_perimeter(&self) -> u32 {
        let Self {
            length: l,
            width: w,
            height: h,
        } = self;
        2 * [l + w, l + h, h + w].iter().min().expect("exists")
    }

    fn volume(&self) -> u32 {
        self.length * self.height * self.width
    }
}

pub fn puzzle1(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let p = Present::new(line);
            p.surface_area() + p.slack()
        })
        .sum()
}

pub fn puzzle2(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let p = Present::new(line);
            p.smallest_perimeter() + p.volume()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "2x3x4";
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 58);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 34);
    }
}
