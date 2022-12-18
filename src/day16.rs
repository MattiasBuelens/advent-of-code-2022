use std::collections::{HashMap, HashSet};

use pathfinding::prelude::dijkstra;

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
    for (left_name, left) in valves {
        map.insert((left_name.clone(), left_name.clone()), 0);
        for (right_name, right) in valves {
            if left_name == right_name {
                continue;
            }
            if map.contains_key(&(left_name.clone(), right_name.clone())) {
                continue;
            }
            let (_path, distance) = dijkstra(
                left,
                |valve| {
                    valve
                        .tunnels
                        .iter()
                        .map(|next| (valves.get(next).unwrap().clone(), 1))
                        .collect::<Vec<_>>()
                },
                |valve| valve == right,
            )
            .unwrap();
            map.insert((left_name.clone(), right_name.clone()), distance);
            map.insert((right_name.clone(), left_name.clone()), distance);
        }
    }
    map
}

#[derive(Debug, Clone)]
struct State {
    time: u32,
    position: String,
    open_valves: HashSet<String>,
    total_flow_rate: u32,
    released_pressure: u32,
}

const MAX_TIME: u32 = 30;

impl State {
    fn new(position: String) -> Self {
        Self {
            time: 0,
            position,
            open_valves: HashSet::new(),
            total_flow_rate: 0,
            released_pressure: 0,
        }
    }

    fn successors(&self, valves: &HashMap<String, Valve>, distances: &DistanceMap) -> Vec<State> {
        let mut successors = Vec::new();
        if self.time == MAX_TIME {
            // Time's up! No more steps.
            return successors;
        }
        // Stay here indefinitely
        successors.push({
            let remaining_time = MAX_TIME - self.time;
            let mut next = self.clone();
            next.time += remaining_time;
            next.released_pressure += remaining_time * next.total_flow_rate;
            next
        });
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
            if self.time + distance >= MAX_TIME {
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
        return successors;
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &[Valve]) -> u32 {
    let valves = input
        .into_iter()
        .cloned()
        .map(|valve| (valve.name.clone(), valve))
        .collect::<HashMap<_, _>>();
    let distances = make_distance_map(&valves);
    let start_state = State::new("AA".to_string());
    let mut queue = Vec::<State>::new();
    queue.push(start_state);
    let mut best_state: Option<State> = None;
    while let Some(state) = queue.pop() {
        for next in state.successors(&valves, &distances) {
            if next.time == MAX_TIME {
                match &best_state {
                    Some(best) if best.released_pressure >= next.released_pressure => {
                        // Current best is still the best.
                    }
                    _ => {
                        best_state = Some(next);
                    }
                };
            } else {
                queue.push(next);
            }
        }
    }
    best_state.unwrap().released_pressure
}

#[aoc(day16, part2)]
pub fn part2(input: &[Valve]) -> i32 {
    todo!()
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

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
