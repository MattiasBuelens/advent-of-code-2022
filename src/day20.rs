#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn mix(values: &mut Vec<i32>, order: &[i32]) {
    let mut indices = (0..values.len()).collect::<Vec<_>>();
    for (i, &shift) in order.into_iter().enumerate() {
        if shift == 0 {
            continue;
        }
        let idx = indices.iter().position(|&x| x == i).unwrap();
        let len = indices.len() as i32 - 1;
        let mut new_idx = (((idx as i32 + shift) % len) + len) % len;
        if new_idx == 0 {
            new_idx = len;
        }
        let value = indices.remove(idx);
        indices.insert(new_idx as usize, value);
    }
    let orig_values = values.clone();
    for (i, idx) in indices.into_iter().enumerate() {
        values[i] = orig_values[idx];
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut values = input.to_vec();
    mix(&mut values, input);
    let zero_pos = values.iter().position(|&x| x == 0).unwrap();
    values[(zero_pos + 1000) % values.len()]
        + values[(zero_pos + 2000) % values.len()]
        + values[(zero_pos + 3000) % values.len()]
}

#[aoc(day20, part2)]
pub fn part2(input: &[i32]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
1
2
-3
3
-2
0
4"
        .trim();
    }

    #[test]
    fn test_mix() {
        let input = input_generator(&TEST_INPUT);
        let mut values = input.clone();
        mix(&mut values, &input);
        assert_eq!(&values, &[1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
