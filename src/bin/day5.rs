use std::str::FromStr;

use advent2025::{Part, advent_main, all_lines};
use anyhow::bail;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| match part {
        Part::One => {
            let mut lines = all_lines(filename)?;
            let ranges = lines
                .by_ref()
                .take_while(|line| line.len() > 0)
                .map(|line| line.parse::<InclusiveRange>().unwrap())
                .collect_vec();
            let fresh = lines
                .by_ref()
                .skip_while(|line| line.len() == 0)
                .map(|line| line.parse::<u64>().unwrap())
                .filter(|ing| ranges.iter().any(|r| r.contains(*ing)))
                .count();
            println!("{fresh}");
            Ok(())
        }
        Part::Two => {
            todo!("No part 2 yet")
        }
    })
}

struct InclusiveRange {
    start: u64,
    end: u64,
}

impl InclusiveRange {
    fn contains(&self, value: u64) -> bool {
        self.start <= value && value <= self.end
    }
}

impl FromStr for InclusiveRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("-").collect_vec();
        if parts.len() == 2 {
            Ok(Self {
                start: parts[0].parse()?,
                end: parts[1].parse()?,
            })
        } else {
            bail!("No dash")
        }
    }
}
