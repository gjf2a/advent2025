use std::cmp::{max, min};

use advent2025::{Part, advent_main, all_lines, multidim::Point, sub_abs};
use itertools::Itertools;

type Corner = Point<u64, 2>;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let red_tiles = get_red_tiles(filename)?;
        let rects = all_rectangles(&red_tiles);
        match part {
            Part::One => {
                println!("{}", largest_rectangle_area(&rects));
            }
            Part::Two => {
                let edges = RedEdges::new(&red_tiles);
                let approved = rects
                    .iter()
                    .filter(|(p1, p2)| edges.rectangle_approved(p1, p2))
                    .copied()
                    .collect_vec();
                if options.contains(&"-approved") {
                    println!("{approved:?}");
                }
                if options.contains(&"-approved-count") {
                    println!("approved {}/{}", approved.len(), rects.len());
                }
                println!("{}", largest_rectangle_area(&approved));
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

// Part 2 ideas

// Possible idea:
// * Is a Corner green?
//   * Examine its y coordinate. Is there an outer edge with that y?
//     * If so, what is the lowest x coordinate?
//       * March across the y row starting at the low x, finding another x at that
//         y that is on an edge. Is our Corner's x between them? If so, great!
//         If not, keep looking at this row for another edge to restart the process.

struct RedEdges {
    edges: Vec<LevelInterval>,
}

impl RedEdges {
    fn new(red_tiles: &Vec<Corner>) -> Self {
        let mut edges = vec![];
        for i in 0..red_tiles.len() - 1 {
            let p1 = red_tiles[i];
            let p2 = red_tiles[i + 1];
            if p1[0] == p2[0] {
                edges.push(LevelInterval {
                    level: p1[0],
                    start: min(p1[1], p2[1]),
                    end: max(p1[1], p2[1]),
                    tilt: Tilt::Vertical,
                });
            } else if p1[1] == p2[1] {
                edges.push(LevelInterval {
                    level: p1[1],
                    start: min(p1[0], p2[0]),
                    end: max(p1[0], p2[0]),
                    tilt: Tilt::Horizontal,
                });
            } else {
                panic!("Invalid red-tiles list!");
            }
        }
        Self { edges }
    }

    fn rectangle_approved(&self, p1: &Corner, p2: &Corner) -> bool {
        let p12 = Point::new([p1[0], p2[1]]);
        let p21 = Point::new([p1[1], p2[0]]);
        self.is_green(&p12) && self.is_green(&p21)
    }

    fn is_green(&self, corner: &Corner) -> bool {
        let mut inside = false;
        //println!("corner: {corner}");
        for edge in self.edges_at(corner[1]) {
            //println!("edge {edge:?}");
            match edge.tilt {
                Tilt::Horizontal => {
                    inside = true;
                    if edge.contains(&corner[0]) {
                        return true;
                    }
                }
                Tilt::Vertical => {
                    if inside {
                        if corner[0] <= edge.level {
                            return true;
                        }
                        inside = false;
                    } else {
                        if corner[0] == edge.level {
                            return true;
                        } else if corner[0] < edge.level {
                            //println!("In here: {corner}");
                            return false;
                        }
                        inside = true;
                    }
                }
            }
        }
        //println!("end: {corner}");
        inside
    }

    fn edges_at(&self, y: u64) -> impl Iterator<Item = LevelInterval> {
        self.edges
            .iter()
            .filter(move |edge| match edge.tilt {
                Tilt::Horizontal => y == edge.level,
                Tilt::Vertical => edge.start <= y && y <= edge.end,
            })
            .copied()
            .sorted()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tilt {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord)]
struct LevelInterval {
    level: u64,
    start: u64,
    end: u64,
    tilt: Tilt,
}

impl LevelInterval {
    fn contains(&self, v: &u64) -> bool {
        self.start <= *v && *v <= self.end
    }
}

impl PartialOrd for LevelInterval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.tilt {
            Tilt::Horizontal => match other.tilt {
                Tilt::Horizontal => self.start.partial_cmp(&other.start),
                Tilt::Vertical => self.start.partial_cmp(&other.level),
            },
            Tilt::Vertical => match other.tilt {
                Tilt::Horizontal => self.level.partial_cmp(&other.start),
                Tilt::Vertical => self.level.partial_cmp(&other.level),
            },
        }
    }
}
