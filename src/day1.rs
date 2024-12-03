use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn gen(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            let left: u32 = l[0..5].parse().unwrap();
            let right: u32 = l[8..].parse().unwrap();
            (left, right)
        })
        .unzip()
}

#[aoc(day1, part1)]
pub fn part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let (mut left, mut right) = (left.clone(), right.clone());
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut right_count: HashMap<&u32, u32> = HashMap::new();
    for r in right {
        let c = right_count
            .entry(r)
            .or_default();
        *c += 1;
    }
    left.iter()
        .filter_map(|l| right_count.get(l).map(|r| l * r))
        .sum()
}