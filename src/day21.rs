use std::{collections::HashSet, fmt::Display};

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
pub struct Step {
    start: (usize, usize),
    lr: (Direction, usize),
    ud: (Direction, usize),
}

fn door(code: &[u8]) -> Vec<Step> {
    fn decode(c: u8) -> (usize, usize) {
        match c {
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
    // Output is a vector of things to enter, where each is a jumble of directions.
    // Only reason it's not a whole jumble is that numbers must be entered
    // in order.
    // I need to go
    let mut out = vec![];
    // Start at 0xa
    let (mut cr, mut cc) = decode(0xa);
    for &digit in code {
        let (nr, nc) = decode(digit);
        // For now... don't worry about the panic spot...
        let ud = if nr < cr { Direction::Up } else { Direction::Down };
        let lr = if nc < cc { Direction::Left } else { Direction::Right };
        out.push(Step {
            start: (cr, cc),
            lr: (lr, nc.abs_diff(cc)),
            ud: (ud, nr.abs_diff(cr)),
        });
        (cr, cc) = (nr, nc);
    }
    out
}

// Thinking:
// For the first robot, I don't actually think there are any efficiency
// concerns. Each input step is going to mean we start at 0xa, go to the
// specified direction button, then come back.
// Which means... we probably can't just do this in one fell swoop. We need
// to consider BOTH robots... I think.
//
// Okay so for the second robot, they're controlling the first robot. What
// is an example of an inefficiency? Let's break down this example from
// the actual keypad to the second robot
//
// Suppose I need to go 2 steps up, 1 step left. I could do it like this:
//
//  * Up 1
//  * Left 1
//  * Up 1
//  * Activate
//
// The first robot, then, would issue:
//
//  * Left 1
//  * Activate
//  * Down 1
//  * Left 1
//  * Activate
//  * Right 1
//  * Up 1
//  * Activate
//  * Right 1
//  * Activate
//
// But, suppose I instead do it like this:
//
//  * Up 1
//  * Up 1
//  * Left 1
//  * Activate
//
// The first robot, then, would issue:
//
//  * Left 1
//  * Activate
//  * Activate
//  * Down 1
//  * Left 1
//  * Activate
//  * Right 1
//  * Right 1
//  * Activate
//
// This saves a move, so we need to consider it. So... hmm. What we want
// to know is "hey, can I just hit the same button again?" That translates
// to "is it necessary to go in the same direction again? And is it safe?"
//
// That's all well and good, but how do we get to an answer there? Okay so
// for the first robot, it should... actually, I think we need to start
// from the last robot. What would be most efficient for it?
//
// Oh ugh does this mean I need to consider, like, ahhh, like how I approached
// a number? Actually... kinda no. because I'll have to hit that activate
// button, I'll have to go back to A on the first robot. And, since every
// activate will be a whole **string** of activates, it means that each time
// I hit activate on robot N, robot N + 1, N + 2, ... will have hit activate
//
// Now... that's an interesting observation. For a given robot, if it's
// hitting the A button, everyone behind it is also hitting the activate
// button. To me, that means the "activate" button must act as some kind
// of partition. This kind of reminds me of compiler optimizations, and
// the activate buttons are "barriers" that allow us to reason locally
//
// So. In general, we always want a path that goes in the same direction
// as long as possible.

fn robots(nums: &[u8]) -> Vec<Option<Direction>> {
    let mut out = vec![];

    out
}

#[aoc(day21, part1)]
pub fn part1(codes: &[Vec<u8>]) -> usize {
    for code in codes {
        let path = door(code);
        println!("{code:?}: {path:?}");
    }
    0
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// enum Action {
//     Move(Direction),
//     Activate,
// }

// impl Display for Action {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Action::Activate => f.write_str("A"),
//             Action::Move(d) => write!(f, "{d}"),
//         }
//     }
// }

// struct DoorBot {
//     posn: u8,
// }

// impl DoorBot {
//     pub fn enter(&mut self, code: u8) -> Vec<Action> {
//         fn posn(p: u8) -> (isize, isize) {
//             match p {
//                 0 => (3, 1),
//                 1 => (2, 0),
//                 2 => (2, 1),
//                 3 => (2, 2),
//                 4 => (1, 0),
//                 5 => (1, 1),
//                 6 => (1, 2),
//                 7 => (0, 0),
//                 8 => (0, 1),
//                 9 => (0, 2),
//                 0xa => (3, 2),
//                 p => panic!("Position {p} not possible"),
//             }
//         }
//         // Figure out where I am, then chart a course for how
//         // to get there
//         let am = posn(self.posn);
//         let want = posn(code);
//         let mut out = vec![];
//         let mut lr = want.1 - am.1;
//         let mut ud = want.0 - am.0;
//         // If I'm moving left or right, go to the center first
//         if am.0 == 3 && want.0 != 3 {
//             // Move up once
//             ud += 1;
//             out.push(Action::Move(Direction::Up));
//         } else if want.0 == 3 && am.0 != 3 && am.1 == 0 {
//             // Move right once
//             lr -= 1;
//             out.push(Action::Move(Direction::Right));
//         }
//         let horz = Action::Move(if lr.is_negative() {
//             Direction::Left
//         } else {
//             Direction::Right
//         });
//         let vert = Action::Move(if ud.is_negative() {
//             Direction::Up
//         } else {
//             Direction::Down
//         });
//         self.posn = code;
//         out.extend(
//             std::iter::repeat_n(horz, lr.unsigned_abs())
//                 .chain(std::iter::repeat_n(vert, ud.unsigned_abs()))
//                 .chain(std::iter::once(Action::Activate)),
//         );
//         out
//     }
// }

// struct RemoteBot {
//     posn: Action,
//     past: Vec<Action>,
// }

// impl RemoteBot {
//     pub fn enter(&mut self, code: Action) -> Vec<Action> {
//         fn posn(a: Action) -> (isize, isize) {
//             match a {
//                 Action::Activate => (0, 2),
//                 Action::Move(Direction::Up) => (0, 1),
//                 Action::Move(Direction::Left) => (1, 0),
//                 Action::Move(Direction::Down) => (1, 1),
//                 Action::Move(Direction::Right) => (1, 2),
//             }
//         }

//         // Figure out where I am, then chart a course for how
//         // to get there
//         let am = posn(self.posn);
//         let want = posn(code);
//         let mut out = vec![];
//         let mut lr = want.1 - am.1;
//         let mut ud = want.0 - am.0;
//         // If I'm moving left or right, go to the center first
//         if am.0 == 0 && want.0 != 0 {
//             // Move down once
//             ud -= 1;
//             out.push(Action::Move(Direction::Down));
//         } else if want.0 == 0 && am.0 != 0 && am.1 == 0 {
//             // Move right once
//             lr -= 1;
//             out.push(Action::Move(Direction::Right));
//         }
//         let horz = Action::Move(if lr.is_negative() {
//             Direction::Left
//         } else {
//             Direction::Right
//         });
//         let vert = Action::Move(if ud.is_negative() {
//             Direction::Up
//         } else {
//             Direction::Down
//         });
//         self.posn = code;
//         out.extend(
//             std::iter::repeat_n(horz, lr.unsigned_abs())
//                 .chain(std::iter::repeat_n(vert, ud.unsigned_abs()))
//                 .chain(std::iter::once(Action::Activate)),
//         );
//         self.past.extend(&out);
//         out
//     }
// }

// // fn parse_example(moves: &str) -> impl Iterator<Item = Action> + use<'_> {
// //     moves
// //         .chars()
// //         .map(|c| match c {
// //             'v' | '>' | '<' | '^' => Action::Move(Direction::from(c)),
// //             'A' => Action::Activate,
// //             _ => unreachable!(),
// //         })
// // }

// #[aoc(day21, part1)]
// pub fn part1(codes: &[Vec<u8>]) -> usize {
//     // let mut bot = DoorBot {
//     //     posn: 0xa,
//     //     presses: vec![],
//     // };
//     // for a in parse_example("<A^A>^^AvvvA") {
//     //     bot.input(a);
//     // }
//     // println!("{:?}", bot.presses);
//     // let mut bot = DoorBot {
//     //     posn: 0xa,
//     //     presses: vec![],
//     // };
//     // for a in parse_example("<A^A^>^AvvvA") {
//     //     bot.input(a);
//     // }
//     // println!("{:?}", bot.presses);
//     // let mut bot = DoorBot {
//     //     posn: 0xa,
//     //     presses: vec![],
//     // };
//     // for a in parse_example("<A^A^^>AvvvA") {
//     //     bot.input(a);
//     // }
//     // println!("{:?}", bot.presses);
//     let mut out = 0;

//     let mut door = DoorBot { posn: 0xa };

//     let mut remote1 = RemoteBot {
//         posn: Action::Activate,
//         past: vec![],
//     };

//     let mut remote2 = RemoteBot {
//         posn: Action::Activate,
//         past: vec![],
//     };
//     for code in codes {
//         let mut count = 0;
//         for &c in code {
//             for a in door.enter(c) {
//                 for a in remote1.enter(a) {
//                     for a in remote2.enter(a) {
//                         count += 1;
//                         print!("{a}");
//                     }
//                 }
//             }
//         }
//         println!();
//         let mut num = 0;
//         for (d, &digit) in code.iter().rev().skip(1).enumerate() {
//             num += (10usize.pow(d as u32)) * (digit as usize);
//         }
//         println!("count * num: {count} * {num}");
//         out += count * num;
//     }
//     println!();
//     println!("Output: {out}");

//     // for c in [0, 2, 9, 0xa] {
//     //     for a in bot.enter(c) {
//     //         print!("{a}");
//     //     }
//     // }
//     // println!();
//     out
// }
