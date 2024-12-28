use aoc_runner_derive::{aoc, aoc_generator};

pub type Lock = [usize; 5];
pub type Key = [usize; 5];

#[aoc_generator(day25)]
pub fn gen(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let lines: Vec<_> = input.lines().chain(std::iter::once("")).collect();
    let mut keys = Vec::with_capacity(lines.len() / 8);
    let mut locks = Vec::with_capacity(lines.len() / 8);
    for obj in lines.chunks_exact(8) {
        let obj = &obj[..7];
        if obj[0].chars().any(|c| c == '#') {
            let mut lock = [0usize; 5];
            // Lock
            for (c, pin) in lock.iter_mut().enumerate() {
                *pin = obj
                    .iter()
                    .position(|row| row.chars().nth(c).unwrap() == '.')
                    .unwrap_or(7)
                    - 1;
            }
            locks.push(lock);
        } else {
            let mut key = [0usize; 5];
            // Key
            for (c, pin) in key.iter_mut().enumerate() {
                *pin = obj
                    .iter()
                    .rev()
                    .position(|row| row.chars().nth(c).unwrap() == '.')
                    .unwrap_or(7)
                    - 1;
            }
            keys.push(key);
        }
    }

    (keys, locks)
}

fn fits(key: &Key, lock: &Lock) -> bool {
    key.iter().zip(lock).all(|(&k, &l)| (k + l) < 6)
}

#[aoc(day25, part1)]
pub fn part1((keys, locks): &(Vec<Key>, Vec<Lock>)) -> usize {
    keys.iter()
        .map(|k| locks.iter().filter(|l| fits(k, l)).count())
        .sum()
}
