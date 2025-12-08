use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use advent2025::{Part, advent_main, all_lines, multidim::Point, search_iter::BfsIter};
use common_macros::b_tree_set;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let junction_boxes = all_lines(filename)?
            .map(|line| {
                Point::<u64, 3>::from_iter(line.split(",").map(|n| n.parse::<u64>().unwrap()))
            })
            .collect_vec();
        match part {
            Part::One => {
                let mut distances = HashMap::new();
                for i in 0..junction_boxes.len() {
                    for j in i + 1..junction_boxes.len() {
                        distances.insert(
                            (i, j),
                            junction_boxes[i].euclidean_distance(&junction_boxes[j]),
                        );
                    }
                }

                let num_pairs = if filename.contains("ex") { 10 } else { 1000 };
                let chosen_pairs = distances
                    .iter()
                    .sorted_by(|((_, _), d1), ((_, _), d2)| d1.total_cmp(d2))
                    .take(num_pairs)
                    .map(|((x, y), _)| (*x, *y))
                    .collect_vec();
                let graph = AdjacencySets::new(&chosen_pairs);
                let score = graph
                    .circuits()
                    .iter()
                    .map(|c| c.len())
                    .sorted_by_key(|c| -(*c as isize))
                    .take(3)
                    .product::<usize>();
                println!("{score}");
            }
            Part::Two => {
                todo!("No part 2 yet")
            }
        }
        Ok(())
    })
}

#[derive(Default)]
struct AdjacencySets {
    adj: BTreeMap<usize, BTreeSet<usize>>,
}

impl AdjacencySets {
    fn connect2(&mut self, a: usize, b: usize) {
        self.connect(a, b);
        self.connect(b, a);
    }

    fn connect(&mut self, a: usize, b: usize) {
        match self.adj.get_mut(&a) {
            Some(ends) => {
                ends.insert(b);
            }
            None => {
                self.adj.insert(a, b_tree_set! {b});
            }
        }
    }

    fn new(pairs: &Vec<(usize, usize)>) -> Self {
        let mut result = Self::default();
        for (a, b) in pairs.iter() {
            result.connect2(*a, *b);
        }
        result
    }

    fn reachable_from(&self, a: usize) -> Vec<usize> {
        BfsIter::new(a, |n| {
            self.adj.get(&n).unwrap().iter().copied().collect_vec()
        })
        .sorted()
        .collect()
    }

    fn circuits(&self) -> HashSet<Vec<usize>> {
        self.adj.keys().map(|k| self.reachable_from(*k)).collect()
    }
}
