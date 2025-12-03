use std::{cmp::max, collections::HashMap};

use advent2025::{Part, advent_main, all_lines};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let num_digits = match part {
            Part::One => 2,
            Part::Two => 12,
        };
        let v = all_lines(filename)?
            .map(|line| {
                let mut table = MemoizedLineJoltage::default();
                table
                    .line_joltage(&str2nums(line.as_str()), num_digits)
                    .unwrap()
            })
            .sum::<u64>();
        println!("{v}");
        Ok(())
    })
}

fn str2nums(line: &str) -> Vec<u64> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

#[derive(Default)]
struct MemoizedLineJoltage {
    table: HashMap<(usize, usize), u64>,
}

impl MemoizedLineJoltage {
    fn line_joltage(&mut self, nums: &Vec<u64>, digits: usize) -> Option<u64> {
        self.line_joltage_recursive(nums, 0, digits)
    }

    fn line_joltage_recursive(
        &mut self,
        nums: &Vec<u64>,
        start: usize,
        digits: usize,
    ) -> Option<u64> {
        if self.table.get(&(start, digits)).is_none() {
            if digits == 0 {
                self.table.insert((start, digits), 0);
            } else if nums.len() >= digits + start {
                let with = self
                    .line_joltage_recursive(nums, start + 1, digits - 1)
                    .map(|r| r + nums[start] * 10_u64.pow(digits as u32 - 1));
                let without = self.line_joltage_recursive(nums, start + 1, digits);
                match (with, without) {
                    (Some(with), Some(without)) => {
                        self.table.insert((start, digits), max(with, without));
                    }
                    (Some(with), None) => {
                        self.table.insert((start, digits), with);
                    }
                    (None, Some(without)) => {
                        self.table.insert((start, digits), without);
                    }
                    _ => {}
                }
            }
        }
        self.table.get(&(start, digits)).copied()
    }
}

fn line_joltage_recursive(nums: &Vec<u64>, start: usize, digits: usize) -> Option<u64> {
    if digits == 0 {
        Some(0)
    } else if nums.len() < digits + start {
        None
    } else {
        let with = line_joltage_recursive(nums, start + 1, digits - 1)
            .map(|r| r + nums[start] * 10_u64.pow(digits as u32 - 1));
        let without = line_joltage_recursive(nums, start + 1, digits);
        if let Some(with) = with {
            Some(if let Some(without) = without {
                max(with, without)
            } else {
                with
            })
        } else {
            without
        }
    }
}

fn line_joltage(nums: &Vec<u64>) -> u64 {
    let mut best = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let value = nums[i] * 10 + nums[j];
            if value > best {
                best = value;
            }
        }
    }
    best
}
