use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: &[u32]) -> bool {
    const DIFF_RNG: RangeInclusive<u32> = 1..=3;
    let is_inc = report[0] < report[1];
    for w in report.windows(2) {
        if is_inc && w[1] < w[0] {
            return false;
        }
        if !DIFF_RNG.contains(&(w[0].abs_diff(w[1]))) {
            return false;
        }
    }
    true
}

#[aoc(day2, part1)]
pub fn part1(reports: &[Vec<u32>]) -> usize {
    reports.iter().filter(|r| is_safe(r)).count()
}

#[aoc(day2, part1, rayon)]
pub fn part1_rayon(reports: &[Vec<u32>]) -> usize {
    use rayon::prelude::*;
    reports.par_iter().filter(|r| is_safe(r)).count()
}

#[aoc(day2, part2)]
pub fn part2(reports: &[Vec<u32>]) -> usize {
    reports
        .iter()
        .filter(|&r| {
            for i in 0..r.len() {
                let mut r = r.clone();
                r.remove(i);
                if is_safe(&r) {
                    return true;
                }
            }
            false
        })
        .count()
}

#[aoc(day2, part2, rayon)]
pub fn part2_rayon(reports: &[Vec<u32>]) -> usize {
    use rayon::prelude::*;
    reports
        .par_iter()
        .filter(|&r| {
            for i in 0..r.len() {
                let mut r = r.clone();
                r.remove(i);
                if is_safe(&r) {
                    return true;
                }
            }
            false
        })
        .count()
}
