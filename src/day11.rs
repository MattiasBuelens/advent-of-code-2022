#[derive(Debug)]
pub struct Monkey {
    number: usize,
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    throw_if_true: usize,
    throw_if_false: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add(i32),
    Multiply(i32),
    Square,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for block in input.split("\n\n") {
        let mut lines = block.lines();
        let number = lines
            .next()
            .unwrap()
            .strip_prefix("Monkey ")
            .unwrap()
            .strip_suffix(":")
            .unwrap();
        let items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|item| item.parse::<i32>().unwrap())
            .collect();
        let operation = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap();
        let operation = if operation == "old * old" {
            Operation::Square
        } else if let Some(value) = operation.strip_prefix("old + ") {
            Operation::Add(value.parse().unwrap())
        } else if let Some(value) = operation.strip_prefix("old * ") {
            Operation::Multiply(value.parse().unwrap())
        } else {
            panic!("invalid operation: {}", operation)
        };
        let test = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap();
        let throw_if_true = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap();
        let throw_if_false = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap();
        monkeys.push(Monkey {
            number: number.parse().unwrap(),
            items,
            operation,
            test: test.parse().unwrap(),
            throw_if_true: throw_if_true.parse().unwrap(),
            throw_if_false: throw_if_false.parse().unwrap(),
        })
    }
    monkeys
}

#[aoc(day11, part1)]
pub fn part1(input: &[Monkey]) -> i32 {
    todo!()
}

#[aoc(day11, part2)]
pub fn part2(input: &[Monkey]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = include_str!("../examples/2022/day11.txt").trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
