use std::{cmp::max, fmt::Display, str::FromStr};

use advent2025::{Part, advent_main, all_lines};
use anyhow::bail;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let machines = all_lines(filename)?
            .map(|line| line.parse::<Machine>().unwrap())
            .collect_vec();
        for machine in machines.iter() {
            println!("{machine}");
        }
        match part {
            Part::One => {
                todo!("No part 1 yet")
            }
            Part::Two => {
                todo!("No part 2 yet")
            }
        }
        Ok(())
    })
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
struct Bits {
    bits: u16,
    num_bits: u16,
}

impl Bits {
    fn set(&mut self, bit: u16) {
        self.bits |= 1 << bit;
        self.num_bits = max(self.num_bits, bit + 1);
    }

    fn clear(&mut self, bit: u16) {
        self.bits &= !(1 << bit);
        self.num_bits = max(self.num_bits, bit + 1);
    }

    fn add(&mut self, value: bool) {
        if value {
            self.set(self.num_bits);
        } else {
            self.clear(self.num_bits);
        }
    }

    fn get(&self, bit: u16) -> bool {
        self.bits & 1 << bit > 0
    }

    fn iter(&self) -> BitIterator {
        BitIterator::new(*self)
    }
}

impl FromStr for Bits {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Bits::default();
        let inner = &s[1..s.len() - 1];
        match &s[0..1] {
            "[" => {
                for c in inner.chars() {
                    result.add(match c {
                        '#' => true,
                        '.' => false,
                        _ => bail!("Unrecognized token: {c}"),
                    });
                }
            }
            "(" => {
                for c in inner.split(',') {
                    result.set(c.parse::<u16>()?);
                }
            }
            _ => bail!("Unrecognized token: {s}"),
        }
        Ok(result)
    }
}

struct BitIterator {
    bits: Bits,
    i: u16,
}

impl BitIterator {
    fn new(bits: Bits) -> Self {
        Self { bits, i: 0 }
    }
}

impl Iterator for BitIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.bits.num_bits {
            None
        } else {
            let result = self.bits.get(self.i);
            self.i += 1;
            Some(result)
        }
    }
}

#[derive(Default)]
struct Machine {
    target: Bits,
    buttons: Vec<Bits>,
    joltages: Vec<u64>,
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Machine::default();
        for part in s.split_whitespace() {
            match &part[0..1] {
                "[" => {
                    result.target = part.parse::<Bits>()?;
                }
                "{" => {
                    let inner = &part[1..part.len() - 1];
                    result.joltages = inner
                        .split(',')
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect();
                }
                "(" => {
                    result.buttons.push(part.parse::<Bits>()?);
                }
                _ => bail!("Unrecognized token: {part}"),
            }
        }
        Ok(result)
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let target = self
            .target
            .iter()
            .map(|v| if v { '#' } else { '.' })
            .collect::<String>();
        write!(f, "[{target}]")?;
        for button in self.buttons.iter() {
            let button_str = button
                .iter()
                .enumerate()
                .filter(|(_, v)| *v)
                .map(|(i, _)| format!("{i}"))
                .join(",");
            write!(f, " ({button_str})")?;
        }
        let joltage_str = self.joltages.iter().join(",");
        write!(f, " {{{joltage_str}}}")
    }
}
