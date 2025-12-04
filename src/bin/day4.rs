use advent2025::{
    Part, advent_main,
    grid::GridCharWorld,
    multidim::{Dir, DirType},
};
use enum_iterator::all;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        let part1 = world
            .position_value_iter()
            .filter(|(p, v)| {
                **v == '@'
                    && all::<Dir>()
                        .filter(|d| world.value(d.neighbor(**p)).map_or(false, |n| n == '@'))
                        .count()
                        < 4
            })
            .count();
        println!("{part1}");
        Ok(())
    })
}
