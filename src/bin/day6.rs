use std::{collections::HashMap, str::FromStr};

use advent2025::{Part, advent_main, all_lines, grid::GridWorld, multidim::Position};
use anyhow::bail;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let (world, ops) = to_map(filename)?;
        match part {
            Part::One => {
                let total = (0..world.width())
                    .map(|x| ops[x].compute_column(&world, x))
                    .sum::<u64>();
                println!("{total}");
            }
            Part::Two => {
                todo!("No part 2 yet")
            }
        }
        Ok(())
    })
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn compute_column(&self, world: &GridWorld<u64>, column: usize) -> u64 {
        (0..world.height())
            .map(|y| world.get(column, y).unwrap())
            .reduce(|a, b| match self {
                Self::Add => a + b,
                Self::Mul => a * b,
            })
            .unwrap()
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => bail!("Did not recognize '{s}'"),
        }
    }
}

fn to_map(filename: &str) -> anyhow::Result<(GridWorld<u64>, Vec<Op>)> {
    let mut nums = HashMap::new();
    let mut ops = vec![];
    for (y, row) in all_lines(filename)?.enumerate() {
        for (x, entry) in row.split_whitespace().enumerate() {
            match entry.parse::<u64>() {
                Ok(n) => {
                    nums.insert(Position::from((x as isize, y as isize)), n);
                }
                Err(_) => {
                    ops.push(entry.parse::<Op>()?);
                }
            }
        }
    }
    let world = GridWorld::from_map(&nums);
    assert_eq!(world.width(), ops.len());
    Ok((world, ops))
}
