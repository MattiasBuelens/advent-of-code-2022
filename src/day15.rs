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

fn is_within_reading_range(reading: &SensorReading, pos: Vector2D) -> bool {
    (pos - reading.sensor).manhattan_distance()
        <= (reading.beacon - reading.sensor).manhattan_distance()
}

fn is_position_without_beacon(readings: &[SensorReading], pos: Vector2D) -> bool {
    readings
        .iter()
        .any(|reading| pos != reading.beacon && is_within_reading_range(reading, pos))
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

fn could_be_distress_beacon(readings: &[SensorReading], pos: Vector2D) -> bool {
    readings
        .iter()
        .all(|reading| pos != reading.beacon && !is_within_reading_range(reading, pos))
}

fn find_distress_beacon(readings: &[SensorReading], max_coord: i32) -> Vector2D {
    for y in 0..=max_coord {
        let mut x = 0;
        while x <= max_coord {
            let pos = Vector2D::new(x, y);
            if let Some(reading) = readings
                .iter()
                .find(|reading| is_within_reading_range(reading, pos))
            {
                // Skip ahead along this row until we're out of range of this reading.
                let to_beacon = reading.beacon - reading.sensor;
                let to_pos = pos - reading.sensor;
                x = reading.sensor.x() + to_beacon.manhattan_distance() - to_pos.y().abs() + 1;
                debug_assert!(is_within_reading_range(reading, Vector2D::new(x - 1, y)));
                debug_assert!(!is_within_reading_range(reading, Vector2D::new(x, y)));
            } else {
                // No sensor reading in range, this must be the distress beacon!
                return pos;
            }
        }
    }
    panic!("distress beacon not found")
}

#[aoc(day15, part2)]
pub fn part2(input: &[SensorReading]) -> i64 {
    let max_coord = 4_000_000;
    let pos = find_distress_beacon(input, max_coord);
    (pos.x() as i64) * (max_coord as i64) + (pos.y() as i64)
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
        assert_eq!(find_distress_beacon(&input, 20), Vector2D::new(14, 11));
    }
}
