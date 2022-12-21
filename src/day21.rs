use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Yell {
    Number(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

pub type Input = HashMap<String, Yell>;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (name, yell) = line.split_once(": ").unwrap();
            let yell = if let Some((left, right)) = yell.split_once(" + ") {
                Yell::Add(left.to_string(), right.to_string())
            } else if let Some((left, right)) = yell.split_once(" - ") {
                Yell::Subtract(left.to_string(), right.to_string())
            } else if let Some((left, right)) = yell.split_once(" * ") {
                Yell::Multiply(left.to_string(), right.to_string())
            } else if let Some((left, right)) = yell.split_once(" / ") {
                Yell::Divide(left.to_string(), right.to_string())
            } else {
                Yell::Number(yell.parse().unwrap())
            };
            (name.to_string(), yell)
        })
        .collect()
}

fn solve(name: &str, yells: &Input, cache: &mut HashMap<String, i64>) -> i64 {
    if let Some(&value) = cache.get(name) {
        return value;
    }
    let value = match yells.get(name).unwrap() {
        Yell::Number(x) => *x,
        Yell::Add(left, right) => solve(left, yells, cache) + solve(right, yells, cache),
        Yell::Subtract(left, right) => solve(left, yells, cache) - solve(right, yells, cache),
        Yell::Multiply(left, right) => solve(left, yells, cache) * solve(right, yells, cache),
        Yell::Divide(left, right) => solve(left, yells, cache) / solve(right, yells, cache),
    };
    cache.insert(name.to_string(), value);
    value
}

#[aoc(day21, part1)]
pub fn part1(input: &Input) -> i64 {
    solve("root", input, &mut HashMap::new())
}

#[aoc(day21, part2)]
pub fn part2(input: &Input) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 152);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
