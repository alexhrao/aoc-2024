use core::str;
use std::{collections::HashMap, ops::Rem};

use aoc_runner_derive::{aoc, aoc_generator};

fn blink(stone: &[u8]) -> Vec<Vec<u8>> {
    if stone == b"0" {
        vec![b"1".to_vec()]
    } else if stone.len().rem(2) == 0 {
        let (left, right) = stone.split_at(stone.len() / 2);
        let idx = right
            .iter()
            .position(|r| *r != b'0')
            .unwrap_or(right.len() - 1);
        vec![left.to_vec(), right[idx..].to_vec()]
    } else {
        // Extract a number from this
        let num: u64 = str::from_utf8(stone).unwrap().parse().unwrap();
        vec![format!("{}", num * 2024).as_bytes().to_vec()]
    }
}

#[aoc_generator(day11)]
pub fn gen(input: &str) -> Vec<Vec<u8>> {
    input
        .split_ascii_whitespace()
        .map(|s| s.as_bytes().to_vec())
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(stones: &[Vec<u8>]) -> usize {
    let mut stones = stones.to_vec();
    for _ in 0..25 {
        stones = stones.iter().flat_map(|s| blink(s)).collect();
    }
    stones.len()
}

#[aoc(day11, part2)]
pub fn part2(stones: &[Vec<u8>]) -> usize {
    let mut summary = HashMap::new();
    for s in stones {
        *summary.entry(s.clone()).or_default() += 1;
    }
    for _ in 0..75 {
        let mut next_summary = HashMap::new();
        summary.into_iter().for_each(|(s, c)| {
            for stone in blink(&s) {
                *next_summary.entry(stone).or_default() += c;
            }
        });
        summary = next_summary;
    }

    summary.values().sum()
}
