pub enum Input {
    Horizontal(i8),
    Vertical(i8),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|x| {
            let split: Vec<&str> = x.split_whitespace().collect();
            let num = split[1].parse::<i8>().unwrap();
            match split[0] {
                "forward" => Input::Horizontal(num),
                "up" => Input::Vertical(-num),
                "down" => Input::Vertical(num),
                _ => panic!("Unknown input"),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    let r = input.iter().fold((0, 0), |(mut hor, mut ver), v| {
        match v {
            Input::Horizontal(x) => hor += *x as i64,
            Input::Vertical(x) => ver += *x as i64,
        };
        (hor, ver)
    });

    return (r.0 * r.1).abs() as usize;
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    let r = input
        .iter()
        .fold((0, 0, 0), |(mut hor, mut ver, mut aim), v| {
            match v {
                Input::Horizontal(x) => {
                    hor += *x as i64;
                    ver += *x as i64 * aim;
                }
                Input::Vertical(x) => aim += *x as i64,
            }

            (hor, ver, aim)
        });

    return (r.0 * r.1).abs() as usize;
}
