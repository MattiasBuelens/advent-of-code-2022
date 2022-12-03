use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn find_shared_item(rucksack: &str) -> char {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    for item in left.chars() {
        if right.contains(item) {
            return item;
        }
    }
    panic!("no shared item type found: {}", rucksack);
}

fn item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - ('a' as u32) + 1,
        'A'..='Z' => (item as u32) - ('A' as u32) + 27,
        _ => panic!("invalid item type: {}", item),
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|rucksack| find_shared_item(&rucksack))
        .map(item_priority)
        .sum()
}

fn find_common_item_in_group(rucksacks: &[String]) -> char {
    let mut common = rucksacks[0].chars().collect::<HashSet<_>>();
    for rucksack in rucksacks.iter().skip(1) {
        common.retain(|c| rucksack.contains(*c));
    }
    assert_eq!(common.len(), 1);
    *common.iter().next().unwrap()
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> u32 {
    input
        .chunks_exact(3)
        .map(find_common_item_in_group)
        .map(item_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 70);
    }
}
