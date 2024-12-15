use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::mat2::Mat2;
use aoclib::vec2::Vector2;

#[derive(Debug)]
struct Input {
    a: Vector2<f64>,
    b: Vector2<f64>,
    target: Vector2<f64>,
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<Input> {
    input.split("\n\n").map(|i| {
        let lines: Vec<&str> = i.lines().collect();
        let a = {
            let split: Vec<&str> = lines[0][9..].trim().split_whitespace().collect();
            Vector2::new(split[0].replace("X+", "").replace(",", "").parse::<u64>().unwrap() as f64, split[1].replace("Y+", "").parse::<u64>().unwrap() as f64)
        };
        let b = {
            let split: Vec<&str> = lines[1][9..].trim().split_whitespace().collect();
            Vector2::new(split[0].replace("X+", "").replace(",", "").parse::<u64>().unwrap() as f64, split[1].replace("Y+", "").parse::<u64>().unwrap() as f64)
        };
        let target = {
            let split: Vec<&str> = lines[2][6..].trim().split_whitespace().collect();
            Vector2::new(split[0].replace("X=", "").replace(",", "").parse::<u64>().unwrap() as f64, split[1].replace("Y=", "").parse::<u64>().unwrap() as f64)
        };

        Input {a, b, target}
    }).collect()
}

#[aoc(day13, part1)]
fn part1(input: &[Input]) -> usize {
    input.iter().map(|i| solve_token_cost(i, 0.0)).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Input]) -> usize {
    input.iter().map(|i| solve_token_cost(i, 10000000000000.0)).sum()
}

fn solve_token_cost(i: &Input, offset: f64) -> usize {
    let mat = Mat2::new_to_basis(i.a, i.b);
    if let Some(presses) = check_vec(mat * (i.target + Vector2::new(offset, offset))) {
        presses.x * 3 + presses.y
    } else {
        0
    }
}

fn check_vec(v: Vector2<f64>) -> Option<Vector2<usize>> {
    if v.x.is_sign_negative() || v.y.is_sign_negative() {
        return None;
    }

    // is it close to 1 or 0?
    if (v.x % 1.0 > 0.001 && v.x % 1.0 < 0.999) || (v.y % 1.0 > 0.001 && v.y % 1.0 < 0.999) {
        return None;
    }

    Some(Vector2::new(v.x.round() as usize, v.y.round() as usize))
}

aoc_test!(test_day13_sample, "../samples/day13.txt", 480, 875318608908);
aoc_test!(test_day13, "../input/2024/day13.txt", 37680, 87550094242995);