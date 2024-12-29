use std::{
    collections::{HashMap, HashSet},
    hash::RandomState,
};

use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::prelude::*;

use crate::util::{coords, DIRS};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Track {
    bounds: (usize, usize),
    walls: HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
}
#[aoc_generator(day20)]
pub fn gen(input: &str) -> Track {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let start = input
        .lines()
        .enumerate()
        .find_map(|(r, line)| line.chars().position(|c| c == 'S').map(|c| (r, c)))
        .unwrap();
    let end = input
        .lines()
        .enumerate()
        .find_map(|(r, line)| line.chars().position(|c| c == 'E').map(|c| (r, c)))
        .unwrap();
    let walls = input
        .lines()
        .enumerate()
        .flat_map(move |(r, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(c, ch)| (ch == '#').then_some((r, c)))
        })
        .collect();
    Track {
        walls,
        start,
        end,
        bounds: (rows, cols),
    }
}

type Pair = (NodeIndex<usize>, NodeIndex<usize>);

impl Track {
    pub fn find_cheats(&self) -> HashSet<(usize, usize)> {
        // A cheat is one where:
        //  1. I'm standing on a dot
        //  2. In the direction I'm looking, there is a wall
        //  3. In the **next** step it's blank
        //
        // So in other words, it's any occurrence of .#., either horizontally
        //  or vertically
        let mut out = HashSet::new();
        for &w in &self.walls {
            for d in DIRS {
                let Some(next) = d.step_bounded(w, self.bounds) else {
                    continue;
                };
                let Some(prev) = d.opposite().step_bounded(w, self.bounds) else {
                    continue;
                };
                // So next and prev need to be not in my walls
                if !self.walls.contains(&next) && !self.walls.contains(&prev) {
                    out.insert(w);
                }
            }
        }
        out
    }
    fn as_graph(&self) -> (Graph<(usize, usize), (), Undirected, usize>, Pair) {
        let mut graph = Graph::<(usize, usize), (), Undirected, usize>::default();
        let nodes: HashMap<_, _, RandomState> =
            HashMap::from_iter(coords(self.bounds).map(|p| (p, graph.add_node(p))));

        let edges = coords(self.bounds)
            .filter(|posn| !self.walls.contains(posn))
            .flat_map(|posn| {
                // If the next position isn't a wall, make the connection
                DIRS.into_iter()
                    .filter_map(|d| {
                        d.step_bounded(posn, self.bounds).and_then(|next| {
                            (!self.walls.contains(&next))
                                .then_some((*nodes.get(&posn).unwrap(), *nodes.get(&next).unwrap()))
                        })
                    })
                    .collect::<Vec<_>>()
            });
        graph.extend_with_edges(edges);
        (
            graph,
            (
                *nodes.get(&self.start).unwrap(),
                *nodes.get(&self.end).unwrap(),
            ),
        )
    }
    fn path(&self) -> HashMap<(usize, usize), usize> {
        // The code below proves there is **exactly** one path. No need to find
        // the "best" path; there's only one
        let mut posn = self.start;
        let mut path = HashMap::new();
        loop {
            path.insert(posn, path.len());
            if self.end == posn {
                break path;
            }
            for dir in DIRS {
                let Some(next) = dir.step_bounded(posn, self.bounds) else {
                    continue;
                };
                if !self.walls.contains(&next) && !path.contains_key(&next) {
                    posn = next;
                    break;
                }
            }
        }
    }
}

#[aoc(day20, part1)]
pub fn part1(track: &Track) -> usize {
    let (graph, (start, end)) = track.as_graph();
    let costs = petgraph::algo::dijkstra(&graph, start, None, |_| 1);
    let worst = *costs.get(&end).unwrap();
    let cheats = track.find_cheats();
    use rayon::prelude::*;
    cheats
        .par_iter()
        .filter(|cheat| {
            let mut track = track.clone();
            track.walls.remove(cheat);
            let (graph, (start, end)) = track.as_graph();
            let costs = petgraph::algo::dijkstra(&graph, start, None, |_| 1);
            let cost = *costs.get(&end).unwrap();
            (worst - cost) >= 100
        })
        .count()
}

#[aoc(day20, part2)]
pub fn part2(track: &Track) -> usize {
    const CHEAT_LEN: usize = 20;
    let dists = track.path();
    let mut path = dists.iter().collect::<Vec<_>>();
    path.sort_unstable_by_key(|(_, d)| **d);
    // Let's see how long (time wise) it would take to get all viable spots within
    // `CHEAT_LEN` steps (rise + run)
    let mut visited = HashSet::new();
    let mut cheats = 0;
    for (&(r, c), &d) in &path {
        visited.insert((r, c));
        let left = c.saturating_sub(CHEAT_LEN);
        let right = (c + CHEAT_LEN).clamp(0, track.bounds.1);
        let top = r.saturating_sub(CHEAT_LEN);
        let bottom = (r + CHEAT_LEN).clamp(0, track.bounds.0);
        for rr in top..=bottom {
            for cc in left..=right {
                let cheat_dist = rr.abs_diff(r) + cc.abs_diff(c);
                if cheat_dist > CHEAT_LEN {
                    continue;
                }
                let there = (rr, cc);
                if visited.contains(&there) {
                    // No use in going back to someplace we've already been
                    continue;
                }
                let Some(&dd) = dists.get(&there) else {
                    // This place isn't on the path (almost certainly because
                    //  it's a wall). No use in travelling somewhere that's
                    //  not going to get us anywhere
                    continue;
                };
                // So theoretically we could cheat our way there, and "there"
                //  is a place I haven't already visited. Now the question is
                //  this: How much time do I save by cheating my way there?
                //  That would be the time it would **normally** take me to get
                //  to (rr, cc), less the time this cheat takes
                // How long it would normally take to get from me to "there"
                if (dd - d) >= (cheat_dist + 100) {
                    // So, the normal distance is 100 + cheat_dist or longer.
                    //  That means we saved at LEAST 100 steps
                    cheats += 1;
                }
            }
        }
    }
    cheats
}
