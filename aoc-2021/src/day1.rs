type Input = u16;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1, loop)]
pub fn solve_part1_1(input: &[Input]) -> usize {
    let mut r = 0;

    for (idx, v) in input.iter().enumerate().skip(1) {
        if input[idx - 1] < *v {
            r += 1;
        }
    }

    r
}

#[aoc(day1, part1, iterator)]
pub fn solve_part1_2(input: &[Input]) -> usize {
    input.iter().enumerate().skip(1).map(|(idx, v)| input[idx - 1] < *v).filter(|r| *r).count()
}

#[aoc(day1, part2, loop)]
pub fn solve_part2_1(input: &[Input]) -> usize {
    let mut r = 0;

    for (idx, v) in input.iter().enumerate().skip(3) {
        let window_val = input[idx - 1] + input[idx - 2] + input[idx - 3];
        let next_window_val = v + input[idx - 1] + input[idx - 2];

        if window_val < next_window_val {
            r += 1;
        }
    }

    r
}

#[aoc(day1, part2, iterator)]
pub fn solve_part2_2(input: &[Input]) -> usize {
    input.iter().enumerate().skip(3).map(|(idx, v)| input[idx - 1] + input[idx - 2] + input[idx - 3] < v + input[idx - 1] + input[idx - 2]).filter(|r| *r).count()
}