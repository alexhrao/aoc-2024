use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use petgraph::{graph::NodeIndex, Directed, Graph};
use rustworkx_core::{
    dictmap::DictMap,
    shortest_path::{all_shortest_paths, dijkstra},
};

use crate::util::{Direction, DIRS};

#[allow(dead_code)]
fn print_path(
    path: &HashMap<(usize, usize), Direction>,
    walls: &HashMap<(usize, usize), bool>,
    bounds: (usize, usize),
) {
    for r in 0..bounds.0 {
        for c in 0..bounds.1 {
            if walls.get(&(r, c)).is_some_and(|b| *b) {
                print!("#");
            } else if let Some(&dir) = path.get(&(r, c)) {
                print!("{dir}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub struct Maze {
    bounds: (usize, usize),
    graph: Graph<(), u32, Directed, u32>,
    nodes: HashMap<((usize, usize), Direction), NodeIndex>,
    posns: HashMap<NodeIndex, ((usize, usize), Direction)>,
    walls: HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    pub fn start(&self) -> NodeIndex {
        *self.nodes.get(&(self.start, Direction::Right)).unwrap()
    }
    pub fn ends(&self) -> impl Iterator<Item = NodeIndex> + use<'_> {
        DIRS.into_iter()
            .map(|d| *self.nodes.get(&(self.end, d)).unwrap())
    }
}

#[aoc_generator(day16)]
pub fn gen(input: &str) -> Maze {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let bounds = (rows, cols);

    let mut nodes: HashMap<((usize, usize), Direction), NodeIndex> = HashMap::new();
    let mut posns: HashMap<NodeIndex, ((usize, usize), Direction)> = HashMap::new();
    let mut walls: HashSet<(usize, usize)> = HashSet::new();

    let mut graph = Graph::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (r, row) in input.lines().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            let posn = (r, c);
            if ch == 'S' {
                start = posn;
            } else if ch == 'E' {
                end = posn;
            }
            for d in DIRS {
                let id = graph.add_node(());
                nodes.insert((posn, d), id);
                posns.insert(id, (posn, d));
            }
            if ch == '#' {
                walls.insert(posn);
            }
        }
    }
    let coords = (0..rows).flat_map(move |r| (0..cols).map(move |c| (r, c)));
    for posn in coords {
        if walls.contains(&posn) {
            continue;
        }
        for d in DIRS {
            let pidx = *nodes.get(&(posn, d)).unwrap();
            // If I can continue to go in this direction, create an edge from me
            // to the next with a cost of one
            if let Some(next) = d.step_bounded(posn, bounds) {
                if !walls.contains(&next) {
                    // Not a wall so we can go
                    let nidx = *nodes.get(&(next, d)).unwrap();
                    graph.add_edge(pidx, nidx, 1);
                }
            }
            // If I can turn cw and go in that direction, create an edge but 1000 cost
            if let Some(next) = d.cw().step_bounded(posn, bounds) {
                if !walls.contains(&next) {
                    let nidx = *nodes.get(&(next, d.cw())).unwrap();
                    graph.add_edge(pidx, nidx, 1001);
                }
            }
            // If I can turn ccw and go in that direction, create an edge but 1000 cost
            if let Some(next) = d.ccw().step_bounded(posn, bounds) {
                if !walls.contains(&next) {
                    let nidx = *nodes.get(&(next, d.ccw())).unwrap();
                    graph.add_edge(pidx, nidx, 1001);
                }
            }
        }
    }

    Maze {
        bounds,
        graph,
        nodes,
        posns,
        walls,
        start,
        end,
    }
}

#[allow(dead_code)]
fn print_posns(maze: &Maze, posns: &HashSet<(usize, usize)>) {
    for r in 0..maze.bounds.0 {
        for c in 0..maze.bounds.1 {
            let posn = (r, c);
            if maze.walls.contains(&posn) {
                print!("#");
            } else if posns.contains(&posn) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[aoc(day16, part1)]
pub fn part1(maze: &Maze) -> u32 {
    let costs: DictMap<NodeIndex, u32> = dijkstra(
        &maze.graph,
        maze.start(),
        None,
        |e| Ok::<u32, ()>(*e.weight()),
        None,
    )
    .unwrap();
    maze.ends()
        .filter_map(|eidx| costs.get(&eidx))
        .min()
        .copied()
        .unwrap()
}

#[aoc(day16, part2)]
pub fn part2(maze: &Maze) -> usize {
    let costs: DictMap<NodeIndex, u32> = dijkstra(
        &maze.graph,
        maze.start(),
        None,
        |e| Ok::<u32, ()>(*e.weight()),
        None,
    )
    .unwrap();
    let cost = maze
        .ends()
        .filter_map(|eidx| costs.get(&eidx))
        .min()
        .unwrap();

    maze.ends()
        .filter(|eidx| costs.get(eidx) == Some(cost))
        .flat_map(|eidx| {
            all_shortest_paths(&maze.graph, maze.start(), eidx, |e| {
                Ok::<u32, ()>(*e.weight())
            })
            .unwrap()
            .into_iter()
            .flat_map(|nodes| {
                nodes
                    .into_iter()
                    .filter_map(|idx| maze.posns.get(&idx).copied().map(|(p, _)| p))
            })
        })
        .collect::<HashSet<_>>()
        .len()
}
