use std::{collections::HashMap, str::FromStr};

use advent2025::{
    Part, advent_main, all_lines,
    grid::GridWorld,
    multidim::{Position, map_width_height},
};
use anyhow::bail;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let (world, ops) = match part {
            Part::One => to_map(filename)?,
            Part::Two => to_wacky_map(filename)?,
        };
        let total = (0..world.width())
            .map(|x| ops[x].compute_column(&world, x))
            .sum::<u64>();
        println!("{total}");
        Ok(())
    })
}

#[derive(Copy, Clone, Debug)]
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

    fn identity(&self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
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
                    nums.insert(Position::from_usize(x, y), n);
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

fn to_wacky_map(filename: &str) -> anyhow::Result<(GridWorld<u64>, Vec<Op>)> {
    let mut rows = all_lines(filename)?.collect_vec();
    let op_row = rows.pop().unwrap();
    let op_starts_widths = op_starts_widths(op_row.as_str());
    let mut nums = HashMap::new();
    for (x, (_, column_start, column_width)) in op_starts_widths.iter().enumerate() {
        for y in 0..*column_width {
            let mut total = 0;
            for digit in 0..rows.len() {
                let digit_column = column_start + y;
                let column_value = rows[digit][digit_column..digit_column + 1]
                    .parse::<u64>()
                    .map(|v| v)
                    .unwrap_or(0);
                if column_value > 0 {
                    total = (total * 10) + column_value;
                }
            }
            let p = Position::from_usize(x, y);
            nums.insert(p, total);
        }
    }

    let (width, height) = map_width_height(&nums);
    for x in 0..width {
        let default = op_starts_widths[x].0.identity();
        for y in 0..height {
            let p = Position::from_usize(x, y);
            if !nums.contains_key(&p) {
                nums.insert(p, default);
            }
        }
    }

    let world = GridWorld::from_map(&nums);
    assert_eq!(world.width(), op_starts_widths.len());
    Ok((
        world,
        op_starts_widths.iter().map(|(op, _, _)| *op).collect(),
    ))
}

fn op_starts_widths(op_row: &str) -> Vec<(Op, usize, usize)> {
    let op_indices = op_row
        .char_indices()
        .filter(|(_, c)| *c != ' ')
        .collect_vec();
    let mut result = vec![];
    for i in 0..op_indices.len() {
        let si = op_indices[i].0;
        let op = op_row[si..si + 1].parse::<Op>().unwrap();
        let next = if i + 1 < op_indices.len() {
            op_indices[i + 1].0
        } else {
            op_row.len() + 1
        };
        result.push((op, si, next - si - 1));
    }
    result
}
