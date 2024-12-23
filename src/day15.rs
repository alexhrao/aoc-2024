use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Direction;

#[aoc_generator(day15, part1)]
pub fn gen_part1(input: &str) -> (Grid, Vec<Direction>) {
    let rows = input.lines().take_while(|l| !l.is_empty());
    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();
    let mut robot = (0, 0);
    for (r, row) in rows.enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == '#' {
                walls.insert((r, c));
            } else if ch == 'O' {
                boxes.insert((r, c));
            } else if ch == '@' {
                robot = (r, c);
            }
        }
    }

    let dirs = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .flat_map(|l| l.chars().map(Direction::from))
        .collect();

    (
        Grid {
            boxes,
            walls,
            robot,
        },
        dirs,
    )
}

#[aoc_generator(day15, part2)]
pub fn gen_part2(input: &str) -> (Grid, Vec<Direction>) {
    let rows = input.lines().take_while(|l| !l.is_empty()).map(|l| {
        l.replace("#", "##")
            .replace("O", "[]")
            .replace(".", "..")
            .replace("@", "@.")
    });
    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();
    let mut robot = (0, 0);
    for (r, row) in rows.enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == '#' {
                walls.insert((r, c));
            } else if ch == '[' {
                boxes.insert((r, c));
            } else if ch == '@' {
                robot = (r, c);
            }
        }
    }

    let dirs = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .flat_map(|l| l.chars().map(Direction::from))
        .collect();

    (
        Grid {
            boxes,
            walls,
            robot,
        },
        dirs,
    )
}

#[derive(Debug, Clone)]
pub struct Grid {
    boxes: HashSet<(usize, usize)>,
    walls: HashSet<(usize, usize)>,
    robot: (usize, usize),
}

impl Grid {
    pub fn push(&mut self, dir: &Direction) {
        // Starting from where the robot stands, look in the direction. I'll find one of three things:
        //  1. A box
        //  2. A wall
        //  3. Nothing
        //
        // Going along that direction, go until I hit either a wall or nothing:
        //  * If it's a wall, nothing happens.
        //  * If it's nothing, then all the boxes (as well as me) get moved one space in that direction
        let mut posn = self.robot;
        let mut swept = HashSet::new();
        loop {
            posn = dir.step(posn).unwrap();
            if self.walls.contains(&posn) {
                // Failed
                return;
            }
            if self.boxes.contains(&posn) {
                // Add it to our running list of boxes
                swept.insert(posn);
            } else {
                // It's not a wall, nor a box, so it's nothing. We can succeed! Break out
                break;
            }
        }
        self.robot = dir.step(self.robot).unwrap();
        for s in &swept {
            self.boxes.remove(s);
        }
        self.boxes
            .extend(swept.into_iter().map(|p| dir.step(p).unwrap()));
    }

    pub fn push_wide(&mut self, dir: &Direction) {
        // This is a little different from last time. Same basic idea, but now we have to worry about
        //  "cascading". The general idea now is to keep track of "lanes" that we are looking at. We
        //  start looking at a new lane when a box demands it, and stop looking at a lane when it has
        //  nothing. If any lane ever contains a wall, we have to bail.

        // loop {}
        // Starting from where the robot stands, look in the direction. I'll find one of three things:
        //  1. A box
        //  2. A wall
        //  3. Nothing
        //
        // Going along that direction, go until I hit either a wall or nothing:
        //  * If it's a wall, nothing happens.
        //  * If it's nothing, then all the boxes (as well as me) get moved one space in that direction
        let mut lanes = HashSet::from([dir.ortho(self.robot).1]);
        let mut head = dir.ortho(self.robot).0;
        let mut swept = HashSet::new();
        let right_boxes = self
            .boxes
            .iter()
            .map(|&(r, c)| (r, c + 1))
            .collect::<HashSet<_>>();
        loop {
            let mut next_lanes = lanes.clone();
            for lane in lanes.drain() {
                let posn = dir.unortho(head, lane);
                let posn = dir.step(posn).unwrap();
                if self.walls.contains(&posn) {
                    // Failed
                    return;
                }
                if self.boxes.contains(&posn) {
                    // Sweep it up and add its right edge to the checked lanes
                    swept.insert(posn);
                    if matches!(dir, Direction::Up | Direction::Down) {
                        next_lanes.insert(dir.ortho(posn).1 + 1);
                    }
                } else if right_boxes.contains(&posn) {
                    // Sweep it up and add it to the checked lanes
                    swept.insert((posn.0, posn.1 - 1));
                    if matches!(dir, Direction::Up | Direction::Down) {
                        next_lanes.insert(dir.ortho(posn).1 - 1);
                    }
                } else {
                    // It's not a wall, and it's not a box. So it must be nothing.
                    // That means this lane no longer matters
                    next_lanes.remove(&lane);
                }
            }
            // This row is complete. If I still have lanes to check, move a row
            if next_lanes.is_empty() {
                break;
            }
            lanes = next_lanes;
            (head, _) = dir.ortho(dir.step(dir.unortho(head, 1)).unwrap());
        }
        self.robot = dir.step(self.robot).unwrap();
        for s in &swept {
            self.boxes.remove(s);
        }
        self.boxes
            .extend(swept.into_iter().map(|p| dir.step(p).unwrap()));
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self.walls.iter().map(|w| w.0).max().unwrap() + 1;
        let cols = self.walls.iter().map(|w| w.1).max().unwrap() + 1;
        let right_boxes = self
            .boxes
            .iter()
            .map(|&(r, c)| (r, c + 1))
            .collect::<HashSet<_>>();
        for r in 0..rows {
            for c in 0..cols {
                let posn = (r, c);
                if self.walls.contains(&posn) {
                    f.write_char('#')?;
                } else if self.boxes.contains(&posn) {
                    f.write_char('[')?;
                } else if right_boxes.contains(&posn) {
                    f.write_char(']')?;
                } else if self.robot == posn {
                    f.write_char('@')?;
                } else {
                    f.write_char('.')?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day15, part1)]
pub fn part1((grid, dirs): &(Grid, Vec<Direction>)) -> usize {
    let mut grid = grid.clone();
    for d in dirs {
        grid.push(d);
    }
    grid.boxes.into_iter().map(|(r, c)| (100 * r) + c).sum()
}

#[aoc(day15, part2)]
pub fn part2((grid, dirs): &(Grid, Vec<Direction>)) -> usize {
    let mut grid = grid.clone();
    for d in dirs {
        grid.push_wide(d);
    }
    grid.boxes.into_iter().map(|(r, c)| (100 * r) + c).sum()
}
