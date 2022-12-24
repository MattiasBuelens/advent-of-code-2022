use std::collections::{HashMap, HashSet};

use crate::util::Vector2D;

type Grid = HashSet<Vector2D>;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.char_indices().filter_map(move |(x, c)| match c {
                '.' => None,
                '#' => Some(Vector2D::new(x as i32, y as i32)),
                _ => panic!("unexpected char: {c}"),
            })
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn step(self) -> Vector2D {
        match self {
            Direction::N => Vector2D::new(0, -1),
            Direction::E => Vector2D::new(1, 0),
            Direction::S => Vector2D::new(0, 1),
            Direction::W => Vector2D::new(-1, 0),
        }
    }

    fn edges(self) -> [Vector2D; 3] {
        match self {
            Direction::N => [
                Vector2D::new(-1, -1),
                Vector2D::new(0, -1),
                Vector2D::new(1, -1),
            ],
            Direction::E => [
                Vector2D::new(1, -1),
                Vector2D::new(1, 0),
                Vector2D::new(1, 1),
            ],
            Direction::S => [
                Vector2D::new(-1, 1),
                Vector2D::new(0, 1),
                Vector2D::new(1, 1),
            ],
            Direction::W => [
                Vector2D::new(-1, -1),
                Vector2D::new(-1, 0),
                Vector2D::new(-1, 1),
            ],
        }
    }
}

fn step(grid: &Grid, directions: &[Direction]) -> (bool, Grid) {
    let mut proposals = HashMap::<Vector2D, Vector2D>::new();
    let mut nb_proposals = HashMap::<Vector2D, usize>::new();

    // First half
    for &elf in grid {
        if elf.neighbours_diagonal().all(|pos| !grid.contains(&pos)) {
            // If no other Elves are in one of those eight positions,
            // the Elf does not do anything during this round.
            continue;
        }
        // Otherwise, the Elf looks in each of four directions in the following order:
        // * If there is no Elf in the N, NE, or NW adjacent positions,
        //   the Elf proposes moving north one step.
        // * ...
        for &dir in directions {
            if dir
                .edges()
                .iter()
                .all(|&step| !grid.contains(&(elf + step)))
            {
                let next_pos = elf + dir.step();
                // println!("{elf} proposes {dir:?} to {next_pos}");
                proposals.insert(elf, next_pos);
                *nb_proposals.entry(next_pos).or_default() += 1;
                break;
            }
        }
    }

    if proposals.is_empty() {
        return (true, grid.clone());
    }

    // Second half: each Elf moves to their proposed destination tile
    // if they were the only Elf to propose moving to that position.
    // If two or more Elves propose moving to the same position, none of those Elves move.
    let new_grid = grid
        .iter()
        .map(|&elf| {
            match proposals.get(&elf) {
                Some(&next) if *nb_proposals.get(&next).unwrap() == 1 => {
                    // Elf wants to move, and is the only one to move there
                    next
                }
                _ => {
                    // Elf cannot move, or doesn't want to move
                    elf
                }
            }
        })
        .collect();
    (false, new_grid)
}

fn count_empty(grid: &Grid) -> i32 {
    let min_x = grid.iter().map(|pos| pos.x()).min().unwrap();
    let max_x = grid.iter().map(|pos| pos.x()).max().unwrap();
    let min_y = grid.iter().map(|pos| pos.y()).min().unwrap();
    let max_y = grid.iter().map(|pos| pos.y()).max().unwrap();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    (width * height) - (grid.len() as i32)
}

#[allow(unused)]
fn print_grid(grid: &Grid) {
    let min_x = grid.iter().map(|pos| pos.x()).min().unwrap();
    let max_x = grid.iter().map(|pos| pos.x()).max().unwrap();
    let min_y = grid.iter().map(|pos| pos.y()).min().unwrap();
    let max_y = grid.iter().map(|pos| pos.y()).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                if grid.contains(&Vector2D::new(x, y)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &Grid) -> i32 {
    let mut grid = input.clone();
    let mut directions = [Direction::N, Direction::S, Direction::W, Direction::E];
    for _round in 1..=10 {
        let (_, new_grid) = step(&grid, &directions);
        grid = new_grid;
        directions.rotate_left(1);
        // println!("After round {_round}:");
        // print_grid(&grid);
        // println!();
    }
    count_empty(&grid)
}

#[aoc(day23, part2)]
pub fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut directions = [Direction::N, Direction::S, Direction::W, Direction::E];
    let mut round = 1;
    loop {
        let (done, new_grid) = step(&grid, &directions);
        if done {
            return round;
        }
        grid = new_grid;
        directions.rotate_left(1);
        round += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref SMALL: &'static str = r"
.....
..##.
..#..
.....
..##.
....."
            .trim();
        static ref LARGE: &'static str = r"
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
            .trim();
    }

    #[test]
    fn test_part1_small() {
        let input = input_generator(&SMALL);
        assert_eq!(part1(&input), 25);
    }

    #[test]
    fn test_part1_large() {
        let input = input_generator(&LARGE);
        assert_eq!(part1(&input), 110);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&LARGE);
        assert_eq!(part2(&input), 20);
    }
}
