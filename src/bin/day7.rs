use std::collections::BTreeSet;

use advent2025::{Part, advent_main, grid::GridCharWorld, multidim::Position};
use common_macros::b_tree_set;
use hash_histogram::HashHistogram;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let world = GridCharWorld::from_char_file(filename)?;
        match part {
            Part::One => {
                println!("{}", count_beam_splits(&world));
            }
            Part::Two => {
                if options.contains(&"-trails") {
                    let trails = find_many_worlds_trails(&world);
                    println!("{}", trails.len());
                } else {
                    println!("{}", count_many_worlds_splits(&world));
                }
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

fn find_splitters(world: &GridCharWorld, row: usize, beams: &BTreeSet<usize>) -> Vec<usize> {
    beams
        .iter()
        .filter(|x| world.get(**x, row).unwrap() == '^')
        .copied()
        .collect()
}

fn count_beam_splits(world: &GridCharWorld) -> u64 {
    let s = start(world);
    let start_row = s[1] as usize;
    let mut beams = b_tree_set![s[0] as usize];
    let mut split_count = 0;
    for y in start_row..(world.height() - 1) {
        for splitter in find_splitters(world, y, &beams) {
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

fn find_many_worlds_trails(world: &GridCharWorld) -> Vec<Vec<usize>> {
    let s = start(world);
    let mut trails = vec![vec![s[0] as usize]];
    let start_row = s[1] as usize;
    let mut beams = b_tree_set![s[0] as usize];
    for y in start_row..(world.height() - 1) {
        let splitters = find_splitters(world, y, &beams);
        for splitter in splitters.iter() {
            beams.remove(splitter);
            beams.insert(splitter - 1);
            beams.insert(splitter + 1);
        }
        let mut new_trails = vec![];
        for trail in trails.iter_mut() {
            let end = trail[trail.len() - 1];
            if splitters.contains(&end) {
                new_trails.push(trail.clone());
                let pushed = new_trails.len() - 1;
                new_trails[pushed].push(end + 1);
                trail.push(end - 1);
            } else {
                trail.push(end);
            }
        }
        for new_trail in new_trails {
            trails.push(new_trail);
        }
    }
    trails
}

fn count_many_worlds_splits(world: &GridCharWorld) -> u64 {
    let s = start(world);
    let start_row = s[1] as usize;
    let mut beam_counts: HashHistogram<usize, u64> = HashHistogram::new();
    beam_counts.bump(&(s[0] as usize));
    let mut active_beams = b_tree_set![s[0] as usize];
    for y in start_row..(world.height() - 1) {
        for splitter in find_splitters(world, y, &active_beams) {
            active_beams.remove(&splitter);
            active_beams.insert(splitter - 1);
            beam_counts.bump(&(splitter - 1));
            active_beams.insert(splitter + 1);
            beam_counts.bump(&(splitter + 1))
        }
    }
    beam_counts.total_count()
}
