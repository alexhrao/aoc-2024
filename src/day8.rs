use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct City {
    rows: usize,
    cols: usize,
    by_antenna: HashMap<char, HashSet<(usize, usize)>>,
}

#[aoc_generator(day8)]
pub fn gen(input: &str) -> City {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut by_antenna: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch != '.' {
                by_antenna.entry(ch).or_default().insert((r, c));
            }
        }
    }
    City {
        rows,
        cols,
        by_antenna,
    }
}

type Point = (usize, usize);

fn antinode_posns(a: Point, b: Point, grid: (usize, usize)) -> (Option<Point>, Option<Point>) {
    let row_rng = 0..(grid.0 as isize);
    let col_rng = 0..(grid.1 as isize);
    let (ar, ac) = (a.0 as isize, a.1 as isize);
    let (br, bc) = (b.0 as isize, b.1 as isize);
    // Get the rise and run
    let rise = ar - br;
    let run = ac - bc;
    let posn1 = (ar + rise, ac + run);
    let posn2 = (br - rise, bc - run);
    let posn1 = if row_rng.contains(&posn1.0) && col_rng.contains(&posn1.1) {
        Some((posn1.0 as usize, posn1.1 as usize))
    } else {
        None
    };
    let posn2 = if row_rng.contains(&posn2.0) && col_rng.contains(&posn2.1) {
        Some((posn2.0 as usize, posn2.1 as usize))
    } else {
        None
    };
    (posn1, posn2)
}

struct PointIter {
    rise: isize,
    run: isize,
    bounds: (Range<isize>, Range<isize>),
    curr: (isize, isize),
}

impl Iterator for PointIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (r, c) = self.curr;
        let out = if self.bounds.0.contains(&r) && self.bounds.1.contains(&c) {
            Some((r as usize, c as usize))
        } else {
            None
        };
        self.curr = (self.curr.0 + self.rise, self.curr.1 + self.run);
        out
    }
}

fn antinodes(a: Point, b: Point, grid: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let row_rng = 0..(grid.0 as isize);
    let col_rng = 0..(grid.1 as isize);
    let (ar, ac) = (a.0 as isize, a.1 as isize);
    let (br, bc) = (b.0 as isize, b.1 as isize);
    // Get the rise and run
    let rise = ar - br;
    let run = ac - bc;
    PointIter {
        bounds: (row_rng.clone(), col_rng.clone()),
        curr: (ar, ac),
        rise,
        run,
    }
    .chain(PointIter {
        bounds: (row_rng.clone(), col_rng.clone()),
        curr: (br, bc),
        rise: -rise,
        run: -run,
    })
}

#[aoc(day8, part1)]
pub fn part1(input: &City) -> usize {
    let mut antinodes = HashSet::new();
    let grid = (input.rows, input.cols);
    for posns in input.by_antenna.values() {
        for me in posns {
            let mut posns = posns.to_owned();
            posns.remove(me);
            for p in posns {
                let (p1, p2) = antinode_posns(*me, p, grid);
                if let Some(p1) = p1 {
                    antinodes.insert(p1);
                }
                if let Some(p2) = p2 {
                    antinodes.insert(p2);
                }
            }
        }
    }

    antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &City) -> usize {
    let mut places = HashSet::new();
    let grid = (input.rows, input.cols);
    for posns in input.by_antenna.values() {
        for me in posns {
            let mut posns = posns.to_owned();
            posns.remove(me);
            for p in posns {
                places.extend(antinodes(*me, p, grid));
            }
        }
    }
    places.len()
}
