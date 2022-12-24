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

fn depends_on_humn<'a>(name: &'a str, yells: &'a Input) -> bool {
    if name == "humn" {
        return true;
    }
    match yells.get(name).unwrap() {
        Yell::Number(_x) => false,
        Yell::Add(left, right)
        | Yell::Subtract(left, right)
        | Yell::Multiply(left, right)
        | Yell::Divide(left, right) => {
            depends_on_humn(left, yells) || depends_on_humn(right, yells)
        }
    }
}

fn solve_for_humn(name: &str, value: i64, yells: &Input, cache: &mut HashMap<String, i64>) -> i64 {
    if name == "humn" {
        return value;
    }
    match yells.get(name).unwrap() {
        Yell::Number(_) => panic!("unexpected number"),
        Yell::Add(left, right) => {
            if depends_on_humn(left, yells) {
                // X = L + R => L = X - R
                let left_value = value - solve(right, yells, cache);
                solve_for_humn(left, left_value, yells, cache)
            } else {
                // X = L + R => R = X - L
                let right_value = value - solve(left, yells, cache);
                solve_for_humn(right, right_value, yells, cache)
            }
        }
        Yell::Subtract(left, right) => {
            if depends_on_humn(left, yells) {
                // X = L - R => L = X + R
                let left_value = value + solve(right, yells, cache);
                solve_for_humn(left, left_value, yells, cache)
            } else {
                // X = L - R => R = L - X
                let right_value = solve(left, yells, cache) - value;
                solve_for_humn(right, right_value, yells, cache)
            }
        }
        Yell::Multiply(left, right) => {
            if depends_on_humn(left, yells) {
                // X = L * R => L = X / R
                let left_value = value / solve(right, yells, cache);
                solve_for_humn(left, left_value, yells, cache)
            } else {
                // X = L * R => R = X / L
                let right_value = value / solve(left, yells, cache);
                solve_for_humn(right, right_value, yells, cache)
            }
        }
        Yell::Divide(left, right) => {
            if depends_on_humn(left, yells) {
                // X = L / R => L = X * R
                let left_value = value * solve(right, yells, cache);
                solve_for_humn(left, left_value, yells, cache)
            } else {
                // X = L / R => R = L / X
                let right_value = solve(left, yells, cache) / value;
                solve_for_humn(right, right_value, yells, cache)
            }
        }
    }
}

#[aoc(day21, part2)]
pub fn part2(yells: &Input) -> i64 {
    let (left, right) = if let Yell::Add(left, right) = yells.get("root").unwrap() {
        (left, right)
    } else {
        panic!("unexpected root equation")
    };
    // One side of the equality depends on "humn", the other side doesn't.
    let (dependent, independent) =
        match (depends_on_humn(left, yells), depends_on_humn(right, yells)) {
            (true, false) => (left, right),
            (false, true) => (right, left),
            (true, true) => panic!("both sides depend on humn"),
            (false, false) => panic!("no side depends on humn"),
        };
    // Compute the value of the independent side
    let mut cache = HashMap::new();
    let value = solve(independent, yells, &mut cache);
    // Set the dependent side equal to the same value and solve for "humn"
    solve_for_humn(dependent, value, yells, &mut cache)
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
        assert_eq!(part2(&input), 301);
    }
}
