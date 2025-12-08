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
    let mut beams = b_tree_set![s[0] as usize];
    let mut beams_through = HashHistogram::new();
    beams_through.bump(&(s[0] as usize, s[1] as usize));
    for y in start_row..(world.height() - 1) {
        let splitters = find_splitters(world, y, &beams);
        for splitter in splitters.iter() {
            beams.remove(splitter);
            beams.insert(splitter - 1);
            beams.insert(splitter + 1);
        }
        let prev_row_keys = prev_row(&beams_through, y);
        for key in prev_row_keys {
            let x = key.0;
            if splitters.contains(&x) {
                beams_through.bump_by(&(x + 1, y), beams_through.count(&key));
                beams_through.bump_by(&(x - 1, y), beams_through.count(&key));
            } else {
                beams_through.bump_by(&(x, y), beams_through.count(&key));
            }
        }
    }
    final_row_count(world, &beams_through)
}

fn prev_row(
    beams_through: &HashHistogram<(usize, usize), u64>,
    current_row: usize,
) -> Vec<(usize, usize)> {
    beams_through
        .iter()
        .filter(|((_, row), _)| *row + 1 == current_row)
        .map(|(k, _)| *k)
        .collect()
}

fn final_row_count(
    world: &GridCharWorld,
    beams_through: &HashHistogram<(usize, usize), u64>,
) -> u64 {
    (0..world.width())
        .map(|x| beams_through.count(&(x, world.height() - 2)))
        .sum()
}

// Incrementally derived count_many_worlds_splits() as follows:
// * Started with count_beam_splits()
// * Removed split_count
// * Added a list of lists of x positions of trails from the start
// * Whenever you hit a splitter, add a left move to the existing trail,
//   and add a new trail with a right move.
// * When you don't hit a splitter, add the current x position to the end
//   of the current trail.
// * Then add a histogram of counts of trails passing through a specific position.
// * Increment the histogram each time you add an x position to the end of a trail.
// * Print the sum of counts for the final row, to ensure it equals the number of trails.
fn find_many_worlds_trails(world: &GridCharWorld) -> Vec<Vec<usize>> {
    let s = start(world);
    let mut trails = vec![vec![s[0] as usize]];
    let start_row = s[1] as usize;
    let mut beams = b_tree_set![s[0] as usize];
    let mut beams_through = HashHistogram::new();
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
    let ending_counts = final_row_count(world, &beams_through);
    println!("{ending_counts}");
    trails
}
