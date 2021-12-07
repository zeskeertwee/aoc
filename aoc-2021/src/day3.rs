type Input = u16;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .map(|(idx, v)| ((v == '1') as u16) << 12 - idx)
                .sum()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    let gamma = (0..12).fold(0_u16, |mut acc, idx| {
        acc = acc << 1;
        acc += find_most_common(input, idx) as u16;
        acc
    });

    let epsilon = !gamma & 0b0000_1111_1111_1111;
    assert_eq!(gamma + epsilon, 0b0000_1111_1111_1111);

    gamma as usize * epsilon as usize
}

pub fn true_at_index(v: Input, idx: usize) -> bool {
    v & 1_u16 << 12 - idx == 1_u16 << 12 - idx
}

pub fn find_most_common(data: &[Input], idx: usize) -> bool {
    let count = data.iter().filter(|v| true_at_index(**v, idx)).count();

    count >= data.len() - count
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    let (oxygen_rating, co2_rating) = {
        let init = split_using_index(input, 0);
        (1..12).fold(init, |(oxygen_acc, co2_acc), idx| {
            (
                if oxygen_acc.len() > 1 {
                    split_using_index(&oxygen_acc, idx).0
                } else {
                    oxygen_acc
                },
                if co2_acc.len() > 1 {
                    split_using_index(&co2_acc, idx).1
                } else {
                    co2_acc
                },
            )
        })
    };

    assert_eq!(oxygen_rating.len(), 1);
    assert_eq!(co2_rating.len(), 1);
    assert!(oxygen_rating[0] < 0b0000_1111_1111_1111);
    assert!(input.contains(&co2_rating[0]));
    assert!(co2_rating[0] < 0b0000_1111_1111_1111);
    assert_ne!(oxygen_rating[0], co2_rating[0]);
    println!("{:b}\n{:b}", oxygen_rating[0], co2_rating[0]);

    oxygen_rating[0] as usize * co2_rating[0] as usize
}

pub fn split_using_index(data: &[Input], idx: usize) -> (Vec<Input>, Vec<Input>) {
    let oxygen_val = find_most_common(data, idx);

    data.iter()
        .map(|v| {
            if true_at_index(*v, idx) == oxygen_val {
                (Some(*v), None)
            } else {
                (None, Some(*v))
            }
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut common_acc, mut uncommon_acc), v| {
                match v {
                    (Some(x), None) => common_acc.push(x),
                    (None, Some(x)) => uncommon_acc.push(x),
                    _ => unreachable!(),
                }

                (common_acc, uncommon_acc)
            },
        )
}
