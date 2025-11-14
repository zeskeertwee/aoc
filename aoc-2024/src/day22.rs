use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use fxhash::FxHashMap;
use rayon::prelude::*;

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day22, part1)]
fn part1(input: &[usize]) -> usize {
    input.iter().par_bridge().map(|n| progress_sequence(*n, 2000)).sum()
}

#[aoc(day22, part2, new)]
fn part2_new(input: &[usize]) -> usize {
    let mut seq: FxHashMap<[i8; 4], usize> = FxHashMap::default();
    input.iter().for_each(|init| {
        let mut map: FxHashMap<[i8; 4], i8> = FxHashMap::default();

        let mut n = *init;
        let mut last_price = (n % 10) as i8;
        let mut last_diffs = [0i8; 4];

        for idx in 0..2000 {
            n = progress_sequence(n, 1);
            //dbg!(n);
            let new_price = (n % 10) as i8;
            let diff = new_price - last_price;
            //dbg!(diff, new_price, last_price);

            last_price = new_price;
            last_diffs[0] = last_diffs[1];
            last_diffs[1] = last_diffs[2];
            last_diffs[2] = last_diffs[3];
            last_diffs[3] = diff;

            if idx < 4 {
                continue;
            }

            //dbg!(&last_diffs);

            let mut entry = map.entry(last_diffs).or_insert(0);
            if new_price > *entry {
                *entry = new_price;
            }
        }

        for (k, v) in map {
            *seq.entry(k).or_insert(0) += v as usize;
        }
    });

    *seq.values().max().unwrap()
}

//#[aoc(day22, part2)]
//fn part2(input: &[usize]) -> usize {
//    let mut sequences: FxHashMap<[i8; 4], usize> = FxHashMap::default();
//    let _ = input.iter().for_each(|n| {
//        let mut start = *n;
//        let mut seq = Vec::with_capacity(2001);
//        seq.push((start % 10) as i8);
//        (0..4).for_each(|_| {
//            start = progress_sequence(start, 1);
//            seq.push((start % 10) as i8);
//        });
//
//        let mut s: FxHashMap<[i8; 4], i8> = FxHashMap::default();
//        for idx in 4..2000 {
//            start = progress_sequence(start, 1);
//            seq.push((start % 10) as i8);
//
//            let sq = [seq[idx - 4], seq[idx - 3], seq[idx - 2], seq[idx - 1], seq[idx]];
//            let sq2 = [sq[0] - sq[1], sq[1] - sq[2], sq[2] - sq[3], sq[3] - sq[4]];
//
//            let mut entry = s.entry(sq2).or_insert(0);
//            if *entry < seq[idx] {
//                *entry = seq[idx];
//            }
//        }
//
//        for i in s {
//            *sequences.entry(i.0).or_insert(0) += i.1 as usize;
//        }
//    });
//
//    *sequences.values().max().unwrap()
//}

fn progress_sequence(start: usize, steps: usize) -> usize {
    let mut secret = start;

    for _ in 0..steps {
        secret = ((secret << 6) ^ secret) & 0xffffff;
        secret = ((secret >> 5) ^ secret) & 0xffffff;
        secret = ((secret << 11) ^ secret) & 0xffffff;
    }

    secret
}

aoc_test!(test_day22_sample, "../samples/day22.txt", 37327623);
aoc_test!(test_day22, "../input/2024/day22.txt", 19927218456);