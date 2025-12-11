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

    let nodes_ordered = toposort(&graph, None).unwrap();

    let get_unique_paths = |start_idx: NodeIndex, end_idx: NodeIndex| -> u64 {
        let mut path_counts: Vec<u64> = vec![0; nodes_ordered.len()];
        path_counts[start_idx.index()] = 1;

        for &node in &nodes_ordered {
            let current_path_count = path_counts[node.index()];
            for next in graph.neighbors(node) {
                path_counts[next.index()] += current_path_count;
            }
        }

        return path_counts[end_idx.index()];
    };

    let (svr_idx, fft_idx, dac_idx, out_idx) = (
        *nodes.get("svr").unwrap(),
        *nodes.get("fft").unwrap(),
        *nodes.get("dac").unwrap(),
        *nodes.get("out").unwrap(),
    );

    let paths: u64 = get_unique_paths(svr_idx, fft_idx)
        * get_unique_paths(fft_idx, dac_idx)
        * get_unique_paths(dac_idx, out_idx)
        + get_unique_paths(svr_idx, dac_idx)
            * get_unique_paths(dac_idx, fft_idx)
            * get_unique_paths(fft_idx, out_idx);

    println!("Paths: {paths}");
}

fn main() {
    print!("[Sample] ");
    solve(include_str!("../../input/input.sample.2.txt"));
    print!("[Real] ");
    solve(include_str!("../../input/input.real.txt"));
}
