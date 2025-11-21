use std::cmp::Ordering;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use std::collections::BinaryHeap;
use fxhash::FxHashSet;

struct Edge {
    src: String,
    dst: String,
    cost: u16
}

#[derive(Clone, Debug)]
struct Route<const REVERSE: bool> {
    via: FxHashSet<String>,
    current_pos: String,
    cost: u16
}

impl<const REVERSE: bool> Eq for Route<REVERSE> {}

impl<const REVERSE: bool> Ord for Route<REVERSE> {
    fn cmp(&self, other: &Self) -> Ordering {
        if REVERSE {
            other.cost.cmp(&self.cost)
        } else {
            self.cost.cmp(&other.cost)
        }
    }
}

impl<const REVERSE: bool> PartialEq<Self> for Route<REVERSE> {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl<const REVERSE: bool> PartialOrd<Self> for Route<REVERSE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if REVERSE {
            other.cost.partial_cmp(&self.cost)
        } else {
            self.cost.partial_cmp(&other.cost)
        }
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Edge> {
    let mut edges: Vec<Edge> = input.lines()
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            [Edge {
                src: split[0].to_string(),
                dst: split[2].to_string(),
                cost: split[4].parse().unwrap()
            }, Edge {
                src: split[2].to_string(),
                dst: split[0].to_string(),
                cost: split[4].parse().unwrap()
            }]
        })
        .flatten()
        .collect();

    edges
}

#[aoc(day9, part1)]
fn part1(input: &[Edge]) -> u16 {
    bfs_search_routes::<true>(input)
}

#[aoc(day9, part2)]
fn part2(input: &[Edge]) -> u16 {
    bfs_search_routes::<false>(input)
}

fn bfs_search_routes<const SEARCH_MIN_COST: bool>(edges: &[Edge]) -> u16 {
    let locations: Vec<String> = {
        let mut locations = FxHashSet::default();
        for i in edges {
            locations.insert(i.src.to_string());
        }
        locations.into_iter().collect()
    };

    let mut queue: BinaryHeap<Route<SEARCH_MIN_COST>> = BinaryHeap::new();
    for loc in &locations {
        queue.push(Route {
            cost: 0,
            current_pos: loc.to_string(),
            via: FxHashSet::default()
        })
    }

    let mut max_cost = 0;

    while !queue.is_empty() {
        let route = queue.pop().unwrap();
        if route.via.len() == locations.len() - 1 {
            if SEARCH_MIN_COST {
                return route.cost;
            } else if route.cost > max_cost {
                max_cost = route.cost;
            }
        }

        for edge in edges {
            if edge.src == route.current_pos && !route.via.contains(&edge.dst) {
                // Go there!
                let mut queue_route = route.clone();
                queue_route.via.insert(queue_route.current_pos);
                queue_route.current_pos = edge.dst.to_string();
                queue_route.cost += edge.cost;
                queue.push(queue_route);
            }
        }
    }

    max_cost
}

aoc_test!(test_day9, "../input/2015/day9.txt", 141, 736);