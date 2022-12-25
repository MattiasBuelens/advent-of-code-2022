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
                let pos = Vector2D::new(x as i32, y as i32);
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
        .filter(|(pos, tile)| pos.y() == 0 && tile == &&Tile::Open)
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

fn final_password(pos: Vector2D, dir: Direction) -> i32 {
    (pos.y() + 1) * 1000 + (pos.x() + 1) * 4 + (dir as i32)
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
    final_password(pos, dir)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Face {
    Front,
    Left,
    Back,
    Right,
    Top,
    Bottom,
}

impl Face {
    fn opposite(self) -> Self {
        match self {
            Face::Front => Face::Back,
            Face::Left => Face::Right,
            Face::Back => Face::Front,
            Face::Right => Face::Left,
            Face::Top => Face::Bottom,
            Face::Bottom => Face::Top,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Orientation {
    /// The face we're looking at.
    facing: Face,
    /// The face that's on the top edge of the face we're looking at.
    /// Cannot be the same face, and cannot be the opposite face.
    up: Face,
}

impl Orientation {
    fn new(facing: Face, up: Face) -> Self {
        assert_ne!(up, facing);
        assert_ne!(up, facing.opposite());
        Orientation { facing, up }
    }

    fn move_up(self) -> Self {
        Orientation::new(self.up, self.facing.opposite())
    }

    fn move_down(self) -> Self {
        Orientation::new(self.up.opposite(), self.facing)
    }

    fn rotate_left(self) -> Self {
        match (self.facing, self.up) {
            (facing @ Face::Front, Face::Top) => Orientation::new(facing, Face::Left),
            (facing @ Face::Front, Face::Left) => Orientation::new(facing, Face::Bottom),
            (facing @ Face::Front, Face::Bottom) => Orientation::new(facing, Face::Right),
            (facing @ Face::Front, Face::Right) => Orientation::new(facing, Face::Top),
            (facing @ Face::Left, Face::Top) => Orientation::new(facing, Face::Back),
            (facing @ Face::Left, Face::Back) => Orientation::new(facing, Face::Bottom),
            (facing @ Face::Left, Face::Bottom) => Orientation::new(facing, Face::Front),
            (facing @ Face::Left, Face::Front) => Orientation::new(facing, Face::Top),
            (facing @ Face::Back, Face::Top) => Orientation::new(facing, Face::Right),
            (facing @ Face::Back, Face::Right) => Orientation::new(facing, Face::Bottom),
            (facing @ Face::Back, Face::Bottom) => Orientation::new(facing, Face::Left),
            (facing @ Face::Back, Face::Left) => Orientation::new(facing, Face::Top),
            (facing @ Face::Right, Face::Top) => Orientation::new(facing, Face::Front),
            (facing @ Face::Right, Face::Front) => Orientation::new(facing, Face::Bottom),
            (facing @ Face::Right, Face::Bottom) => Orientation::new(facing, Face::Back),
            (facing @ Face::Right, Face::Back) => Orientation::new(facing, Face::Top),
            (facing @ Face::Top, Face::Back) => Orientation::new(facing, Face::Left),
            (facing @ Face::Top, Face::Left) => Orientation::new(facing, Face::Front),
            (facing @ Face::Top, Face::Front) => Orientation::new(facing, Face::Right),
            (facing @ Face::Top, Face::Right) => Orientation::new(facing, Face::Back),
            (facing @ Face::Bottom, Face::Front) => Orientation::new(facing, Face::Left),
            (facing @ Face::Bottom, Face::Left) => Orientation::new(facing, Face::Back),
            (facing @ Face::Bottom, Face::Back) => Orientation::new(facing, Face::Right),
            (facing @ Face::Bottom, Face::Right) => Orientation::new(facing, Face::Front),
            (Face::Front | Face::Back, Face::Front | Face::Back)
            | (Face::Left | Face::Right, Face::Left | Face::Right)
            | (Face::Top | Face::Bottom, Face::Top | Face::Bottom) => panic!("invalid orientation"),
        }
    }

    fn rotate_right(self) -> Self {
        self.rotate_left().rotate_left().rotate_left()
    }

    fn move_left(self) -> Self {
        self.rotate_left().move_up().rotate_right()
    }

    fn move_right(self) -> Self {
        self.rotate_right().move_up().rotate_left()
    }
}

// Map from (position / cube side length) to face orientation
type CubeNet = HashMap<Vector2D, Orientation>;

fn fold_cube(board: &Board, side: i32, start: Vector2D) -> CubeNet {
    let mut net = CubeNet::new();
    let start_pos = start / side;
    let start_orientation = Orientation::new(Face::Front, Face::Top);
    fold_cube_inner(board, side, start_pos, start_orientation, &mut net);
    net
}

fn fold_cube_inner(
    board: &Board,
    side: i32,
    pos: Vector2D,
    orientation: Orientation,
    net: &mut CubeNet,
) {
    net.insert(pos, orientation);
    let pos_up = pos + Direction::Up.step();
    if !net.contains_key(&pos_up) && board.contains_key(&(pos_up * side)) {
        fold_cube_inner(board, side, pos_up, orientation.move_up(), net);
    }
    let pos_down = pos + Direction::Down.step();
    if !net.contains_key(&pos_down) && board.contains_key(&(pos_down * side)) {
        fold_cube_inner(board, side, pos_down, orientation.move_down(), net);
    }
    let pos_left = pos + Direction::Left.step();
    if !net.contains_key(&pos_left) && board.contains_key(&(pos_left * side)) {
        fold_cube_inner(board, side, pos_left, orientation.move_left(), net);
    }
    let pos_right = pos + Direction::Right.step();
    if !net.contains_key(&pos_right) && board.contains_key(&(pos_right * side)) {
        fold_cube_inner(board, side, pos_right, orientation.move_right(), net);
    }
}

fn find_connected_edge(
    net: &CubeNet,
    side: i32,
    pos: Vector2D,
    dir: Direction,
) -> (Vector2D, Direction) {
    // Find the next face (and orientation) we need to move to.
    let current_face = *net.get(&(pos / side)).unwrap();
    let next_face = match dir {
        Direction::Up => current_face.move_up(),
        Direction::Right => current_face.move_right(),
        Direction::Down => current_face.move_down(),
        Direction::Left => current_face.move_left(),
    };
    // Find where this next face appears on the board, possibly with a different orientation.
    let (&board_pos, &board_face) = net
        .iter()
        .find(|(_, orientation)| orientation.facing == next_face.facing)
        .unwrap();
    // Get the relative position on the next face.
    let next_pos = pos + dir.step();
    let mut next_pos = Vector2D::new((next_pos.x() + side) % side, (next_pos.y() + side) % side);
    // Rotate until the next face aligns with the one on the board.
    let mut next_face = next_face;
    let mut next_dir = dir;
    while next_face.up != board_face.up {
        // Rotate face to the left.
        next_face = next_face.rotate_left();
        // Rotate relative position and direction to the right.
        next_pos = Vector2D::new((side - 1) - next_pos.y(), next_pos.x());
        next_dir = next_dir.turn_right();
    }
    // Get the absolute position on the board.
    let next_pos = (board_pos * side) + next_pos;
    (next_pos, next_dir)
}

fn solve_cube(input: &Input, side: i32) -> i32 {
    let start = find_start(&input.board);
    let net = fold_cube(&input.board, side, start);
    let mut pos = start;
    let mut dir = Direction::Right;
    for &step in &input.path {
        match step {
            Step::Forward(amount) => {
                'forward: for _ in 0..amount {
                    let mut next_pos = pos + dir.step();
                    let mut next_dir = dir;
                    let next_tile = match input.board.get(&next_pos) {
                        Some(tile) => tile,
                        None => {
                            (next_pos, next_dir) = find_connected_edge(&net, side, pos, dir);
                            input.board.get(&next_pos).unwrap()
                        }
                    };
                    match next_tile {
                        Tile::Open => {
                            pos = next_pos;
                            dir = next_dir;
                        }
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
    final_password(pos, dir)
}

#[aoc(day22, part2)]
pub fn part2(input: &Input) -> i32 {
    solve_cube(input, 50)
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
        assert_eq!(solve_cube(&input, 4), 5031);
    }

    #[test]
    fn test_fold_cube() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(
            fold_cube(&input.board, 4, Vector2D::new(8, 0)),
            CubeNet::from_iter([
                (
                    Vector2D::new(2, 0),
                    Orientation::new(Face::Front, Face::Top)
                ),
                (
                    Vector2D::new(0, 1),
                    Orientation::new(Face::Top, Face::Front)
                ),
                (
                    Vector2D::new(1, 1),
                    Orientation::new(Face::Left, Face::Front)
                ),
                (
                    Vector2D::new(2, 1),
                    Orientation::new(Face::Bottom, Face::Front)
                ),
                (
                    Vector2D::new(2, 2),
                    Orientation::new(Face::Back, Face::Bottom)
                ),
                (
                    Vector2D::new(3, 2),
                    Orientation::new(Face::Right, Face::Bottom)
                ),
            ])
        );
    }
}
