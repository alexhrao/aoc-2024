use aoc_runner_derive::{aoc_generator, aoc};

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
pub fn solve_p1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let (mut left, mut right) = (left.clone(), right.clone());
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter().zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_p2((left, right): &(Vec<u32>, Vec<u32>)) -> usize {
    left.iter()
        .map(|l| {
            (*l as usize) * right.iter().filter(|&r| l == r).count()
        })
        .sum()
}