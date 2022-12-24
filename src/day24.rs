use std::iter::once;

use pathfinding::prelude::astar;

use crate::util::Vector2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Valley {
    width: i32,
    height: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Blizzard {
    pos: Vector2D,
    dir: Direction,
}

pub struct Input {
    valley: Valley,
    blizzards: Vec<Blizzard>,
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Input {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let valley = Valley { width, height };
    let blizzards = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.char_indices().filter_map(move |(x, c)| {
                let dir = match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => return None,
                };
                let pos = Vector2D::new(x as i32, y as i32);
                Some(Blizzard { pos, dir })
            })
        })
        .collect();
    Input { valley, blizzards }
}

impl Direction {
    fn step(self) -> Vector2D {
        match self {
            Direction::Up => Vector2D::new(0, -1),
            Direction::Down => Vector2D::new(0, 1),
            Direction::Left => Vector2D::new(-1, 0),
            Direction::Right => Vector2D::new(1, 0),
        }
    }
}

impl Valley {
    fn start(&self) -> Vector2D {
        Vector2D::new(1, 0)
    }

    fn goal(&self) -> Vector2D {
        Vector2D::new(self.width - 2, self.height - 1)
    }

    fn is_wall(&self, pos: &Vector2D) -> bool {
        if pos.y() == 0 {
            *pos != self.start()
        } else if pos.y() == self.height - 1 {
            *pos != self.goal()
        } else {
            pos.x() < 1 || pos.x() >= self.width - 1
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    you: Vector2D,
    blizzards: Vec<Blizzard>,
}

impl State {
    fn step(&self, valley: &Valley) -> State {
        let blizzards = self
            .blizzards
            .iter()
            .map(|blizzard| {
                let Blizzard { pos, dir } = blizzard.clone();
                let mut pos = pos + dir.step();
                if valley.is_wall(&pos) {
                    pos = match (pos.x(), pos.y()) {
                        (0, y) => Vector2D::new(valley.width - 2, y),
                        (x, y) if x == valley.width - 1 => Vector2D::new(1, y),
                        (x, 0) => Vector2D::new(x, valley.height - 2),
                        (x, y) if y == valley.height - 1 => Vector2D::new(x, 1),
                        _ => panic!("blizzard out of bounds at {pos}"),
                    }
                }
                Blizzard { pos, dir }
            })
            .collect();
        Self {
            you: self.you,
            blizzards,
        }
    }
}

#[aoc(day24, part1)]
pub fn part1(input: &Input) -> i32 {
    let start = State {
        you: input.valley.start(),
        blizzards: input.blizzards.clone(),
    };
    let (_, time) = astar(
        &start,
        |state| {
            // Move the blizzards
            let state = state.step(&input.valley);
            // Wait in current position...
            let states = once(state.clone());
            // ...or move in any direction...
            let states = states.chain(
                [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]
                .into_iter()
                .map(|dir| State {
                    you: state.you + dir.step(),
                    ..state.clone()
                }),
            );
            // Make sure we're not in a wall or a blizzard
            let states = states.filter(|state| {
                !input.valley.is_wall(&state.you)
                    && !state
                        .blizzards
                        .iter()
                        .any(|blizzard| blizzard.pos == state.you)
            });
            // Each step takes 1 minute
            let states = states.map(|state| (state, 1));
            states.collect::<Vec<_>>()
        },
        |state| (state.you - input.valley.goal()).manhattan_distance(),
        |state| &state.you == &input.valley.goal(),
    )
    .unwrap();
    time
}

#[aoc(day24, part2)]
pub fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
