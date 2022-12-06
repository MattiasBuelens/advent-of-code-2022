#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

const PACKET_MARKER_LEN: usize = 4;

fn is_marker(s: &[u8]) -> bool {
    for (i, b) in s.iter().enumerate() {
        if s[(i + 1)..].contains(b) {
            return false;
        }
    }
    true
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let (pos, marker) = input
        .as_bytes()
        .windows(PACKET_MARKER_LEN)
        .enumerate()
        .find(|(_, s)| is_marker(s))
        .unwrap();
    pos + marker.len()
}

const MESSAGE_MARKER_LEN: usize = 14;

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (pos, marker) = input
        .as_bytes()
        .windows(MESSAGE_MARKER_LEN)
        .enumerate()
        .find(|(_, s)| is_marker(s))
        .unwrap();
    pos + marker.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT1: &'static str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        static ref INPUT2: &'static str = r"bvwbjplbgvbhsrlpgdmjqwftvncz";
        static ref INPUT3: &'static str = r"nppdvjthqldpwncqszvftbrmjlhg";
        static ref INPUT4: &'static str = r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        static ref INPUT5: &'static str = r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT1), 7);
        assert_eq!(part1(&INPUT2), 5);
        assert_eq!(part1(&INPUT3), 6);
        assert_eq!(part1(&INPUT4), 10);
        assert_eq!(part1(&INPUT5), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT1), 19);
        assert_eq!(part2(&INPUT2), 23);
        assert_eq!(part2(&INPUT3), 23);
        assert_eq!(part2(&INPUT4), 29);
        assert_eq!(part2(&INPUT5), 26);
    }
}
