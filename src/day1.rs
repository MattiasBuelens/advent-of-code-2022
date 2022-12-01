#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n\n")
        .map(|block| block.lines().map(|line| line.parse().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|inventory| inventory.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[Vec<i32>]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        ".trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
