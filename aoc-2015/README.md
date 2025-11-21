## Advent of Code 2015
| Day | Part 1  | Part 2  | Libraries     |
|-----|---------|---------|---------------|
| 1   | 258.2ns | 620.4ns |               |
| 2   | 1.371μs | 3.655μs |               |
| 3   | 106.5μs | 123.9μs | FxHash        |
| 4   | 32.00ms | 130.2ms | Md5**         |
| 5   | 58.64μs | 96.82μs | FxHash, Rayon |
| 6   | 16.79ms | 25.84ms |               |
| 7   | 704.3μs | 1.410ms | FxHash        |
| 8   | 26.88μs | 29.34μs | Rayon         |
| 9   | 426.1μs | 380.9μs | FxHash        |
| 10  | 1.467ms | 21.43ms |               |
| 11  |         |         |               |
| 12  |         |         |               |
| 13  |         |         |               |
| 14  |         |         |               |
| 15  |         |         |               |
| 16  |         |         |               |
| 17  |         |         |               |
| 18  |         |         |               |
| 19  |         |         |               |
| 20  |         |         |               |
| 21  |         |         |               |
| 22  |         |         |               |
| 23  |         |         |               |
| 24  |         |         |               |
| 25  |         |         |               |

\*\* = TODO: Implement MD5 in aoclib

All benchmarked on `Ryzen 7 3700X, 48GB (2 x 8GB, 2 x 16GB) 3200MT/s` desktop using `cargo aoc bench -d [day]`.
