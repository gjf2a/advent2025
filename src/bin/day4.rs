use advent2025::{
    Part, advent_main,
    grid::GridCharWorld,
    multidim::{Dir, DirType, Position},
};
use enum_iterator::all;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let mut world = GridCharWorld::from_char_file(filename)?;
        let result = match part {
            Part::One => removable_rolls(&world).count(),
            Part::Two => {
                if options.contains(&"-map") {
                    num_rolls_removed_map(&mut world)
                } else {
                    num_rolls_removed(&mut world)
                }
            }
        };
        println!("{result}");
        Ok(())
    })
}

fn removable_rolls(world: &GridCharWorld) -> impl Iterator<Item = Position> {
    world
        .position_value_iter()
        .filter(|(p, v)| *v == '@' && is_removable(p, world))
        .map(|(p, _)| p)
}

fn is_removable(p: &Position, world: &GridCharWorld) -> bool {
    all::<Dir>()
        .filter(|d| world.value(d.neighbor(*p)).map_or(false, |n| n == '@'))
        .count()
        < 4
}

fn num_rolls_removed(world: &mut GridCharWorld) -> usize {
    let mut removed = 0;
    loop {
        let removable = removable_rolls(&world).collect_vec();
        if removable.len() == 0 {
            return removed;
        } else {
            for p in removable {
                world.update(p, '.');
                removed += 1;
            }
        }
    }
}

fn roll_count(world: &GridCharWorld) -> usize {
    world
        .position_value_iter()
        .filter(|(_, v)| *v == '@')
        .count()
}

fn num_rolls_removed_map(world: &mut GridCharWorld) -> usize {
    let mut removed = 0;
    let mut current_rolls = roll_count(world);
    loop {
        let mut updated = world.map(|p, v| if is_removable(&p, world) { '.' } else { *v });
        let updated_rolls = roll_count(&updated);
        if current_rolls == updated_rolls {
            return removed;
        } else {
            removed += current_rolls - updated_rolls;
            std::mem::swap(&mut updated, world);
            current_rolls = updated_rolls;
        }
    }
}
