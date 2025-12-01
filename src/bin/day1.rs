use advent2025::{Part, advent_main, all_lines};
use anyhow::bail;
use bare_metal_modulo::ModNumC;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let mut position = ModNumC::<i16, 100>::new(50);
        let mut zeros = 0;
        for line in all_lines(filename).unwrap() {
            let prev = position;
            let rotation = parse_line(line.as_str())?;
            position += rotation;
            if position == 0 || part == Part::Two && (rotation < 0 && prev < position || rotation > 0 && prev > position) {
                zeros += 1;
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
