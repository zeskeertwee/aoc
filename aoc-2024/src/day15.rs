use std::clone::Clone;
use std::collections::{HashMap, VecDeque};
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::bitvec::order::Lsb0;
use aoclib::grid::Grid;
use aoclib::vec2::{Direction, Vector2};
use fxhash::{FxHashMap, FxHashSet};
use aoclib::bitvec::vec::BitVec;

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
    let mut position = Vector2::new(input.starting_position.x * 2, input.starting_position.y);
    input.grid.iter_squares().for_each(|(c, pos)| {
        let new_pos_a = Vector2::new(pos.x * 2, pos.y);
        let new_pos_b = new_pos_a + Vector2::new(1, 0);
    
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

    for i in input.sequence.iter() {
        position = make_move_p2(&mut grid, position, *i);
        dbg!(&grid);
    }

    grid.iter_squares().filter(|(c, _)| **c == 'O').map(|(_, v)| 100 * v.y + v.x).sum()
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

// checks if the move can be made and makes it, if possible
fn make_move_p2(grid: &mut Grid<char>, current_pos: Vector2<usize>, direction: Direction) -> Vector2<usize> {
    let new_pos = direction + current_pos;
    let c = grid[&new_pos];

    match c {
        '#' => current_pos,
        '.' => new_pos,
        '[' | ']' => {
            let elements = if direction == Direction::Right || direction == Direction::Left {
                if let Some(free_space) = check_move(grid, new_pos, direction) {
                    match direction {
                        Direction::Right => (new_pos.x..free_space.x),
                        Direction::Left => (free_space.x + 1..current_pos.x),
                        _ => panic!()
                    }.into_iter().map(|x| Vector2::new(x, current_pos.y)).collect()
                } else {
                    // can't move
                    return current_pos;
                }
            } else {
                let region = flood_fill_set(grid, new_pos);
                let boundary = region_boundary(&region, direction);
                if !boundary.into_iter().all(|v| check_move(grid, direction + v, direction).is_some()) {
                    // not possible
                    return current_pos;
                }
                
                region
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

// returns all elements forming the front boundary when looking in the direction
fn region_boundary(region: &Vec<Vector2<usize>>, direction: Direction) -> Vec<Vector2<usize>> {
    // x, lowest/highest y depending on direction
    let mut map: FxHashMap<usize, usize> = FxHashMap::default();
    let up = match direction {
        Direction::Down => false,
        Direction::Up => true,
        _ => panic!()
    };

    for v in region {
        match map.get(&v.x) {
            Some(y) => {
                // increasing y coordinate means going down in the coordinate system, as 0,0 is in the top left
                if up && v.y < *y {
                    map.insert(v.x, v.y);
                } else if v.y > *y {
                    map.insert(v.x, v.y);
                }
            },
            None => {
                map.insert(v.x, v.y);
            },
        }
    }

    map.into_iter().map(|(x, y)| Vector2::new(x, y)).collect()
}

/// flood-fills starting at a given point, matching items in a set of items
pub fn flood_fill_set(grid: &Grid<char>, pos: Vector2<usize>) -> Vec<Vector2<usize>> {
    // index into array as x + y * height
    let mut visited: BitVec<u8, Lsb0> = BitVec::repeat(false, grid.width * grid.height);

    let mut region = Vec::new();
    let mut stack = vec![pos];
    while let Some(v) = stack.pop() {
        let idx = grid.calculate_index(&v);
        if visited[idx] {
            continue;
        }
        visited.set(idx, true);
        region.push(v);
        
        let complementary_char = match grid.grid[idx] {
            ']' => '[',
            '[' => ']',
            _ => panic!()
        };
        
        for n in grid.neighbour_squares(&v) {
            let idx = grid.calculate_index(&n);
            let cn = grid.grid[idx];
            
            // if it's left or right, only flood if the region is part of the same box
            let is_sideways = v.y == n.y;
            if ((is_sideways && cn == complementary_char) || (!is_sideways && (cn == '[' || cn == ']'))) && !visited[idx] {
                stack.push(n);
            }
        }
    }

    region
}

aoc_test!(test_day15_sample, "../samples/day15.txt", 10092, 9021);
aoc_test!(test_day15, "../input/2024/day15.txt", 1490942);