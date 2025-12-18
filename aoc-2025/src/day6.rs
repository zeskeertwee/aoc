use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;

#[derive(Debug)]
struct Number {
    v: u64,
    shifted: u64
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<(Vec<Number>, char)> {
    // Get the indicies for the empty columns between numbers so we can split them there whilst keeping the number alignment
    let space_indicies: Vec<usize> = input.lines()
        .last()
        .map(|l| l.chars()
            .skip(1)
            .enumerate()
            .filter(|(_, c)| c != &' ')
            .map(|(i, _)| i)
            .collect())
        .unwrap();

    let n_lines = input.lines().count();

    input.lines().map(|l| {
        let mut split = vec![];

        split.push(l[..space_indicies[0]].to_string());
        for i in 0..space_indicies.len() - 1 {
            split.push(l[space_indicies[i] + 1..space_indicies[i+1]].to_string());
        }
        split.push(l[space_indicies[space_indicies.len() - 1] + 1..].to_string());

        split
    })
        .enumerate()
        .fold(Vec::new(), |mut acc: Vec<(Vec<Number>, char)>, (line_idx, v): (usize, Vec<String>)| {
            for (idx, val) in v.iter().enumerate() {
                if acc.get(idx).is_none() {
                    acc.push((Vec::new(), ' '));
                }

                if line_idx == n_lines - 1 {
                    // we are on the final line
                    acc[idx].1 = val.chars().nth(0).unwrap();
                } else {
                    // it's a number, get the whitespace alignment to determine the shift
                    let mut shift = 0;
                    let c: Vec<char> = val.chars().collect();

                    let mut i = c.len() - 1;
                    while i > 0 {
                        if c[i] == ' ' {
                            shift += 1;
                        } else { break }
                        i -= 1;
                    }

                    let v = val.trim().parse().unwrap();
                    acc[idx].0.push(Number {
                        v,
                        shifted: v * 10_u64.pow(shift as u32)
                    });
                }
            }

            acc
        })
}

#[aoc(day6, part1)]
fn part1(input: &[(Vec<Number>, char)]) -> u64 {
    input.iter()
        .map(|(n, op)| match op {
            '+' => n.iter().map(|n| n.v).sum(),
            '*' => n.iter().map(|n| n.v).product(),
            _ => 0
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &[(Vec<Number>, char)]) -> u64 {
    input.iter()
        .map(|(n, op)| {
            let longest_num = n.iter().map(|n| n.v.ilog10() + 1).max().unwrap() as usize;
            let mut numbers: Vec<u64> = vec![0; longest_num];
            let mut col_counter = vec![0; longest_num];

            for v in n.iter().rev() {
                for i in 0..longest_num {
                    let digit = extract_digit(v.shifted, longest_num - i - 1);
                    if digit == 0 {
                        continue;
                    }

                    numbers[i] += digit * 10_u64.pow(col_counter[i]);
                    col_counter[i] += 1;
                }
            }

            match op {
                '+' => numbers.iter().sum(),
                '*' => numbers.iter().product(),
                _ => 0
            }
        })
        .sum()
}

/// extracts the digit at index idx, 0=lowest digit
fn extract_digit(num: u64, idx: usize) -> u64 {
   let exp = 10_u64.pow(idx as u32);

    (num / exp) - ((num / (exp * 10)) * 10)
}

aoc_test!(test_day6, "../input/2025/day6.txt", 4805473544166, 8907730960817);