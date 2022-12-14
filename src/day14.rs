use std::collections::HashSet;

use crate::util::{array_windows, Vector2D};

pub type Input = Vec<Vec<Vector2D>>;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let (x, y) = coords.split_once(',').unwrap();
                    Vector2D::new(x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect()
}

type Cave = HashSet<Vector2D>;

fn make_line(start: Vector2D, end: Vector2D) -> impl Iterator<Item = Vector2D> {
    let min_x = start.x().min(end.x());
    let max_x = start.x().max(end.x());
    let min_y = start.y().min(end.y());
    let max_y = start.y().max(end.y());
    (min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| Vector2D::new(x, y)))
}

fn place_rocks(rocks: &Input) -> Cave {
    rocks
        .iter()
        .flat_map(|rock| {
            array_windows::<_, 2>(rock).flat_map(|&[start, end]| make_line(start, end))
        })
        .collect()
}

fn drop_sand(cave: &Cave, source: Vector2D, floor: Option<i32>) -> Option<Vector2D> {
    let max_y = cave.iter().map(|pos| pos.y()).max().unwrap();
    let mut pos = source;
    loop {
        if let Some(floor) = floor {
            // Part 2: fall on the floor
            if pos.y() == floor - 1 {
                return Some(pos);
            }
        } else {
            // Part 1: fall into the void
            if pos.y() > max_y {
                return None;
            }
        }
        let down = pos + Vector2D::new(0, 1);
        if !cave.contains(&down) {
            // Move down
            pos = down;
            continue;
        }
        let down_left = pos + Vector2D::new(-1, 1);
        if !cave.contains(&down_left) {
            // Move down-left
            pos = down_left;
            continue;
        }
        let down_right = pos + Vector2D::new(1, 1);
        if !cave.contains(&down_right) {
            // Move down-right
            pos = down_right;
            continue;
        }
        // Come to rest
        return Some(pos);
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> i32 {
    let mut cave = place_rocks(input);
    let source = Vector2D::new(500, 0);
    let mut sand_count = 0;
    while let Some(sand_pos) = drop_sand(&cave, source, None) {
        cave.insert(sand_pos);
        sand_count += 1;
    }
    sand_count
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> i32 {
    let mut cave = place_rocks(input);
    let floor = cave.iter().map(|pos| pos.y()).max().unwrap() + 2;
    let source = Vector2D::new(500, 0);
    let mut sand_count = 0;
    while let Some(sand_pos) = drop_sand(&cave, source, Some(floor)) {
        cave.insert(sand_pos);
        sand_count += 1;
        if sand_pos == source {
            break;
        }
    }
    sand_count
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 24);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 93);
    }
}
