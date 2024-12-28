use std::fmt::Display;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Direction;

#[aoc_generator(day21)]
pub fn gen(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(16).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Action {
    Move(Direction),
    Activate,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Activate => f.write_str("A"),
            Action::Move(d) => write!(f, "{d}"),
        }
    }
}

struct DoorBot {
    posn: u8,
}

impl DoorBot {
    // pub fn input(&mut self, action: Action) {
    //     match action {
    //         Action::Activate => {
    //             self.presses.push(self.posn);
    //         }
    //         Action::Move(dir) => {
    //             match (dir, self.posn) {
    //                 (Direction::Up, 1..=6) => self.posn += 3,
    //                 (Direction::Up, 0) => self.posn = 2,
    //                 (Direction::Up, 0xa) => self.posn = 3,
    //                 (Direction::Down, 4..=9) => self.posn -= 3,
    //                 (Direction::Down, 2) => self.posn = 0,
    //                 (Direction::Down, 3) => self.posn = 0xa,
    //                 (Direction::Left, 2 | 3 | 5 | 6 | 8 | 9) => self.posn -= 1,
    //                 (Direction::Left, 0xa) => self.posn = 0,
    //                 (Direction::Right, 1 | 2 | 4 | 5 | 7 | 8) => self.posn += 1,
    //                 (Direction::Right, 0) => self.posn = 0xa,
    //                 (d, x) => panic!("Should not be going {d:?} from {x}"),
    //             };
    //         }
    //     }
    // }

    pub fn enter(&mut self, code: u8) -> Vec<Action> {
        fn posn(p: u8) -> (isize, isize) {
            match p {
                0 => (3, 1),
                1 => (2, 0),
                2 => (2, 1),
                3 => (2, 2),
                4 => (1, 0),
                5 => (1, 1),
                6 => (1, 2),
                7 => (0, 0),
                8 => (0, 1),
                9 => (0, 2),
                0xa => (3, 2),
                p => panic!("Position {p} not possible"),
            }
        }
        // Figure out where I am, then chart a course for how
        // to get there
        let am = posn(self.posn);
        let want = posn(code);
        let mut out = vec![];
        let mut lr = want.1 - am.1;
        let mut ud = want.0 - am.0;
        // If I'm moving left or right, go to the center first
        if am.0 == 3 && want.0 != 3 {
            // Move up once
            ud += 1;
            out.push(Action::Move(Direction::Up));
        } else if want.0 == 3 && am.0 != 3 && am.1 == 0 {
            // Move right once
            lr -= 1;
            out.push(Action::Move(Direction::Right));
        }
        let horz = Action::Move(if lr.is_negative() {
            Direction::Left
        } else {
            Direction::Right
        });
        let vert = Action::Move(if ud.is_negative() {
            Direction::Up
        } else {
            Direction::Down
        });
        self.posn = code;
        out.extend(
            std::iter::repeat_n(horz, lr.unsigned_abs())
                .chain(std::iter::repeat_n(vert, ud.unsigned_abs()))
                .chain(std::iter::once(Action::Activate)),
        );
        out
    }
}

struct RemoteBot {
    posn: Action,
    past: Vec<Action>,
}

impl RemoteBot {
    pub fn enter(&mut self, code: Action) -> Vec<Action> {
        fn posn(a: Action) -> (isize, isize) {
            match a {
                Action::Activate => (0, 2),
                Action::Move(Direction::Up) => (0, 1),
                Action::Move(Direction::Left) => (1, 0),
                Action::Move(Direction::Down) => (1, 1),
                Action::Move(Direction::Right) => (1, 2),
            }
        }

        // Figure out where I am, then chart a course for how
        // to get there
        let am = posn(self.posn);
        let want = posn(code);
        let mut out = vec![];
        let mut lr = want.1 - am.1;
        let mut ud = want.0 - am.0;
        // If I'm moving left or right, go to the center first
        if am.0 == 0 && want.0 != 0 {
            // Move down once
            ud -= 1;
            out.push(Action::Move(Direction::Down));
        } else if want.0 == 0 && am.0 != 0 && am.1 == 0 {
            // Move right once
            lr -= 1;
            out.push(Action::Move(Direction::Right));
        }
        let horz = Action::Move(if lr.is_negative() {
            Direction::Left
        } else {
            Direction::Right
        });
        let vert = Action::Move(if ud.is_negative() {
            Direction::Up
        } else {
            Direction::Down
        });
        self.posn = code;
        out.extend(
            std::iter::repeat_n(horz, lr.unsigned_abs())
                .chain(std::iter::repeat_n(vert, ud.unsigned_abs()))
                .chain(std::iter::once(Action::Activate)),
        );
        self.past.extend(&out);
        out
    }
}

// fn parse_example(moves: &str) -> impl Iterator<Item = Action> + use<'_> {
//     moves
//         .chars()
//         .map(|c| match c {
//             'v' | '>' | '<' | '^' => Action::Move(Direction::from(c)),
//             'A' => Action::Activate,
//             _ => unreachable!(),
//         })
// }

#[aoc(day21, part1)]
pub fn part1(codes: &[Vec<u8>]) -> usize {
    // let mut bot = DoorBot {
    //     posn: 0xa,
    //     presses: vec![],
    // };
    // for a in parse_example("<A^A>^^AvvvA") {
    //     bot.input(a);
    // }
    // println!("{:?}", bot.presses);
    // let mut bot = DoorBot {
    //     posn: 0xa,
    //     presses: vec![],
    // };
    // for a in parse_example("<A^A^>^AvvvA") {
    //     bot.input(a);
    // }
    // println!("{:?}", bot.presses);
    // let mut bot = DoorBot {
    //     posn: 0xa,
    //     presses: vec![],
    // };
    // for a in parse_example("<A^A^^>AvvvA") {
    //     bot.input(a);
    // }
    // println!("{:?}", bot.presses);
    let mut out = 0;

    let mut door = DoorBot { posn: 0xa };

    let mut remote1 = RemoteBot {
        posn: Action::Activate,
        past: vec![],
    };

    let mut remote2 = RemoteBot {
        posn: Action::Activate,
        past: vec![],
    };
    for code in codes {
        let mut count = 0;
        for &c in code {
            for a in door.enter(c) {
                for a in remote1.enter(a) {
                    for a in remote2.enter(a) {
                        count += 1;
                        print!("{a}");
                    }
                }
            }
        }
        println!();
        let mut num = 0;
        for (d, &digit) in code.iter().rev().skip(1).enumerate() {
            num += (10usize.pow(d as u32)) * (digit as usize);
        }
        println!("count * num: {count} * {num}");
        out += count * num;
    }
    println!();
    println!("Output: {out}");

    // for c in [0, 2, 9, 0xa] {
    //     for a in bot.enter(c) {
    //         print!("{a}");
    //     }
    // }
    // println!();
    out
}
