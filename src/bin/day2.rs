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
        let total = ranges
            .iter()
            .map(|r| r.invalid_ids(part).iter().sum::<u64>())
            .sum::<u64>();
        println!("{total}");
        Ok(())
    })
}

fn invalid_part_1(id_num: u64) -> bool {
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

fn invalid_part_2(id_num: u64) -> bool {
    let id_str = format!("{id_num}");
    (1..=id_str.len() / 2).any(|prefix_size| has_repeating_prefix(id_str.as_str(), prefix_size))
}

fn has_repeating_prefix(s: &str, prefix_size: usize) -> bool {
    if s.len() % prefix_size != 0 {
        return false;
    }
    let prefix = &s[..prefix_size];
    let rest = &s[prefix_size..];
    (0..rest.len())
        .step_by(prefix.len())
        .all(|start| prefix == &rest[start..start + prefix.len()])
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

    fn invalid_ids(&self, part: Part) -> Vec<u64> {
        (self.start..=self.end)
            .filter(|n| match part {
                Part::One => invalid_part_1(*n),
                Part::Two => invalid_part_2(*n),
            })
            .collect()
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use crate::{has_repeating_prefix, invalid_part_1, invalid_part_2};

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
            (3435, true),
        ] {
            assert!(invalid_part_1(id_num) != valid);
        }
    }

    #[test]
    fn test_invalid_2() {
        for (s, outcome) in [
            (12, false),
            (11, true),
            (111, true),
            (12121, false),
            (121212, true),
            (12312, false),
            (123124, false),
            (123123, true),
            (123123123, true),
            (123123124, false),
            (123223123, false),
            (123123223, false),
        ] {
            assert!(invalid_part_2(s) == outcome);
        }
    }

    #[test]
    fn test_repeating_prefix() {
        for (s, p, outcome) in [("11", 1, true)] {
            assert_eq!(has_repeating_prefix(s, p), outcome);
        }
    }
}
