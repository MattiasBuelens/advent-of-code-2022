#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

const MARKER_LEN: usize = 4;

fn is_marker(s: &[u8]) -> bool {
    if s.len() != MARKER_LEN {
        return false;
    }
    for (i, b) in s.iter().enumerate() {
        if s[(i + 1)..].contains(b) {
            return false;
        }
    }
    true
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let (pos, _marker) = input
        .as_bytes()
        .windows(MARKER_LEN)
        .enumerate()
        .find(|(_, s)| is_marker(s))
        .unwrap();
    pos + MARKER_LEN
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 7);
        assert_eq!(part1(&"bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
