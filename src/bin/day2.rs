use std::fmt::Display;

use advent2025::{Part, advent_main, all_lines, log_floor};

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let ranges = Range::from_filename(filename)?;
        if options.contains(&"-range") {
            for range in ranges.iter() {
                println!("{range}: {}", range.span());
            }
        }
        match part {
            Part::One => {
                let total = ranges.iter().map(|r| r.invalid_ids().iter().sum::<u64>()).sum::<u64>();
                println!("{total}");
                Ok(())
            }
            Part::Two => {
                todo!("No part 2 yet")
            }
        }
    })
}

fn is_invalid_id(id_num: u64) -> bool {
    let num_digits = log_floor(id_num, 10) + 1;
    if num_digits % 2 == 0 {
        let mut id_num = id_num;
        let mut multiplier = 1;
        let mut lower_half = 0;
        for _ in 0..num_digits / 2 {
            let digit = id_num % 10;
            id_num /= 10;
            lower_half += digit * multiplier;
            multiplier *= 10;
        }
        lower_half == id_num
    } else {
        false
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn from_filename(filename: &str) -> anyhow::Result<Vec<Self>> {
        Ok(all_lines(filename)?
            .next()
            .unwrap()
            .split(",")
            .map(|s| {
                let mid = s.find("-").unwrap();
                Self {
                    start: s[..mid].parse().unwrap(),
                    end: s[mid + 1..].parse().unwrap(),
                }
            })
            .collect())
    }

    fn span(&self) -> u64 {
        1 + self.end - self.start
    }

    fn invalid_ids(&self) -> Vec<u64> {
        (self.start..=self.end).filter(|n| is_invalid_id(*n)).collect()
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use crate::is_invalid_id;

    #[test]
    fn test_invalid_id() {
        for (id_num, valid) in [
            (5, true),
            (11, false),
            (12, true),
            (19, true),
            (21, true),
            (22, false),
            (333, true),
            (3333, false),
            (3434, false),
            (3435, true)
        ] {
            assert!(is_invalid_id(id_num) != valid);
        }
    }
}