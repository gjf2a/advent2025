use std::cmp::max;

use advent2025::{Part, advent_main, all_lines, multidim::Point, sub_abs};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let red_tiles = get_red_tiles(filename)?;
        match part {
            Part::One => {
                println!("{}", largest_rectangle_area(&red_tiles));
            }
            Part::Two => {
                todo!("No part 2 yet")
            }
        }
        Ok(())
    })
}

fn get_red_tiles(filename: &str) -> anyhow::Result<Vec<Point<u64, 2>>> {
    Ok(all_lines(filename)?
        .map(|line| line.parse::<Point<u64, 2>>().unwrap())
        .collect())
}

fn largest_rectangle_area(red_tiles: &Vec<Point<u64, 2>>) -> u64 {
    let mut biggest = 0;
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            biggest = max(biggest, rectangle_area(&red_tiles[i], &red_tiles[j]))
        }
    }
    biggest
}

fn rectangle_area(p1: &Point<u64, 2>, p2: &Point<u64, 2>) -> u64 {
    p1.values()
        .zip(p2.values())
        .map(|(v1, v2)| 1 + sub_abs(v1, v2))
        .product()
}
