use crate::common::parse_chars;
use crate::common::{Coord, CoordMethods, Direction, Lines};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::process::exit;

//#[cfg(debug_assertions)]
//use std::{thread, time};

struct Trail {
    start: Coord,
    ends: HashSet<Coord>,
}

impl Trail {
    const MAX: u8 = 9;
    fn count_score(&mut self, lines: &Vec<Vec<u8>>) -> usize {
        let mut forks: HashMap<u8, HashSet<Coord>> = HashMap::new();
        forks.insert(0, HashSet::from_iter([self.start]));
        for k in 0..=9 as u8 {
            let next_point = k + 1;
            let current_coords = forks.entry(k).or_default().clone();
            for coord in current_coords {
                let mut round = Direction::ccw();
                while let Some(dir) = round() {
                    let diff = dir.next();
                    let next_coord = coord.get_next(diff, lines);
                    if !next_coord.is_none() {
                        let next_coord = next_coord.unwrap();
                        if lines[next_coord.i][next_coord.j] == next_point {
                            if let Some(entry) = forks.get_mut(&next_point) {
                                entry.insert(next_coord);
                            } else {
                                forks.insert(next_point, HashSet::from_iter([next_coord]));
                            }
                        }
                    }
                }
            }
        }
        if let Some(entry) = forks.get_mut(&9) {
            self.ends = entry.clone();
        }
        return self.ends.len();
    }

    fn count_rating(&mut self, lines: &Vec<Vec<u8>>) -> usize {
        return Trail::find_trails(self.start, lines);
    }

    fn find_trails(coord: Coord, lines: &Vec<Vec<u8>>) -> usize {
        let mut trails: usize = 0;
        let current = lines[coord.i][coord.j];
        if current == Trail::MAX {
            return 1;
        }
        let mut round = Direction::ccw();
        while let Some(dir) = round() {
            let diff = dir.next();
            let next_coord = coord.get_next(diff, lines);
            if !next_coord.is_none() {
                let next_coord = next_coord.unwrap();
                if lines[next_coord.i][next_coord.j] == current + 1 {
                    trails += Trail::find_trails(next_coord, lines);
                }
            }
        }
        return trails;
    }
}

enum TrailStat {
    Scores,
    Raitings,
}

fn count_trail_stats(lines: &Lines, variant: TrailStat) -> u64 {
    let mut scores: usize = 0;
    let lines: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| {
            l.iter()
                .map(|c| c.to_digit(10).or(Some(111)).expect("") as u8)
                .collect()
        })
        .collect();
    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            if line[j] != 0 {
                // the trail is not started here
                continue;
            }
            let mut trail = Trail {
                start: Coord { i, j },
                ends: HashSet::new(),
            };
            scores += match variant {
                TrailStat::Scores => trail.count_score(&lines),
                TrailStat::Raitings => trail.count_rating(&lines),
            }
        }
    }
    return scores as u64;
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day2 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let lines = parse_chars(file_name)?;

    println!(
        "trail scores {}",
        count_trail_stats(&lines, TrailStat::Scores)
    );
    println!(
        "trail raitings {}",
        count_trail_stats(&lines, TrailStat::Raitings)
    );

    Ok(())
}
