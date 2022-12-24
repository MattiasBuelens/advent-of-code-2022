use std::collections::HashMap;

use crate::util::Vector2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Open,
    Wall,
}

pub type Board = HashMap<Vector2D, Tile>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Step {
    Forward(i32),
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Input {
    board: Board,
    path: Vec<Step>,
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Input {
    let (board, path) = input.split_once("\n\n").unwrap();
    let board = board
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.char_indices().flat_map(move |(x, c)| {
                let pos = Vector2D::new(x as i32 + 1, y as i32 + 1);
                match c {
                    ' ' => None,
                    '.' => Some((pos, Tile::Open)),
                    '#' => Some((pos, Tile::Wall)),
                    _ => panic!("unknown tile at {pos}: {c}"),
                }
            })
        })
        .collect();
    let path = path
        .trim()
        .split_inclusive(|c| c == 'L' || c == 'R')
        .flat_map(|s| {
            if let Some(s) = s.strip_suffix('L') {
                vec![Step::Forward(s.parse().unwrap()), Step::Left]
            } else if let Some(s) = s.strip_suffix('R') {
                vec![Step::Forward(s.parse().unwrap()), Step::Right]
            } else {
                vec![Step::Forward(s.parse().unwrap())]
            }
        })
        .collect();
    Input { board, path }
}

impl Direction {
    fn step(self) -> Vector2D {
        match self {
            Direction::Up => Vector2D::new(0, -1),
            Direction::Left => Vector2D::new(-1, 0),
            Direction::Down => Vector2D::new(0, 1),
            Direction::Right => Vector2D::new(1, 0),
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn find_start(board: &Board) -> Vector2D {
    let (&start, _) = board
        .iter()
        .filter(|(pos, tile)| pos.y() == 1 && tile == &&Tile::Open)
        .min_by_key(|(pos, _)| pos.x())
        .unwrap();
    start
}

fn find_opposite_edge(board: &Board, start: Vector2D, dir: Direction) -> Vector2D {
    let mut pos = start;
    let step = dir.turn_left().turn_left().step();
    while board.contains_key(&(pos + step)) {
        pos += step;
    }
    pos
}

#[aoc(day22, part1)]
pub fn part1(input: &Input) -> i32 {
    let start = find_start(&input.board);
    let mut pos = start;
    let mut dir = Direction::Right;
    for &step in &input.path {
        match step {
            Step::Forward(amount) => {
                'forward: for _ in 0..amount {
                    let mut next_pos = pos + dir.step();
                    let next_tile = match input.board.get(&next_pos) {
                        Some(tile) => tile,
                        None => {
                            next_pos = find_opposite_edge(&input.board, pos, dir);
                            input.board.get(&next_pos).unwrap()
                        }
                    };
                    match next_tile {
                        Tile::Open => pos = next_pos,
                        Tile::Wall => {
                            break 'forward;
                        }
                    }
                }
            }
            Step::Left => dir = dir.turn_left(),
            Step::Right => dir = dir.turn_right(),
        }
    }
    pos.y() * 1000 + pos.x() * 4 + (dir as i32)
}

#[aoc(day22, part2)]
pub fn part2(_input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = include_str!("../examples/2022/day22.txt");
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 6032);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
