use std::cmp::Ordering;

#[derive(Default)]
struct Reindeer {
    fly: u32,
    rest: u32,
    speed: u32,
    second: u32,
    position: u32,
}
impl Reindeer {
    fn new(fly: u32, rest: u32, speed: u32) -> Self {
        Self {
            fly,
            rest,
            speed,
            ..Default::default()
        }
    }
    fn increment_position(&mut self) {
        self.second += 1;
        let k = self.second / (self.fly + self.rest);
        let diff = self.second - k * (self.fly + self.rest);
        match diff > 0 && diff <= self.fly {
            true => self.position += self.speed,
            false => (),
        }
    }
    fn position_at_second(&self, second: u32) -> u32 {
        let Reindeer {
            fly,
            rest,
            speed,
            second: _,
            position: _,
        } = self;
        if second <= *fly {
            return speed * second;
        }
        let k = second / (fly + rest);
        let mut extra = 0;
        if k * (fly + rest) < second {
            let bonus_time = (second - (k * (fly + rest))).min(*fly);
            extra = bonus_time * speed;
        }
        k * (fly * speed) + extra
    }
}

pub fn puzzle1(input: &str) -> u32 {
    let seconds = input.parse::<u32>().unwrap();
    let comet = Reindeer::new(10, 127, 14);
    let dancer = Reindeer::new(11, 162, 16);
    comet
        .position_at_second(seconds)
        .max(dancer.position_at_second(seconds))
}

pub fn puzzle2(input: &str) -> u32 {
    let seconds = input.parse::<u32>().unwrap();
    let mut comet = Reindeer::new(10, 127, 14);
    let mut dancer = Reindeer::new(11, 162, 16);

    let mut d_points = 0;
    let mut c_points = 0;
    for second in 1..seconds + 1 {
        comet.increment_position();
        dancer.increment_position();
        let d_pos = dancer.position;
        let c_pos = comet.position;

        match c_pos.cmp(&d_pos) {
            Ordering::Greater => c_points += 1,
            Ordering::Less => d_points += 1,
            Ordering::Equal => {
                println!("\n*** Tied at {second}");
                d_points += 1;
                c_points += 1;
            }
        }
        assert_eq!(
            comet.position_at_second(second),
            comet.position,
            "comet failure"
        );
        assert_eq!(
            dancer.position_at_second(second),
            dancer.position,
            "dasher failure"
        );
        println!("Second {second}: D at {d_pos}, C at {c_pos} with {d_points} vs {c_points}");
    }

    d_points.max(c_points)
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "1000";
    const REAL_INPUT: &str = "2503";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 1120);
        assert_eq!(super::puzzle1(REAL_INPUT), 2660);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 689);
        // Too high!
        // 1351 (too low)
        assert_eq!(super::puzzle2(REAL_INPUT), 1564);
    }
}
