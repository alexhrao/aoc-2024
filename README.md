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
| [12](src/day12.rs) |   ✅   |   ✅   | [Commentary](#day-12-commentary) |

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

### Day 12 Commentary

As with yesterday's problem, the first part was relatively simple. I could have
used `petgraph` (and I haven't ruled out exploring that), but basically if you
have an undirected graph that tells you where you can walk to from a given plot,
you can use that graph to ask what nodes are connected to each other. From there
it's pretty simple to just see how many sides aren't bordering a fellow node,
and there's your answer.

Part 2 was significantly harder, but also a lot more fun. The solution I ended up
choosing was to use the "hand on wall" approach. Imagine you stand with a wall to
your left, and you put your hand on it and just start walking forward. If you know
that the shape is bounded (which, of course, it is in this case), if you just never
let go of the wall, you will eventually trace the shape and end up back at where
you started. That works, but the problem is that these shapes can have "holes" that
represent inner different plots. The solution here is to remove walls once you've
traced a single border, and then see if there are any other borders that are not
accounted for. Solving that led to the final answer, although for awhile I'd
accidentally filtered the tiles on if they had **two** or more walls, which meant
depending on the order you looked at walls (which was effectively random),
you might remove some walls that meant a valid starting wall wasn't found.
