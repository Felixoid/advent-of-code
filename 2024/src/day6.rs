use crate::common::{Coord, Direction, MazePoint};
use crate::day4;
use std::collections::HashSet;
use std::io;
use std::process::exit;

fn count_route(lines: &Vec<Vec<char>>) -> u32 {
    let mut maze = lines.to_vec();
    let mut positions: u32 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut dir: Direction = Direction::new();
    // find starting point
    for r in 0..maze.len() {
        for c in 0..maze[r].len() {
            if let Some(_dir) = Direction::from_dir(&maze[r][c]) {
                (i, j) = (r, c);
                dir = _dir;
                break;
            }
        }
    }
    while dir.is_valid() {
        let di: isize;
        let dj: isize;
        (di, dj) = dir.next();
        if maze[i][j] != 'X' {
            positions += 1;
            maze[i][j] = 'X';
        }
        let ni = i.checked_add_signed(di);
        let nj = j.checked_add_signed(dj);
        if ni.is_none() || Some(maze.len()) <= ni || nj.is_none() || Some(maze[i].len()) <= nj {
            break;
        }
        let ni = ni.expect("Checked it's correct");
        let nj = nj.expect("Checked it's correct");
        if maze[ni][nj] == '#' {
            dir.turn_right();
            continue;
        }
        (i, j) = (ni, nj);
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
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut dir: Direction = Direction::new();
    // find starting point
    for r in 0..maze.len() {
        for c in 0..maze[r].len() {
            if let Some(_dir) = Direction::from_dir(&maze[r][c]) {
                (i, j) = (r, c);
                dir = _dir;
                break;
            }
        }
    }
    let mut obstacle_coordinates: HashSet<Coord> = HashSet::new();
    let mut visited_coordinates: HashSet<Coord> = HashSet::new();
    while dir.is_valid() {
        let di: isize;
        let dj: isize;
        (di, dj) = dir.next();
        let ni = i.checked_add_signed(di);
        let nj = j.checked_add_signed(dj);
        if ni.is_none() || Some(maze.len()) <= ni || nj.is_none() || Some(maze[i].len()) <= nj {
            break;
        }
        visited_coordinates.insert(Coord { i, j });
        let ni = ni.expect("Checked it's correct");
        let nj = nj.expect("Checked it's correct");
        // loop detection

        let mut int_maze = maze.to_vec();
        let mut changed: bool = false;
        let obstacle_coordinate = Coord { i: ni, j: nj };
        if int_maze[ni][nj] != '#'
            && !obstacle_coordinates.contains(&obstacle_coordinate)
            && !visited_coordinates.contains(&obstacle_coordinate)
        {
            int_maze[ni][nj] = '#';
            obstacle_coordinates.insert(obstacle_coordinate);
            changed = true;
        }
        let mut int_dir = dir.clone();
        int_dir.turn_right();
        let mut int_i = i;
        let mut int_j = j;
        let mut coordinates: HashSet<MazePoint> = HashSet::new();
        while int_dir.is_valid() && changed {
            int_maze[int_i][int_j] = 'X';
            let current_coordinate = MazePoint {
                i: int_i,
                j: int_j,
                dir: int_dir.clone(),
            };
            if coordinates.contains(&current_coordinate) {
                for row in int_maze {
                    let row: String = row.iter().collect();
                    println!("{}", row);
                }
                obstacles += 1;
                break;
            }
            coordinates.insert(current_coordinate);
            let di: isize;
            let dj: isize;
            (di, dj) = int_dir.next();
            let ni = int_i.checked_add_signed(di);
            let nj = int_j.checked_add_signed(dj);
            if ni.is_none()
                || Some(int_maze.len()) <= ni
                || nj.is_none()
                || Some(int_maze[int_i].len()) <= nj
            {
                // not a loop
                break;
            }
            let ni = ni.expect("Checked it's correct");
            let nj = nj.expect("Checked it's correct");
            if int_maze[ni][nj] == '#' {
                int_dir.turn_right();
                continue;
            }
            (int_i, int_j) = (ni, nj);
        }

        //
        // continue normal route
        if maze[ni][nj] == '#' {
            dir.turn_right();
            continue;
        }
        (i, j) = (ni, nj);
        maze[ni][nj] = dir.dir;
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
