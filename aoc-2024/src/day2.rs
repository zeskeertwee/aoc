use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|l| l.split_whitespace().map(|v| v.parse().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|v| check_report(v)).sum()
}

fn check_report(i: &[u32]) -> u32 {
    let mut diff: i32 = i[1] as i32 - i[0] as i32;

    if !good_diff(diff) {
        return 0;
    }

    for j in 1..i.len() - 1 {
        let diff2: i32 = i[j + 1] as i32 - i[j] as i32;

        if diff * diff2 < 0 || !good_diff(diff2) { // negative -> not same sign
            return 0;
        }
        diff = diff2;
    }

    return 1;
}

fn good_diff(diff: i32) -> bool {
    !(diff == 0 || diff > 3 || diff < -3)
}


#[aoc(day2, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    let mut count = 0;

    'outer: for i in input {
        if check_report(i) == 1 {
            count += 1;
            continue;
        }

        for j in 0..i.len() {
            let mut i = i.clone();
            i.remove(j);
            if check_report(&i) == 1 {
                count += 1;
                continue 'outer;
            }
        }
    }

    count
}

aoc_test!(test_day2_sample, "../samples/day2.txt", 2, 4);
aoc_test!(test_day2, "../input/2024/day2.txt", 371, 426);