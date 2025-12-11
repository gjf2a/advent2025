use std::collections::HashMap;

use advent2025::{Part, advent_main, all_lines, graph::AdjacencySets};
use common_macros::hash_map;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let graph = graph_from_file(filename)?;
        match part {
            Part::One => {
                let mut path_length_table = PathsEndingAtTable::new(graph.reversed(), &"you");
                let total_paths = path_length_table.paths_ending_at(&"out");
                println!("{total_paths}");
            }
            Part::Two => {
                todo!("No part 2 yet")
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
