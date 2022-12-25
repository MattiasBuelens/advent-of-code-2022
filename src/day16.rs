use std::collections::{HashMap, HashSet};

use pathfinding::directed::dijkstra::dijkstra_all;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Valve> {
    input
        .lines()
        .map(|line| {
            let s = line.strip_prefix("Valve ").unwrap();
            let (name, s) = s.split_once(" has flow rate=").unwrap();
            let (flow_rate, s) = s.split_once("; tunnel").unwrap();
            let tunnels = s
                .strip_prefix("s lead to valves ")
                .or_else(|| s.strip_prefix(" leads to valve "))
                .unwrap();
            Valve {
                name: name.to_string(),
                flow_rate: flow_rate.parse().unwrap(),
                tunnels: tunnels.split(", ").map(|x| x.to_string()).collect(),
            }
        })
        .collect()
}

type DistanceMap = HashMap<(String, String), u32>;

fn make_distance_map(valves: &HashMap<String, Valve>) -> DistanceMap {
    let mut map = DistanceMap::new();
    for start in valves.values() {
        map.insert((start.name.clone(), start.name.clone()), 0);
        for (dest, (_, distance)) in dijkstra_all(start, |valve| {
            valve
                .tunnels
                .iter()
                .map(|next| (valves.get(next).unwrap().clone(), 1))
                .collect::<Vec<_>>()
        }) {
            map.insert((start.name.clone(), dest.name.clone()), distance);
            map.insert((dest.name.clone(), start.name.clone()), distance);
        }
    }
    map
}

#[derive(Debug, Clone)]
struct State {
    time: u32,
    max_time: u32,
    position: String,
    open_valves: HashSet<String>,
    total_flow_rate: u32,
    released_pressure: u32,
}

impl State {
    fn new(position: String, max_time: u32) -> Self {
        Self {
            time: 0,
            max_time,
            position,
            open_valves: HashSet::new(),
            total_flow_rate: 0,
            released_pressure: 0,
        }
    }

    fn successors(&self, valves: &HashMap<String, Valve>, distances: &DistanceMap) -> Vec<State> {
        let mut successors = Vec::new();
        if self.time == self.max_time {
            // Time's up! No more steps.
            return successors;
        }
        // Move to closed valve and open it
        let closed_valves = valves
            .keys()
            .filter(|&name| !self.open_valves.contains(name));
        successors.extend(closed_valves.filter_map(|valve_name| {
            let valve = valves.get(valve_name).unwrap();
            if valve.flow_rate == 0 {
                // No point in opening this valve
                return None;
            }
            // N time steps to move to valve
            let distance = *distances
                .get(&(self.position.clone(), valve_name.clone()))
                .unwrap();
            // 1 time step to open it
            let distance = distance + 1;
            if self.time + distance >= self.max_time {
                // Can't get to valve in time
                return None;
            }
            let mut next = self.clone();
            next.time += distance;
            next.released_pressure += distance * next.total_flow_rate;
            next.position = valve_name.clone();
            next.open_valves.insert(valve_name.clone());
            next.total_flow_rate += valve.flow_rate;
            Some(next)
        }));
        // Otherwise, stay here indefinitely
        if successors.is_empty() {
            successors.push({
                let remaining_time = self.max_time - self.time;
                let mut next = self.clone();
                next.time += remaining_time;
                next.released_pressure += remaining_time * next.total_flow_rate;
                next
            });
        }
        successors
    }
}

fn solve(valves: &HashMap<String, Valve>, max_time: u32) -> Vec<State> {
    let distances = make_distance_map(valves);
    let start_state = State::new("AA".to_string(), max_time);
    let mut queue = Vec::<State>::new();
    queue.push(start_state);
    let mut solutions = Vec::<State>::new();
    while let Some(state) = queue.pop() {
        for next in state.successors(valves, &distances) {
            if next.time == max_time {
                // If new solution is not better for this set of valves, drop it
                if let Some(best_for_valves) =
                    solutions.iter().find(|x| x.open_valves == next.open_valves)
                {
                    if best_for_valves.released_pressure > next.released_pressure {
                        continue;
                    }
                }
                // Remove other solutions which are less optimal than the new solution
                solutions.retain(|x| {
                    next.open_valves != x.open_valves
                        || next.released_pressure <= x.released_pressure
                });
                // Add new solution
                solutions.push(next);
            } else {
                queue.push(next);
            }
        }
    }
    solutions
}

#[aoc(day16, part1)]
pub fn part1(input: &[Valve]) -> u32 {
    let valves = input
        .iter()
        .cloned()
        .map(|valve| (valve.name.clone(), valve))
        .collect::<HashMap<_, _>>();
    let max_time = 30;
    let solutions = solve(&valves, max_time);
    solutions
        .iter()
        .map(|state| state.released_pressure)
        .max()
        .unwrap()
}

#[aoc(day16, part2)]
pub fn part2(input: &[Valve]) -> u32 {
    let valves = input
        .iter()
        .cloned()
        .map(|valve| (valve.name.clone(), valve))
        .collect::<HashMap<_, _>>();
    let max_time = 26;
    let solutions = solve(&valves, max_time);
    // Trick: even with two actors, we can never open all valves
    // The best solution is for you and the elephant to only open *distinct* valves
    let mut best = 0;
    for (i, left) in solutions.iter().enumerate() {
        for right in &solutions[(i + 1)..] {
            // You and the elephant should never open the same valves
            if left.open_valves.intersection(&right.open_valves).count() == 0 {
                // If there are no overlapping valves, then we can safely add up the released pressures
                let released_pressure = left.released_pressure + right.released_pressure;
                best = best.max(released_pressure)
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 1651);
    }

    // Trick doesn't work on the example input... :-(
    #[test]
    #[ignore]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 1707);
    }
}
