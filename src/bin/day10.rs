use std::{cmp::max, fmt::Display, iter::repeat, ops::BitXor, str::FromStr};

use advent2025::{Part, advent_main, all_lines, search_iter::BfsIter};
use anyhow::bail;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let machines = all_lines(filename)?
            .map(|line| line.parse::<MachineSpec>().unwrap())
            .collect_vec();
        MachineSpec::assert_valid(&machines, filename)?;

        match part {
            Part::One => {
                let score = machines
                    .iter()
                    .map(|m| m.min_button_presses_indicator_lights())
                    .sum::<usize>();
                println!("{score}");
            }
            Part::Two => {
                let score = machines
                    .iter()
                    .map(|m| m.min_button_presses_joltage())
                    .sum::<usize>();
                println!("{score}");
            }
        }
        Ok(())
    })
}

#[derive(Default)]
struct MachineSpec {
    target: Bits,
    buttons: Vec<Bits>,
    joltages: Vec<usize>,
}

impl MachineSpec {
    fn min_button_presses_indicator_lights(&self) -> usize {
        let mut iter = BfsIter::new(Bits::default(), |s| self.successors_indicator_lights(s));
        let found = iter.by_ref().find(|b| b.bits == self.target.bits).unwrap();
        iter.depth_for(&found)
    }

    fn min_button_presses_joltage(&self) -> usize {
        let mut iter = BfsIter::new(
            repeat(0).take(self.target.num_bits as usize).collect(),
            |s| self.successors_joltage(s),
        );
        let found = iter
            .by_ref()
            .filter(|counters| {
                counters
                    .iter()
                    .enumerate()
                    .all(|(i, c)| *c <= self.joltages[i])
            })
            .find(|counters| {
                counters
                    .iter()
                    .enumerate()
                    .all(|(i, c)| *c == self.joltages[i])
            })
            .unwrap();
        iter.depth_for(&found)
    }

    fn successors_indicator_lights(&self, bits: &Bits) -> Vec<Bits> {
        self.buttons.iter().map(|button| *bits ^ *button).collect()
    }

    fn successors_joltage(&self, counters: &Vec<usize>) -> Vec<Vec<usize>> {
        self.buttons
            .iter()
            .map(|button| {
                let mut counters = counters.clone();
                joltage_count(button, &mut counters);
                counters
            })
            .collect()
    }

    fn assert_valid(machines: &Vec<Self>, filename: &str) -> anyhow::Result<()> {
        assert!(
            machines
                .iter()
                .zip(all_lines(filename)?)
                .all(|(machine, line)| format!("{machine}") == *line)
        );
        Ok(())
    }
}

fn joltage_count(button: &Bits, counters: &mut Vec<usize>) {
    for (i, _) in button.iter().enumerate().filter(|(_, v)| *v) {
        counters[i] += 1;
    }
}

impl FromStr for MachineSpec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = MachineSpec::default();
        for part in s.split_whitespace() {
            match &part[0..1] {
                "[" => {
                    result.target = part.parse::<Bits>()?;
                }
                "{" => {
                    let inner = &part[1..part.len() - 1];
                    result.joltages = inner
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
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

impl Display for MachineSpec {
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

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Bits {
    bits: u16,
    num_bits: u16,
}

impl BitXor for Bits {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits,
            num_bits: max(self.num_bits, rhs.num_bits),
        }
    }
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
