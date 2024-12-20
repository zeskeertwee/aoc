#![feature(associated_type_defaults)]

pub mod vec2;
pub mod grid;
pub mod mat2;

use std::time::{Duration, Instant};
pub use bitvec;

#[macro_export]
macro_rules! aoc_test {
    ($name:ident, $input:literal, $part1:literal) => {
        #[test]
        fn $name() {
            let input = parse_input(include_str!($input));
            assert_eq!(part1(&input), $part1);
        }
    };
    ($name:ident, $input:literal, $part1:literal, $part2:literal) => {
        #[test]
        fn $name() {
            let input = parse_input(include_str!($input));
            assert_eq!(part1(&input), $part1);
            assert_eq!(part2(&input), $part2);
        }
    };
}

pub fn timed_closure<R, F: FnOnce() -> R>(mut f: F) -> (Duration, R) {
    let start = Instant::now();
    let r = f();
    (start.elapsed(), r)
}

pub trait AocDay {
    type Input;
    type PreprocessedInput = Self::Input;
    const NAME: &'static str;

    fn parse_input(input: &str) -> Self::Input;
    fn preprocess(input: Self::Input) -> Self::PreprocessedInput;
    fn part1(input: &Self::PreprocessedInput) -> impl ToString;
    fn part2(input: &Self::PreprocessedInput) -> impl ToString;

    fn run(input: &str) {
        println!("Executing {}", Self::NAME);
        let (parse_time, parse) = timed_closure(move || {
            Self::parse_input(input)
        });
        println!("├── Input parsed in {}us", parse_time.as_micros());
        let (prep_time, prep) = timed_closure(move || {
            Self::preprocess(parse)
        });
        println!("├── Preprocessed in {}us", prep_time.as_micros());
        let (part1_time, p1) = timed_closure(|| {
            Self::part1(&prep)
        });
        println!("├── Part 1 in {}us: {}", part1_time.as_micros(), p1.to_string());
        let (part2_time, p2) = timed_closure(|| {
            Self::part2(&prep)
        });
        println!("├── Part 2 in {}us: {}", part2_time.as_micros(), p2.to_string());
        println!("└── Total time: {}us", (parse_time + prep_time + part1_time + part2_time).as_micros())
    }
}

#[macro_export]
macro_rules! aoc_day {
    ($name:ident, $input:ty, $preprocess:expr, $prpr:ty) => {
        pub struct $name;

        impl ::aoclib::AocDay for $name {
            type Input = $input;
            type PreprocessedInput = $prpr;
            const NAME: &'static str = stringify!($name);

            fn parse_input(input: &str) -> Self::Input {
                parse_input(input)
            }

            fn preprocess(input: Self::Input) -> Self::PreprocessedInput {
                $preprocess(input)
            }

            fn part1(input: &Self::Input) -> impl ToString {
                part1(input)
            }

            fn part2(input: &Self::Input) -> impl ToString {
                part2(input)
            }
        }
    };
    ($name:ident, $input:ty) => {
        pub struct $name;

        impl ::aoclib::AocDay for $name {
            type Input = $input;
            type PreprocessedInput = $input;
            const NAME: &'static str = stringify!($name);

            fn parse_input(input: &str) -> Self::Input {
                parse_input(input)
            }

            fn preprocess(input: Self::Input) -> Self::PreprocessedInput {
                input
            }

            fn part1(input: &Self::Input) -> impl ToString {
                part1(input)
            }

            fn part2(input: &Self::Input) -> impl ToString {
                part2(input)
            }
        }
    };
}