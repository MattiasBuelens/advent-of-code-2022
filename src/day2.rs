#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Strategy {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone)]
pub struct Input {
    enemy: Hand,
    you: Strategy,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let (enemy, you) = line.split_once(' ').unwrap();
            let enemy = match enemy {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissors,
                _ => panic!("invalid input"),
            };
            let you = match you {
                "X" => Strategy::X,
                "Y" => Strategy::Y,
                "Z" => Strategy::Z,
                _ => panic!("invalid input"),
            };
            Input { you, enemy }
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
pub struct Game {
    enemy: Hand,
    you: Hand,
}

impl Strategy {
    fn to_hand(self) -> Hand {
        match self {
            Strategy::X => Hand::Rock,
            Strategy::Y => Hand::Paper,
            Strategy::Z => Hand::Scissors,
        }
    }
}

impl Input {
    fn to_game_part1(self) -> Game {
        Game {
            enemy: self.enemy,
            you: self.you.to_hand(),
        }
    }
}

impl Game {
    fn score(self) -> i32 {
        self.score_hand() + self.score_outcome()
    }
    fn score_hand(self) -> i32 {
        match self.you {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
    fn score_outcome(self) -> i32 {
        match (self.you, self.enemy) {
            // Win
            (Hand::Rock, Hand::Scissors)
            | (Hand::Scissors, Hand::Paper)
            | (Hand::Paper, Hand::Rock) => 6,
            // Draw
            (x, y) if x == y => 3,
            // Loss
            _ => 0,
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &[Input]) -> i32 {
    input
        .iter()
        .map(|input| input.to_game_part1().score())
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Input]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
A Y
B X
C Z
"
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
