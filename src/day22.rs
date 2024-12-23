use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Secret(u64);

impl Secret {
    pub fn generate(&self) -> Self {
        let s = self.0;
        let s = ((s * 64) ^ s) % 16777216;
        let s = ((s / 32) ^ s) % 16777216;
        let s = ((s * 2048) ^ s) % 16777216;
        Self(s)
    }
}

#[aoc_generator(day22)]
pub fn gen(input: &str) -> Vec<Secret> {
    input
        .lines()
        .map(|line| Secret(line.parse().unwrap()))
        .collect()
}

#[aoc(day22, part1)]
pub fn part1(secrets: &[Secret]) -> u64 {
    use rayon::prelude::*;
    secrets
        .par_iter()
        .map(|s| {
            let mut s = s.generate();
            for _ in 1..2000 {
                s = s.generate();
            }
            s.0
        })
        .sum()
}

#[aoc(day22, part2)]
pub fn part2(secrets: &[Secret]) -> u64 {
    use rayon::prelude::*;
    let buyers = secrets
        .par_iter()
        .map(|s| {
            let mut s = *s;
            (0..=2000)
                .map(|_| {
                    let out = s.0 % 10;
                    s = s.generate();
                    out
                })
                .collect::<Vec<_>>()
                .windows(2)
                .map(|w| (w[1], ((w[1] as i64) - (w[0] as i64))))
                .collect()
        })
        .collect::<Vec<Vec<(u64, i64)>>>()
        .par_iter()
        .map(|diffs| {
            diffs
                .windows(4)
                .map(|diff| {
                    // diff is 4 units long. Convert to the last number
                    let out = [diff[0].1, diff[1].1, diff[2].1, diff[3].1];
                    (diff[3].0, out)
                })
                .collect()
        })
        .collect::<Vec<Vec<(u64, [i64; 4])>>>();
    // Let's get a map. Each entry is keyed by a diff. The
    // value is the expected value of that diff.

    // The first step is just getting all the diffs
    let mut diffs = HashSet::new();
    diffs.extend(
        buyers
            .iter()
            .flat_map(|buyer| buyer.iter().map(|&(_, diff)| diff)),
    );
    // Now, for each diff, see what each buyer would result in
    diffs
        .into_iter()
        .map(|diff| {
            buyers
                .par_iter()
                .map(|buyer| {
                    buyer
                        .iter()
                        .find_map(|&(p, d)| (d == diff).then_some(p))
                        .unwrap_or_default()
                })
                .sum::<u64>()
        })
        .max()
        .unwrap()
}
