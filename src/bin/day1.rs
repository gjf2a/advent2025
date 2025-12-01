use advent2025::{Part, advent_main, all_lines};
use anyhow::bail;
use bare_metal_modulo::ModNumC;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mut position = ModNumC::<i16, 100>::new(50);
        let mut zeros = 0;
        for line in all_lines(filename).unwrap() {
            match part {
                Part::One => {
                    position += parse_line(line.as_str())?;
                    if position == 0 {
                        zeros += 1;
                    }
                }
                Part::Two => {
                    let instruction = parse_line(line.as_str())?;
                    let update = if instruction < 0 { -1 } else { 1 };
                    for _ in 0..instruction.abs() {
                        position += update;
                        if position == 0 {
                            zeros += 1;
                        }
                    }
                }
            }
        }
        println!("{zeros}");
        Ok(())
    })
}

fn parse_line(line: &str) -> anyhow::Result<i16> {
    let num = line[1..].parse::<i16>()?;
    match &line[0..1] {
        "L" => Ok(-num),
        "R" => Ok(num),
        _ => bail!("Unrecognized"),
    }
}
