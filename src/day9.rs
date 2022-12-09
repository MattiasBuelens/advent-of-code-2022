use std::collections::HashSet;

use crate::util::Vector2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub type Move = (Direction, i32);

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let (dir, steps) = line.split_once(' ').unwrap();
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("invalid direction: {}", dir),
            };
            let steps = steps.parse().unwrap();
            (dir, steps)
        })
        .collect()
}

#[derive(Debug, Default)]
struct Rope {
    head: Vector2D,
    tail: Vector2D,
}

impl Rope {
    fn move_head(&mut self, dir: Direction) {
        self.head += match dir {
            Direction::Up => Vector2D::new(0, -1),
            Direction::Down => Vector2D::new(0, 1),
            Direction::Left => Vector2D::new(-1, 0),
            Direction::Right => Vector2D::new(1, 0),
        }
    }

    fn update_tail(&mut self) {
        let diff = self.head - self.tail;
        if diff.x() == 0 && diff.y().abs() <= 1 {
            // Overlapping or touching vertically
            return;
        }
        if diff.y() == 0 && diff.x().abs() <= 1 {
            // Touching horizontally
            return;
        }
        let distance = diff.manhattan_distance();
        if diff.x() != 0 && diff.y() != 0 && distance == 2 {
            // Touching diagonally
            return;
        }
        self.tail += Vector2D::new(diff.x().signum(), diff.y().signum());
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &[Move]) -> usize {
    let mut rope = Rope::default();
    let mut visited = HashSet::<Vector2D>::new();
    visited.insert(rope.tail);
    for &(dir, steps) in input {
        for _ in 0..steps {
            rope.move_head(dir);
            rope.update_tail();
            visited.insert(rope.tail);
        }
    }
    visited.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Move]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
