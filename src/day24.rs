use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Command {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gate {
    left: String,
    right: String,
    out: String,
    cmd: Command,
}

#[aoc_generator(day24)]
pub fn gen(input: &str) -> (HashMap<String, bool>, Vec<Gate>) {
    let gate_re =
        Regex::new(r"([a-zA-Z0-9]+) ((?:AND)|(?:OR)|(?:XOR)) ([a-zA-Z0-9]+) -> ([a-zA-Z0-9]+)")
            .unwrap();
    let init = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (wire, value) = line.split_once(": ").unwrap();
            (String::from(wire), value == "1")
        })
        .collect();

    let gates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let caps = gate_re.captures(line).unwrap();
            let left = caps.get(1).unwrap().as_str().to_owned();
            let cmd = match caps.get(2).unwrap().as_str() {
                "AND" => Command::And,
                "OR" => Command::Or,
                "XOR" => Command::Xor,
                _ => unreachable!(),
            };
            let right = caps.get(3).unwrap().as_str().to_owned();
            let out = caps.get(4).unwrap().as_str().to_owned();
            Gate {
                left,
                right,
                out,
                cmd,
            }
        })
        .collect();
    (init, gates)
}

pub fn compute(wire: &str, results: &mut HashMap<String, bool>, gates: &[Gate]) -> bool {
    if let Some(ans) = results.get(wire) {
        return *ans;
    }
    let gate = gates.iter().find(|g| g.out == wire).unwrap();
    let left = compute(&gate.left, results, gates);
    let right = compute(&gate.right, results, gates);
    let ans = match gate.cmd {
        Command::And => left && right,
        Command::Or => left || right,
        Command::Xor => left ^ right,
    };
    results.insert(wire.to_owned(), ans);
    ans
}

fn num(prefix: &str, results: &HashMap<String, bool>) -> u64 {
    results
        .iter()
        .filter_map(|(wire, &val)| {
            if val && wire.starts_with(prefix) {
                Some(0x1 << wire[1..].parse::<u64>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day24, part1)]
pub fn part1((inits, gates): &(HashMap<String, bool>, Vec<Gate>)) -> u64 {
    let mut results = inits.clone();

    let wires = gates
        .iter()
        .filter_map(|g| g.out.starts_with("z").then_some(&g.out));
    for wire in wires {
        compute(wire, &mut results, gates);
    }

    num("z", &results)
}

fn swap_wires(gates: &mut [Gate], w1: &str, w2: &str) {
    let mut g1: Option<usize> = None;
    let mut g2: Option<usize> = None;
    for (g, gate) in gates.iter().enumerate() {
        if gate.out == w1 {
            g1 = Some(g);
        } else if gate.out == w2 {
            g2 = Some(g);
        }
    }
    let g1 = g1.unwrap();
    let g2 = g2.unwrap();
    let tmp = gates[g1].out.clone();
    gates[g1].out = gates[g2].out.clone();
    gates[g2].out = tmp;
}

#[aoc(day24, part2)]
pub fn part2((inits, gates): &(HashMap<String, bool>, Vec<Gate>)) -> String {
    let mut gates = gates.clone();
    let mut swapped = vec![];

    loop {
        let mut swap = None;
        let mut cin: Option<&Gate> = None;
        let bits = inits
            .keys()
            .map(|k| k[1..].parse::<u32>().unwrap())
            .max()
            .unwrap();
        for i in 0..=bits {
            let x = format!("x{i:02}");
            let y = format!("y{i:02}");
            let mut x_xor_y = None;
            let mut x_and_y = None;
            for gate in &gates {
                // We need to find:
                // cin (except for i == 0. No cin there?)
                // x
                // y
                // x ^ y
                // x & y
                if (gate.left == x && gate.right == y) || (gate.left == y && gate.right == x) {
                    if gate.cmd == Command::And {
                        x_and_y = Some(gate);
                    } else if gate.cmd == Command::Xor {
                        x_xor_y = Some(gate);
                    }
                }
            }
            let x_xor_y = x_xor_y.unwrap();
            let x_and_y = x_and_y.unwrap();

            let mut z = None;
            let mut x_xor_y_and_c = None;
            let mut cout = None;
            if let Some(cing) = cin {
                for gate in &gates {
                    if (gate.left == cing.out && gate.right == x_xor_y.out)
                        || (gate.left == x_xor_y.out && gate.right == cing.out)
                    {
                        if gate.cmd == Command::Xor {
                            // We found our next z!
                            z = Some(gate);
                        } else if gate.cmd == Command::And {
                            x_xor_y_and_c = Some(gate);
                        }
                    }
                }
            } else {
                // z is just x_xor_y
                z = Some(x_xor_y);
                // cout (so next cin) is just x_and_y
                cout = Some(x_and_y);
            }

            let Some(z) = z else {
                // Couldn't find z. Find z and see who it depends on;
                // that will tell us what's wrong
                let should = format!("z{i:02}");
                for g in &gates {
                    if g.out == should {
                        // It'll depend on cin and what should be
                        // x ^ y. So if the cin is wrong, swap that out;
                        // otherwise, swap out x ^ y with what's our actual
                        // input signal
                        let left = &g.left;
                        let right = &g.right;
                        let cing = &cin.unwrap().out;
                        if left != cing && right != cing {
                            // swap it out
                            if left == &x_xor_y.out {
                                // swap out right
                                swap = Some((right.clone(), cing.clone()));
                            } else if right == &x_xor_y.out {
                                // swap out left
                                swap = Some((left.clone(), cing.clone()));
                            } else {
                                panic!("Both z inputs appear to be wrong");
                            }
                        } else if left != &x_xor_y.out && right != &x_xor_y.out {
                            if left == cing {
                                swap = Some((right.clone(), x_xor_y.out.clone()));
                            } else if right == cing {
                                swap = Some((left.clone(), x_xor_y.out.clone()));
                            } else {
                                panic!("Both z inputs appear to be wrong");
                            }
                        }
                    }
                }
                break;
            };
            if !z.out.starts_with("z") {
                // Need to swap with whoever it **should** be
                swap = Some((z.out.clone(), format!("z{i:02}")));
                break;
            }
            if let Some(xxoryandc) = x_xor_y_and_c {
                // cout = x_xor_y_and_c | x & y
                for gate in &gates {
                    if gate.cmd == Command::Or
                        && ((gate.left == xxoryandc.out && gate.right == x_and_y.out)
                            || (gate.left == x_and_y.out && gate.right == xxoryandc.out))
                    {
                        // We found cout
                        cout = Some(gate);
                    }
                }
            }
            let cout = cout.unwrap();
            cin = Some(cout);
        }
        if let Some((w1, w2)) = swap {
            swapped.push(w1.clone());
            swapped.push(w2.clone());
            swap_wires(&mut gates, &w1, &w2);
        } else {
            // We're done!
            break;
        }
    }
    // Now try adding x and y
    let mut results = inits.clone();

    let wires = gates.iter().map(|g| &g.out);
    for wire in wires {
        compute(wire, &mut results, &gates);
    }
    let x = num("x", &results);
    let y = num("y", &results);
    let z = num("z", &results);
    assert_eq!(x + y, z);

    swapped.sort();
    swapped.join(",")
}
