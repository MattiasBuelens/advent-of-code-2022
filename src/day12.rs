use std::collections::HashMap;

use pathfinding::prelude::dijkstra;

use crate::util::Vector2D;

#[derive(Debug, Clone)]
pub struct HeightMap {
    squares: HashMap<Vector2D, i8>,
    start: Vector2D,
    goal: Vector2D,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> HeightMap {
    let mut squares = HashMap::new();
    let mut start = Vector2D::zero();
    let mut goal = Vector2D::zero();
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.char_indices() {
            let pos = Vector2D::new(x as i32, y as i32);
            let elevation = match c {
                'a'..='z' => c,
                'S' => {
                    start = pos;
                    'a'
                }
                'E' => {
                    goal = pos;
                    'z'
                }
                _ => panic!("invalid elevation: {}", c),
            };
            squares.insert(pos, ((elevation as u8) - ('a' as u8)) as i8);
        }
    }
    HeightMap {
        squares,
        start,
        goal,
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &HeightMap) -> i32 {
    let (_path, steps) = dijkstra(
        &input.start,
        |pos| {
            pos.neighbours()
                .filter_map(|neighbour| {
                    let current = input.squares.get(pos).copied().unwrap();
                    let next = input.squares.get(&neighbour).copied()?;
                    if next - current <= 1 {
                        Some((neighbour, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
        |pos| pos == &input.goal,
    )
    .expect("no path found");
    steps
}

#[aoc(day12, part2)]
pub fn part2(input: &HeightMap) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
