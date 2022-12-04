use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Range {
    begin: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (begin, end) = s.split_once('-').unwrap();
        let begin = begin.parse().unwrap();
        let end = end.parse().unwrap();
        Ok(Range { begin, end })
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.begin <= other.begin && other.end <= self.end
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(left, right)| left.contains(right) || right.contains(left))
        .count()
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        self.begin <= other.end && other.begin <= self.end
    }
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(left, right)| left.overlaps(right))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 4);
    }
}
