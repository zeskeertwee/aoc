use std::collections::VecDeque;
use aoc_runner_derive::{aoc_generator, aoc};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Input {
    File {
        id: u64,
        size: u8
    },
    Free {
        size: u8
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Input> {
    let mut file = true;
    let mut result = Vec::new();
    let mut id = 0;

    for char in input.trim().chars() {
        let n = (char as u8) - '0' as u8;
        if n != 0 {
            match file {
                true => {
                    result.push(Input::File {
                        id,
                        size: n
                    });
                    id += 1;
                },
                false => result.push(Input::Free { size: n })
            };
        }

        file = !file;
    }

    result
}

#[aoc(day9, part1)]
fn part1(input: &Vec<Input>) -> u64 {
    let mut input = compute_layout(input.iter());
    let mut end = input.len();

    let free = Input::Free { size: 1 };

    for idx in 0..input.len() {
        if input[idx] == free {
            for i in (0..end).rev() {
                if input[i] != free {
                    // swap
                    input[idx] = input[i];
                    input[i] = free.clone();
                    end = i;
                    break;
                }
            }
        }

        if idx >= end - 1 {
            break;
        }
    }

    input[0..end].iter().enumerate().map(|(idx, v)| idx as u64 * match v {
        Input::File { id, .. } => *id,
        Input::Free { .. } => 0,
    }).sum()
}

#[aoc(day9, part2)]
fn part2(input: &Vec<Input>) -> u64 {
    let mut fs: VecDeque<Input> = VecDeque::new();
    fs.extend(input);
    coalesce_free_space(&mut fs);
    let mut last_id = u64::MAX;

    let mut idx = fs.len();
    while idx > 0 {
        idx -= 1;

        // iterate over the fs in reverse
        match fs[idx] {
            Input::File { size, id } => {
                if id >= last_id {
                    // only move each file once with descending ID order
                    continue;
                }
                last_id = id;
                // try if we can find free space where we can put the file
                for idx2 in 0..idx {
                    if let Input::Free { size: free_size } = fs[idx2] && free_size >= size {
                        // we can put the file here
                        // insert the file
                        fs.insert(idx2, Input::File { size, id });
                        // remove the original item
                        fs.remove(idx2 + 1);
                        // insert the free space where we removed the item
                        fs.insert(idx, Input::Free { size });
                        // remove the original item
                        fs.remove(idx + 1);
                        if free_size > size {
                            // also insert the remaining free size
                            fs.insert(idx2 + 1, Input::Free { size: free_size - size });
                            // we grew the fs length by one, so increase idx to not skip over it
                            if idx > idx2 {
                                idx += 1;
                            }
                        }
                        break;
                    }
                }
            },
            _ => (),
        }
    }

    let input = compute_layout(fs.iter());
    input.iter().enumerate().map(|(idx, v)| idx as u64 * match v {
        Input::File { id, .. } => *id,
        Input::Free { .. } => 0,
    }).sum()
}

// turns the layout into the same layout but with 1-long items for part 1
fn compute_layout<'a, T: Iterator<Item = &'a Input>>(input: T) -> Vec<Input> {
    let mut s = Vec::new();

    for i in input {
        match i {
            &Input::File { id, size } => {
                for _ in 0..size {
                    s.push(Input::File { id, size: 1 })
                }
            },
            &Input::Free { size } => {
                for _ in 0..size {
                    s.push(Input::Free { size: 1 })
                }
            }
        }
    }

    s
}

// combines all free space that is adjacent
fn coalesce_free_space(layout: &mut VecDeque<Input>) {
    let mut i = 0;
    while i < layout.len() - 1 {
        if let Input::Free { size } = layout[i] && let Input::Free { size: size2 } = layout[i+1] {
             // two adjacent free spaces, combine into one
            layout[i] = Input::Free { size: size + size2 };
            layout.remove(i + 1);
        }

        i += 1;
    }
}