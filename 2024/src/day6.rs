use crate::common::{Coord, Direction, MazePoint};
use crate::day4;
use std::collections::HashSet;
use std::io;
use std::process::exit;

fn count_route(lines: &Vec<Vec<char>>) -> u32 {
    let mut maze = lines.to_vec();
    let mut positions: u32 = 0;
    let mut coord = Coord::new(0, 0);
    let mut dir: Direction = Direction::new();
    // find starting point
    for r in 0..maze.len() {
        for c in 0..maze[r].len() {
            if let Some(_dir) = Direction::from_dir(&maze[r][c]) {
                coord = Coord::new(r, c);
                dir = _dir;
                break;
            }
        }
    }
    let c_wall = Some(&'#');
    let c_x = Some(&'X');
    while dir.is_valid() {
        let diff = dir.next();
        if coord.get(&maze) != c_x {
            positions += 1;
            maze[coord.i][coord.j] = 'X';
        }
        let next_coord = coord.get_next(diff, &maze);
        if next_coord.is_none() {
            break;
        }
        let next_coord = next_coord.expect("checked next coordinations");
        if next_coord.get(&maze) == c_wall {
            dir.turn_right();
            continue;
        }
        coord = next_coord;
    }
    for row in maze {
        let row: String = row.iter().collect();
        println!("{}", row);
    }
    return positions;
}

fn count_obstacles(lines: &Vec<Vec<char>>) -> u32 {
    let mut maze = lines.to_vec();
    let mut obstacles: u32 = 0;
    let mut coord = Coord::new(0, 0);
    let mut dir: Direction = Direction::new();
    // find starting point
    for r in 0..maze.len() {
        for c in 0..maze[r].len() {
            if let Some(_dir) = Direction::from_dir(&maze[r][c]) {
                coord = Coord::new(r, c);
                dir = _dir;
                break;
            }
        }
    }
    let mut obstacle_coordinates: HashSet<Coord> = HashSet::new();
    let mut visited_coordinates: HashSet<Coord> = HashSet::new();
    let c_wall = Some(&'#');
    while dir.is_valid() {
        let diff = dir.next();
        let next_coord = coord.get_next(diff, &maze);
        if next_coord.is_none() {
            break;
        }
        let next_coord = next_coord.expect("checked next coordinations");
        // loop detection
        visited_coordinates.insert(coord);

        let mut int_maze = maze.to_vec();
        let mut changed: bool = false;
        let obstacle_coordinate = next_coord;
        if next_coord.get(&int_maze) != c_wall
            && !obstacle_coordinates.contains(&obstacle_coordinate)
            && !visited_coordinates.contains(&obstacle_coordinate)
        {
            int_maze[next_coord.i][next_coord.j] = '#';
            obstacle_coordinates.insert(obstacle_coordinate);
            changed = true;
        }
        let mut int_dir = dir.clone();
        int_dir.turn_right();
        let mut int_coord = coord.clone();
        let mut coordinates: HashSet<MazePoint> = HashSet::new();
        while int_dir.is_valid() && changed {
            int_maze[int_coord.i][int_coord.j] = 'X';
            let current_coordinate = MazePoint {
                i: int_coord.i,
                j: int_coord.j,
                dir: int_dir.clone(),
            };
            if coordinates.contains(&current_coordinate) {
                obstacles += 1;
                println!("");
                for row in &int_maze {
                    let row: String = row.iter().collect();
                    println!("{}", row);
                }
                break;
            }
            coordinates.insert(current_coordinate);
            let diff = int_dir.next();
            let next_coord = int_coord.get_next(diff, &int_maze);
            if next_coord.is_none() {
                // not a loop
                break;
            }
            let next_coord = next_coord.expect("checked next coordinations");
            if next_coord.get(&int_maze) == c_wall {
                int_dir.turn_right();
                continue;
            }
            int_coord = next_coord;
        }

        //
        // continue normal route
        if next_coord.get(&maze) == c_wall {
            dir.turn_right();
            continue;
        }
        coord = next_coord;
        maze[coord.i][coord.j] = dir.dir;
    }
    for row in maze {
        let row: String = row.iter().collect();
        println!("{}", row);
    }
    return obstacles;
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day2 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let lines = day4::parse_file(file_name)?;
    println!("The finished maze in {} steps", count_route(&lines));
    println!("The possible obstacles {}", count_obstacles(&lines));

    Ok(())
}
