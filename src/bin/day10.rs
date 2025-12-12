use std::{cmp::max, fmt::Display, ops::BitXor, str::FromStr};

use advent2025::{Part, advent_main, all_lines, search_iter::BfsIter};
use anyhow::bail;
use itertools::Itertools;
use z3::{Optimize, SatResult, ast::Int};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let machines = all_lines(filename)?
            .map(|line| line.parse::<MachineSpec>().unwrap())
            .collect_vec();
        MachineSpec::assert_valid(&machines, filename)?;
        let score = machines
            .iter()
            .map(|m| match part {
                Part::One => m.min_button_presses_indicator_lights(),
                Part::Two => m.min_button_presses_joltage(),
            })
            .sum::<u64>();
        println!("{score}");
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
    fn min_button_presses_indicator_lights(&self) -> u64 {
        let mut iter = BfsIter::new(Bits::default(), |s| self.successors_indicator_lights(s));
        let found = iter.by_ref().find(|b| b.bits == self.target.bits).unwrap();
        iter.depth_for(&found) as u64
    }

    fn successors_indicator_lights(&self, bits: &Bits) -> Vec<Bits> {
        self.buttons.iter().map(|button| *bits ^ *button).collect()
    }

    fn min_button_presses_joltage(&self) -> u64 {
        let vars = (0..self.buttons.len())
            .map(|i| Int::fresh_const(format!("n{i}").as_str()))
            .collect_vec();
        let solver = Optimize::new();
        for var in vars.iter() {
            solver.assert(&var.ge(0));
        }
        for i in 0..self.joltages.len() {
            solver.assert(
                &self
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| b.get(i as u16))
                    .map(|(i, _)| &vars[i])
                    .sum::<Int>()
                    .eq(Int::from_u64(self.joltages[i] as u64)),
            );
        }
        solver.minimize(&vars.iter().sum::<Int>());
        match solver.check(&[]) {
            SatResult::Sat => {
                let model = solver.get_model().unwrap();
                vars.iter()
                    .map(|var| model.eval(var, true).unwrap().as_u64().unwrap())
                    .sum()
            }
            _ => panic!("Unsolvable"),
        }
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
            let button_str = button.active_bits().map(|i| format!("{i}")).join(",");
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

    fn active_bits(&self) -> impl Iterator<Item = usize> {
        self.iter().enumerate().filter(|(_, v)| *v).map(|(i, _)| i)
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
