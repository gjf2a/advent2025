use std::cmp::{max, min};

use advent2025::{Part, advent_main, all_lines, multidim::Point, sub_abs};
use num::Integer;

type Corner = Point<u64, 2>;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let red_tiles = get_red_tiles(filename)?;
        let rects = match part {
            Part::One => all_rectangles(&red_tiles),
            Part::Two => legit_rectangles(&red_tiles),
        };
        println!("{}", largest_rectangle_area(&rects));
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

fn legit_rectangles(red_tiles: &Vec<Corner>) -> Vec<(Corner, Corner)> {
    let mut result = vec![];
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            if is_legit(red_tiles, i, j) {
                result.push((red_tiles[i], red_tiles[j]));
            }
        }
    }
    result
}

// Concept
// If a candidate corner is in-bounds, we can look at how it extends past the next points in the sequence.
// If the current corner is horizontal, we look for the next point that goes below (if going right) or above (if going left).
// If that next point has an x coordinate greater than our candidate, the candidate is legit.
//
// Apply the inverse logic in the y direction.
//
fn is_legit(red_tiles: &Vec<Corner>, i: usize, j: usize) -> bool {
    let red1 = red_tiles[i];
    let red2 = red_tiles[j];
    let pre1 = red_tiles[(i - 1).mod_floor(&red_tiles.len())];
    assert!(j > 0);
    let pre2 = red_tiles[j - 1];

    red1[0] == red2[0] || red1[1] == red2[1]
}

fn find_horizontal_ys(red_tiles: &Vec<Corner>) -> Vec<(usize, u64)> {
    (0..red_tiles.len())
        .filter(|i| red_tiles[*i][0] == red_tiles[(i + 1) % red_tiles.len()][0])
        .map(|i| (i, red_tiles[i][1]))
        .collect()
}

fn find_vertical_xs(red_tiles: &Vec<Corner>) -> Vec<(usize, u64)> {
    (0..red_tiles.len())
        .filter(|i| red_tiles[*i][1] == red_tiles[(i + 1) % red_tiles.len()][1])
        .map(|i| (i, red_tiles[i][0]))
        .collect()
}
