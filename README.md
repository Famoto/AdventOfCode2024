# Advent of Code

<p>
  <a href="https://github.com/Famoto/AdventOfCode2024/actions/workflows/rust.yml">   <img alt="Rust"   src="https://github.com/Famoto/AdventOfCode2024/actions/workflows/rust.yml/badge.svg"></a>
</p>

My solutions to [Advent of Code](https://adventofcode.com) puzzles.

### [Advent of Code 2024](https://adventofcode.com/2024)

<table>
<tr><th>Day</th><th>Part 1 Performance</th><th>Part 2 Performance</th></tr>
<tr><td>

| D  | Puzzle                                                    |           Code           |
|:--:|-----------------------------------------------------------|:------------------------:|
| 1  | [Historian Hysteria](https://adventofcode.com/2024/day/1) |  [day1.rs](src/day1.rs)  |
| 2  | [Red-Nosed Reports](https://adventofcode.com/2024/day/2)  |  [day2.rs](src/day2.rs)  |
| 3  | [Mull It Over](https://adventofcode.com/2024/day/3)       |  [day3.rs](src/day3.rs)  |
| 4  | [Ceres Search](https://adventofcode.com/2024/day/4)       |  [day4.rs](src/day4.rs)  |
| 5  | [Print Queue](https://adventofcode.com/2024/day/5)        |  [day5.rs](src/day5.rs)  |
| 6  | [?](https://adventofcode.com/2024/day/6)                  |  [day6.rs](src/day6.rs)  |
| 7  | [?](https://adventofcode.com/2024/day/7)                  |  [day7.rs](src/day7.rs)  |
| 8  | [?](https://adventofcode.com/2024/day/8)                  |  [day8.rs](src/day8.rs)  |
| 9  | [?](https://adventofcode.com/2024/day/9)                  |  [day9.rs](src/day9.rs)  |
| 10 | [?](https://adventofcode.com/2024/day/10)                 | [day10.rs](src/day10.rs) |
| 11 | [?](https://adventofcode.com/2024/day/11)                 | [day11.rs](src/day11.rs) |
| 12 | [?](https://adventofcode.com/2024/day/12)                 | [day12.rs](src/day12.rs) |
| 13 | [?](https://adventofcode.com/2024/day/13)                 | [day13.rs](src/day13.rs) |
| 14 | [?](https://adventofcode.com/2024/day/14)                 | [day14.rs](src/day14.rs) |
| 15 | [?](https://adventofcode.com/2024/day/15)                 | [day15.rs](src/day15.rs) |
| 16 | [?](https://adventofcode.com/2024/day/16)                 | [day16.rs](src/day16.rs) |
| 17 | [?](https://adventofcode.com/2024/day/17)                 | [day17.rs](src/day17.rs) |
| 18 | [?](https://adventofcode.com/2024/day/18)                 | [day18.rs](src/day18.rs) |
| 19 | [?](https://adventofcode.com/2024/day/19)                 | [day19.rs](src/day19.rs) |
| 20 | [?](https://adventofcode.com/2024/day/20)                 | [day20.rs](src/day20.rs) |
| 21 | [?](https://adventofcode.com/2024/day/21)                 | [day21.rs](src/day21.rs) |
| 22 | [?](https://adventofcode.com/2024/day/22)                 | [day22.rs](src/day22.rs) |
| 23 | [?](https://adventofcode.com/2024/day/23)                 | [day23.rs](src/day23.rs) |
| 24 | [?](https://adventofcode.com/2024/day/24)                 | [day24.rs](src/day24.rs) |
| 25 | [?](https://adventofcode.com/2024/day/25)                 | [day25.rs](src/day25.rs) |

</td><td>

| Generator  |   Runner   |
|:----------:|:----------:|
| 54.553 µs  | 21.079 µs  |
| 187.815 µs | 37.451 µs  |
|     --     | 612.367 µs |
|  31.82 µs  | 255.242µs  |
| 866.267 µs | 38.273 ms  |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |

</td><td>

| Generator  |   Runner   |
|:----------:|:----------:|
|  42.42 µs  | 192.804 µs |
| 141.617 µs | 219.144 µs |
|     --     | 601.015 µs |
| 30.167 µs  | 114.336 µs |
| 833.887 µs | 55.487 ms  |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |
|     ?      |     ?      |

</td></tr>
</table>

## Instructions

<details open>
<summary><h3><a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust</h3></summary>

#### <a href="#-test-the-correctness-of-solutions"><img src="https://www.svgrepo.com/show/271355/rocket-ship-rocket.svg" width="14" height="14"></a> Get Answers and Run Performance Benchmarks

Thanks to [`cargo-aoc`](https://github.com/gobanos/cargo-aoc), answers and performance benchmarks for all days are
obtainable by running the main binary.

```bash
cargo run --release
```

#### <a href="#-test-the-correctness-of-solutions"><img src="https://www.svgrepo.com/show/269868/lab.svg" width="14" height="14"></a> Test the Correctness of Solutions

All days also include tests using sample inputs from the puzzle descriptions.

```bash
cargo test
```

</details>
