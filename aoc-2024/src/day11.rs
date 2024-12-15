use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use fxhash::FxHashMap;

#[derive(Copy, Clone)]
enum Next {
    Single(u64),
    Double(u64, u64)
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<u64> {
    input.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect()
}

#[aoc(day11, part1)]
fn part1(input: &[u64]) -> u64 {
    blink(input, 25)
}

#[aoc(day11, part2)]
fn part2(input: &[u64]) -> u64 {
    blink(input, 75)
}

fn blink(input: &[u64], times: usize) -> u64 {
    let mut map: FxHashMap<u64, u64> = FxHashMap::default();

    input.iter().for_each(|v| {
        *map.entry(*v).or_insert(0) += 1;
    });

    for _ in 0..times {
        let mut new_map = FxHashMap::default();
        for (k, v) in map.iter() {
            match next_number(*k) {
                Next::Single(num) => *new_map.entry(num).or_insert(0) += *v,
                Next::Double(num1, num2) => {
                    *new_map.entry(num1).or_insert(0) += *v;
                    *new_map.entry(num2).or_insert(0) += *v;
                }
            }
        }
        map = new_map;
    }

    map.values().sum()
}

fn next_number(n: u64) -> Next {
    if n == 0 {
        return Next::Single(1)
    }

    let digits = digits(n);
    if is_even(digits) {
        let (a, b) = split_num(n, digits);
        return Next::Double(a, b);
    }

    Next::Single(n * 2024)
}

fn digits(n: u64) -> u32 {
    n.ilog10() + 1
}

fn is_even(n: u32) -> bool {
    n & 1 == 0
}

fn split_num(n: u64, digits: u32) -> (u64, u64) {
    let factor = 10u64.pow(digits / 2);

    (n / factor, n % factor)
}

aoc_test!(test_day11_sample, "../samples/day11.txt", 55312, 65601038650482);
aoc_test!(test_day11, "../input/2024/day11.txt", 199982, 237149922829154);