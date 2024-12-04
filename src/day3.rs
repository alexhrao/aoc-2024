use aoc_runner_derive::aoc;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Start,
    M,
    U,
    L,
    LeftParenMul,
    LeftDigit,
    Comma,
    RightDigit,
    D,
    O,
    N,
    Apostrophe,
    T,
    LeftParenEnable,
    LeftParenDisable,
}
#[derive(Debug, Clone)]
struct State {
    step: Step,
    left: String,
    right: String,
    enable: bool,
}

impl State {
    fn reset(&mut self) {
        self.step = Step::Start;
        self.left = String::new();
        self.right = String::new();
    }
    fn blind_next(&mut self, next: char) -> Option<u32> {
        match (self.step, next) {
            (Step::Start, 'm') if self.enable => {
                self.step = Step::M;
            }
            (Step::M, 'u') if self.enable => {
                self.step = Step::U;
            }
            (Step::U, 'l') if self.enable => {
                self.step = Step::L;
            }
            (Step::L, '(') if self.enable => {
                self.step = Step::LeftParenMul;
            }
            (Step::LeftDigit, ',') if self.enable => {
                self.step = Step::Comma;
            }
            (Step::LeftParenMul, c) | (Step::LeftDigit, c) if self.enable => {
                if !c.is_numeric() {
                    // Bad!
                    self.reset();
                    self.next(c);
                } else {
                    self.step = Step::LeftDigit;
                    self.left.push(c);
                }
            }
            (Step::RightDigit, ')') if self.enable => {
                // Done! Calculate and reset
                let left: u32 = self.left.parse().unwrap();
                let right: u32 = self.right.parse().unwrap();

                // Reset
                self.reset();
                return Some(left * right);
            }
            (Step::Comma, c) | (Step::RightDigit, c) if self.enable => {
                if !c.is_numeric() {
                    self.reset();
                    self.next(c);
                } else {
                    self.step = Step::RightDigit;
                    self.right.push(c);
                }
            }
            (s, c) if s != Step::Start => {
                self.reset();
                self.next(c);
            }
            _ => self.reset(),
        }
        None
    }
    fn next(&mut self, next: char) -> Option<u32> {
        match (self.step, next) {
            (Step::Start, 'm') if self.enable => {
                self.step = Step::M;
            }
            (Step::Start, 'd') => {
                self.step = Step::D;
            }
            (Step::M, 'u') if self.enable => {
                self.step = Step::U;
            }
            (Step::U, 'l') if self.enable => {
                self.step = Step::L;
            }
            (Step::L, '(') if self.enable => {
                self.step = Step::LeftParenMul;
            }
            (Step::LeftDigit, ',') if self.enable => {
                self.step = Step::Comma;
            }
            (Step::LeftParenMul, c) | (Step::LeftDigit, c) if self.enable => {
                if !c.is_numeric() {
                    // Bad!
                    self.reset();
                    self.next(c);
                } else {
                    self.step = Step::LeftDigit;
                    self.left.push(c);
                }
            }
            (Step::RightDigit, ')') if self.enable => {
                // Done! Calculate and reset
                let left: u32 = self.left.parse().unwrap();
                let right: u32 = self.right.parse().unwrap();

                // Reset
                self.reset();
                return Some(left * right);
            }
            (Step::Comma, c) | (Step::RightDigit, c) if self.enable => {
                if !c.is_numeric() {
                    self.reset();
                    self.next(c);
                } else {
                    self.step = Step::RightDigit;
                    self.right.push(c);
                }
            }
            (Step::D, 'o') => {
                self.step = Step::O;
            }
            (Step::O, '(') => {
                self.step = Step::LeftParenEnable;
            }
            (Step::O, 'n') => {
                self.step = Step::N;
            }
            (Step::N, '\'') => {
                self.step = Step::Apostrophe;
            }
            (Step::Apostrophe, 't') => {
                self.step = Step::T;
            }
            (Step::T, '(') => {
                self.step = Step::LeftParenDisable;
            }
            (Step::LeftParenEnable, ')') => {
                self.enable = true;
                self.reset();
            }
            (Step::LeftParenDisable, ')') => {
                self.enable = false;
                self.reset();
            }
            (s, c) if s != Step::Start => {
                self.reset();
                self.next(c);
            }
            _ => self.reset(),
        }
        None
    }
}

#[aoc(day3, part1, no_regex)]
pub fn part1_no_regex(input: &str) -> u32 {
    let mut state = State {
        step: Step::Start,
        left: String::new(),
        right: String::new(),
        enable: true,
    };
    input.chars().filter_map(|c| state.blind_next(c)).sum()
}

#[aoc(day3, part1, regex)]
pub fn part1_regex(input: &str) -> u32 {
    let pat = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    pat.captures_iter(input)
        .map(|c| {
            let left: u32 = c.get(1).unwrap().as_str().parse().unwrap();
            let right: u32 = c.get(2).unwrap().as_str().parse().unwrap();
            left * right
        })
        .sum()
}
#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let mut state = State {
        step: Step::Start,
        left: String::new(),
        right: String::new(),
        enable: true,
    };
    input.chars().filter_map(|c| state.next(c)).sum()
}
