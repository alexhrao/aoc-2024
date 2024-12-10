use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

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
                let Some(next) = dir.step(posn, (self.rows, self.cols)) else {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl Direction {
    pub fn step(&self, posn: (usize, usize), grid: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => posn.0.checked_sub(1).map(|r| (r, posn.1)),
            Direction::Right => {
                if posn.1 == (grid.1 - 1) {
                    None
                } else {
                    Some((posn.0, posn.1 + 1))
                }
            }
            Direction::Down => {
                if posn.0 == (grid.0 - 1) {
                    None
                } else {
                    Some((posn.0 + 1, posn.1))
                }
            }
            Direction::Left => posn.1.checked_sub(1).map(|c| (posn.0, c)),
        }
    }
    pub fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
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
            let Some(next) = dir.step(posn, (grid.rows, grid.cols)) else {
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
            let Some(next) = dir.step(posn, (grid.rows, grid.cols)) else {
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
