use std::collections::HashMap;

use advent2025::{Part, advent_main, all_lines, multidim::Point};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let junction_boxes = all_lines(filename)?.map(|line| Point::<u64,3>::from_iter(line.split(",").map(|n| n.parse::<u64>().unwrap()))).collect_vec();
        match part {
            Part::One => {
                let mut distances = HashMap::new();
                for i in 0..junction_boxes.len() {
                    for j in i + 1..junction_boxes.len() {
                        distances.insert((i, j), junction_boxes[i].euclidean_distance(&junction_boxes[j]));
                    }
                }

                let num_pairs = if filename.contains("ex") { 10 } else { 1000 };
                let closest_to_farthest = distances.iter().sorted_by(|((_,_), d1), ((_,_), d2)| d1.total_cmp(d2)).take(num_pairs).map(|((x, y), _)| (*x, *y)).collect_vec();

                todo!("No part 1 yet")
            }
            Part::Two => {
                todo!("No part 2 yet")
            }
        }
        Ok(())
    })
}

