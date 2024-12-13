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
| [11](src/day11.rs) |   ✅   |   ✅   | [Commentary](#day-11-commentary) |

### Day 7 Commentary

I got bit by using `u8` instead of `u64` as my bitmask. So when you have more
than 8 operations... it just wrapped to 0. I should make `release` mode check
for overflows...

### Day 10 Commentary

Delighted that I was able to use the `Direction` enum from my work on Day 6. I
was especially proud that I was able to see the need for a `HashSet` in part 1
before even testing it.

### Day 11 Commentary

The first part was pretty simple -- mostly just a simulator. I made the mistake
of assuming that order would definitively matter, since that was called out in
the description.

Part 2 was an interesting problem. Based on some back of the envelope math, my
Part 1 solution would take a few months to complete it, even if I added some
parallelization. For awhile I focused on trying to "cache" entries; if I'd seen
a stone before, remember what it transforms into. It was on a run that I realized
order probably **didn't** matter because you never merged stones back. With that,
at first my plan was to just process each stone in parallel. Then I started thinking
about doing it in batches and trying to remember bigger strings of stones across
several steps, or trying to predict a stone's future and replace it with that.
It wasn't until I went on that run when I realized if you have `N` identical stones,
you can just calculate the next step for one of those stones, and then multiply
that output by `N`. That is, each step you figure out an inventory, a count of each
type of stone you have. Then for each type, calculate the next step and create a new
inventory for the next step. That one ran even ffaster than my part 1, taking
less than half the time. A really great problem that forced me to think about where
I was doing extra work.
