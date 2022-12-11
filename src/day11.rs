use std::cmp::Reverse;
use std::mem;

#[derive(Debug, Clone)]
pub struct Monkey {
    number: usize,
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    throw_if_true: usize,
    throw_if_false: usize,
    total_inspected: usize,
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
            total_inspected: 0,
        })
    }
    monkeys
}

struct Game {
    monkeys: Vec<Monkey>,
}

impl Game {
    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            self.turn(i);
        }
    }

    fn turn(&mut self, monkey_number: usize) {
        let monkey: &mut Monkey = &mut self.monkeys[monkey_number];
        // Mutate monkey first
        let items = mem::replace(&mut monkey.items, Vec::new());
        monkey.total_inspected += items.len();
        // Mutate other monkeys
        let &Monkey {
            operation,
            test,
            throw_if_true,
            throw_if_false,
            ..
        } = &*monkey;
        for mut item in items {
            item = match operation {
                Operation::Add(value) => item + value,
                Operation::Multiply(value) => item * value,
                Operation::Square => item * item,
            };
            item /= 3;
            if item % test == 0 {
                self.monkeys[throw_if_true].items.push(item);
            } else {
                self.monkeys[throw_if_false].items.push(item);
            }
        }
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &[Monkey]) -> usize {
    let mut game = Game {
        monkeys: input.to_vec(),
    };
    for _ in 0..20 {
        game.round();
    }
    game.monkeys
        .sort_unstable_by_key(|monkey| Reverse(monkey.total_inspected));
    game.monkeys[0].total_inspected * game.monkeys[1].total_inspected
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
    fn test_part1_example() {
        let monkeys = input_generator(&TEST_INPUT);
        let mut game = Game { monkeys };
        game.round();
        assert_eq!(game.monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(game.monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(game.monkeys[2].items, vec![]);
        assert_eq!(game.monkeys[3].items, vec![]);
        for _ in 1..20 {
            game.round();
        }
        assert_eq!(game.monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(game.monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(game.monkeys[2].items, vec![]);
        assert_eq!(game.monkeys[3].items, vec![]);
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 10605);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
