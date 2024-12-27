use std::collections::{BTreeSet, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::prelude::*;

#[aoc_generator(day23)]
pub fn gen(input: &str) -> Vec<([u8; 2], [u8; 2])> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('-').unwrap();
            (
                left.as_bytes().try_into().unwrap(),
                right.as_bytes().try_into().unwrap(),
            )
        })
        .collect()
}

#[aoc(day23, part1)]
pub fn part1(computers: &[([u8; 2], [u8; 2])]) -> usize {
    let mut graph = UnGraph::new_undirected();
    let comps: HashSet<[u8; 2]> = computers
        .iter()
        .copied()
        .flat_map(|(left, right)| [left, right])
        .collect();
    let mut nodes = HashMap::new();
    for &c in &comps {
        nodes.insert(c, graph.add_node(c));
    }
    for (left, right) in computers {
        let left = *nodes.get(left).unwrap();
        let right = *nodes.get(right).unwrap();
        graph.add_edge(left, right, ());
    }

    let mut triplets = HashSet::new();

    for me in graph.node_indices() {
        for n1 in graph.neighbors(me) {
            for n2 in graph.neighbors(me) {
                if n1 == n2 {
                    continue;
                }
                // if n1 has n2 as a neighbor, good to go
                if graph.neighbors(n1).any(|n3| n3 == n2) {
                    let mut triplet = [me, n1, n2];
                    triplet.sort();
                    triplets.insert(triplet);
                }
            }
        }
    }
    triplets
        .iter()
        .filter(|triplet| {
            triplet
                .iter()
                .any(|&n| graph[n][0] == b't' || graph[n][0] == b'T')
        })
        .count()
}

type NodeGraph = petgraph::Graph<[u8; 2], (), petgraph::Undirected>;
type NodeSet = BTreeSet<NodeIndex<u32>>;

fn bron1(graph: &NodeGraph, r: NodeSet, mut p: NodeSet, mut x: NodeSet) -> BTreeSet<NodeSet> {
    let mut out = BTreeSet::new();
    if p.is_empty() && x.is_empty() {
        out.insert(r);
        return out;
    }

    for n in p.clone() {
        let mut r = r.clone();
        r.insert(n);
        let neighbors = graph.neighbors(n).collect();
        let p_call = p.intersection(&neighbors).copied().collect();
        let x_call = x.intersection(&neighbors).copied().collect();
        out.extend(bron1(graph, r, p_call, x_call));
        p.remove(&n);
        x.insert(n);
    }
    out
}

#[aoc(day23, part2)]
pub fn part2(computers: &[([u8; 2], [u8; 2])]) -> String {
    let mut graph = UnGraph::new_undirected();
    let comps: HashSet<[u8; 2]> = computers
        .iter()
        .copied()
        .flat_map(|(left, right)| [left, right])
        .collect();
    let mut nodes = HashMap::new();
    for &c in &comps {
        nodes.insert(c, graph.add_node(c));
    }
    for (left, right) in computers {
        let left = *nodes.get(left).unwrap();
        let right = *nodes.get(right).unwrap();
        graph.add_edge(left, right, ());
    }
    let p = graph.node_indices().collect();
    let mut clique = bron1(&graph, NodeSet::new(), p, NodeSet::new())
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .into_iter()
        .map(|idx| std::str::from_utf8(&graph[idx]).unwrap())
        .collect::<Vec<_>>();
    clique.sort();
    clique.join(",")
}
