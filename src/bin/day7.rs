use advent2025::{Part, advent_main, grid::GridCharWorld, multidim::Position};
use common_macros::b_tree_set;
use hash_histogram::HashHistogram;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, _| {
        let world = GridCharWorld::from_char_file(filename)?;
        match part {
            Part::One => {
                println!("{}", count_beam_splits(&world));
            }
            Part::Two => {
                println!("{}", count_many_worlds_splits(&world));
            }
        }
        Ok(())
    })
}

fn start(world: &GridCharWorld) -> Position {
    world
        .position_value_iter()
        .find(|(_, c)| *c == 'S')
        .map(|(p, _)| p)
        .unwrap()
}

fn count_beam_splits(world: &GridCharWorld) -> u64 {
    let s = start(world);
    let start_row = s[1] as usize;
    let mut beams = b_tree_set![s[0] as usize];
    let mut split_count = 0;
    for y in start_row..(world.height() - 1) {
        let splitters = beams
            .iter()
            .filter(|x| world.get(**x, y).unwrap() == '^')
            .copied()
            .collect_vec();
        for splitter in splitters {
            assert!(splitter > 0);
            assert!(splitter + 1 < world.width());
            split_count += 1;
            beams.remove(&splitter);
            beams.insert(splitter - 1);
            beams.insert(splitter + 1);
        }
    }
    split_count
}

fn count_many_worlds_splits(world: &GridCharWorld) -> u64 {
    let s = start(world);
    let start_row = s[1] as usize;
    let mut beam_counts: HashHistogram<usize, u64> = HashHistogram::new();
    beam_counts.bump(&(s[0] as usize));
    let mut active_beams = b_tree_set![s[0] as usize];
    for y in start_row..(world.height() - 1) {
        let splitters = active_beams
            .iter()
            .filter(|x| world.get(**x, y).unwrap() == '^')
            .copied()
            .collect_vec();
        for splitter in splitters {
            assert!(splitter > 0);
            assert!(splitter + 1 < world.width());
            active_beams.remove(&splitter);
            active_beams.insert(splitter - 1);
            beam_counts.bump(&(splitter - 1));
            active_beams.insert(splitter + 1);
            beam_counts.bump(&(splitter + 1))
        }
    }
    beam_counts.total_count()
}