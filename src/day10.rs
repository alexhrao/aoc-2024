use std::collections::HashSet;

use crate::day6::DIRS;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|b| if b == b'.' { 10 } else { b - b'0' })
                .collect()
        })
        .collect()
}

fn score(curr: (usize, usize), grid: &[Vec<u8>]) -> HashSet<(usize, usize)> {
    match grid[curr.0][curr.1] {
        10.. => HashSet::new(),
        9 => HashSet::from([curr]),
        me => {
            // For everyone around me, see if anyone is me plus one
            let mut out = HashSet::new();
            let bounds = (grid.len(), grid[0].len());
            for d in DIRS {
                if let Some((r, c)) = d.step(curr, bounds) {
                    if grid[r][c] == me + 1 {
                        // Follow this
                        out.extend(&score((r, c), grid));
                    }
                }
            }
            out
        }
    }
}

fn rate(curr: (usize, usize), grid: &[Vec<u8>]) -> usize {
    match grid[curr.0][curr.1] {
        10.. => 0,
        9 => 1,
        me => {
            // For everyone around me, see if anyone is me plus one
            let mut count = 0;
            let bounds = (grid.len(), grid[0].len());
            for d in DIRS {
                if let Some((r, c)) = d.step(curr, bounds) {
                    if grid[r][c] == me + 1 {
                        // Follow this
                        count += rate((r, c), grid);
                    }
                }
            }
            count
        }
    }
}

#[aoc(day10, part1)]
pub fn part1(map: &[Vec<u8>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, h)| (*h == 0))
                .map(move |(c, _)| score((r, c), map).len())
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(map: &[Vec<u8>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, h)| (*h == 0))
                .map(move |(c, _)| rate((r, c), map))
        })
        .sum()
}
