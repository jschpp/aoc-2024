use petgraph::{
    dot::{Config, Dot},
    prelude::*,
};
use std::collections::{HashSet, VecDeque};
use std::io::Write;
use std::{collections::HashMap, fs::File};

// TODO: This will be part of petgraph in the upcoming 0.7 release. Than vendoring it will not be necessary any longer
// when https://github.com/petgraph/petgraph/pull/662 is merged ths can be removed
mod maximal_cliques;
use maximal_cliques::maximal_cliques;

fn main() {
    let input = include_str!("../input.txt");
    let g = parse(input);

    // was used for debuggin purposes
    // save_graph(&g);

    let part1_result = part1(g.clone());
    println!("part1: {part1_result}");

    let cliques = maximal_cliques(&g);
    let mut code_parts: Vec<&str> = cliques
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .expect("at least one clique")
        .iter()
        .map(|i| g[*i])
        .collect();
    code_parts.sort();
    let code = code_parts.join(",");
    println!("part2: {code}");
}

#[allow(dead_code)]
fn save_graph(g: &Graph<&str, usize, Undirected>) {
    let mut file = File::create("graph.dot").unwrap();
    write!(file, "{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel])).unwrap();
}

fn parse(input: &str) -> Graph<&str, usize, Undirected> {
    let mut graph = Graph::new_undirected();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::default();
    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        let a = if !nodes.contains_key(a) {
            let a_idx = graph.add_node(a);
            nodes.insert(a, a_idx);
            a_idx
        } else {
            *nodes.get(a).expect("contains a")
        };
        let b = if !nodes.contains_key(b) {
            let b_idx = graph.add_node(b);
            nodes.insert(b, b_idx);
            b_idx
        } else {
            *nodes.get(b).expect("contains b")
        };
        graph.add_edge(a, b, 1);
    }
    graph
}

fn part1(g: Graph<&str, usize, Undirected>) -> usize {
    let mut nodes_to_search_for = Vec::default();
    for start in g.node_indices() {
        if g[start].starts_with("t") {
            nodes_to_search_for.push(start);
        }
    }
    const MAX_DEPTH: usize = 3;
    let mut result: HashSet<Vec<&str>> = HashSet::default();
    for node in nodes_to_search_for {
        let mut queue: VecDeque<(NodeIndex, Vec<NodeIndex>)> = VecDeque::default();
        let mut seen: HashSet<NodeIndex> = HashSet::default();
        seen.insert(node);
        for n in g.neighbors(node) {
            queue.push_back((n, vec![node]));
        }
        while let Some((idx, mut path)) = queue.pop_front() {
            seen.insert(idx);
            if path.len() > MAX_DEPTH {
                continue;
            }
            if path.len() == MAX_DEPTH && idx == node {
                let mut path: Vec<_> = path.iter().map(|idx| g[*idx]).collect();
                path.sort();
                result.insert(path);
                continue;
            }
            path.push(idx);
            for n in g.neighbors(idx) {
                if !seen.contains(&n) || n == node {
                    queue.push_back((n, path.clone()));
                }
            }
        }
    }
    result.len()
}
