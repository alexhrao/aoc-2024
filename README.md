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
| [13](src/day13.rs) |   ✅   |   ✅   | [Commentary](#day-13-commentary) |
| [14](src/day14.rs) |   ❌   |   ❌   |                                  |
| [15](src/day15.rs) |   ✅   |   ✅   | [Commentary](#day-15-commentary) |
| [16](src/day16.rs) |   ✅   |   ✅   | [Commentary](#day-16-commentary) |
| [17](src/day17.rs) |   ✅   |   ❌   |                                  |
| [18](src/day18.rs) |   ✅   |   ✅   | [Commentary](#day-18-commentary) |
| [19](src/day19.rs) |   ✅   |   ✅   | [Commentary](#day-19-commentary) |
| [20](src/day20.rs) |   ❌   |   ❌   |                                  |
| [21](src/day21.rs) |   ❌   |   ❌   |                                  |
| [22](src/day21.rs) |   ✅   |   ✅   | [Commentary](#day-22-commentary) |

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

### Day 13 Commentary

This one took me a bit to recognize. The finding of the "best" combination
meant, to me, that there would necessarily be multiple combinations (and
perhaps there are) that would lead to the same conclusion. Thus, I figured
I'd need to loop over something. It wasn't until I started thinking about how
one would check an answer that I realized you could actually write this as a
system of equations; from there, I instantly recognized that this was similar
to the hailstone problem last year (e.g., the answer is to use a matrix). This
time, however, instead of attempting to roll my own `rref()`, I used the one
built into `mathru`, which made the solution a breeze.

### Day 15 Commentary

This one was fun, I must say. Watching the robot push around all the boxes was
pretty entertaining. The tricky bit of part 2 was recognizing that I needed to
differentiate up/down from left/right, but I think I found a way of doing that
where the same code is called regardless of orientation (hence the previous
changes to [`util.rs](src/util.rs)).

### Day 16 Commentary

Once I saw the maze I knew this had to be a graphing problem. The trick,
which I came up with on my run, was keying a graph node not just by its
location (which is what I first thought of), but **also** by its incoming
direction. In this way, I was able to differentiate going through a node
by turning from just going straight through. Another option would have been
to connect perpendicular directions with a cost of 1000 **in the same node**,
and while I do think that would be cleaner, my method works, so for now I think
I'm sticking with it.

### Day 18 Commentary

This one consisted of a relatively straightforward graphing problem. Once I
got that for part 1, `petgraph` made it pretty simple -- just call
`has_path_connecting` on the graph after each nanosecond. As soon as it
return `false`... there you go

In past years I think I would have found this insurmountably difficult. But
`AoC`, along with my graph & algorithms class at Georgia Tech, have made me
much more confident at using graphs, and much better at recognizing when they
might be useful.

### Day 19 Commentary

I came up with the answer in two phases. The first was on my run this morning,
when I came up with the idea of starting at the beginning and looking at
ever-decreasing suffixes of the original input. That worked on the example,
but ended up taking far too long for the actual input. I ended by printing
all the suffixes I was trying, and then giving up... until later when it hit
me that a lot of the stuff that was printed looked **very** similar to each
other. That's when I realized that a cache could prove decisive, and after
adding that it ran in less than a second. Throwing a cache at a problem
to make it faster is a classic `AoC` solution, and I'm kind of surprised
that I didn't see it sooner

### Day 22 Commentary

This one was surprisingly straightforward, although I very clearly missed
an opportunity for optimization. My first part's solution was a basic
implementation of exactly what was described, what with the mixing and
pruning, and doing so 2000 times. I didn't bother trying to optimize
it since when I ran it, it only took around 2 milliseconds to complete.

The second part, I was not so lucky. I'm proud that it basically worked the
first time, but it took almost 200 seconds to get there! There's surely
a better solution... but honestly 3 minutes is good enough for me.
