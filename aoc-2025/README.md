## Advent of Code 2024
The goal is to solve all AoC 2025 exersises (both parts) in <500ms.

| Day | Part 1  | Part 2  | Libraries      |
|-----|---------|---------|----------------|
| 1   | 8.274μs | 11.81μs |                |
| 2   | 2.601ms |         | Rayon          |
| 3   | 18.12μs | 42.03μs | Rayon (part 2) |
| 4   | 1.066ms | 23.99ms |                |
| 5   | 69.86μs | 38.04μs | Rayon (part 1) |
| 6   | 5.741μs | 56.16μs | Rayon (part 2) |
| 7   | 163.7μs | 110.5μs | FxHash         |
| 8   |         |         |                |
| 9   |         |         |                |
| 10  |         |         |                |
| 11  |         |         |                |
| 12  |         |         |                |

\* = Part 1/2 solution require (nearly) the same computation, can both be solved in this time

All benchmarked on `Ryzen 7 3700X, 48GB (2 x 8GB, 2 x 16GB) 3200MT/s` desktop using `cargo aoc bench -d [day]`.
