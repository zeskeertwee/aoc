use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use fxhash::{FxHashMap, FxHashSet};
use rayon::prelude::*;

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[String]) -> usize {
    input.par_iter()
        .filter(|s| s.chars().filter(|c| *c == 'a' || *c == 'e' || *c == 'o' || *c == 'u' || *c == 'i').count() >= 3)
        .filter(|s| s.chars().map_windows(|[c1, c2]| c1 == c2).any(|v| v))
        .filter(|s| !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy"))
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &[String]) -> usize {
    input.par_iter()
        .filter(|s| {
            let mut map: FxHashMap<[char; 2], usize> = FxHashMap::default();

            s.chars()
                .enumerate()
                .map_windows(|[(i1, c1), (i2, c2)]| {
                    if let Some(v) = map.get(&[*c1, *c2]) {
                        if v + 1 != *i1 {
                            // We found the same pattern again (non-overlapping)
                            return true
                        }

                        false
                    } else {
                        map.insert([*c1, *c2], *i1);
                        false
                    }
                })
                .any(|v| v == true)
        })
        .filter(|s| s.chars().map_windows(|[c1, _, c3]| c1 == c3).any(|v| v))
        .count()
}

aoc_test!(test_day5, "../input/2015/day5.txt", 258, 53);