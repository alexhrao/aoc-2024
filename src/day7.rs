use std::hint::unreachable_unchecked;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Equation {
    pub answer: u64,
    pub numbers: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Equation {
    pub fn parse(s: &str) -> Self {
        let (answer, rest) = s.split_once(": ").unwrap();
        let answer = answer.parse().unwrap();
        let numbers = rest
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Self { answer, numbers }
    }
    pub fn can_solve(&self) -> bool {
        let num_ops = self.numbers.len() - 1;
        for ops in 0u64..2u64.pow(num_ops as u32) {
            let op_iter = (0..num_ops)
                .map(|m| {
                    if (ops & (0x1 << m)) == 0 {
                        Operation::Multiply
                    } else {
                        Operation::Add
                    }
                })
                .zip(&self.numbers[1..]);
            let mut out = self.numbers[0];
            for (op, num) in op_iter {
                out = match op {
                    Operation::Add => out.checked_add(*num).unwrap(),
                    Operation::Multiply => out.checked_mul(*num).unwrap(),
                    _ => unsafe { unreachable_unchecked() },
                };
            }
            if out == self.answer {
                return true;
            }
        }
        false
    }
    pub fn can_solve_extended(&self) -> bool {
        let num_ops = self.numbers.len() - 1;
        let mut ops = vec![Operation::Add; num_ops];
        for mut op in 0u64..3u64.pow(num_ops as u32) {
            for i in ops.iter_mut() {
                *i = match op % 3 {
                    0 => Operation::Add,
                    1 => Operation::Multiply,
                    2 => Operation::Concatenate,
                    _ => unreachable!(),
                };
                op /= 3;
            }
            let mut out = self.numbers[0];
            for (op, num) in ops.iter().zip(&self.numbers[1..]) {
                out = match op {
                    Operation::Add => out.checked_add(*num).unwrap(),
                    Operation::Multiply => out.checked_mul(*num).unwrap(),
                    Operation::Concatenate => format!("{out}{num}").parse().unwrap(),
                };
            }
            if out == self.answer {
                return true;
            }
        }
        false
    }
}
#[aoc_generator(day7)]
pub fn gen(input: &str) -> Vec<Equation> {
    input.lines().map(Equation::parse).collect()
}

#[aoc(day7, part1)]
pub fn part1(eqns: &[Equation]) -> u64 {
    use rayon::prelude::*;
    eqns.par_iter()
        .filter_map(|e| e.can_solve().then_some(e.answer))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(eqns: &[Equation]) -> u64 {
    use rayon::prelude::*;
    eqns.par_iter()
        .filter_map(|e| (e.can_solve() || e.can_solve_extended()).then_some(e.answer))
        .sum()
}
