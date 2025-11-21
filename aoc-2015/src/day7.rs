use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use fxhash::FxHashMap;

#[derive(Debug, Clone)]
struct Instruction {
    operation: Operation,
    target: String
}

#[derive(Debug, Clone)]
enum Operation {
    And(String, String),
    NumAnd(u16, String),
    Or(String, String),
    LShift(String, u8),
    RShift(String, u8),
    Not(String),
    AssignValue(u16),
    Assign(String)
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| {
            let split: Vec<String> = l.split(" -> ").map(|s| s.to_string()).collect();

            if l.starts_with("NOT") {
                Instruction {
                    operation: Operation::Not(split[0].replace("NOT ", "").to_string()),
                    target: split[1].clone()
                }
            } else if let Ok(n) = split[0].parse() {
                Instruction {
                    operation: Operation::AssignValue(n),
                    target: split[1].clone()
                }
            } else {
                let split2: Vec<&str> = split[0].split_whitespace().collect();

                Instruction {
                    target: split[1].clone(),
                    operation: if split2.len() == 1 {
                        Operation::Assign(split2[0].to_string())
                    } else {
                        match split2[1] {
                            "AND" => {
                                if let Ok(n) = split2[0].parse() {
                                    Operation::NumAnd(n, split2[2].to_string())
                                } else {
                                    Operation::And(split2[0].to_string(), split2[2].to_string())
                                }
                            },
                            "OR" => Operation::Or(split2[0].to_string(), split2[2].to_string()),
                            "LSHIFT" => Operation::LShift(split2[0].to_string(), split2[2].parse().unwrap()),
                            "RSHIFT" => Operation::RShift(split2[0].to_string(), split2[2].parse().unwrap()),
                            _ => panic!()
                    }}
                }
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Instruction]) -> u16 {
    simulate_wires_until_value_for(input, "a", FxHashMap::default())
}

#[aoc(day7, part2)]
fn part2(input: &[Instruction]) -> u16 {
    let b_override = simulate_wires_until_value_for(input, "a", FxHashMap::default());
    let mut wires = FxHashMap::default();
    wires.insert("b".to_string(), b_override);
    simulate_wires_until_value_for(input, "a", wires)
}

fn simulate_wires_until_value_for(instructions: &[Instruction], target_wire: &str, wires: FxHashMap<String, u16>) -> u16 {
    let mut wires = wires;

    loop {
        for i in instructions {
            let val = match &i.operation {
                Operation::Assign(wire) => if let Some(value) = wires.get(wire) {
                    Some(*value)
                } else { None },
                Operation::AssignValue(n) => Some(*n),
                Operation::Not(wire) => if let Some(value) = wires.get(wire) {
                    Some(!value)
                } else { None },
                Operation::And(wire_a, wire_b) => if let (Some(val_a), Some(val_b)) = (wires.get(wire_a), wires.get(wire_b)) {
                    Some(val_a & val_b)
                } else { None },
                Operation::NumAnd(num, wire) => if let Some(val) = wires.get(wire) {
                    Some(num & val)
                } else { None },
                Operation::Or(wire_a, wire_b) => if let (Some(val_a), Some(val_b)) = (wires.get(wire_a), wires.get(wire_b)) {
                    Some(val_a | val_b)
                } else { None },
                Operation::LShift(wire, shift) => if let Some(value) = wires.get(wire) {
                    Some(value << shift)
                } else { None },
                Operation::RShift(wire, shift) => if let Some(value) = wires.get(wire) {
                    Some(value >> shift)
                } else { None }
            };

            if let Some(val) = val && !wires.contains_key(&i.target) {
                wires.insert(i.target.to_string(), val);
            }
        }

        if let Some(v) = wires.get(target_wire) {
            return *v;
        }
    }
}

aoc_test!(test_day7, "../input/2015/day7.txt", 3176, 14710);