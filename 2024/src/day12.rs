use crate::common::{parse_chars, Coord, CoordMethods, Direction};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io;
use std::process::exit;

//#[cfg(debug_assertions)]
//use std::{thread, time};

#[derive(Debug)]
struct Garden {
    regions: Vec<HashSet<GardenCoord>>,
    /// Visited coordinates
    coordinates: HashSet<GardenCoord>,
}

impl Garden {
    fn fill_region(&mut self, start: GardenCoord, raw_garden: &Vec<Vec<char>>) {
        let mut unprocessed_coords = HashSet::new();
        let mut seen_coords = HashSet::new();
        let mut region_coordinates = HashSet::new();
        unprocessed_coords.insert(start);
        while !unprocessed_coords.is_empty() {
            let current_batch: Vec<GardenCoord> = unprocessed_coords.drain().collect();

            for mut current_coord in current_batch {
                let mut round = Direction::ccw();
                while let Some(dir) = round() {
                    let neighbor = current_coord.get_next(dir.next(), &raw_garden);
                    if neighbor.is_none() {
                        // out of bounds
                        continue;
                    }
                    let neighbor = neighbor.unwrap();
                    if current_coord.get(raw_garden) != neighbor.get(raw_garden) {
                        continue;
                    }
                    current_coord.neighbors += 1;
                    if seen_coords.contains(&neighbor) {
                        continue;
                    }
                    unprocessed_coords.insert(neighbor);
                    seen_coords.insert(neighbor);
                }
                region_coordinates.insert(current_coord);
                self.coordinates.insert(current_coord);
            }
        }
        self.regions.push(region_coordinates);
    }

    fn count_costs(&self) -> usize {
        return self
            .regions
            .iter()
            .map(|reg| reg.len() * reg.iter().map(|coord| 4 - coord.neighbors).sum::<usize>())
            .sum();
    }

    fn count_w_discount(&self, garden: &Vec<Vec<char>>) -> usize {
        let mut total_costs = 0;
        let external_pairs: [[usize; 2]; 4] = [[0, 2], [0, 6], [4, 2], [4, 6]];
        let internal_corners: [[usize; 3]; 4] = [[0, 1, 2], [6, 7, 0], [2, 3, 4], [4, 5, 6]];
        for reg in self.regions.iter() {
            let mut corners: usize = 0;
            for coord in reg.iter() {
                // Search for other neighbors around the coordinate
                let mut round = Direction::dcw();
                let mut f_neigh: [bool; 8] = [false; 8];
                let mut _n = 0;
                while let Some(dir) = round() {
                    if let Some(neighbor) = coord.get_next(dir.next(), garden) {
                        f_neigh[_n] = neighbor.get(garden) == coord.get(garden);
                    }
                    _n += 1;
                }
                // Check external corners
                for pair in external_pairs {
                    if !(f_neigh[pair[0]] || f_neigh[pair[1]]) {
                        corners += 1
                    }
                }
                for corner in internal_corners {
                    if (f_neigh[corner[0]] && f_neigh[corner[2]]) && !f_neigh[corner[1]] {
                        corners += 1
                    }
                }
            }
            total_costs += reg.len() * corners;
        }
        return total_costs;
    }

    fn new() -> Self {
        return Garden {
            regions: Vec::new(),
            coordinates: HashSet::new(),
        };
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct GardenCoord {
    coord: Coord,
    neighbors: usize,
}

impl Hash for GardenCoord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
    }
}
impl PartialEq for GardenCoord {
    fn eq(&self, other: &Self) -> bool {
        return self.coord == other.coord;
    }
}

impl GardenCoord {
    fn new(i: usize, j: usize, neighbors: usize) -> Self {
        GardenCoord {
            coord: Coord::new(i, j),
            neighbors,
        }
    }
}

fn count_fences(raw_garden: &Vec<Vec<char>>) -> usize {
    // First, let's get all regions
    let mut garden = Garden::new();
    for i in 0..raw_garden.len() {
        let row = &raw_garden[i];
        for j in 0..row.len() {
            let current_coord = GardenCoord::new(i, j, 0);
            if !garden.coordinates.contains(&current_coord) {
                garden.fill_region(current_coord, raw_garden);
            }
        }
    }
    dbg!(
        garden.coordinates.len(),
        garden.regions.len(),
        garden.count_costs(),
        garden.count_w_discount(&raw_garden),
    );
    return garden.count_costs();
}

impl CoordMethods for GardenCoord {
    fn get_next<'a, T>(&self, diff: [isize; 2], field: &'a Vec<Vec<T>>) -> Option<Self> {
        if let Some(coord) = Coord::get_next(&self.coord, diff, field) {
            return Some(GardenCoord::new(coord.i, coord.j, 0));
        }
        None
    }

    fn get<'a, T>(&self, field: &'a [Vec<T>]) -> Option<&'a T> {
        field.get(self.coord.i)?.get(self.coord.j)
    }
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day12 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let garden: Vec<Vec<char>> = parse_chars(file_name)?;
    count_fences(&garden);

    Ok(())
}
