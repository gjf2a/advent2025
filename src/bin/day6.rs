use std::{collections::HashMap, fmt::Display, str::FromStr};

use advent2025::{Part, advent_main, all_lines, grid::GridWorld, multidim::Position};
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

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sym = match self {
            Self::Add => "+",
            Self::Mul => "*",
        };
        write!(f, "{sym}")
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

fn to_wacky_map(filename: &str) -> anyhow::Result<(GridWorld<u64>, Vec<Op>)> {
    let mut rows = all_lines(filename)?.collect_vec();
    let op_row = rows.pop().unwrap();
    let op_widths = op_widths(op_row.as_str());
    let mut nums = HashMap::new();
    for x in 0..op_widths.len() {
        let column_width = op_widths[x].1;
        for y in 0..column_width {
            let mut total = 0;
            for digit in 0..rows.len() {
                let digit_column = x * column_width + y;
                let column_value = rows[digit][digit_column..digit_column + 1].parse::<u64>().map(|v| v).unwrap_or(0);
                if column_value > 0 {
                    total = (total * 10) + column_value;
                }
            }
            let p = Position::from((x as isize, y as isize));
            nums.insert(p, total);
        }
    }

    let world = GridWorld::from_map(&nums);
    println!("{world:?}");
    assert_eq!(world.width(), op_widths.len());
    Ok((world, op_widths.iter().map(|(op,_)| *op).collect()))
}

fn op_widths(op_row: &str) -> Vec<(Op, usize)> {
    let op_indices = op_row.char_indices().filter(|(_,c)| *c != ' ').collect_vec();
    let mut result = vec![];
    for i in 0..op_indices.len() {
        let si = op_indices[i].0;
        let op = op_row[si..si+1].parse::<Op>().unwrap();
        let next = if i + 1 < op_indices.len() { op_indices[i + 1].0} else {op_row.len()};
        println!("{si} {next}");
        result.push((op, next - si));
    }
    result
}