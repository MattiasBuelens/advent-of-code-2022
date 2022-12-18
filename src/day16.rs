use std::array::from_fn;
use std::collections::{HashMap, HashSet};
use std::mem;

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

fn make_valve_map(valves: &[Valve]) -> HashMap<String, Valve> {
    valves
        .iter()
        .cloned()
        .map(|valve| (valve.name.clone(), valve))
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
enum Action {
    Idle(String),
    Moving(String, String, u32),
}

#[derive(Debug, Clone)]
struct State<const N: usize> {
    time: u32,
    max_time: u32,
    actions: [Action; N],
    open_valves: HashSet<String>,
    total_flow_rate: u32,
    released_pressure: u32,
}

impl<const N: usize> State<N> {
    fn new(position: String, max_time: u32) -> Self {
        Self {
            time: 0,
            max_time,
            actions: from_fn(|_| Action::Idle(position.clone())),
            open_valves: HashSet::new(),
            total_flow_rate: 0,
            released_pressure: 0,
        }
    }

    fn successors(
        &self,
        valves: &HashMap<String, Valve>,
        distances: &DistanceMap,
    ) -> Vec<State<N>> {
        if self.time == self.max_time {
            // Time's up! No more steps.
            return Vec::new();
        }
        // Update each actor
        let mut states = vec![self.clone()];
        for actor_idx in 0..N {
            states = states
                .into_iter()
                .flat_map(|state| {
                    state
                        .next_actions(actor_idx, valves, distances)
                        .into_iter()
                        .map(move |next_action| {
                            let mut next = state.clone();
                            next.actions[actor_idx] = next_action;
                            next
                        })
                })
                .collect()
        }
        // Tick the time
        for state in states.iter_mut() {
            state.tick(valves);
        }
        states
    }

    fn next_actions(
        &self,
        actor_idx: usize,
        valves: &HashMap<String, Valve>,
        distances: &DistanceMap,
    ) -> Vec<Action> {
        // Ensure actor is idle
        let pos = match &self.actions[actor_idx] {
            Action::Idle(pos) => pos.clone(),
            action => return vec![action.clone()],
        };
        // Try to move to each closed valve and open it
        let mut actions = valves
            .iter()
            .flat_map(|(valve_name, valve)| {
                // No point in opening a jammed valve
                if valve.flow_rate == 0 {
                    return None;
                }
                // Don't go to an already opened valve
                if self.open_valves.contains(valve_name) {
                    return None;
                }
                // Don't go to a closed valve if someone is already going there
                for action in &self.actions {
                    if let Action::Moving(_, to, _) = action {
                        if to == valve_name {
                            return None;
                        }
                    }
                }
                // N time steps to move to valve
                let distance = *distances.get(&(pos.clone(), valve_name.clone())).unwrap();
                // 1 time step to open it
                let distance = distance + 1;
                if self.time + distance >= self.max_time {
                    // Not enough time to open valve
                    return None;
                }
                Some(Action::Moving(pos.clone(), valve_name.clone(), distance))
            })
            .collect::<Vec<_>>();
        // If actor cannot move to any valve, stay idle
        if actions.is_empty() {
            actions.push(Action::Idle(pos));
        }
        actions
    }

    fn tick(&mut self, valves: &HashMap<String, Valve>) {
        self.time += 1;
        self.released_pressure += self.total_flow_rate;
        for action in self.actions.as_mut() {
            *action = match mem::replace(action, Action::Idle(String::new())) {
                Action::Moving(_, to, distance) if distance == 1 => {
                    // Open the valve
                    let valve = valves.get(&to).unwrap();
                    self.open_valves.insert(to.clone());
                    self.total_flow_rate += valve.flow_rate;
                    // Return to idle
                    Action::Idle(to)
                }
                Action::Moving(from, to, distance) => {
                    // Keep moving
                    Action::Moving(from, to, distance - 1)
                }
                Action::Idle(pos) => Action::Idle(pos),
            };
        }
    }

    fn solve(self, valves: &HashMap<String, Valve>, distances: &DistanceMap) -> Option<Self> {
        let max_time = self.max_time;
        let mut queue = Vec::<Self>::new();
        queue.push(self);
        let mut best_state: Option<Self> = None;
        while let Some(state) = queue.pop() {
            for next in state.successors(valves, distances) {
                if next.time == max_time {
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
        best_state
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &[Valve]) -> u32 {
    let valves = make_valve_map(input);
    let distances = make_distance_map(&valves);
    let max_time = 30;
    let start_state = State::<1>::new("AA".to_string(), max_time);
    let best_state = start_state.solve(&valves, &distances).unwrap();
    best_state.released_pressure
}

#[aoc(day16, part2)]
pub fn part2(input: &[Valve]) -> u32 {
    let valves = make_valve_map(input);
    let distances = make_distance_map(&valves);
    let max_time = 26;
    let start_state = State::<2>::new("AA".to_string(), max_time);
    let best_state = start_state.solve(&valves, &distances).unwrap();
    dbg!(&best_state);
    best_state.released_pressure
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
        assert_eq!(part2(&input), 1707);
    }
}
