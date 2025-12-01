use advent2025::{advent_main, all_lines, Part};
use anyhow::bail;
use bare_metal_modulo::ModNumC;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        match part {
            Part::One => {
                let mut position = ModNumC::<i16, 100>::new(50);
                let mut zeros = 0;
                for line in all_lines(filename).unwrap() {
                    position += parse_line(line.as_str())?;
                    if position == 0 {
                        zeros += 1;
                    }
                }
                println!("{zeros}");
                Ok(())
            }
            Part::Two => {
                todo!("No part 2 yet")
            }
        }
    })
}

fn parse_line(line: &str) -> anyhow::Result<i16> {
    let num = line[1..].parse::<i16>()?;
    match &line[0..1] {
        "L" => Ok(-num),
        "R" => Ok(num),
        _ => bail!("Unrecognized")
    }
}