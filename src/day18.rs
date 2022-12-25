use std::collections::{HashSet, VecDeque};

use crate::util::Vector3D;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Vector3D> {
    input
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            Vector3D::new(coords[0], coords[1], coords[2])
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Vector3D]) -> u64 {
    let mut area = 0;
    let cubes = input.iter().cloned().collect::<HashSet<_>>();
    for cube in &cubes {
        for neighbour in cube.neighbours() {
            if !cubes.contains(&neighbour) {
                area += 1;
            }
        }
    }
    area
}

#[aoc(day18, part2)]
pub fn part2(input: &[Vector3D]) -> u64 {
    let max_coord = input
        .iter()
        .flat_map(|cube| cube.coords.into_iter())
        .max()
        .unwrap();
    let cubes = input.iter().cloned().collect::<HashSet<_>>();
    // Flood-fill to find all exterior cubes
    let mut queue = VecDeque::from([Vector3D::zero()]);
    let mut exterior = HashSet::new();
    while let Some(pos) = queue.pop_front() {
        for neighbour in pos.neighbours() {
            if exterior.contains(&neighbour) || cubes.contains(&neighbour) {
                // Already visited, or cube is internal
            } else if neighbour
                .coords
                .iter()
                .all(|&coord| coord >= -1 && coord <= max_coord + 1)
            {
                // Expand steam around droplet
                exterior.insert(neighbour);
                queue.push_back(neighbour);
            }
        }
    }
    // Count the area
    let mut area = 0;
    for cube in &cubes {
        for neighbour in cube.neighbours() {
            if exterior.contains(&neighbour) {
                area += 1;
            }
        }
    }
    area
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 64);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 58);
    }
}
