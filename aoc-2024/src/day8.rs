use fxhash::{FxHashSet, FxHashMap};
use aoc_runner_derive::{aoc, aoc_generator};
use crate::util::Vector2;

type Grid = Vec<Vec<char>>;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day8, part1)]
fn part1(input: &Grid) -> usize {
    find_antinodes::<false>(input)
}

#[aoc(day8, part2)]
fn part2(input: &Grid) -> usize {
    find_antinodes::<true>(input)
}

fn find_antinodes<const PART2: bool>(grid: &Grid) -> usize {
    let width = grid[0].len() as i64;
    let height = grid.len() as i64;

    let mut antennas: FxHashMap<char, Vec<Vector2>> = FxHashMap::default();

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '.' {
                continue;
            }

            antennas.entry(*c).or_insert_with(Vec::new).push(Vector2::new(x as i64, y as i64));
        }
    }

    let mut antinodes: FxHashSet<Vector2> = FxHashSet::default();
    
    for (_, v) in antennas.iter() {
        calculate_antenna_antinodes::<PART2>(&v, width, height).into_iter().for_each(|n| {
            antinodes.insert(n);
        });
    }
    
    antinodes.len()
}

fn calculate_antenna_antinodes<const PART2: bool>(positions: &Vec<Vector2>, map_width: i64, map_height: i64) -> Vec<Vector2> {
    let mut antinodes: Vec<Vector2> = Vec::new();

    for (idx, a) in positions.iter().enumerate() {
        for (idx2, b) in positions.iter().enumerate() {
            if idx == idx2 {
                continue;
            }

            // vector pointing from a to b
            let dist = *a - *b;

            if !PART2 {
                let na = *a + dist;
                let nb = *b - dist;

                if check_bounds(&na, map_width, map_height) {
                    antinodes.push(na);
                }

                if check_bounds(&nb, map_width, map_height) {
                    antinodes.push(nb);
                }
            } else {
                for i in 0.. {
                    let node = *a + (dist * i);
                    if check_bounds(&node, map_width, map_height) {
                        antinodes.push(node);
                    } else {
                        // fallen off the map
                        break;
                    }
                }

                for i in 0.. {
                    let node = *b - (dist * i);
                    if check_bounds(&node, map_width, map_height) {
                        antinodes.push(node);
                    } else {
                        // fallen off the map
                        break;
                    }
                }
            }
        }
    }

    antinodes
}

fn check_bounds(n: &Vector2, map_width: i64, map_height: i64) -> bool {
    n.x >= 0 && n.y >= 0 && n.x < map_width && n.y < map_height
}