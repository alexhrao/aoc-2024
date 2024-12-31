# 2024 Advent of Code

This repository holds my solutions for the
[2024 Advent of Code](https://adventofcode.com/2024) challenges. It follows the
format set out by [`cargo-aoc`](https://github.com/gobanos/cargo-aoc)

## Progress Tracking

| Status |        Description        |
|--------|---------------------------|
|   ❌   | Unsolved or unavailable   |
|   ✅   | Completed                 |

## 2023 Solutions

|        Day         | Part 1 | Part 2 |      Commentary      |
|--------------------|--------|--------|----------------------|
| [1](src/day1.rs)   |   ✅   |   ✅   |                       |
| [2](src/day2.rs)   |   ✅   |   ✅   |                       |
| [3](src/day3.rs)   |   ✅   |   ✅   |                       |
| [4](src/day4.rs)   |   ✅   |   ✅   |                       |
| [5](src/day5.rs)   |   ✅   |   ✅   |                       |
| [6](src/day6.rs)   |   ✅   |   ✅   |                       |
| [7](src/day7.rs)   |   ✅   |   ✅   | [Commentary](#day-7)  |
| [8](src/day8.rs)   |   ✅   |   ✅   |                       |
| [9](src/day9.rs)   |   ✅   |   ✅   |                       |
| [10](src/day10.rs) |   ✅   |   ✅   | [Commentary](#day-10) |
| [11](src/day11.rs) |   ✅   |   ✅   | [Commentary](#day-11) |
| [12](src/day12.rs) |   ✅   |   ✅   | [Commentary](#day-12) |
| [13](src/day13.rs) |   ✅   |   ✅   | [Commentary](#day-13) |
| [14](src/day14.rs) |   ✅   |   ✅   |                       |
| [15](src/day15.rs) |   ✅   |   ✅   | [Commentary](#day-15) |
| [16](src/day16.rs) |   ✅   |   ✅   | [Commentary](#day-16) |
| [17](src/day17.rs) |   ✅   |   ✅   | [Commentary](#day-17) |
| [18](src/day18.rs) |   ✅   |   ✅   | [Commentary](#day-18) |
| [19](src/day19.rs) |   ✅   |   ✅   | [Commentary](#day-19) |
| [20](src/day20.rs) |   ✅   |   ✅   | [Commentary](#day-20) |
| [21](src/day21.rs) |   ❌   |   ❌   |                       |
| [22](src/day22.rs) |   ✅   |   ✅   | [Commentary](#day-22) |
| [23](src/day23.rs) |   ✅   |   ✅   | [Commentary](#day-23) |
| [24](src/day24.rs) |   ✅   |   ✅   | [Commentary](#day-24) |
| [25](src/day25.rs) |   ✅   |   ❌   | [Commentary](#day-25) |

### Day 7

I got bit by using `u8` instead of `u64` as my bitmask. So when you have more
than 8 operations... it just wrapped to 0. I should make `release` mode check
for overflows...

### Day 10

Delighted that I was able to use the `Direction` enum from my work on Day 6. I
was especially proud that I was able to see the need for a `HashSet` in part 1
before even testing it.

### Day 11

The first part was pretty simple -- mostly just a simulator. I made the mistake
of assuming that order would definitively matter, since that was called out in
the description.

Part 2 was an interesting problem. Based on some back of the envelope math, my
Part 1 solution would take a few months to complete it, even if I added some
parallelization. For awhile I focused on trying to "cache" entries; if I'd seen
a stone before, remember what it transforms into. It was on a run that I
realized order probably **didn't** matter because you never merged stones back.
With that, at first my plan was to just process each stone in parallel. Then I
started thinking about doing it in batches and trying to remember bigger strings
of stones across several steps, or trying to predict a stone's future and
replace it with that. It wasn't until I went on that run when I realized if you
have `N` identical stones, you can just calculate the next step for one of those
stones, and then multiply that output by `N`. That is, each step you figure out
an inventory, a count of each type of stone you have. Then for each type,
calculate the next step and create a new inventory for the next step. That one
ran even faster than my part 1, taking less than half the time. A really great
problem that forced me to think about where I was doing extra work.

### Day 12

As with yesterday's problem, the first part was relatively simple. I could have
used `petgraph` (and I haven't ruled out exploring that), but basically if you
have an undirected graph that tells you where you can walk to from a given plot,
you can use that graph to ask what nodes are connected to each other. From there
it's pretty simple to just see how many sides aren't bordering a fellow node,
and there's your answer.

Part 2 was significantly harder, but also a lot more fun. The solution I ended
up choosing was to use the "hand on wall" approach. Imagine you stand with a
wall to your left, and you put your hand on it and just start walking forward.
If you know that the shape is bounded (which, of course, it is in this case), if
you just never let go of the wall, you will eventually trace the shape and end
up back at where you started. That works, but the problem is that these shapes
can have "holes" that represent inner different plots. The solution here is to
remove walls once you've traced a single border, and then see if there are any
other borders that are not accounted for. Solving that led to the final answer,
although for awhile I'd accidentally filtered the tiles on if they had **two**
or more walls, which meant depending on the order you looked at walls (which was
effectively random), you might remove some walls that meant a valid starting
wall wasn't found.

### Day 13

This one took me a bit to recognize. The finding of the "best" combination
meant, to me, that there would necessarily be multiple combinations (and
perhaps there are) that would lead to the same conclusion. Thus, I figured
I'd need to loop over something. It wasn't until I started thinking about how
one would check an answer that I realized you could actually write this as a
system of equations; from there, I instantly recognized that this was similar
to the hailstone problem last year (e.g., the answer is to use a matrix). This
time, however, instead of attempting to roll my own `rref()`, I used the one
built into `mathru`, which made the solution a breeze.

### Day 14

I meant to get to this one earlier, but I wanted to get [Day 13](#day-13)
finished first. In any case, I created a new utility type - the `Point<T>` -
but that didn't really pan out in terms of being useful in other days (unlike
the `Direction` type which has been **so** useful).

Part 2 I needed a small hint, mostly because I had no idea what I was looking
for. To be honest, part 2 was probably my least favorite `AoC` puzzle... but I'm
well aware that this Advent of Code was made for more than just me 😉.

### Day 15

This one was fun, I must say. Watching the robot push around all the boxes was
pretty entertaining. The tricky bit of part 2 was recognizing that I needed to
differentiate up/down from left/right, but I think I found a way of doing that
where the same code is called regardless of orientation (hence the previous
changes to [`util.rs](src/util.rs)).

### Day 16

Once I saw the maze I knew this had to be a graphing problem. The trick,
which I came up with on my run, was keying a graph node not just by its
location (which is what I first thought of), but **also** by its incoming
direction. In this way, I was able to differentiate going through a node
by turning from just going straight through. Another option would have been
to connect perpendicular directions with a cost of 1000 **in the same node**,
and while I do think that would be cleaner, my method works, so for now I think
I'm sticking with it.

### Day 17

Part 1 was relatively straightforward; fun, even. Simulate a simple 3-bit
computer with 8 simple instructions. Didn't take long, and had fun modelling
how the machine might actually work.

Part 2 was considerably more difficult. At first I just tried a brute force
approach, hoping (in vain, I might add) that the true value of `a` would be
something that wasn't very big. Unfortunately, I was sorely mistaken; based
on my final answer, I was only 0.3% through. According to some back of the
envelope math, it would have taken my initial solution 32 _days_ to complete.
For reference, my ultimate solution only takes 1.3 **µs**.

The solution started when I was at home with my dad. He and I were looking at it
when he suggested I work through what the example program was actually
**doing**, not just trying random numbers to see what worked. From there I was
able to boil down the instructions to two things: A manipulation of `B` that
solely depends on `A`, and then dividing `A` by 8. Slowly it began to dawn on me
that instead of starting from the _beginning_ of the instructions, I should
start from the end, since `A` will necessarily decrease with each step (of
course, if `A` is 0, it stays the same, but that's also never going to work).
Starting from the end means I can start from smaller numbers and then work my
way up to the beginning.

The second breakthrough happened when I realized that dividing by 8 will
**truncate** the value; in this way, all numbers `[8,15]` will become `1`. So
I added a loop that checked all of the offsets. That got me much further, but I
kept hitting a problem where eventually the chain would end (even though I did
make it to instructions 8 and 9).

The final breakthrough happened while staring at the output, and realizing that
sometimes, the algorithm would miss viable answers; that was when I realized I
needed a way to backtrack. So I wrote a recursive function that didn't bail out
the moment it hit a dead end, and that function - `chain` - ended up being my
final answer.

Overall a really great problem that made me think about what I was actually
trying to solve, but more importantly it made me think critically about what
a partial solution might look like; this insight is what enabled me to build up
to the ultimate answer.

### Day 18

This one consisted of a relatively straightforward graphing problem. Once I
got that for part 1, `petgraph` made it pretty simple -- just call
`has_path_connecting` on the graph after each nanosecond. As soon as it
return `false`... there you go

In past years I think I would have found this insurmountably difficult. But
`AoC`, along with my graph & algorithms class at Georgia Tech, have made me
much more confident at using graphs, and much better at recognizing when they
might be useful.

### Day 19

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

### Day 20

Both parts threw me for a good long while. The first part, which I'm leaving
as-is as a monument to what could have been.

At first, I took a sort of "brute force" approach. In this method, I found all
walls that were surrounded by at least two non-walls. Then, one by one, I tested
each of them, and saw if the resulting shortest path from `S` to `E` was at
least 100 steps fewer. This... worked, but took several _minutes_ to complete.
Once I saw the second part, I realized there wasn't any way my part 1 logic
would work for part 2.

The first breakthrough happened on a run, when I realized that if I knew the
optimal path, I maybe could solve this. The basic principle was that for a step
`p` on the optimal path, I could collect all other nodes that were in a 20-step
radius from me. For each of them, I could compute the distance from them to the
end, and using **that** information, I could see if that cheat would save me
any time. However, when I got back and tried to implement this approach, I got
stuck on finding the optimal path; using the `rustworkx` implementation of
`all_shortest_paths`, my program basically just hung.

The second breakthrough happened while I was waiting at the Tulsa International
Airport in Oklahoma. It was there that I realized that it's possible there's
only **one** possible path from start to end; I was so used to assuming that
there are multiple paths, that I forgot to check this at first. Once I was able
to prove to myself that there was indeed only one path, finding that optimal
path was trivial. I spent the rest of the flight implementing my approach, and
now I can solve **both** parts in under half a second each.

This problem, especially part 2, taught me to check my assumptions first, and
that, while many of these problems **do** require graph algorithms... they don't
always. Sometimes good old-fashioned simple looping is all that's necessary to
solve the problem at hand.

### Day 22

This one was surprisingly straightforward, although I very clearly missed
an opportunity for optimization. My first part's solution was a basic
implementation of exactly what was described, what with the mixing and
pruning, and doing so 2000 times. I didn't bother trying to optimize
it since when I ran it, it only took around 2 milliseconds to complete.

The second part, I was not so lucky. I'm proud that it basically worked the
first time, but it took almost 200 seconds to get there! There's surely
a better solution... but honestly 3 minutes is good enough for me.

### Day 23

Today was surprisingly straightforward. The first basically involved finding
triplicates, which I just did the brute force way. I'm convinced there's
probably a more elegant, faster way to look for that, but my solution runs in
less than a second, so I'm happy with it.

The second part sort of faked me out. I pretty quickly recognized that this was
a problem that boiled down to finding the maximal
[clique](https://en.wikipedia.org/wiki/Clique_(graph_theory)) in the graph of
connected computer nodes. This problem is known to be NP-hard, so I was looking
for something about my input that would make it, well, **not** as hard. Finally
I just tried the basic approach - just doing the exponential-time search for the
maximally sized clique - and it actually worked on the first try. So I suppose
the lesson here might be try the "stupid" approach first, you might be surprised
with the results.

### Day 24

The first part was a relatively straighforward simulation of a circuit.
The signals propagated as described.

The second part took me quite awhile to get. I started by playing around with
just swapping random gates, which of course didn't get me very far. I noticed
that the puzzle specified an adder, so I brushed up on the hardware needed for
a half- and full-adder. That led me to start by just printing out each bit
output (e.g., each `z`). From there, I was able to manually deduce what signals
to swap. Once I determined that I was right, I went back and implemented the
deduction as a crude set of fixed `if` statements. I'm not entirely sure that
this would work for _any_ input... but it works for mine, so I'm content with
that.

### Day 25

I've completed step one, which was simpler than I thought it would be based on
the description. There were some minor hiccups on if I should subtract 1 or not,
but I ultimately decided to match the problem description.
