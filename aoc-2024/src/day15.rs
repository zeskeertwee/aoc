use std::clone::Clone;
use std::collections::VecDeque;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::grid::Grid;
use aoclib::vec2::{Direction, Vector2};
use fxhash::FxHashSet;

struct Input {
    grid: Grid<char>,
    sequence: Vec<Direction>,
    starting_position: Vector2<usize>
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Input {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut grid = Grid::parse(split[0], |c| c);
    let sequence = split[1].replace("\n", "").chars().map(|c| match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        '^' => Direction::Up,
        'v' => Direction::Down,
        _ => panic!()
    }).collect();

    let starting_position = grid.find_first_occurance(&'@').unwrap();
    grid[&starting_position] = '.';

    Input {
        grid, sequence, starting_position
    }
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> usize {
    let mut grid = input.grid.clone();
    let _ = input.sequence.iter().fold(input.starting_position, |pos, dir| make_move(&mut grid, pos, *dir));

    grid.iter_squares().filter(|(c, _)| **c == 'O').map(|(_, v)| 100 * v.y + v.x).sum()
}

#[aoc(day15, part2)]
fn part2(input: &Input) -> usize {
    let mut grid = Grid::fill('.', input.grid.width * 2, input.grid.height);
    let position = Vector2::new(input.starting_position.x * 2, input.starting_position.y);
    input.grid.iter_squares().for_each(|(c, pos)| {
        let new_pos_a = Vector2::new(pos.x * 2, pos.y);
        let new_pos_b = new_pos_a + Vector2::new(1_usize, 0);
    
        match *c {
            '#' | '.' => {
                grid[&new_pos_a] = *c;
                grid[&new_pos_b] = *c;
            },
            'O' => {
                grid[&new_pos_a] = '[';
                grid[&new_pos_b] = ']';
            },
            _ => panic!("Unexpected grid character: {}", *c),
        }
    });

    let _ = input.sequence.iter().fold(position, |pos, dir| make_move_p2(&mut grid, pos, *dir));

    grid.iter_squares().filter(|(c, _)| **c == '[').map(|(_, v)| 100 * v.y + v.x).sum()
}

// checks if the movement can be made, and if so, makes it. Returns the new position
fn make_move(grid: &mut Grid<char>, current_pos: Vector2<usize>, direction: Direction) -> Vector2<usize> {
    let new_pos = direction + current_pos;
    if grid[&new_pos] == '#' {
        // we can't make the move.
        return current_pos;
    }

    if grid[&new_pos] == '.' {
        // we can make the move
        return new_pos;
    }

    if let Some(pos) = check_move(grid, new_pos, direction) {
        grid[&pos] = 'O';
        grid[&new_pos] = '.';
        new_pos
    } else {
        current_pos
    }
}

// finds the first empty space in a given direction, or none if it hits a wall first
fn check_move(grid: &Grid<char>, position: Vector2<usize>, direction: Direction) -> Option<Vector2<usize>> {
    match grid[&position] {
        '.' => Some(position),
        '#' => None,
        'O' | '[' | ']' => check_move(grid, direction + position, direction),
        _ => panic!()
    }
}

fn check_move2(grid: &Grid<char>, pos: Vector2<usize>, direction: Direction, checked_boxes: &mut FxHashSet<Vector2<usize>>) -> bool {
    // take the left of the box as the 'primary' position
    let box_pos = match grid[&pos] {
        '[' => pos,
        ']' => Direction::Left + pos,
        '#' => return false,
        '.' => return true,
        _ => panic!()
    };

    if checked_boxes.contains(&box_pos) {
        return true;
    }

    let box_pos_right = Direction::Right + box_pos;
    if check_move2(grid, direction + box_pos, direction, checked_boxes) && check_move2(grid, direction + box_pos_right, direction, checked_boxes) {
        checked_boxes.insert(box_pos);
        return true;
    }

    false
}

// checks if the move can be made and makes it, if possible
fn make_move_p2(grid: &mut Grid<char>, current_pos: Vector2<usize>, direction: Direction) -> Vector2<usize> {
    let new_pos = direction + current_pos;
    let c = grid[&new_pos];

    match c {
        '#' => current_pos,
        '.' => new_pos,
        '[' | ']' => {
            let elements: Vec<Vector2<usize>> = if direction == Direction::Right || direction == Direction::Left {
                if let Some(free_space) = check_move(grid, new_pos, direction) {
                    match direction {
                        Direction::Right => new_pos.x..free_space.x,
                        Direction::Left => free_space.x + 1..current_pos.x,
                        _ => panic!()
                    }.into_iter().map(|x| Vector2::new(x, current_pos.y)).collect()
                } else {
                    // can't move
                    return current_pos;
                }
            } else {
                let mut set = FxHashSet::default();
                if !check_move2(grid, new_pos, direction, &mut set) {
                    // can't move
                    return current_pos;
                }
                let mut elements = Vec::with_capacity(set.len() * 2);
                set.into_iter().for_each(|v| {
                    elements.push(Direction::Right + v);
                    elements.push(v);
                });
                elements
            };
            
            let mut val = VecDeque::with_capacity(elements.len());
            // move the whole region in the correct direction
            for i in elements.iter() {
                val.push_back(grid[i]);
                grid[i] = '.';
            }

            for i in elements {
                grid[&(direction + i)] = val.pop_front().unwrap();
            }

            new_pos
        },
        _ => panic!()
    }
}

aoc_test!(test_day15_sample, "../samples/day15.txt", 10092, 9021);
aoc_test!(test_day15, "../input/2024/day15.txt", 1490942, 1519202);
