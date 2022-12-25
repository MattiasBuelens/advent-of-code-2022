use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Ore(u32);
#[derive(Debug, Clone)]
pub struct Clay(u32);
#[derive(Debug, Clone)]
pub struct Obsidian(u32);

#[derive(Debug, Clone)]
pub struct Blueprint {
    number: u8,
    ore_robot: Ore,
    clay_robot: Ore,
    obsidian_robot: (Ore, Clay),
    geode_robot: (Ore, Obsidian),
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Blueprint ").unwrap();
        let (number, s) = s.split_once(": Each ore robot costs ").unwrap();
        let (ore_robot_ore, s) = s.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay_robot_ore, s) = s.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_robot_ore, s) = s.split_once(" ore and ").unwrap();
        let (obsidian_robot_clay, s) = s.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_robot_ore, s) = s.split_once(" ore and ").unwrap();
        let geode_robot_obsidian = s.strip_suffix(" obsidian.").unwrap();
        Ok(Blueprint {
            number: number.parse().unwrap(),
            ore_robot: Ore(ore_robot_ore.parse().unwrap()),
            clay_robot: Ore(clay_robot_ore.parse().unwrap()),
            obsidian_robot: (
                Ore(obsidian_robot_ore.parse().unwrap()),
                Clay(obsidian_robot_clay.parse().unwrap()),
            ),
            geode_robot: (
                Ore(geode_robot_ore.parse().unwrap()),
                Obsidian(geode_robot_obsidian.parse().unwrap()),
            ),
        })
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Blueprint> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

const DURATION: u32 = 24;

#[derive(Debug, Default, Clone)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
}

fn div_round_up(quotient: u32, divisor: u32) -> u32 {
    (quotient + divisor - 1) / divisor
}

impl State {
    fn new() -> Self {
        Self {
            ore_robots: 1,
            ..Self::default()
        }
    }

    fn most_geodes(&mut self, time: u32, blueprint: &Blueprint) -> u32 {
        let mut most_geodes = 0;
        self.solve(time, blueprint, &mut most_geodes);
        most_geodes
    }

    fn solve(&mut self, time: u32, blueprint: &Blueprint, most_geodes: &mut u32) {
        if time == 0 {
            return;
        }
        // The theoretical maximum number of geodes we could make in the remaining time
        // if we had infinite resources. That is: if we could create a new geode-cracking robot
        // in every time step from now until the end.
        // If `most_geodes` is already better than this maximum, then this state is useless.
        let max_geodes = self.geode + (time - 1) * time;
        if max_geodes <= *most_geodes {
            return;
        }
        *most_geodes = (*most_geodes).max(self.geode);
        // Try to build a geode-cracking robot.
        if self.obsidian_robots > 0 {
            let needed_ore = blueprint.geode_robot.0 .0.saturating_sub(self.ore);
            let needed_obsidian = blueprint.geode_robot.1 .0.saturating_sub(self.obsidian);
            // Time to create needed ore is (needed ore) / (number of ore robots), *rounded up*.
            let time_to_ore = div_round_up(needed_ore, self.ore_robots);
            let time_to_obsidian = div_round_up(needed_obsidian, self.obsidian_robots);
            // Need 1 more time unit to make the geode-cracking robot itself
            let time_to_robot = time_to_ore.max(time_to_obsidian) + 1;
            if time > time_to_robot {
                let mut next = self.clone();
                let time = time - time_to_robot;
                next.advance(time_to_robot);
                next.ore -= blueprint.geode_robot.0 .0;
                next.obsidian -= blueprint.geode_robot.1 .0;
                // Optimization: instead of tracking the number of geode-cracking robots,
                // compute how many geodes the new robot will crack ahead of time.
                next.geode += time;
                next.solve(time, blueprint, most_geodes);
            }
        }
        // Check again, in case we already found a much better solution in the previous step.
        if max_geodes <= *most_geodes {
            return;
        }
        // Try to build an obsidian-collecting robot *only* if we could still theoretically
        // use up all of the current obsidian for building geode-cracking robots.
        if self.clay_robots > 0
            && self.obsidian + (self.obsidian_robots * time) < blueprint.geode_robot.1 .0 * time
        {
            let needed_ore = blueprint.obsidian_robot.0 .0.saturating_sub(self.ore);
            let needed_clay = blueprint.obsidian_robot.1 .0.saturating_sub(self.clay);
            let time_to_ore = div_round_up(needed_ore, self.ore_robots);
            let time_to_clay = div_round_up(needed_clay, self.clay_robots);
            let time_to_robot = time_to_ore.max(time_to_clay) + 1;
            if time > time_to_robot {
                let mut next = self.clone();
                let time = time - time_to_robot;
                next.advance(time_to_robot);
                next.ore -= blueprint.obsidian_robot.0 .0;
                next.clay -= blueprint.obsidian_robot.1 .0;
                next.obsidian_robots += 1;
                next.solve(time, blueprint, most_geodes);
            }
        }
        if max_geodes <= *most_geodes {
            return;
        }
        // Try to build a clay-collecting robot *only* if we could still theoretically
        // use up all of the current clay for building obsidian-collecting robots.
        if self.clay + (self.clay_robots * time) < blueprint.obsidian_robot.1 .0 * time {
            let needed_ore = blueprint.clay_robot.0.saturating_sub(self.ore);
            let time_to_ore = div_round_up(needed_ore, self.ore_robots);
            let time_to_robot = time_to_ore + 1;
            if time > time_to_robot {
                let mut next = self.clone();
                let time = time - time_to_robot;
                next.advance(time_to_robot);
                next.ore -= blueprint.clay_robot.0;
                next.clay_robots += 1;
                next.solve(time, blueprint, most_geodes);
            }
        }
        if max_geodes <= *most_geodes {
            return;
        }
        // Try to build an ore-collecting robot *only* if we could still theoretically
        // use up all of the current ore for building robots that needs the most ore.
        let most_ore_needed = blueprint
            .clay_robot
            .0
            .max(blueprint.obsidian_robot.0 .0)
            .max(blueprint.geode_robot.0 .0);
        if self.ore + (self.ore_robots * time) < most_ore_needed * time {
            let needed_ore = blueprint.ore_robot.0.saturating_sub(self.ore);
            let time_to_ore = div_round_up(needed_ore, self.ore_robots);
            let time_to_robot = time_to_ore + 1;
            if time > time_to_robot {
                let mut next = self.clone();
                let time = time - time_to_robot;
                next.advance(time_to_robot);
                next.ore -= blueprint.ore_robot.0;
                next.ore_robots += 1;
                next.solve(time, blueprint, most_geodes);
            }
        }
    }

    fn advance(&mut self, steps: u32) {
        self.ore += self.ore_robots * steps;
        self.clay += self.clay_robots * steps;
        self.obsidian += self.obsidian_robots * steps;
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> u32 {
    input
        .into_iter()
        .map(|blueprint| (blueprint.number as u32) * State::new().most_geodes(DURATION, blueprint))
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Blueprint]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = include_str!("../examples/2022/day19.txt").trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 33);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
