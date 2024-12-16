use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    rows: usize,
    cols: usize,
    guard: (usize, usize),
    obstacles: HashSet<(usize, usize)>,
}

impl Grid {
    pub fn makes_cycle(&self, extra: &(usize, usize)) -> bool {
        let mut visited = HashSet::new();
        let mut posn = self.guard;
        let mut dir = Direction::Up;
        visited.insert((posn, dir));
        loop {
            let next = loop {
                let Some(next) = dir.step_bounded(posn, (self.rows, self.cols)) else {
                    break None;
                };
                if self.obstacles.contains(&next) || &next == extra {
                    dir = dir.turn();
                    continue;
                }
                break Some(next);
            };
            posn = match next {
                Some(p) => p,
                None => break,
            };
            if !visited.insert((posn, dir)) {
                // We've gone this exact direction before -- cyclic!
                return true;
            }
        }
        false
    }
}

#[aoc_generator(day6)]
pub fn gen(input: &str) -> Grid {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let guard = input
        .lines()
        .enumerate()
        .flat_map(|(r, l)| l.chars().position(|ch| ch == '^').map(move |c| (r, c)))
        .next()
        .unwrap();
    let obstacles = input
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(c, ch)| if ch == '#' { Some((r, c)) } else { None })
        })
        .collect();
    Grid {
        rows,
        cols,
        guard,
        obstacles,
    }
}

#[aoc(day6, part1)]
pub fn part1(grid: &Grid) -> usize {
    let mut visited = HashSet::new();
    let mut posn = grid.guard;
    let mut dir = Direction::Up;
    visited.insert(posn);
    loop {
        let next = loop {
            let Some(next) = dir.step_bounded(posn, (grid.rows, grid.cols)) else {
                break None;
            };
            if grid.obstacles.contains(&next) {
                dir = dir.turn();
                continue;
            }
            break Some(next);
        };
        posn = match next {
            Some(p) => p,
            None => break,
        };
        visited.insert(posn);
    }
    visited.len()
}

#[aoc(day6, part2)]
pub fn part2(grid: &Grid) -> usize {
    use rayon::prelude::*;
    let mut visited = HashSet::new();
    let mut posn = grid.guard;
    let mut dir = Direction::Up;
    visited.insert(posn);
    loop {
        let next = loop {
            let Some(next) = dir.step_bounded(posn, (grid.rows, grid.cols)) else {
                break None;
            };
            if grid.obstacles.contains(&next) {
                dir = dir.turn();
                continue;
            }
            break Some(next);
        };
        posn = match next {
            Some(p) => p,
            None => break,
        };
        visited.insert(posn);
    }

    visited
        .par_drain()
        .filter(|posn| grid.makes_cycle(posn))
        .count()
}
