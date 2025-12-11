use std::collections::HashMap;

use advent2025::{Part, advent_main, all_lines, graph::AdjacencySets};
use common_macros::hash_map;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let graph = graph_from_file(filename)?;
        match part {
            Part::One => {
                let count = PathsEndingAtTable::path_count(graph.reversed(), "you", "out");
                println!("{count}");
            }
            Part::Two => {
                let svr2fft = PathsEndingAtTable::path_count(graph.without("dac").reversed(), "svr", "fft");
                let fft2dac = PathsEndingAtTable::path_count(graph.reversed(), "fft", "dac");
                let dac2out = PathsEndingAtTable::path_count(graph.without("fft").reversed(), "fft", "out");
                let svr2dac = PathsEndingAtTable::path_count(graph.without("fft").reversed(), "svr", "dac");
                let dac2fft = PathsEndingAtTable::path_count(graph.reversed(), "dac", "fft");
                let fft2out = PathsEndingAtTable::path_count(graph.without("dac"), "fft", "out");
                let total = svr2fft + fft2dac + dac2out + svr2dac + dac2fft + fft2out;
                println!("{total}");
            }
        }
        Ok(())
    })
}

pub fn graph_from_file(filename: &str) -> anyhow::Result<AdjacencySets> {
    let mut result = AdjacencySets::default();
    for line in all_lines(filename)? {
        let mut outer_parts = line.split(':');
        let src = outer_parts.next().unwrap();
        for dest in outer_parts.next().unwrap().trim().split_whitespace() {
            result.connect(src, dest);
        }
    }
    Ok(result)
}

// paths_ending_at(you) = 1
// paths_ending_at(n) = sum of paths_ending_at(n') for all n' with an edge into n
struct PathsEndingAtTable {
    reversed: AdjacencySets,
    ending_at: HashMap<String, u64>,
}

impl PathsEndingAtTable {
    fn new(reversed: AdjacencySets, start_node: &str) -> Self {
        Self {
            reversed,
            ending_at: hash_map!(start_node.to_string() => 1),
        }
    }

    fn path_count(reversed: AdjacencySets, start_node: &str, end_node: &str) -> u64 {
        let mut table = Self::new(reversed, start_node);
        table.paths_ending_at(end_node)
    }

    fn paths_ending_at(&mut self, node: &str) -> u64 {
        match self.ending_at.get(node) {
            Some(v) => *v,
            None => {
                let neighbors = self
                    .reversed
                    .neighbors_of(node)
                    .map(|s| s.to_string())
                    .collect_vec();
                let total = neighbors
                    .iter()
                    .map(|neighbor| self.paths_ending_at(neighbor.as_str()))
                    .sum();
                self.ending_at.insert(node.to_string(), total);
                total
            }
        }
    }
}

// no_dac.paths_starting_ending(svr, fft) + everyone.paths_starting_ending(fft, dac) + no_fft.paths_starting_ending(dac, out)
// + no_fft.paths_starting_ending(svr, dac) + everyone.paths_starting_ending(dac, fft) + no_dac.paths_starting_ending(fft, out)
//
// Use three tables: one with everyone, one without fft, and one without dac.
