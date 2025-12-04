use advent2025::{
    Part, advent_main,
    grid::GridCharWorld,
    multidim::{Dir, DirType, Position},
};
use enum_iterator::all;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        match part {
            Part::One => println!("{}", removable_rolls(&world).count()),
            Part::Two => {
                let mut world = world;
                let mut removed = 0;
                loop {
                    let removable = removable_rolls(&world).collect_vec();
                    if removable.len() == 0 {
                        break;
                    } else {
                        for p in removable {
                            world.update(p, '.');
                            removed += 1;
                        }
                    }
                }
                println!("{removed}");
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
