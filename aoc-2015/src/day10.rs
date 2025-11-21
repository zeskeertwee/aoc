use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<u8> {
    input.chars().map(|c| c as u8 - '0' as u8).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[u8]) -> usize {
    let mut number = input.to_vec();

    for i in 0..40 {
        number = apply_process(&number);
    }

    number.len()
}

#[aoc(day10, part2)]
fn part2(input: &[u8]) -> usize {
    let mut number = input.to_vec();

    for i in 0..50 {
        number = apply_process(&number);
    }

    number.len()
}

fn apply_process(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();

    let mut counter = 0;
    let mut n = 0;

    for i in input {
        if *i == n {
            counter += 1;
        } else if n == 0 {
            n = *i;
            counter = 1;
        } else {
            // we found a different digit, so start a new one and append the previous one to the output
            output.extend([counter, n]);
            n = *i;
            counter = 1;
        }
    }

    output.extend([counter, n]);

    output
}