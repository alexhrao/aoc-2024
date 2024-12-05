use aoc_runner_derive::{aoc, aoc_generator};

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
const WORD_R: [char; 4] = ['S', 'A', 'M', 'X'];

const MAS: [char; 3] = ['M', 'A', 'S'];
const MAS_R: [char; 3] = ['S', 'A', 'M'];

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day4, part1)]
pub fn part1(lines: &[Vec<char>]) -> usize {
    // Start horizontal
    let mut count = 0;
    for line in lines {
        for w in line.windows(WORD.len()) {
            if w == WORD || w == WORD_R {
                count += 1;
            }
        }
    }
    // Vertical
    for c in 0..lines[0].len() {
        // Create the column of chars
        let vert = lines.iter().map(|l| l[c]).collect::<Vec<_>>();
        for w in vert.windows(WORD.len()) {
            if w == WORD || w == WORD_R {
                count += 1;
            }
        }
    }
    // Diagonals
    for r in 0..lines.len() {
        for c in 0..lines[0].len() {
            // Get next four characters down and to the right, if I can
            if r <= (lines.len() - WORD.len()) && c <= (lines[0].len() - WORD.len()) {
                let c1 = lines[r][c];
                let c2 = lines[r + 1][c + 1];
                let c3 = lines[r + 2][c + 2];
                let c4 = lines[r + 3][c + 3];
                let tmp = [c1, c2, c3, c4];
                if tmp == WORD || tmp == WORD_R {
                    count += 1;
                }
            }
            // Get next four characters down and to the left, if I can
            if r <= (lines.len() - WORD.len()) && c >= (WORD.len() - 1) {
                let c1 = lines[r][c];
                let c2 = lines[r + 1][c - 1];
                let c3 = lines[r + 2][c - 2];
                let c4 = lines[r + 3][c - 3];
                let tmp = [c1, c2, c3, c4];
                if tmp == WORD || tmp == WORD_R {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day4, part1, rayon)]
pub fn part1_rayon(lines: &[Vec<char>]) -> usize {
    use rayon::prelude::*;
    // Start horizontal
    let mut count = lines
        .par_iter()
        .map(|line| {
            line.windows(WORD.len())
                .filter(|w| w == &WORD || w == &WORD_R)
                .count()
        })
        .sum();
    // Vertical
    for c in 0..lines[0].len() {
        // Create the column of chars
        let vert = lines.iter().map(|l| l[c]).collect::<Vec<_>>();
        for w in vert.windows(WORD.len()) {
            if w == WORD || w == WORD_R {
                count += 1;
            }
        }
    }
    // Diagonals
    for r in 0..lines.len() {
        for c in 0..lines[0].len() {
            // Get next four characters down and to the right, if I can
            if r <= (lines.len() - WORD.len()) && c <= (lines[0].len() - WORD.len()) {
                let c1 = lines[r][c];
                let c2 = lines[r + 1][c + 1];
                let c3 = lines[r + 2][c + 2];
                let c4 = lines[r + 3][c + 3];
                let tmp = [c1, c2, c3, c4];
                if tmp == WORD || tmp == WORD_R {
                    count += 1;
                }
            }
            // Get next four characters down and to the left, if I can
            if r <= (lines.len() - WORD.len()) && c >= (WORD.len() - 1) {
                let c1 = lines[r][c];
                let c2 = lines[r + 1][c - 1];
                let c3 = lines[r + 2][c - 2];
                let c4 = lines[r + 3][c - 3];
                let tmp = [c1, c2, c3, c4];
                if tmp == WORD || tmp == WORD_R {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(lines: &[Vec<char>]) -> usize {
    let mut count = 0;
    // Diagonals
    for r in 0..lines.len() {
        for c in 0..lines[0].len() {
            // Get next three characters down and to the right, if I can
            if r <= (lines.len() - MAS.len()) && c <= (lines[0].len() - MAS.len()) {
                let c1 = lines[r][c];
                let c2 = lines[r + 1][c + 1];
                let c3 = lines[r + 2][c + 2];
                let tmp = [c1, c2, c3];
                if tmp == MAS || tmp == MAS_R {
                    let c1 = lines[r][c + 2];
                    let c2 = lines[r + 1][c + 1];
                    let c3 = lines[r + 2][c];
                    let tmp = [c1, c2, c3];
                    if tmp == MAS || tmp == MAS_R {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
