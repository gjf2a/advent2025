use std::collections::HashMap;

use advent2025::{Part, advent_main, all_lines, multidim::Point, union_find::DisjointSetForest};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let junction_boxes = parse(filename)?;
        let distances = distances(&junction_boxes);
        match part {
            Part::One => {
                let num_pairs = if filename.contains("ex") { 10 } else { 1000 };
                let mut forest = DisjointSetForest::default();
                for (p1, p2) in sorted_distances(&distances)
                    .take(num_pairs)
                    .map(|((x, y), _)| (*x, *y))
                {
                    forest.union(&p1, &p2);
                }
                let score = forest
                    .all_sizes()
                    .sorted_by_key(|s| -(*s as isize))
                    .take(3)
                    .product::<usize>();
                println!("{score}");
            }
            Part::Two => {
                let mut forest = DisjointSetForest::default();
                for i in 0..junction_boxes.len() {
                    forest.make_set(i);
                }
                for ((p1, p2), _) in sorted_distances(&distances) {
                    forest.union(p1, p2);
                    if forest.num_roots() == 1 {
                        let score = junction_boxes[*p1][0] * junction_boxes[*p2][0];
                        println!("{score}");
                        break;
                    }
                }
            }
        }
        Ok(())
    })
}

fn parse(filename: &str) -> anyhow::Result<Vec<Point<u64, 3>>> {
    Ok(all_lines(filename)?
        .map(|line| Point::<u64, 3>::from_iter(line.split(",").map(|n| n.parse::<u64>().unwrap())))
        .collect())
}

fn distances(junction_boxes: &Vec<Point<u64, 3>>) -> HashMap<(usize, usize), f64> {
    let mut distances = HashMap::new();
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            distances.insert(
                (i, j),
                junction_boxes[i].euclidean_distance(&junction_boxes[j]),
            );
        }
    }
    distances
}

fn sorted_distances(
    distances: &HashMap<(usize, usize), f64>,
) -> impl Iterator<Item = (&(usize, usize), &f64)> {
    distances
        .iter()
        .sorted_by(|((_, _), d1), ((_, _), d2)| d1.total_cmp(d2))
}
