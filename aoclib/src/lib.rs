pub mod vec2;
pub mod grid;
pub mod mat2;
pub mod memoizer;

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