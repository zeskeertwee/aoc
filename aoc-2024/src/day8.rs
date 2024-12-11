use fxhash::{FxHashSet, FxHashMap};
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::vec2::Vector2;
use aoclib::grid::Grid;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Grid<char> {
    Grid::parse(input, |c| c)
}

#[aoc(day8, part1)]
fn part1(input: &Grid<char>) -> usize {
    find_antinodes::<false>(input)
}

#[aoc(day8, part2)]
fn part2(input: &Grid<char>) -> usize {
    find_antinodes::<true>(input)
}

fn find_antinodes<const PART2: bool>(grid: &Grid<char>) -> usize {
    let mut antennas: FxHashMap<char, Vec<Vector2<usize>>> = FxHashMap::default();
    
    grid.iter_squares()
        .filter(|(c, _)| **c != '.')
        .for_each(|(c, v)| {
            antennas.entry(*c).or_insert_with(Vec::new).push(v);
        });

    let mut antinodes: FxHashSet<Vector2<usize>> = FxHashSet::default();
    
    for (_, v) in antennas.iter() {
        calculate_antenna_antinodes::<PART2>(&v, grid.width, grid.height).into_iter().for_each(|n| {
            antinodes.insert(n);
        });
    }
    
    antinodes.len()
}

fn calculate_antenna_antinodes<const PART2: bool>(positions: &Vec<Vector2<usize>>, map_width: usize, map_height: usize) -> Vec<Vector2<usize>> {
    let mut antinodes: Vec<Vector2<usize>> = Vec::new();
    let n = positions.len();
    
    for idx in 0..n {
        for idx2 in idx..n {
            if idx == idx2 {
                continue;
            }
            let a = positions[idx];
            let b = positions[idx2];

            // vector pointing from a to b
            let dist = a - b;

            if !PART2 {
                let na = a + dist;
                let nb = b - dist;

                if check_bounds(&na, map_width, map_height) {
                    antinodes.push(na);
                }

                if check_bounds(&nb, map_width, map_height) {
                    antinodes.push(nb);
                }
            } else {
                for i in 0.. {
                    let node = a + (dist * i);
                    if check_bounds(&node, map_width, map_height) {
                        antinodes.push(node);
                    } else {
                        // fallen off the map
                        break;
                    }
                }

                for i in 0.. {
                    let node = b - (dist * i);
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

fn check_bounds(n: &Vector2<usize>, map_width: usize, map_height: usize) -> bool {
    n.x < map_width && n.y < map_height
}