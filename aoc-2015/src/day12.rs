use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day12, part1)]
fn part1(input: &[char]) -> isize {
    let mut sum = 0;
    let mut acc = vec![];

    for c in input {
        if c.is_numeric() || *c == '-' {
            acc.push(*c);
        } else if acc.len() > 0 {
            sum += String::from_iter(&acc).parse::<isize>().unwrap();
            acc.clear();
        }
    }

    sum
}

#[aoc(day12, part2)]
fn part2(input: &[char]) -> isize {
    get_object_sum(input, 0).0
}

fn get_object_sum(data: &[char], idx: usize) -> (isize, usize) {
    dbg!(idx);
    let mut sum = 0;
    let mut red = false;
    let mut acc = vec![];
    let mut i = idx;

    while i < data.len() {
        let c = data[i];

        if c.is_numeric() || c == '-' {
            acc.push(c);
            i += 1;
            continue;
        } else if c == '{' {
            let (a,b) = get_object_sum(data, i + 1);
            sum += a;
            i = b;

            if acc.len() > 0 {
                dbg!(String::from_iter(&acc));
                sum += String::from_iter(&acc).parse::<isize>().unwrap();
                acc.clear();
            }
        } else if c == '}' {
            if red {
                return (0, i+1);
            } else {
                return (sum, i + 1);
            }
        } else if c == 'r' && data[i+1] == 'e' && data[i+2] == 'd' {
            // make sure that it's a value and not a key
            if data[i+4] != ':' {
                red = true;
            }
        }

        if acc.len() > 0 {
            dbg!(String::from_iter(&acc));
            sum += String::from_iter(&acc).parse::<isize>().unwrap();
            acc.clear();
        }

        i += 1;
    }

    (if red { 0 } else { sum }, i)
}