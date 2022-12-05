use std::collections::{HashMap, VecDeque};

pub type Stacks = HashMap<usize, VecDeque<char>>;

#[derive(Debug, Clone)]
pub struct Input {
    stacks: Stacks,
    moves: Vec<Move>,
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let (stack_text, move_text) = input.split_once("\n\n").unwrap();
    let mut stacks = HashMap::new();
    for mut line in stack_text.lines() {
        if line.starts_with(" 1 ") {
            break;
        }
        let mut i = 1usize;
        while line.len() >= 3 {
            let (chunk, rest) = line.split_at(3);
            let stack: &mut VecDeque<char> = stacks.entry(i).or_default();
            if chunk == "   " {
                // empty
            } else if let Some(s) = chunk.strip_prefix('[') {
                // crate
                let c = s.chars().next().unwrap();
                stack.push_back(c);
            } else {
                panic!("invalid stack: {}", line);
            }
            line = rest.strip_prefix(' ').unwrap_or_default();
            i += 1;
        }
    }
    let moves = move_text.trim().lines().map(|line| {
        match line.split(' ').collect::<Vec<_>>()[..] {
            [_, amount, _, from, _, to, ..] => {
                Move {
                    amount: amount.parse().unwrap(),
                    from: from.parse().unwrap(),
                    to: to.parse().unwrap(),
                }
            }
            _ => panic!("invalid move: {}", line)
        }
    }).collect();
    Input { stacks, moves }
}

impl Move {
    fn perform_part1(&self, stacks: &mut Stacks) {
        for _ in 1..=self.amount {
            let moved_crate = stacks.get_mut(&self.from).unwrap().pop_front().unwrap();
            stacks.get_mut(&self.to).unwrap().push_front(moved_crate);
        }
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> String {
    let mut input = input.clone();
    for mv in input.moves {
        mv.perform_part1(&mut input.stacks);
    }
    let mut result = String::new();
    for i in 1..=input.stacks.len() {
        result.push(*input.stacks.get(&i).unwrap().front().unwrap());
    }
    result
}

impl Move {
    fn perform_part2(&self, stacks: &mut Stacks) {
        let moved_crates = stacks.get_mut(&self.from).unwrap().drain(0..self.amount).collect::<Vec<_>>();
        let to_stack = stacks.get_mut(&self.to).unwrap();
        for c in moved_crates.into_iter().rev() {
            to_stack.push_front(c);
        }
    }
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> String {
    let mut input = input.clone();
    for mv in input.moves {
        mv.perform_part2(&mut input.stacks);
    }
    let mut result = String::new();
    for i in 1..=input.stacks.len() {
        result.push(*input.stacks.get(&i).unwrap().front().unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        ";
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(&part1(&input), "CMZ");
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(&part2(&input), "MCD");
    }
}
