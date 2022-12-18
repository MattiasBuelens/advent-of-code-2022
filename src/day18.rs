use std::collections::HashSet;

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
    let cubes = input.into_iter().cloned().collect::<HashSet<_>>();
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
pub fn part2(input: &[Vector3D]) -> i32 {
    todo!()
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
        assert_eq!(part2(&input), 0);
    }
}
