use std::cmp::Ordering;

use itertools::{EitherOrBoth, Itertools};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

pub type Input = Vec<(Packet, Packet)>;

fn parse_packet(s: &str) -> Packet {
    let (packet, rest) = consume_packet(s);
    assert!(rest.is_empty());
    packet
}

fn consume_packet(s: &str) -> (Packet, &str) {
    if let Some(mut s) = s.strip_prefix('[') {
        let mut packets = Vec::new();
        loop {
            if let Some(s) = s.strip_prefix(']') {
                return (Packet::List(packets), s);
            } else {
                let (packet, rest) = consume_packet(s);
                packets.push(packet);
                s = rest.strip_prefix(',').unwrap_or(rest);
            }
        }
    } else if let Some(offset) = s.find(|c: char| !c.is_ascii_digit()) {
        let (digits, s) = s.split_at(offset);
        (Packet::Number(digits.parse().expect("invalid number")), s)
    } else {
        (Packet::Number(s.parse().expect("invalid number")), "")
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            let left = parse_packet(lines.next().unwrap());
            let right = parse_packet(lines.next().unwrap());
            (left, right)
        })
        .collect()
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(left), Packet::Number(right)) => {
                // If both values are integers, the lower integer should come first.
                left.partial_cmp(right)
            }
            (Packet::List(left), Packet::List(right)) => {
                // If both values are lists, compare the first value of each list,
                // then the second value, and so on.
                for pair in left.iter().zip_longest(right.iter()) {
                    match pair {
                        EitherOrBoth::Both(left, right) => {
                            match left.cmp(right) {
                                ord @ (Ordering::Less | Ordering::Greater) => return Some(ord),
                                Ordering::Equal => {}
                            };
                        }
                        EitherOrBoth::Right(_) => {
                            // If the left list runs out of items first,
                            // the inputs are in the right order.
                            return Some(Ordering::Less);
                        }
                        EitherOrBoth::Left(_) => {
                            // If the right list runs out of items first,
                            // the inputs are not in the right order.
                            return Some(Ordering::Greater);
                        }
                    }
                }
                // If the lists are the same length and no comparison makes a decision
                // about the order, continue checking the next part of the input.
                Some(Ordering::Equal)
            }
            (left @ Packet::Number(_), right @ Packet::List(_)) => {
                // If exactly one value is an integer, convert the integer to a list
                // which contains that integer as its only value, then retry the comparison.
                Packet::List(vec![left.clone()]).partial_cmp(right)
            }
            (left @ Packet::List(_), right @ Packet::Number(_)) => {
                left.partial_cmp(&Packet::List(vec![right.clone()]))
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> usize {
    let mut sum = 0;
    for (i, (left, right)) in input.iter().enumerate() {
        if left < right {
            sum += i + 1;
        }
    }
    sum
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
