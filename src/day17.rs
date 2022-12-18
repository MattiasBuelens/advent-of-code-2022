use std::collections::{HashMap, HashSet};

use crate::util::Vector2D;

#[derive(Debug, Copy, Clone)]
pub enum Jet {
    Left,
    Right,
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            c => panic!("invalid jet: {}", c),
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RockType {
    One,
    Two,
    Three,
    Four,
    Five,
}

// X is left to right
// Y is bottom to top
const ROCK1: [Vector2D; 4] = [
    Vector2D::new(0, 0),
    Vector2D::new(1, 0),
    Vector2D::new(2, 0),
    Vector2D::new(3, 0),
];
const ROCK2: [Vector2D; 5] = [
    Vector2D::new(1, 0),
    Vector2D::new(0, 1),
    Vector2D::new(1, 1),
    Vector2D::new(2, 1),
    Vector2D::new(1, 2),
];
const ROCK3: [Vector2D; 5] = [
    Vector2D::new(0, 0),
    Vector2D::new(1, 0),
    Vector2D::new(2, 0),
    Vector2D::new(2, 1),
    Vector2D::new(2, 2),
];
const ROCK4: [Vector2D; 4] = [
    Vector2D::new(0, 0),
    Vector2D::new(0, 1),
    Vector2D::new(0, 2),
    Vector2D::new(0, 3),
];
const ROCK5: [Vector2D; 4] = [
    Vector2D::new(0, 0),
    Vector2D::new(1, 0),
    Vector2D::new(0, 1),
    Vector2D::new(1, 1),
];

impl RockType {
    fn all() -> [RockType; 5] {
        [
            RockType::One,
            RockType::Two,
            RockType::Three,
            RockType::Four,
            RockType::Five,
        ]
    }

    fn to_blocks(self) -> &'static [Vector2D] {
        match self {
            RockType::One => &ROCK1,
            RockType::Two => &ROCK2,
            RockType::Three => &ROCK3,
            RockType::Four => &ROCK4,
            RockType::Five => &ROCK5,
        }
    }
}

#[derive(Debug, Default)]
struct Tower {
    height: i32,
    blocks: HashSet<Vector2D>,
}

impl Tower {
    fn overlaps(&self, rock: RockType, pos: Vector2D) -> bool {
        for block in rock.to_blocks() {
            let block = *block + pos;
            if block.x() < 0 || block.x() >= 7 || block.y() < 0 || self.blocks.contains(&block) {
                return true;
            }
        }
        false
    }

    fn place_rock(&mut self, rock: RockType, pos: Vector2D) {
        for block in rock.to_blocks() {
            let block = *block + pos;
            self.height = self.height.max(block.y() + 1);
            self.blocks.insert(block);
        }
    }

    fn drop_rock(&self, rock: RockType, jets: &mut impl Iterator<Item = (usize, Jet)>) -> Vector2D {
        let mut pos = Vector2D::new(2, self.height + 3);
        loop {
            // Push left or right, if possible
            let (_, jet) = jets.next().unwrap();
            let next_pos = match jet {
                Jet::Left => pos + Vector2D::new(-1, 0),
                Jet::Right => pos + Vector2D::new(1, 0),
            };
            if !self.overlaps(rock, next_pos) {
                pos = next_pos;
            }
            // Drop down
            let next_pos = pos + Vector2D::new(0, -1);
            if self.overlaps(rock, next_pos) {
                // Landed
                return pos;
            }
            pos = next_pos;
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &[Jet]) -> i32 {
    let mut tower = Tower::default();
    let mut rocks = RockType::all().into_iter().cycle();
    let mut jets = input.iter().cloned().enumerate().cycle();
    for _ in 0..2022 {
        let rock = rocks.next().unwrap();
        let pos = tower.drop_rock(rock, &mut jets);
        tower.place_rock(rock, pos);
    }
    tower.height
}

fn find_deepest_drop(jets: &[Jet], cycles: usize) -> i32 {
    let mut tower = Tower::default();
    let mut rocks = RockType::all().into_iter().cycle();
    let mut jets = jets.iter().cloned().enumerate().cycle();
    let mut deepest = 0;
    for _ in 0..cycles {
        let rock = rocks.next().unwrap();
        let pos = tower.drop_rock(rock, &mut jets);
        tower.place_rock(rock, pos);
        deepest = deepest.max(tower.height - pos.y());
    }
    deepest
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    rock_index: usize,
    jet_index: usize,
    top: Vec<Vector2D>,
}

impl Tower {
    fn capture(&self, rock_index: usize, jet_index: usize, depth: i32) -> State {
        let mut top = Vec::new();
        for y in (-depth)..=0 {
            for x in 0..7 {
                if self.blocks.contains(&Vector2D::new(x, self.height + y)) {
                    top.push(Vector2D::new(x, y));
                }
            }
        }
        State {
            rock_index,
            jet_index,
            top,
        }
    }
}

#[aoc(day17, part2)]
pub fn part2(input: &[Jet]) -> u64 {
    let mut tower = Tower::default();
    let mut rocks = RockType::all().into_iter().enumerate().cycle().peekable();
    let mut jets = input.iter().cloned().enumerate().cycle().peekable();
    let cache_depth = find_deepest_drop(input, 1000);
    let target = 1_000_000_000_000u64;
    let mut cache = HashMap::<State, (u64, i32)>::new();
    // Find first cycle
    let mut i = 0;
    let (cycle_length, cycle_height) = loop {
        let (rock_index, _) = *rocks.peek().unwrap();
        let (jet_index, _) = *jets.peek().unwrap();
        let state = tower.capture(rock_index, jet_index, cache_depth);
        if let Some(&(previous_i, previous_height)) = cache.get(&state) {
            break (i - previous_i, tower.height - previous_height);
        }
        cache.insert(state, (i, tower.height));
        let (_, rock) = rocks.next().unwrap();
        let pos = tower.drop_rock(rock, &mut jets);
        tower.place_rock(rock, pos);
        i += 1;
    };
    // Skip over repeated cycles
    let cycle_count = (target - i) / cycle_length;
    i += cycle_count * cycle_length;
    // Finish remaining drops
    while i < target {
        let (_, rock) = rocks.next().unwrap();
        let pos = tower.drop_rock(rock, &mut jets);
        tower.place_rock(rock, pos);
        i += 1;
    }
    tower.height as u64 + (cycle_count * cycle_height as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 3068);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 1_514_285_714_288);
    }
}
