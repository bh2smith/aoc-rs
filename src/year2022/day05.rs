use std::str::FromStr;
use std::usize;
use itertools::Itertools;

#[derive(PartialEq, Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(input: &[&str]) -> Vec<Vec<char>> {
    // Remove last entry for stack size
    let stack_char = input.last().unwrap().trim().chars().last().unwrap();
    let num_stacks = usize::from_str(&stack_char.to_string()).unwrap();

    let rev_stack_rows: Vec<_> = input.iter().rev().collect();
    println!("Found {} stacks", num_stacks);
    let mut stacks = vec![vec![]; num_stacks];
    for row in rev_stack_rows[1..].iter() {
        let mut k = 0usize;
        println!("Row {:?}", row);
        while 1 + 4 * k < row.len() {

            let entry = row.chars().nth(1 + 4 * k).unwrap();
            println!("    key={}, entry={}", k, entry);
            if entry != ' ' {
                stacks[k].push(entry)
            }
            k += 1;
        }
    }
    stacks
}

fn parse_moves(input: &[Vec<&str>]) -> Vec<Move> {
    let mut result: Vec<Move> = vec![];
    for row in input.iter() {
        if row.len() > 1 {
            result.push(Move {
                count: usize::from_str(row.get(1).unwrap()).unwrap(),
                from: usize::from_str(row.get(3).unwrap()).unwrap() - 1,
                to:usize::from_str(row.get(5).unwrap()).unwrap() - 1,
            })
        }

    }
    result.reverse();
    result
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let rows: Vec<_> = input.split('\n').collect();
    println!("Got {} rows", rows.len());
    let midpoint = rows.iter().position(|x| x.is_empty()).unwrap();
    println!("Splitting at stacks and moves at {}", midpoint);
    let (stack_rows, move_rows) = rows.split_at(midpoint);
    let parsed_moves: Vec<Vec<_>> = move_rows.iter().map(|s| s.split(' ').collect()).collect();

    (parse_stacks(stack_rows), parse_moves(&parsed_moves[1..]))
}

fn move_crates(move_item: Move, mut stacks: Vec<Vec<char>>, singles: bool) -> Vec<Vec<char>> {
    if singles {
        for _ in 0..move_item.count {

            let to_move = stacks[move_item.from].pop().unwrap();
            stacks[move_item.to].push(to_move);
        }
    } else {
        let remove_index = stacks[move_item.from].len() - move_item.count;
        let stack = stacks[move_item.from].clone();
        let (left, right) = stack.split_at(remove_index);
        stacks[move_item.from] = left.to_vec();
        stacks[move_item.to].append(&mut right.to_vec());
    }
    stacks
}

pub fn puzzle1(input: &str) -> String {
    let (mut stacks,mut moves) = parse_input(input);
    while let Some(move_item) = moves.pop()  {
        println!("Applying move {:?}", move_item);
        println!("stacks before {:?}", stacks);
        stacks = move_crates(move_item,stacks.clone(), true);
        println!("stacks after move: {:?}", stacks);
    }
    stacks.iter().map(|stack| stack.last().unwrap()).join("")
}

pub fn puzzle2(input: &str) -> String {
    let (mut stacks,mut moves) = parse_input(input);
    while let Some(move_item) = moves.pop()  {
        println!("Applying move {:?}", move_item);
        println!("stacks before {:?}", stacks);
        stacks = move_crates(move_item,stacks.clone(), false);
        println!("stacks after move: {:?}", stacks);
    }
    stacks.iter().map(|stack| stack.last().unwrap()).join("")
}

#[cfg(test)]
mod tests {
    use crate::year2022::day05::{Move, parse_moves, parse_stacks};

    const SAMPLE_INPUT: &str = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_stacks() {
        let test_vec = [
            "[D]",
            "[N] [C]",
            "[Z] [M] [P]",
            "1", "", "", "2", "", "", "3",
        ];
        assert_eq!(
            parse_stacks(&test_vec),
            vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]
        )
    }

    #[test]
    fn test_parse_moves() {
        let test_vec = [
            vec!["move", "1", "from", "2", "to", "1"],
        ];
        assert_eq!(
            parse_moves(&test_vec),
            vec![Move{ count: 1, from: 1, to: 0 }]
        )
    }


    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), "CMZ");
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), "MCD");
    }
}
