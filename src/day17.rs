use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
pub fn gen(input: &str) -> Machine {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let insts = lines
        .nth(1)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    Machine {
        a,
        b,
        c,
        inst_counter: 0,
        output: vec![],
        insts,
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    a: u64,
    b: u64,
    c: u64,
    inst_counter: usize,
    output: Vec<u64>,
    insts: Vec<u64>,
}

impl Machine {
    #[allow(dead_code)]
    fn reset(&mut self, a: u64) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.inst_counter = 0;
        self.output.clear();
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Operation {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u64> for Operation {
    fn from(value: u64) -> Self {
        match value {
            0 => Operation::Adv,
            1 => Operation::Bxl,
            2 => Operation::Bst,
            3 => Operation::Jnz,
            4 => Operation::Bxc,
            5 => Operation::Out,
            6 => Operation::Bdv,
            7 => Operation::Cdv,
            _ => unreachable!(),
        }
    }
}

fn combo(arg: u8, machine: &Machine) -> u64 {
    match arg {
        0..=3 => arg as u64,
        4 => machine.a,
        5 => machine.b,
        6 => machine.c,
        _ => unreachable!(),
    }
}

impl Operation {
    pub fn execute(&self, arg: u8, machine: &mut Machine) {
        match self {
            Self::Jnz => {
                if machine.a != 0 {
                    machine.inst_counter = arg as usize;
                    return;
                }
            }
            Self::Adv => machine.a /= 1u64 << combo(arg, machine),
            Self::Bdv => machine.b = machine.a / (1u64 << combo(arg, machine)),
            Self::Bxl => machine.b ^= u64::from(arg),
            Self::Bxc => machine.b ^= machine.c,
            Self::Bst => machine.b = combo(arg, machine) % 8,
            Self::Cdv => machine.c = machine.a / (1u64 << combo(arg, machine)),
            Self::Out => machine.output.push(combo(arg, machine) % 8),
        }
        machine.inst_counter += 2;
    }
}

#[aoc(day17, part1)]
pub fn part1(machine: &Machine) -> String {
    let mut machine = machine.clone();

    while let Some(args) = machine
        .insts
        .get(machine.inst_counter..=machine.inst_counter + 1)
    {
        let op = Operation::from(args[0]);
        let arg = args[1] as u8;
        op.execute(arg, &mut machine);
    }
    let mut out = machine.output.iter().fold(
        String::with_capacity(machine.output.len() * 2),
        |mut s, b| {
            use std::fmt::Write;
            write!(s, "{b},").unwrap();
            s
        },
    );
    out.pop();
    out
}

#[inline(always)]
fn out(a: u64) -> u64 {
    (((a & 0b111) ^ 0b1) ^ (a / (1 << ((a & 0b111) ^ 0b010)))) % 8
}

fn chain(insts: &[u64], mut seed: u64) -> Option<u64> {
    if insts.is_empty() {
        return Some(seed);
    }
    seed *= 8;
    let tgt = insts[0];
    let rest = &insts[1..];
    for offset in 0..8 {
        if out(seed + offset) == tgt {
            if let Some(ret) = chain(rest, seed + offset) {
                return Some(ret);
            }
        }
    }
    None
}

#[aoc(day17, part2)]
pub fn part2(machine: &Machine) -> u64 {
    let mut insts = machine.insts.clone();
    insts.reverse();

    for seed in 0.. {
        if let Some(a) = chain(&insts, seed) {
            return a;
        }
    }
    panic!("Could not find a suitable value for a");
}
