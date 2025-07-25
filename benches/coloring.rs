#![feature(test)]

extern crate petgraph;
extern crate test;

use core::cmp::{max, min};
use petgraph::prelude::*;
use test::Bencher;

use petgraph::algo::dsatur_coloring;

#[bench]
fn dsatur_coloring_bench(bench: &mut Bencher) {
    static NODE_COUNT: usize = 10_000;
    let mut g = Graph::new_undirected();
    let nodes: Vec<NodeIndex<_>> = (0..NODE_COUNT).map(|i| g.add_node(i)).collect();
    for i in 0..NODE_COUNT {
        let n1 = nodes[i];
        let neighbour_count = i % 8 + 3;
        let j_from = max(0, i as i32 - neighbour_count as i32 / 2) as usize;
        let j_to = min(NODE_COUNT, j_from + neighbour_count);

        #[allow(clippy::needless_range_loop)]
        for j in j_from..j_to {
            let n2 = nodes[j];
            g.add_edge(n1, n2, ());
        }
    }

    bench.iter(|| {
        let _scores = dsatur_coloring(&g);
    });
}
