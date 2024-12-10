# 2024 Advent of Code

This repository holds my solutions for the [2024 Advent of Code](https://adventofcode.com/2024)
challenges. It follows the format set out by [`cargo-aoc`](https://github.com/gobanos/cargo-aoc)

## Progress Tracking

| Status |        Description        |
|--------|---------------------------|
|   ❌   | Unsolved or unavailable   |
|   ✅   | Completed                 |

## 2023 Solutions

|        Day         | Part 1 | Part 2 |            Commentary            |
|--------------------|--------|--------|----------------------------------|
| [1](src/day1.rs)   |   ✅   |   ✅   |                                  |
| [2](src/day2.rs)   |   ✅   |   ✅   |                                  |
| [3](src/day3.rs)   |   ✅   |   ✅   |                                  |
| [4](src/day4.rs)   |   ✅   |   ✅   |                                  |
| [5](src/day5.rs)   |   ✅   |   ✅   |                                  |
| [6](src/day6.rs)   |   ✅   |   ✅   |                                  |
| [7](src/day7.rs)   |   ✅   |   ✅   | [Commentary](#day-7-commentary)  |
| [8](src/day8.rs)   |   ✅   |   ✅   |                                  |
| [9](src/day9.rs)   |   ✅   |   ✅   |                                  |
| [10](src/day10.rs) |   ✅   |   ✅   | [Commentary](#day-10-commentary) |

### Day 7 Commentary

I got bit by using `u8` instead of `u64` as my bitmask. So when you have more
than 8 operations... it just wrapped to 0. I should make `release` mode check
for overflows...

### Day 10 Commentary

Delighted that I was able to use the `Direction` enum from my work on Day 6. I
was especially proud that I was able to see the need for a `HashSet` in part 1
before even testing it.
