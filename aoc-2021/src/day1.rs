type Input = u16;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    let mut r = 0;

    for (idx, v) in input.iter().enumerate().skip(1) {
        if input[idx - 1] < *v {
            r += 1;
        }
    }

    r
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
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
