use std::{fmt::Display, str::FromStr};

use advent2025::{Part, advent_main, all_lines};
use anyhow::bail;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mut lines = all_lines(filename)?;
        let ranges = InclusiveRange::from(lines.by_ref());
        match part {
            Part::One => {
                let fresh = lines
                    .skip_while(|line| line.len() == 0)
                    .map(|line| line.parse::<u64>().unwrap())
                    .filter(|ing| ranges.iter().any(|r| r.contains(*ing)))
                    .count();
                println!("{fresh}");
            }
            Part::Two => {
                let mut nonoverlapping = vec![];
                for i in 0..ranges.len() - 1 {
                    let mut candidates = vec![ranges[i]];
                    for j in i+1..ranges.len() {
                        let mut new_candidates = vec![];
                        for c in 0..candidates.len() {
                            for isplit in candidates[c].without_overlap(&ranges[j]) {
                                new_candidates.push(isplit);
                            }
                        }
                        std::mem::swap(&mut new_candidates, &mut candidates);
                    }
                    for c in candidates {
                        nonoverlapping.push(c);
                    }
                }
                nonoverlapping.push(ranges[ranges.len() - 1]);
                for n in nonoverlapping {
                    println!("{n}");
                }
                todo!("No part 2 yet")
            }
        }
        Ok(())
    })
}

#[derive(Copy, Clone)]
struct InclusiveRange {
    start: u64,
    end: u64,
}

impl InclusiveRange {
    fn from<I: Iterator<Item = String>>(lines: &mut I) -> Vec<Self> {
        lines
            .take_while(|line| line.len() > 0)
            .map(|line| line.parse::<InclusiveRange>().unwrap())
            .collect()
    }

    fn span(&self) -> u64 {
        self.end - self.start + 1
    }

    fn without_overlap(&self, other: &Self) -> Vec<Self> {
        let second = Self {start: other.end + 1, end: self.end};
        if other.start <= self.start {
            if other.end < self.start {
                vec![*self]
            } else if other.end >= self.end {
                vec![]
            } else {
                vec![second]
            }
        } else {
            let first = Self {start: self.start, end: other.start - 1};
            if self.end < other.start {
                vec![*self]
            } else if other.end >= self.end {
                vec![first]
            } else {
                vec![first, second]
            }
        }
    }

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

impl Display for InclusiveRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}