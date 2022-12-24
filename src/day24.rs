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
    let height = input.lines().count() as i32 - 2;
    let width = input.lines().next().unwrap().len() as i32 - 2;
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
                let pos = Vector2D::new(x as i32 - 1, y as i32 - 1);
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

    fn print(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Valley {
    fn start(&self) -> Vector2D {
        Vector2D::new(0, -1)
    }

    fn goal(&self) -> Vector2D {
        Vector2D::new(self.width - 1, self.height)
    }

    fn is_wall(&self, pos: &Vector2D) -> bool {
        if pos.y() == -1 {
            *pos != self.start()
        } else if pos.y() == self.height {
            *pos != self.goal()
        } else {
            pos.x() < 0 || pos.x() >= self.width
        }
    }

    #[allow(unused)]
    fn print(&self, blizzards: &[Blizzard]) {
        for y in -1..=self.height {
            for x in -1..=self.width {
                let pos = Vector2D::new(x, y);
                let c = if self.is_wall(&pos) {
                    '#'
                } else if let Some(blizzard) = blizzards.iter().find(|x| x.pos == pos) {
                    blizzard.dir.print()
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    you: Vector2D,
    time: i32,
}

impl State {
    fn step(self) -> Self {
        Self {
            time: self.time + 1,
            ..self
        }
    }

    fn in_blizzard(&self, valley: &Valley, blizzards: &[Blizzard]) -> bool {
        blizzards
            .iter()
            .filter(|blizzard| {
                // Only consider blizzards that are aligned with us
                if blizzard.dir == Direction::Up || blizzard.dir == Direction::Down {
                    self.you.x() == blizzard.pos.x()
                } else {
                    self.you.y() == blizzard.pos.y()
                }
            })
            .any(|blizzard| {
                // Compute current position of blizzard
                let pos = blizzard.pos + (blizzard.dir.step() * self.time);
                let pos = Vector2D::new(
                    (pos.x() % valley.width + valley.width) % valley.width,
                    (pos.y() % valley.height + valley.height) % valley.height,
                );
                // Check if we overlap with it
                pos == self.you
            })
    }

    fn is_valid(&self, valley: &Valley, blizzards: &[Blizzard]) -> bool {
        !valley.is_wall(&self.you) && !self.in_blizzard(valley, blizzards)
    }

    fn successors<'a>(
        &'a self,
        valley: &'a Valley,
        blizzards: &'a [Blizzard],
    ) -> impl Iterator<Item = State> + 'a {
        // Increase the time
        let state = self.clone().step();
        // Wait in current position...
        let positions = once(self.you);
        // ...or move in any direction...
        let positions = positions.chain(
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .into_iter()
            .map(move |dir| state.you + dir.step()),
        );
        let states = positions.into_iter().map(move |you| State {
            you,
            ..state.clone()
        });
        // Make sure we're not in a wall or a blizzard
        let states = states.filter(|state| state.is_valid(valley, blizzards));
        states
    }
}

fn shortest_path(input: &Input, start: State, goal: Vector2D) -> State {
    let (path, _time) = astar(
        &start,
        |state| {
            state
                .successors(&input.valley, &input.blizzards)
                .map(|state| (state, 1))
                .collect::<Vec<_>>()
        },
        |state| (state.you - goal).manhattan_distance(),
        |state| &state.you == &goal,
    )
    .unwrap();
    path.last().unwrap().clone()
}

#[aoc(day24, part1)]
pub fn part1(input: &Input) -> i32 {
    // input.valley.print(&input.blizzards);
    let start = State {
        you: input.valley.start(),
        time: 0,
    };
    shortest_path(input, start, input.valley.goal()).time
}

#[aoc(day24, part2)]
pub fn part2(input: &Input) -> i32 {
    let start = State {
        you: input.valley.start(),
        time: 0,
    };
    let first_goal = shortest_path(input, start, input.valley.goal());
    let start_again = shortest_path(input, first_goal, input.valley.start());
    let second_goal = shortest_path(input, start_again, input.valley.goal());
    second_goal.time
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
        assert_eq!(part2(&input), 54);
    }
}
