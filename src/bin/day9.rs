use std::cmp::{max, min};

use advent2025::{Part, advent_main, all_lines, multidim::Point, sub_abs};
use itertools::Itertools;
use num::integer::gcd;

type Corner = Point<u64, 2>;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let red_tiles = get_red_tiles(filename)?;
        let rects = all_rectangles(&red_tiles);
        match part {
            Part::One => {
                println!("{}", largest_rectangle_area(&rects));
            }
            Part::Two => {
                let diffs = (0..red_tiles.len() - 1)
                    .map(|i| red_tiles[i].manhattan_distance(&red_tiles[i + 1]))
                    .collect_vec();
                let mut gcd_diffs = gcd(diffs[0], diffs[1]);
                for i in 1..diffs.len() {
                    gcd_diffs = gcd(gcd_diffs, diffs[i]);
                }
                println!("diffs: {diffs:?}");
                println!("diff GCD: {gcd_diffs}");
            }
        }
        Ok(())
    })
}

fn all_rectangles(red_tiles: &Vec<Corner>) -> Vec<(Corner, Corner)> {
    let mut result = vec![];
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            result.push((red_tiles[i], red_tiles[j]));
        }
    }
    result
}

fn largest_rectangle_area(rects: &Vec<(Corner, Corner)>) -> u64 {
    rects
        .iter()
        .map(|(p1, p2)| rectangle_area(p1, p2))
        .max()
        .unwrap()
}

fn get_red_tiles(filename: &str) -> anyhow::Result<Vec<Point<u64, 2>>> {
    Ok(all_lines(filename)?
        .map(|line| line.parse::<Corner>().unwrap())
        .collect())
}

fn rectangle_area(p1: &Corner, p2: &Corner) -> u64 {
    p1.values()
        .zip(p2.values())
        .map(|(v1, v2)| 1 + sub_abs(v1, v2))
        .product()
}
