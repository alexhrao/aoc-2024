use std::collections::{BTreeSet, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::{Direction, DIRS};

#[aoc_generator(day12)]
pub fn gen(input: &str) -> HashMap<u8, BTreeSet<(usize, usize)>> {
    let mut out: HashMap<u8, BTreeSet<(usize, usize)>> = HashMap::new();

    let iter = input.lines().enumerate().flat_map(move |(r, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .map(move |(c, b)| (*b, (r, c)))
    });

    for (b, posn) in iter {
        out.entry(b).or_default().insert(posn);
    }
    out
}

fn explore(curr: (usize, usize), posns: &mut BTreeSet<(usize, usize)>) -> BTreeSet<(usize, usize)> {
    let mut out = BTreeSet::new();
    out.insert(curr);
    for d in DIRS {
        if let Some(new) = d.step(curr) {
            if posns.remove(&new) {
                out.extend(explore(new, posns));
            }
        }
    }
    out
}

fn compute(region: &BTreeSet<(usize, usize)>) -> usize {
    let area = region.len();
    let mut perim = area * 4;
    for p1 in region {
        for p2 in region {
            if p1 == p2 {
                continue;
            }
            let (r1, c1) = p1;
            let (r2, c2) = p2;
            if (r1 == r2 && c1.abs_diff(*c2) == 1) || (c1 == c2 && r1.abs_diff(*r2) == 1) {
                perim -= 1;
            }
        }
    }

    area * perim
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Plot {
    posn: (usize, usize),
    sides: BTreeSet<Direction>,
}

fn sides(region: &BTreeSet<(usize, usize)>) -> BTreeSet<Plot> {
    let mut out = HashMap::new();
    for p in region {
        out.insert(
            *p,
            Plot {
                posn: *p,
                sides: DIRS.into(),
            },
        );
    }
    for p1 in region {
        for p2 in region {
            if p1 == p2 {
                continue;
            }
            let (r1, c1) = p1;
            let (r2, c2) = p2;
            let plot = out.get_mut(p1).unwrap();
            if r1 == r2 && c1.abs_diff(*c2) == 1 {
                if c1 < c2 {
                    plot.sides.remove(&Direction::Right);
                } else {
                    plot.sides.remove(&Direction::Left);
                }
            } else if c1 == c2 && r1.abs_diff(*r2) == 1 {
                if r1 < r2 {
                    plot.sides.remove(&Direction::Down);
                } else {
                    plot.sides.remove(&Direction::Up);
                }
            }
        }
    }

    BTreeSet::from_iter(out.into_values())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TouchedWall {
    posn: (usize, usize),
    wall: Direction,
}

fn follow_with_walls(
    mut posn: (usize, usize),
    mut wall: Direction,
    plots: &mut HashMap<(usize, usize), Plot>,
) -> Vec<TouchedWall> {
    let mut out = vec![];
    let base = plots.clone();
    let start = (posn, wall);
    // I should get back to that same place, that's how I know I've drawn the right outline
    loop {
        // Holding on to that wall means we're actually moving perpendicular (e.g., if I have a wall up, move right, etc.)
        let dir = wall.cw();
        // Now walk in that direction until my hand stops finding a wall to hold onto. This can happen in one of two ways:
        // 1. The wall turns "outwards". This means there's no wall stopping me from continuing to walk (e.g.,
        //  no wall in front of me), BUT moving forward would mean losing my hold on my wall. At this point
        //  I need to turn in that "outward" direction. This is difficult because the plot in front of me wouldn't
        //  maybe have **any** sides. So what I need to look for is the plot in front of me, and then in the "reverse"
        //  direction
        // 2. The wall turns "inwards". This means I get stopped by hitting, well, a wall, **but I still have my
        //  hand on the target wall**. In this case there's only one thing to do; turn once
        loop {
            let plot = base.get(&posn).unwrap();
            // print!("I'm at ({}, {})", plot.posn.0, plot.posn.1);
            if !plot.sides.contains(&wall) {
                // Ah shit, I lost my wall. Previously I **did** have my wall. So now I need to
                // go one step in the direction of where my wall would have been
                // this is the "outside" case
                posn = wall.step(posn).unwrap();
                wall = wall.ccw();
                break;
            }
            // I had that wall, so time to remove it
            plots.get_mut(&posn).unwrap().sides.remove(&wall);
            // Add the touched wall
            out.push(TouchedWall { posn, wall });
            // So I have my wall. Am I blocked from going further?
            // Hitting a wall means hitting a wall blocking my same direction. Theoretically it
            // **could** also mean hitting a wall blocking my opposite direction,
            //  but that wall shouldn't be possible.
            if plot.sides.contains(&dir) {
                wall = wall.cw();
                break;
            }
            // Okay, just keep moving forward
            posn = dir.step(posn).unwrap();
            // Have I made it back to where I started?
            if (posn, wall) == start {
                return out;
            }
        }
        // Am I back at the start?
        if (posn, wall) == start {
            return out;
        }
    }
}

fn follow(
    mut posn: (usize, usize),
    mut wall: Direction,
    plots: &mut HashMap<(usize, usize), Plot>,
) -> usize {
    let base = plots.clone();
    let start = (posn, wall);
    // I should get back to that same place, that's how I know I've drawn the right outline
    let mut num_sides = 1;
    loop {
        // Holding on to that wall means we're actually moving perpendicular (e.g., if I have a wall up, move right, etc.)
        let dir = wall.cw();
        // Now walk in that direction until my hand stops finding a wall to hold onto. This can happen in one of two ways:
        // 1. The wall turns "outwards". This means there's no wall stopping me from continuing to walk (e.g.,
        //  no wall in front of me), BUT moving forward would mean losing my hold on my wall. At this point
        //  I need to turn in that "outward" direction. This is difficult because the plot in front of me wouldn't
        //  maybe have **any** sides. So what I need to look for is the plot in front of me, and then in the "reverse"
        //  direction
        // 2. The wall turns "inwards". This means I get stopped by hitting, well, a wall, **but I still have my
        //  hand on the target wall**. In this case there's only one thing to do; turn once
        loop {
            let plot = base.get(&posn).unwrap();
            if !plot.sides.contains(&wall) {
                // Ah shit, I lost my wall. Previously I **did** have my wall. So now I need to
                // go one step in the direction of where my wall would have been
                // this is the "outside" case
                posn = wall.step(posn).unwrap();
                wall = wall.ccw();
                break;
            }
            // I had that wall, so time to remove it
            plots.get_mut(&posn).unwrap().sides.remove(&wall);
            // So I have my wall. Am I blocked from going further?
            // Hitting a wall means hitting a wall blocking my same direction. Theoretically it
            // **could** also mean hitting a wall blocking my opposite direction,
            //  but that wall shouldn't be possible.
            if plot.sides.contains(&dir) {
                wall = wall.cw();
                break;
            }
            // Okay, just keep moving forward
            posn = dir.step(posn).unwrap();
            // Have I made it back to where I started?
            if (posn, wall) == start {
                // Remove a side because it will have been double counted
                return num_sides - 1;
            }
        }
        // Am I back at the start?
        if (posn, wall) == start {
            return num_sides;
        }
        // Well, I stopped moving forward, and I haven't made it back to
        //  where I started. So I must have turned
        num_sides += 1;
    }
}

fn cartograph(region: &BTreeSet<(usize, usize)>) -> usize {
    let mut plots = HashMap::from_iter(sides(region).into_iter().map(|p| (p.posn, p)));
    let mut count = 0;
    while let Some(Plot { posn, sides }) = plots.values().find(|&p| !p.sides.is_empty()) {
        // Follow
        count += follow(*posn, *sides.first().unwrap(), &mut plots);
    }
    count
}

fn cartograph_with_walls(region: &BTreeSet<(usize, usize)>) -> Vec<Vec<TouchedWall>> {
    let mut plots = HashMap::from_iter(sides(region).into_iter().map(|p| (p.posn, p)));
    let mut out = vec![];
    while let Some(Plot { posn, sides }) = plots.values().find(|&p| !p.sides.is_empty()) {
        // Follow
        out.push(follow_with_walls(
            *posn,
            *sides.first().unwrap(),
            &mut plots,
        ));
    }
    out
}

#[aoc(day12, part1)]
pub fn part1(garden: &HashMap<u8, BTreeSet<(usize, usize)>>) -> usize {
    let mut garden_regions = vec![];
    for posns in garden.values() {
        let mut posns = posns.clone();
        while let Some(curr) = posns.pop_first() {
            garden_regions.push(explore(curr, &mut posns));
        }
    }

    garden_regions.iter().map(compute).sum()
}

#[aoc(day12, part2)]
pub fn part2(garden: &HashMap<u8, BTreeSet<(usize, usize)>>) -> usize {
    let mut garden_regions = vec![];
    for posns in garden.values() {
        let mut posns = posns.clone();
        while let Some(curr) = posns.pop_first() {
            garden_regions.push(explore(curr, &mut posns));
        }
    }

    garden_regions.iter().map(|s| cartograph(s) * s.len()).sum()
}

#[aoc(day12, part2, vis)]
pub fn part2_vis(garden: &HashMap<u8, BTreeSet<(usize, usize)>>) -> usize {
    let mut garden_regions = vec![];
    for posns in garden.values() {
        let mut posns = posns.clone();
        while let Some(curr) = posns.pop_first() {
            garden_regions.push(explore(curr, &mut posns));
        }
    }

    let rows = garden_regions
        .iter()
        .flat_map(|s| s.iter().map(|(r, _)| *r))
        .max()
        .unwrap();
    let cols = garden_regions
        .iter()
        .flat_map(|s| s.iter().map(|(_, c)| *c))
        .max()
        .unwrap();

    let mut grid: Vec<Vec<()>> = vec![];

    for r in 0..rows {
        let mut row = vec![];
        for c in 0..cols {}
        grid.push(row);
    }

    garden_regions.iter().map(|s| cartograph(s) * s.len()).sum()
}
