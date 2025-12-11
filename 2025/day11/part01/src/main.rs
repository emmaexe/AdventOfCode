use petgraph::{algo::toposort, prelude::*};
use std::collections::HashMap;

fn solve(input: &str) {
    let mut graph = DiGraph::<(), ()>::new();
    let mut nodes = HashMap::<&str, NodeIndex>::new();

    input.lines().for_each(|line| {
        let (src_str, targets) = line.split_once(": ").unwrap();
        let src_idx = *nodes.entry(src_str).or_insert_with(|| graph.add_node(()));
        for target in targets.split_whitespace() {
            let target_idx = *nodes.entry(target).or_insert_with(|| graph.add_node(()));
            graph.add_edge(src_idx, target_idx, ());
        }
    });

    let (start_idx, end_idx) = (*nodes.get("you").unwrap(), *nodes.get("out").unwrap());
    let nodes_ordered = toposort(&graph, None).unwrap();
    let mut path_counts = vec![0; graph.node_count()];
    path_counts[start_idx.index()] = 1;

    for &node in &nodes_ordered {
        let current_path_count = path_counts[node.index()];
        for next in graph.neighbors(node) {
            path_counts[next.index()] += current_path_count;
        }
    }

    println!("Paths: {}", path_counts[end_idx.index()]);
}

fn main() {
    print!("[Sample] ");
    solve(include_str!("../../input/input.sample.1.txt"));
    print!("[Real] ");
    solve(include_str!("../../input/input.real.txt"));
}
