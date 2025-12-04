use advent2025::{
    Part, advent_main,
    grid::GridCharWorld,
    multidim::{Dir, DirType, Position},
};
use enum_iterator::all;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        match part {
            Part::One => println!("{}", removable_rolls(&world).count()),
            Part::Two => {
                todo!()
            }
        }
        Ok(())
    })
}

fn removable_rolls(world: &GridCharWorld) -> impl Iterator<Item = Position> {
    world
        .position_value_iter()
        .filter(|(p, v)| **v == '@' && is_removable(*p, world))
        .map(|(p, _)| *p)
}

fn is_removable(p: &Position, world: &GridCharWorld) -> bool {
    all::<Dir>()
        .filter(|d| world.value(d.neighbor(*p)).map_or(false, |n| n == '@'))
        .count()
        < 4
}
