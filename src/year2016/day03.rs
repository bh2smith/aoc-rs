use itertools::Itertools;
use std::convert::TryInto;


#[derive(Debug)]
struct Trip {
  x: u32, 
  y: u32,
  z: u32
}

impl Trip {
  fn triangle_inequality(&self) -> bool {
    self.x + self.y > self.z
  }
}

impl From<&str> for Trip {
    fn from(s: &str) -> Self {
      let nums = s
                .split_whitespace()
                .map(|t| t.parse::<u32>().unwrap())
                .collect_vec();
        // Convert Vec<u32> -> [u32; 3] -> (u32, u32, u32)
        let mut arr: [u32; 3] = nums.try_into().expect("Each line must have exactly 3 numbers");
        // Largest will always be z.
        arr.sort();
        Self {
          x: arr[0],
          y: arr[1],
          z: arr[2]
        }
    }
}



pub fn puzzle1(input: &str) -> usize {
    // parse rows:
    let trips = input.trim().lines().map(Trip::from).collect_vec();
    trips.iter().filter(|t| t.triangle_inequality()).count()
}

pub fn puzzle2(input: &str) -> usize {

    // parsing colum defined trips
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut trips = Vec::new();

    // Process in chunks of 3 rows at a time
    for chunk in rows.chunks(3) {
        if chunk.len() == 3 {
            // Transpose: zip each column downward
            for i in 0..chunk[0].len() {
            let line = format!("{} {} {}", chunk[0][i], chunk[1][i], chunk[2][i]);
            trips.push(Trip::from(line.as_str()));
        }
        }
    }
    trips.iter().filter(|t| t.triangle_inequality()).count()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "5 10 25";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}
