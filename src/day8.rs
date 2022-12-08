use std::collections::HashMap;

use crate::util::Vector2D;

#[derive(Debug)]
pub struct Forest {
    width: i32,
    height: i32,
    trees: HashMap<Vector2D, u32>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Forest {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut trees = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .map(move |(x, c)| (Vector2D::new(x as i32, y as i32), c.to_digit(10).unwrap()))
        })
        .collect();
    Forest {
        width,
        height,
        trees,
    }
}

impl Forest {
    fn count_visible(&self) -> usize {
        self.trees
            .keys()
            .filter(|&&pos| self.is_visible(pos))
            .count()
    }

    fn is_visible(&self, tree_pos: Vector2D) -> bool {
        self.is_visible_in_direction(tree_pos, Vector2D::new(0, 1))
            || self.is_visible_in_direction(tree_pos, Vector2D::new(0, -1))
            || self.is_visible_in_direction(tree_pos, Vector2D::new(1, 0))
            || self.is_visible_in_direction(tree_pos, Vector2D::new(-1, 0))
    }

    fn is_visible_in_direction(&self, tree_pos: Vector2D, dir: Vector2D) -> bool {
        let tree = *self.trees.get(&tree_pos).unwrap();
        let mut pos = tree_pos + dir;
        while (0..self.width).contains(&pos.x()) && (0..self.height).contains(&pos.y()) {
            let other_tree = *self.trees.get(&pos).unwrap();
            if other_tree >= tree {
                return false;
            }
            pos += dir;
        }
        true
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &Forest) -> usize {
    input.count_visible()
}

impl Forest {
    fn scenic_score(&self, tree_pos: Vector2D) -> u64 {
        self.viewing_distance(tree_pos, Vector2D::new(0, 1))
            * self.viewing_distance(tree_pos, Vector2D::new(0, -1))
            * self.viewing_distance(tree_pos, Vector2D::new(1, 0))
            * self.viewing_distance(tree_pos, Vector2D::new(-1, 0))
    }

    fn viewing_distance(&self, tree_pos: Vector2D, dir: Vector2D) -> u64 {
        let tree = *self.trees.get(&tree_pos).unwrap();
        let mut viewing_distance = 0;
        let mut pos = tree_pos + dir;
        while (0..self.width).contains(&pos.x()) && (0..self.height).contains(&pos.y()) {
            viewing_distance += 1;
            let other_tree = *self.trees.get(&pos).unwrap();
            if other_tree >= tree {
                break;
            }
            pos += dir;
        }
        viewing_distance
    }
}

#[aoc(day8, part2)]
pub fn part2(input: &Forest) -> u64 {
    input
        .trees
        .keys()
        .map(|&pos| input.scenic_score(pos))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
30373
25512
65332
33549
35390"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 8);
    }
}
