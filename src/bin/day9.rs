use std::{cmp::{max, min}, collections::HashSet};

use advent2025::{Part, advent_main, all_lines, multidim::Point, sub_abs};
use itertools::Itertools;

type Corner = Point<u64, 2>;
type Edge = (Corner, Corner);

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let red_tiles = get_red_tiles(filename)?;
        match part {
            Part::One => {
                println!("{}", largest_rectangle_area(&red_tiles));
            }
            Part::Two => {
                let everyone = all_rectangles(&red_tiles);
                let no_crossings = no_crossing_rectangles(&red_tiles);
                let no_inside = no_inside_rectangles(&red_tiles);
                let neither = no_inside.iter().filter(|r| no_crossings.contains(r)).collect_vec();
                /*for candidate in everyone.iter() {
                    println!("{candidate:?} no crossings? {} no inside? {}", no_crossings.contains(candidate), no_inside.contains(candidate));
                }*/
                println!("Everyone: {}", everyone.len());
                println!("No crossings: {}", no_crossings.len());
                println!("No inside: {}", no_inside.len());
                println!("Neither: {}", neither.len());
                for candidate in neither.iter() {
                    println!("{candidate:?} {}", rectangle_area(&candidate.0, &candidate.1));
                }
                let largest_available = neither.iter().map(|r| rectangle_area(&r.0, &r.1)).max().unwrap();
                println!("{largest_available}");
            }
        }
        Ok(())
    })
}

fn get_red_tiles(filename: &str) -> anyhow::Result<Vec<Point<u64, 2>>> {
    Ok(all_lines(filename)?
        .map(|line| line.parse::<Corner>().unwrap())
        .collect())
}

fn largest_rectangle_area(red_tiles: &Vec<Corner>) -> u64 {
    let mut biggest = 0;
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            biggest = max(biggest, rectangle_area(&red_tiles[i], &red_tiles[j]))
        }
    }
    biggest
}

fn rectangle_area(p1: &Corner, p2: &Corner) -> u64 {
    p1.values()
        .zip(p2.values())
        .map(|(v1, v2)| 1 + sub_abs(v1, v2))
        .product()
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

fn no_crossing_rectangles(red_tiles: &Vec<Corner>) -> HashSet<(Corner, Corner)> {
    let mut result = HashSet::new();
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let edges = rectangle_edges(&red_tiles[i], &red_tiles[j]);
            let mut count = 0;
            for edge in edges.iter() {
                count += crossings(edge, red_tiles).count();
            }

            if count == 0 {
                result.insert((red_tiles[i], red_tiles[j]));
            }
        }
    }
    result
}

fn no_inside_rectangles(red_tiles: &Vec<Corner>) -> Vec<(Corner, Corner)> {
    let mut result = vec![];
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let rect = (red_tiles[i], red_tiles[j]);
            if !red_tiles.iter().any(|p| contains(&rect, p)) {
                result.push(rect);
            }
        }
    }
    result
}

fn rectangle_edges(p1: &Corner, p2: &Corner) -> [Edge; 4] {
    let p12 = Corner::new([p1[0], p2[1]]);
    let p21 = Corner::new([p2[0], p1[1]]);
    [(*p1, p12), (p12, *p2), (*p2, p21), (p21, *p1)]
}

fn contains(rect: &(Corner, Corner), p: &Corner) -> bool {
    let min_x = min(rect.0[0], rect.1[0]);
    let max_x = max(rect.0[0], rect.1[0]);
    let min_y = min(rect.0[1], rect.1[1]);
    let max_y = max(rect.0[1], rect.1[1]);
    min_x < p[0] && p[0] < max_x && min_y < p[1] && p[1] < max_y
}

fn crossings(edge: &Edge, red_tiles: &Vec<Corner>) -> impl Iterator<Item = Edge> {
    (0..red_tiles.len() - 1)
        .map(|i| (red_tiles[i], red_tiles[i + 1]))
        .filter(|target| crosses(edge, target))
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Segment {
    Horizontal { y: u64, x_start: u64, x_end: u64 },
    Vertical { x: u64, y_start: u64, y_end: u64 },
}

impl Segment {
    fn of(edge: &Edge) -> Option<Self> {
        if edge.0 == edge.1 {
            None
        } else if edge.0[0] == edge.1[0] {
            Some(Self::Vertical {
                x: edge.0[0],
                y_start: min(edge.0[1], edge.1[1]),
                y_end: max(edge.0[1], edge.1[1]),
            })
        } else if edge.0[1] == edge.1[1] {
            Some(Self::Horizontal {
                y: edge.0[1],
                x_start: min(edge.0[0], edge.1[0]),
                x_end: max(edge.0[0], edge.1[0]),
            })
        } else {
            None
        }
    }

    fn perpendicular(&self, other: &Self) -> bool {
        match self {
            Segment::Horizontal {
                y: _,
                x_start: _,
                x_end: _,
            } => match other {
                Segment::Horizontal {
                    y: _,
                    x_start: _,
                    x_end: _,
                } => false,
                Segment::Vertical {
                    x: _,
                    y_start: _,
                    y_end: _,
                } => true,
            },
            Segment::Vertical {
                x: _,
                y_start: _,
                y_end: _,
            } => match other {
                Segment::Horizontal {
                    y: _,
                    x_start: _,
                    x_end: _,
                } => true,
                Segment::Vertical {
                    x: _,
                    y_start: _,
                    y_end: _,
                } => false,
            },
        }
    }

    fn crosses(&self, other: &Self) -> bool {
        match self {
            Segment::Horizontal { y, x_start, x_end } => match other {
                Segment::Horizontal {
                    y: _,
                    x_start: _,
                    x_end: _,
                } => false,
                Segment::Vertical { x, y_start, y_end } => {
                    x_start < x && x < x_end && y_start < y && y < y_end
                }
            },
            Segment::Vertical { x, y_start, y_end } => match other {
                Segment::Horizontal { y, x_start, x_end } => {
                    x_start < x && x < x_end && y_start < y && y < y_end
                }
                Segment::Vertical {
                    x: _,
                    y_start: _,
                    y_end: _,
                } => false,
            },
        }
    }
}

fn crosses(source: &Edge, target: &Edge) -> bool {
    if let Some(src) = Segment::of(source) {
        if let Some(targ) = Segment::of(target) {
            return src.crosses(&targ);
        }
    }
    false
}

fn is_green_tile_edge(red_tiles: &Vec<Corner>, candidate: &Corner) -> bool {
    (0..red_tiles.len() - 1)
        .map(|i| (red_tiles[i], red_tiles[i + 1]))
        .any(|(p1, p2)| between(&p1, &p2, candidate))
}

fn between(p1: &Corner, p2: &Corner, candidate: &Corner) -> bool {
    if p1[0] == p2[0] {
        candidate[1] >= p1[1] && candidate[1] <= p2[1]
            || candidate[1] >= p2[1] && candidate[1] <= p1[1]
    } else if p1[1] == p2[1] {
        candidate[0] >= p1[0] && candidate[0] <= p2[0]
            || candidate[0] >= p2[0] && candidate[1] <= p1[0]
    } else {
        false
    }
}
