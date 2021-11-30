#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| {
        line.chars().map(|c| c == '#').collect()
    }).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<bool>]) -> usize {
    count_trees_on_path(input, 3, 1)
}

const PART2_SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<bool>]) -> usize {
    PART2_SLOPES.iter().map(|(dx, dy)| count_trees_on_path(input, *dx, *dy)).product()
}

fn count_trees_on_path(map: &[Vec<bool>], dx: usize, dy: usize) -> usize {
    let (mut posx, mut posy) = (0, 0);
    let mut count = 0;

    for _ in 0..map.len() / dy {
        if map[posy][posx % map[0].len()] {
            count += 1;
        }

        posx += dx;
        posy += dy;
    }

    count
}