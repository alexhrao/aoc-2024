use std::{
    collections::HashMap,
    convert::Infallible,
    fmt::{Display, Write},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => unreachable!(),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Color::Black => 'b',
            Color::Blue => 'u',
            Color::Green => 'g',
            Color::Red => 'r',
            Color::White => 'w',
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pattern(Vec<Color>);

impl FromStr for Pattern {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(Color::from).collect()))
    }
}

impl Deref for Pattern {
    type Target = Vec<Color>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pattern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

#[aoc_generator(day19)]
pub fn gen(input: &str) -> (Vec<Pattern>, Vec<Pattern>) {
    let mut lines = input.lines();
    let mut have: Vec<Pattern> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|pat| pat.parse().unwrap())
        .collect();

    let want = lines.skip(1).map(|pat| pat.parse().unwrap()).collect();

    have.sort_unstable_by_key(|p| p.len());
    have.reverse();
    (have, want)
}

type Cache = HashMap<Vec<Color>, usize>;

fn matches(pattern: &[Color], inventory: &[Pattern], cache: &mut Cache) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    // If we've seen this before, just return that
    if let Some(answer) = cache.get(pattern).cloned() {
        return answer;
    }
    // Find the longest matching prefix
    let answer = inventory
        .iter()
        .filter_map(|pre| {
            if !pattern.starts_with(pre) {
                return None;
            }

            Some(matches(&pattern[pre.len()..], inventory, cache))
        })
        .sum();
    cache.insert(pattern.to_vec(), answer);
    answer
}

#[aoc(day19, part1)]
pub fn part1((inventory, patterns): &(Vec<Pattern>, Vec<Pattern>)) -> usize {
    let mut cache = HashMap::new();
    patterns
        .iter()
        .filter(|p| matches(p, inventory, &mut cache) != 0)
        .count()
}

#[aoc(day19, part2)]
pub fn part2((inventory, patterns): &(Vec<Pattern>, Vec<Pattern>)) -> usize {
    let mut cache = HashMap::new();
    patterns
        .iter()
        .map(|p| matches(p, inventory, &mut cache))
        .sum()
}
