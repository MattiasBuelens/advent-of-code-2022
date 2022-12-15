use crate::util::Vector2D;

#[derive(Debug, Clone)]
pub struct SensorReading {
    sensor: Vector2D,
    beacon: Vector2D,
}

fn parse_pos(s: &str) -> Vector2D {
    let s = s.strip_prefix("x=").unwrap();
    let (x, y) = s.split_once(", y=").unwrap();
    Vector2D::new(x.parse().unwrap(), y.parse().unwrap())
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<SensorReading> {
    input
        .lines()
        .map(|line| {
            let s = line.strip_prefix("Sensor at ").unwrap();
            let (sensor, beacon) = s.split_once(": closest beacon is at ").unwrap();
            SensorReading {
                sensor: parse_pos(sensor),
                beacon: parse_pos(beacon),
            }
        })
        .collect()
}

fn is_position_without_beacon(readings: &[SensorReading], pos: Vector2D) -> bool {
    readings.iter().any(|reading| {
        pos != reading.beacon
            && (pos - reading.sensor).manhattan_distance()
                <= (reading.beacon - reading.sensor).manhattan_distance()
    })
}

fn count_positions_without_beacons(readings: &[SensorReading], y: i32) -> usize {
    let min_x = readings
        .iter()
        .map(|reading| reading.sensor.x() - (reading.sensor - reading.beacon).manhattan_distance())
        .min()
        .unwrap();
    let max_x = readings
        .iter()
        .map(|reading| reading.sensor.x() + (reading.sensor - reading.beacon).manhattan_distance())
        .max()
        .unwrap();
    (min_x..=max_x)
        .filter(|&x| is_position_without_beacon(readings, Vector2D::new(x, y)))
        .count()
}

#[aoc(day15, part1)]
pub fn part1(input: &[SensorReading]) -> usize {
    count_positions_without_beacons(input, 2_000_000)
}

#[aoc(day15, part2)]
pub fn part2(input: &[SensorReading]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = include_str!("../examples/2022/day15.txt").trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(count_positions_without_beacons(&input, 10), 26);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
