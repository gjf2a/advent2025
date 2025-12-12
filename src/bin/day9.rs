use std::cmp::{max, min};

use advent2025::{Part, advent_main, all_lines, multidim::Point, sub_abs};

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
        let red1 = red_tiles[i];
        for j in i + 1..red_tiles.len() {
            let red2 = red_tiles[j];
            let others = [(red1[0], red2[1]), (red2[0], red1[1])];
            if others.iter().all(|(x, y)| {
                let candidate = Corner::new([*x, *y]);
                along_some_edge(red_tiles, &candidate)
            }) {
                result.push((red_tiles[i], red_tiles[j]));
            }
        }
    }
    result
}

fn along_some_edge(red_tiles: &Vec<Corner>, candidate: &Corner) -> bool {
    (0..red_tiles.len()).any(|i| along_edge(&red_tiles[i], &red_tiles[(i + 1) % red_tiles.len()], candidate))
}

fn along_edge(p1: &Corner, p2: &Corner, candidate: &Corner) -> bool {
    p1[0] == p2[0] && candidate[0] == p1[0] && min(p1[1], p2[1]) <= candidate[1] && candidate[1] <= max(p1[1], p2[1])
    || p1[1] == p2[1] && candidate[1] == p1[1] && min(p1[0], p2[0]) <= candidate[0] && candidate[0] <= max(p1[0], p2[0])
}