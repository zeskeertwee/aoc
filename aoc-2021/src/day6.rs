type Input = u8;

#[aoc_generator(day6)]
pub fn generate_input(input: &str) -> Vec<Input> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    (0..80)
        .fold(input.to_owned(), |acc, _| run_iteration(&acc))
        .len()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    (0..256)
        .fold(input.to_owned(), |acc, _| run_iteration(&acc))
        .len()
}

pub fn run_iteration(state: &[Input]) -> Vec<Input> {
    let mut result = Vec::new();

    for v in state.iter() {
        if *v == 0 {
            result.push(8);
            result.push(6);
            continue;
        }
        result.push(*v - 1);
    }

    result
}
