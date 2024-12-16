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
}
