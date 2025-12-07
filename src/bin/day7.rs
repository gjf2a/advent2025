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
    let mut beams_through: HashHistogram<(usize, usize), u64> = HashHistogram::new();
    beams_through.bump(&(s[0] as usize, s[1] as usize));
    for y in start_row..(world.height() - 1) {
        let splitters = find_splitters(world, y, &beams);
        for splitter in splitters.iter() {
            beams.remove(splitter);
            beams.insert(splitter - 1);
            beams.insert(splitter + 1);
        }
        let mut new_trails = vec![];
        for trail in trails.iter_mut() {
            let x = trail[trail.len() - 1];
            if splitters.contains(&x) {
                new_trails.push(trail.clone());
                let pushed = new_trails.len() - 1;
                new_trails[pushed].push(x + 1);
                beams_through.bump(&(x + 1, y));
                trail.push(x - 1);
                beams_through.bump(&(x - 1, y));
            } else {
                trail.push(x);
                beams_through.bump(&(x, y));
            }
        }
        for new_trail in new_trails {
            trails.push(new_trail);
        }
    }
    let ending_counts = (0..world.width()).map(|x| beams_through.count(&(x, world.height() - 2))).sum::<u64>();
    println!("{ending_counts}");
    trails
}

fn count_many_worlds_splits(world: &GridCharWorld) -> u64 {
    let s = start(world);
    let start_row = s[1] as usize;
    let mut beams = b_tree_set![s[0] as usize];
    let mut beams_through: HashHistogram<(usize, usize), u64> = HashHistogram::new();
    beams_through.bump(&(s[0] as usize, s[1] as usize));
    for y in start_row..(world.height() - 1) {
        let splitters = find_splitters(world, y, &beams);
        for splitter in splitters.iter() {
            beams.remove(splitter);
            beams.insert(splitter - 1);
            beams.insert(splitter + 1);
        }
        for key in beams_through.all_labels().iter().filter(|(_, row)| *row + 1 == y) {
            let x = key.0;
            if splitters.contains(&x) {
                beams_through.bump_by(&(x + 1, y), beams_through.count(key));
                beams_through.bump_by(&(x - 1, y), beams_through.count(key));
            } else {
                beams_through.bump_by(&(x, y), beams_through.count(key));
            }
        }
    }
    (0..world.width()).map(|x| beams_through.count(&(x, world.height() - 2))).sum()
}
