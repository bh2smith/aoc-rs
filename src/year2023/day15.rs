use itertools::Itertools;

fn hash(word: &str) -> usize {
    let mut res = 0u8;
    for ch in word.chars() {
        res = res.wrapping_add(ch as u8);
        res = res.wrapping_mul(17)
    }
    res as usize
}

#[derive(Debug)]
enum Operation {
    Remove,
    Add(usize),
}

#[derive(Debug)]
struct Step {
    label: String,
    operation: Operation,
}

impl Step {
    fn from(input: &str) -> Self {
        println!("parsing input {}", input);
        let label: String;
        let operation = match input.contains('=') {
            true => {
                label = input[..input.find('=').unwrap()].to_string();
                let digit = input.chars().last().unwrap().to_digit(10).expect("msg");
                Operation::Add(digit as usize)
            }
            false => {
                label = input[..input.find('-').unwrap()].to_string();
                Operation::Remove
            }
        };
        Self { label, operation }
    }

    fn hash(&self) -> usize {
        hash(&self.label) as usize
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl Lens {
    fn new(label: &str, focal_length: usize) -> Self {
        Self {
            label: label.to_string(),
            focal_length,
        }
    }
}

#[derive(Debug)]
struct Boxes {
    boxes: Vec<Vec<Lens>>,
}

impl Boxes {
    fn new() -> Self {
        Self {
            boxes: vec![vec![]; 256],
        }
    }

    fn perform_steps(&mut self, steps: &[Step]) {
        for step in steps {
            self.perform_step(step);
        }
    }

    fn perform_step(&mut self, step: &Step) {
        match step.operation {
            Operation::Remove => self.remove(step.hash(), &step.label),
            Operation::Add(focal_length) => self.add(step.hash(), &step.label, focal_length),
        }
    }

    fn add(&mut self, hash: usize, label: &str, focal_length: usize) {
        // Look to see if it already exists.
        for l in self.boxes[hash].iter_mut() {
            if &l.label == label {
                // If we find it, we just need to update the focal
                // length.
                l.focal_length = focal_length;
                return;
            }
        }
        // Otherwise, we need to add it.
        let lens = Lens::new(label, focal_length);
        self.boxes[hash].push(lens);
    }

    fn remove(&mut self, hash: usize, label: &str) {
        // Try to find it.
        for (i, l) in self.boxes[hash].iter().enumerate() {
            if &l.label == label {
                // If we find it, we need to remove it.
                self.boxes[hash].remove(i);
                return;
            }
        }
    }

    fn focusing_power(&self) -> usize {
        let mut power = 0;
        // in rust: box is a keyword.
        for (box_number, bx) in self.boxes.iter().enumerate() {
            for (slot, lens) in bx.iter().enumerate() {
                power += (box_number + 1) * (slot + 1) * lens.focal_length;
            }
        }
        power
    }
}

pub fn puzzle1(input: &str) -> usize {
    input.trim().split(',').map(hash).sum()
}

pub fn puzzle2(input: &str) -> usize {
    let steps = input.trim().split(',').map(Step::from).collect_vec();
    let mut boxes = Boxes::new();
    boxes.perform_steps(&steps);
    boxes.focusing_power()
}

#[cfg(test)]
mod tests {

    const SAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash() {
        assert_eq!(super::hash("HASH"), 52)
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 1320);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 145);
    }
}
