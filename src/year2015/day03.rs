// from collections import defaultdict

// direction_map = {'^': 1j, 'v': -1j, '>': 1, '<': -1}

// if __name__ == '__main__':
//     directions = open('input').read().strip()

//     visited = defaultdict(int)
//     pos = 0
//     for d in directions:
//         visited[pos] += 1
//         pos += direction_map[d]

//     print("part 1:", len(visited.keys()))

//     santa = defaultdict(int)
//     robot = defaultdict(int)
//     s_pos, r_pos = 0, 0
//     santa[0] = robot[0] = 1
//     for i, d in enumerate(directions):
//         if i % 2:
//             r_pos += direction_map[d]
//             robot[r_pos] += 1
//         else:
//             s_pos += direction_map[d]
//             santa[s_pos] += 1

//     print("part 2:", len(santa.keys() | robot.keys()))
use num::complex::Complex;

pub fn puzzle1(_input: &str) -> i64 {
    0
}

pub fn puzzle2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
