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

#[derive(Debug)]
struct Rope<const N: usize> {
    head: Vector2D,
    tails: [Vector2D; N],
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            head: Vector2D::zero(),
            tails: [Vector2D::zero(); N],
        }
    }

    fn move_head(&mut self, dir: Direction) {
        self.head += match dir {
            Direction::Up => Vector2D::new(0, -1),
            Direction::Down => Vector2D::new(0, 1),
            Direction::Left => Vector2D::new(-1, 0),
            Direction::Right => Vector2D::new(1, 0),
        }
    }

    fn tail(&self) -> Vector2D {
        *self.tails.last().unwrap()
    }

    fn update_tail(&mut self) {
        if N == 0 {
            return;
        }
        self.tails[0] = Self::update_knot(self.tails[0], self.head);
        for i in 1..N {
            self.tails[i] = Self::update_knot(self.tails[i], self.tails[i - 1]);
        }
    }

    fn update_knot(knot: Vector2D, prev_knot: Vector2D) -> Vector2D {
        let diff = prev_knot - knot;
        if diff.x() == 0 && diff.y().abs() <= 1 {
            // Overlapping or touching vertically
            return knot;
        }
        if diff.y() == 0 && diff.x().abs() <= 1 {
            // Touching horizontally
            return knot;
        }
        let distance = diff.manhattan_distance();
        if diff.x() != 0 && diff.y() != 0 && distance == 2 {
            // Touching diagonally
            return knot;
        }
        knot + Vector2D::new(diff.x().signum(), diff.y().signum())
    }
}

fn solve<const N: usize>(input: &[Move]) -> usize {
    let mut rope = Rope::<N>::new();
    let mut visited = HashSet::<Vector2D>::new();
    visited.insert(rope.tail());
    for &(dir, steps) in input {
        for _ in 0..steps {
            rope.move_head(dir);
            rope.update_tail();
            visited.insert(rope.tail());
        }
    }
    visited.len()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Move]) -> usize {
    solve::<1>(input)
}

#[aoc(day9, part2)]
pub fn part2(input: &[Move]) -> usize {
    solve::<9>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT1: &'static str = r"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .trim();
        static ref INPUT2: &'static str = r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&INPUT1);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2_input1() {
        let input = input_generator(&INPUT1);
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_input2() {
        let input = input_generator(&INPUT2);
        assert_eq!(part2(&input), 36);
    }
}
