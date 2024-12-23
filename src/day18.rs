use std::{
    collections::{HashMap, HashSet},
    hash::RandomState,
};

use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::prelude::*;

use crate::util::DIRS;

const ROWS: usize = 70 + 1;
const COLS: usize = 70 + 1;

#[aoc_generator(day18)]
pub fn gen(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            (right.parse().unwrap(), left.parse().unwrap())
        })
        .collect()
}

type NodeMap = HashMap<(usize, usize), NodeIndex>;

fn construct_graph(
    grid: &HashSet<(usize, usize)>,
) -> (Graph<(usize, usize), (), Undirected, u32>, NodeMap) {
    let mut nodes = HashMap::new();
    let mut graph = Graph::new_undirected();
    for &posn in grid {
        let id = graph.add_node(posn);
        nodes.insert(posn, id);
    }
    let bounds = (ROWS, COLS);
    for posn in grid {
        let a = *nodes.get(posn).unwrap();
        for d in DIRS {
            if let Some(next) = d.step_bounded(*posn, bounds) {
                if grid.contains(&next) {
                    let b = *nodes.get(&next).unwrap();
                    graph.add_edge(a, b, ());
                }
            }
        }
    }
    (graph, nodes)
}

#[aoc(day18, part1)]
pub fn part1(bytes: &[(usize, usize)]) -> usize {
    const TIME: usize = 1024;
    let mut grid: HashSet<(usize, usize), RandomState> =
        HashSet::from_iter((0..ROWS).flat_map(|r| (0..COLS).map(move |c| (r, c))));
    for posn in &bytes[..TIME] {
        grid.remove(posn);
    }

    let (graph, nodes) = construct_graph(&grid);

    let sidx = *nodes.get(&(0, 0)).unwrap();
    let eidx = nodes.get(&(ROWS - 1, COLS - 1)).unwrap();
    let costs = petgraph::algo::dijkstra(&graph, sidx, None, |_| 1usize);
    *costs.get(eidx).unwrap()
}

#[aoc(day18, part2)]
pub fn part2(bytes: &[(usize, usize)]) -> String {
    let mut grid: HashSet<(usize, usize), RandomState> =
        HashSet::from_iter((0..ROWS).flat_map(|r| (0..COLS).map(move |c| (r, c))));
    for posn in bytes {
        grid.remove(posn);
        let (graph, nodes) = construct_graph(&grid);
        let sidx = *nodes.get(&(0, 0)).unwrap();
        let eidx = *nodes.get(&(ROWS - 1, COLS - 1)).unwrap();
        if !petgraph::algo::has_path_connecting(&graph, sidx, eidx, None) {
            return format!("{},{}", posn.1, posn.0);
        }
    }
    panic!("No point found");
}
