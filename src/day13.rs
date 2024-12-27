use aoc_runner_derive::{aoc, aoc_generator};
use mathru::{
    algebra::linear::{
        matrix::{General, Solve},
        vector::Vector,
    },
    matrix, vector,
};
use regex::Regex;

use crate::util::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Claw {
    a: Point<u64>,
    b: Point<u64>,
    prize: Point<u64>,
}

impl Claw {
    pub fn solve(&self) -> Option<(u64, u64)> {
        // 1. Ax + Bx = px
        // 2. Ay + By = py
        let a: General<f64> = matrix![
            self.a.x as f64, self.b.x as f64;
            self.a.y as f64, self.b.y as f64
        ];
        let b: Vector<f64> = vector![self.prize.x as f64; self.prize.y as f64];
        let x = a.solve(&b).ok()?;
        let (a, b) = (x[0].round() as u64, x[1].round() as u64);
        // Check that it works
        if ((self.a * a) + (self.b * b)) != self.prize {
            None
        } else {
            Some((a, b))
        }
    }
}

#[aoc_generator(day13)]
pub fn gen(input: &str) -> Vec<Claw> {
    let button_re = Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_re = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
    let lines = input.lines().chain(std::iter::once("")).collect::<Vec<_>>();
    lines
        .chunks_exact(4)
        .map(|claw| {
            let a = button_re.captures(claw[0]).unwrap();
            let a = Point {
                x: a.get(1).unwrap().as_str().parse().unwrap(),
                y: a.get(2).unwrap().as_str().parse().unwrap(),
            };
            let b = button_re.captures(claw[1]).unwrap();
            let b = Point {
                x: b.get(1).unwrap().as_str().parse().unwrap(),
                y: b.get(2).unwrap().as_str().parse().unwrap(),
            };
            let prize = prize_re.captures(claw[2]).unwrap();
            let prize = Point {
                x: prize.get(1).unwrap().as_str().parse().unwrap(),
                y: prize.get(2).unwrap().as_str().parse().unwrap(),
            };

            Claw { a, b, prize }
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(claws: &[Claw]) -> u64 {
    use rayon::prelude::*;
    claws
        .par_iter()
        .filter_map(|c| {
            c.solve()
                .filter(|&(a, b)| a <= 100 && b <= 100)
                .map(|(a, b)| 3 * a + b)
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(claws: &[Claw]) -> u64 {
    use rayon::prelude::*;
    const FUDGE: u64 = 10000000000000;
    claws
        .par_iter()
        .filter_map(|c| {
            let mut c = *c;
            c.prize.x += FUDGE;
            c.prize.y += FUDGE;
            c.solve().map(|(a, b)| 3 * a + b)
        })
        .sum()
}
