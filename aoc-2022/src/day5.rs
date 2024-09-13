use std::collections::VecDeque;
use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    containers: [VecDeque<char>; 9],
    movements: Vec<Movement>
}

struct Movement {
    source: u8,
    destination: u8,
    count: u8
}

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Input {
    let mut containers: [VecDeque<char>; 9] = Default::default();

    'containers: for l in input.lines() {
        let chars: Vec<char> = l.chars().collect();
        for i in 0..9 {
            let value = chars[(i * 4) + 1];
            if (value != ' ') {
                if (value == '1') {
                    break 'containers;
                }
                containers[i].push_front(value);
            }
        }
    }

    let mut movements = Vec::new();
    for l in input.lines() {
        if !l.starts_with("move") {
            continue;
        }

        let l = l.replace("move ", "").replace(" from ", " ").replace(" to ", " ");
        let n: Vec<u8> = l.split_whitespace().map(|v| v.parse().unwrap()).collect();
        movements.push(Movement {
            count: n[0],
            // subtract 1 because arrays start at 0
            source: n[1] - 1,
            destination: n[2] - 1
        });
    }

    Input {
        containers, movements
    }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> String {
    let mut containers = input.containers.clone();

    for movement in input.movements.iter() {
        for i in 0..movement.count {
            let v  = containers[movement.source as usize].pop_back().unwrap();
            containers[movement.destination as usize].push_back(v);
        }
    }

    String::from_iter(containers.map(|r| *r.back().unwrap()))
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> String {
    let mut containers = input.containers.clone();

    for movement in input.movements.iter() {
        for i in 0..movement.count {
            let last_idx = containers[movement.source as usize].len() - 1;
            let containers_to_go = movement.count - i - 1;
            let v  = containers[movement.source as usize].remove(last_idx - containers_to_go as usize).unwrap();
            containers[movement.destination as usize].push_back(v);
        }
    }

    String::from_iter(containers.map(|r| *r.back().unwrap()))
}