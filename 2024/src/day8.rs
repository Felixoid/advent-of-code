use crate::day4;
use std::cmp;
use std::collections;
use std::io;
use std::process::exit;

#[cfg(debug_assertions)]
use crate::common;
#[cfg(debug_assertions)]
use std::{thread, time};

type Stations = collections::HashMap<char, Vec<[usize; 2]>>;

fn read_stations(lines: &Vec<Vec<char>>) -> Stations {
    let mut stations: Stations = collections::HashMap::new();
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let st = lines[i][j];
            if st == '.' {
                continue;
            }
            if let Some(coordinates) = stations.get_mut(&st) {
                coordinates.push([i, j]);
            } else {
                stations.insert(st, vec![[i, j]]);
            }
        }
    }
    return stations;
}

fn count_antinodes(stations: &Stations, lines: &Vec<Vec<char>>, with_harmonics: bool) -> u32 {
    let mut antinodes: collections::HashSet<[usize; 2]> = collections::HashSet::new();
    let (max_i, max_j) = (lines.len(), lines[0].len());
    #[cfg(debug_assertions)]
    let mut lines = lines.to_vec();
    for (k, v) in stations {
        _ = k;
        for fst in 0..v.len() {
            for scn in fst + 1..v.len() {
                let (i1, j1, i2, j2) = (v[fst][0], v[fst][1], v[scn][0], v[scn][1]);
                let d_i = i2 as isize - i1 as isize;
                let d_j = j2 as isize - j1 as isize;

                for (d_i, d_j, i, j) in [(-d_i, -d_j, i1, j1), (d_i, d_j, i2, j2)] {
                    let max_step = if with_harmonics {
                        cmp::max(max_i, max_j) as isize
                    } else {
                        1
                    };
                    if with_harmonics {
                        antinodes.insert([i, j]);
                    }
                    for step in 1..=max_step {
                        if let (Some(ni), Some(nj)) = (
                            i.checked_add_signed(d_i * step),
                            j.checked_add_signed(d_j * step),
                        ) {
                            if ni >= max_i || nj >= max_j {
                                break;
                            }
                            #[cfg(debug_assertions)]
                            {
                                // debug
                                if !with_harmonics {
                                    lines[i1][j1] = '#';
                                    lines[i2][j2] = '#';
                                }
                                lines[ni][nj] = '#';
                                println!("{}", k);
                                common::prnt_lines(&lines);
                                thread::sleep(time::Duration::from_millis(30));
                            }

                            antinodes.insert([ni, nj]);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    return antinodes.len() as u32;
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day2 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let lines = day4::parse_file(file_name)?;
    let stations = read_stations(&lines);

    println!(
        "Valid antinodes {}",
        count_antinodes(&stations, &lines, false)
    );
    println!(
        "Valid antinodes, include harmonics {}",
        count_antinodes(&stations, &lines, true)
    );

    Ok(())
}
