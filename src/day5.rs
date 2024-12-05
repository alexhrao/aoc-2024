use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Hash, Default)]
pub struct Rules {
    comes_before: Vec<u32>,
    comes_after: Vec<u32>,
}

#[aoc_generator(day5)]
pub fn generate(input: &str) -> (HashMap<u32, Rules>, Vec<Vec<u32>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(|r| {
        let (left, right) = r.split_once("|").unwrap();
        (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
    });
    let mut rulebook: HashMap<u32, Rules> = HashMap::new();
    for (before, after) in rules {
        rulebook.entry(before).or_default().comes_before.push(after);
        rulebook.entry(after).or_default().comes_after.push(before);
    }
    let pages = pages
        .lines()
        .map(|p| p.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    (rulebook, pages)
}

fn is_valid(pages: &[u32], rulebook: &HashMap<u32, Rules>) -> Result<(), (usize, usize)> {
    for (p, page) in pages.iter().enumerate() {
        // If there are no rules, it's fine
        let Some(rules) = rulebook.get(page) else {
            continue;
        };
        if let Some(idx) = rules
            .comes_before
            .iter()
            .find_map(|b| pages[..p].iter().position(|pg| pg == b))
        {
            return Err((p, idx));
        }
        if let Some(idx) = rules
            .comes_after
            .iter()
            .find_map(|a| pages[p + 1..].iter().position(|pg| pg == a))
        {
            return Err((idx + p + 1, p));
        }
    }
    Ok(())
}

#[aoc(day5, part1, serial)]
pub fn part1_serial((rulebook, manuals): &(HashMap<u32, Rules>, Vec<Vec<u32>>)) -> u32 {
    manuals
        .iter()
        .filter_map(|pages| {
            if is_valid(pages, rulebook).is_ok() {
                Some(pages[pages.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part1)]
pub fn part1((rulebook, manuals): &(HashMap<u32, Rules>, Vec<Vec<u32>>)) -> u32 {
    use rayon::prelude::*;
    manuals
        .par_iter()
        .filter_map(|pages| {
            if is_valid(pages, rulebook).is_ok() {
                Some(pages[pages.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part2, serial)]
pub fn part2_serial((rulebook, manuals): &(HashMap<u32, Rules>, Vec<Vec<u32>>)) -> u32 {
    manuals
        .iter()
        .filter_map(|pages| {
            if is_valid(pages, rulebook).is_ok() {
                return None;
            }
            let mut pages = pages.to_owned();
            while let Err((a, b)) = is_valid(&pages, rulebook) {
                pages.swap(a, b);
            }
            Some(pages[pages.len() / 2])
        })
        .sum()
}

#[aoc(day5, part2)]
pub fn part2((rulebook, manuals): &(HashMap<u32, Rules>, Vec<Vec<u32>>)) -> u32 {
    use rayon::prelude::*;
    manuals
        .par_iter()
        .filter_map(|pages| {
            if is_valid(pages, rulebook).is_ok() {
                return None;
            }
            let mut pages = pages.to_owned();
            while let Err((a, b)) = is_valid(&pages, rulebook) {
                pages.swap(a, b);
            }
            Some(pages[pages.len() / 2])
        })
        .sum()
}
