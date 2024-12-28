use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Point;

const ROOM_ROWS: isize = 103;
const ROOM_COLS: isize = 101;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Robot {
    posn: Point<isize>,
    velocity: Point<isize>,
}

impl Robot {
    pub fn step(&mut self) {
        let Point { x, y } = self.posn;
        let Point { x: vx, y: vy } = self.velocity;

        self.posn = Point {
            x: (x + vx).rem_euclid(ROOM_COLS),
            y: (y + vy).rem_euclid(ROOM_ROWS),
        };
    }

    pub fn quadrant(&self) -> Option<usize> {
        // if I'm exactly in the middle, nope
        let midx = (ROOM_COLS - 1) / 2;
        let midy = (ROOM_ROWS - 1) / 2;
        if self.posn.x == midx || self.posn.y == midy {
            None
        } else {
            Some(match self.posn {
                Point { x, y } if x > midx && y < midy => 1,
                Point { x, y } if x < midx && y < midy => 0,
                Point { x, y } if x > midx && y > midy => 2,
                Point { x, y } if x < midx && y > midy => 3,
                _ => unreachable!(),
            })
        }
    }
}

#[aoc_generator(day14)]
pub fn gen(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (posn, velocity) = line.split_once(' ').unwrap();
            let (left, right) = posn[2..].split_once(',').unwrap();
            let posn = Point {
                x: left.parse().unwrap(),
                y: right.parse().unwrap(),
            };
            let (left, right) = velocity[2..].split_once(',').unwrap();
            let velocity = Point {
                x: left.parse().unwrap(),
                y: right.parse().unwrap(),
            };
            Robot { posn, velocity }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(robots: &[Robot]) -> usize {
    let mut robots = robots.to_vec();
    for _ in 0..100 {
        for r in &mut robots {
            r.step();
        }
    }
    let mut quads = [0, 0, 0, 0];
    for r in &robots {
        if let Some(q) = r.quadrant() {
            quads[q] += 1;
        }
    }
    quads.iter().product()
}

#[aoc(day14, part2)]
pub fn part2(robots: &[Robot]) -> usize {
    let mut robots = robots.to_vec();
    for t in 1.. {
        for r in &mut robots {
            r.step();
        }
        let posns: HashSet<(isize, isize)> = robots.iter().map(|r| (r.posn.x, r.posn.y)).collect();
        if posns.len() == robots.len() {
            println!();
            println!("t: {t}");
            for r in 0..ROOM_ROWS {
                for c in 0..ROOM_COLS {
                    if posns.contains(&(r, c)) {
                        print!("▮");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            return t;
        }
    }
    panic!();
}
