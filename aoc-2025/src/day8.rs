use std::cmp::Ordering;
use std::collections::BinaryHeap;
use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;

struct Dist {
    node_a: usize,
    node_b: usize,
    dist: u64
}

impl PartialEq for Dist {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}

impl Eq for Dist {}

impl PartialOrd for Dist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for Dist {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<[u64; 3]> {
    input.lines()
        .map(|l| {
            let split: Vec<u64> = l.split(',').map(|v| v.parse().unwrap()).collect();
            [split[0], split[1], split[2]]
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[[u64; 3]]) -> usize {
    // the highest distance to be included in the top 1000 shortest distances
    let mut max_dist = u64::MAX;
    let mut euclid_dist: BinaryHeap<Dist> = BinaryHeap::with_capacity(1000);

    for i in 0..input.len() {
        for j in i..input.len() {
            let dist = u64::isqrt(input[i][0] * input[j][0] + input[i][1] * input[j][1] + input[i][2] * input[j][2]);
            if dist > max_dist {
                continue;
            }

            euclid_dist.push(Dist {
                dist,
                node_a: i,
                node_b: j
            });

            if euclid_dist.len() > 10 {
                // pop the highest distance
                euclid_dist.pop();
                max_dist = euclid_dist.peek().unwrap().dist;
            }
        }
    }

    let mut circuit_counter = 0;
    let mut mapping: Vec<u16> = vec![0; input.len()];
    let mut circuit_connections: FxHashMap<u16, u16> = FxHashMap::default();

    for i in euclid_dist {
        if mapping[i.node_a] != 0 && mapping[i.node_b] == 0 {
            mapping[i.node_b] = mapping[i.node_a];
        } else if mapping[i.node_b] != 0 && mapping[i.node_a] == 0 {
            mapping[i.node_a] = mapping[i.node_b];
        } else if mapping[i.node_a] != 0 && mapping[i.node_b] != 0 {
            circuit_connections.insert(mapping[i.node_a], mapping[i.node_b]);
            circuit_connections.insert(mapping[i.node_b], mapping[i.node_a]);
        } else {
            circuit_counter += 1;
            mapping[i.node_a] = circuit_counter;
            mapping[i.node_b] = circuit_counter;
        };
    }

    let mut circuit_sizes = vec![0; circuit_counter as usize + 1];
    for i in mapping {
        if i != 0 {
            circuit_sizes[i as usize - 1] += 1;
        }
    }

    for (a, b) in circuit_connections {
        circuit_sizes[a as usize] += circuit_sizes[b as usize];
        circuit_sizes[b as usize] = 0;
    }
    dbg!(&circuit_sizes);

    circuit_sizes.sort();
    circuit_sizes.iter().rev().take(3).product()
}