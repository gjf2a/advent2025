use advent2025::{Part, advent_main, all_lines};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| match part {
        Part::One => {
            let v = all_lines(filename)?.map(|line| line_voltage(line.as_str())).sum::<u32>();
            println!("{v}");
            Ok(())
        }
        Part::Two => {
            todo!("No part 2 yet")
        }
    })
}

fn line_voltage(nums: &str) -> u32 {
    let mut best = 0;
    let nums = nums.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
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