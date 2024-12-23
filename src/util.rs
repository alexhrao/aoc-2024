use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl Direction {
    pub fn step(&self, posn: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => posn.0.checked_sub(1).map(|r| (r, posn.1)),
            Direction::Right => Some((posn.0, posn.1 + 1)),
            Direction::Down => Some((posn.0 + 1, posn.1)),
            Direction::Left => posn.1.checked_sub(1).map(|c| (posn.0, c)),
        }
    }
    pub fn step_bounded(
        &self,
        posn: (usize, usize),
        grid: (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => posn.0.checked_sub(1).map(|r| (r, posn.1)),
            Direction::Right => {
                if posn.1 == (grid.1 - 1) {
                    None
                } else {
                    Some((posn.0, posn.1 + 1))
                }
            }
            Direction::Down => {
                if posn.0 == (grid.0 - 1) {
                    None
                } else {
                    Some((posn.0 + 1, posn.1))
                }
            }
            Direction::Left => posn.1.checked_sub(1).map(|c| (posn.0, c)),
        }
    }
    pub fn turn(&self) -> Self {
        self.cw()
    }
    pub fn cw(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    pub fn ccw(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
    pub fn ortho(&self, posn: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up | Direction::Down => posn,
            Direction::Left | Direction::Right => (posn.1, posn.0),
        }
    }
    pub fn unortho(&self, para: usize, perp: usize) -> (usize, usize) {
        match self {
            Direction::Up | Direction::Down => (para, perp),
            Direction::Left | Direction::Right => (perp, para),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Up => '^',
            Self::Right => '>',
            Self::Down => 'v',
            Self::Left => '<',
        })
    }
}

pub fn coords(bounds: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (r, c) = bounds;
    (0..r).flat_map(move |r| (0..c).map(move |c| (r, c)))
}
