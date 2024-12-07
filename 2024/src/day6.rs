use crate::day4;
use std::io;
use std::process::exit;

const VALID_DIRECTION: [char; 4] = ['^', '>', 'v', '<'];

fn count_route(lines: &Vec<Vec<char>>) -> u32 {
    let mut maze = lines.to_vec();
    let mut positions: u32 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut dir = '.';
    // find starting point
    for r in 0..maze.len() {
        for c in 0..maze[r].len() {
            if VALID_DIRECTION.contains(&maze[r][c]) {
                (i, j) = (r, c);
                dir = maze[r][c]
            }
        }
    }
    while VALID_DIRECTION.contains(&dir) {
        let di: isize;
        let dj: isize;
        (di, dj) = direction_coordinates(dir);
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
            dir = new_direction(dir);
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

#[derive(PartialEq)]
struct Coord {
    i: usize,
    j: usize,
    dir: char,
}

fn count_obstacles(lines: &Vec<Vec<char>>) -> u32 {
    let mut maze = lines.to_vec();
    let mut obstacles: u32 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut dir = '.';
    // find starting point
    for r in 0..maze.len() {
        for c in 0..maze[r].len() {
            if VALID_DIRECTION.contains(&maze[r][c]) {
                (i, j) = (r, c);
                dir = maze[r][c]
            }
        }
    }
    let mut obstacle_coordinates: Vec<Coord> = Vec::new();
    while VALID_DIRECTION.contains(&dir) {
        let di: isize;
        let dj: isize;
        (di, dj) = direction_coordinates(dir);
        let ni = i.checked_add_signed(di);
        let nj = j.checked_add_signed(dj);
        if ni.is_none() || Some(maze.len()) <= ni || nj.is_none() || Some(maze[i].len()) <= nj {
            break;
        }
        let ni = ni.expect("Checked it's correct");
        let nj = nj.expect("Checked it's correct");
        // loop detection

        let mut int_maze = maze.to_vec();
        let mut changed: bool = false;
        let obstacle_coordinate = Coord {
            i: ni,
            j: nj,
            dir: '.',
        };
        if int_maze[ni][nj] != '#' && !obstacle_coordinates.contains(&obstacle_coordinate) {
            int_maze[ni][nj] = '#';
            obstacle_coordinates.push(obstacle_coordinate);
            changed = true;
        }
        let mut int_dir = new_direction(dir);
        let mut int_i = i;
        let mut int_j = j;
        let mut coordinates: Vec<Coord> = Vec::new();
        while VALID_DIRECTION.contains(&int_dir) && changed {
            int_maze[int_i][int_j] = 'X';
            let current_coordinate = Coord {
                i: int_i,
                j: int_j,
                dir: int_dir,
            };
            if coordinates.contains(&current_coordinate) {
                for row in int_maze {
                    let row: String = row.iter().collect();
                    println!("{}", row);
                }
                obstacles += 1;
                break;
            }
            coordinates.push(current_coordinate);
            let di: isize;
            let dj: isize;
            (di, dj) = direction_coordinates(int_dir);
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
                int_dir = new_direction(int_dir);
                continue;
            }
            (int_i, int_j) = (ni, nj);
        }

        //
        // continue normal route
        if maze[ni][nj] == '#' {
            dir = new_direction(dir);
            continue;
        }
        (i, j) = (ni, nj);
        maze[ni][nj] = dir;
    }
    for row in maze {
        let row: String = row.iter().collect();
        println!("{}", row);
    }
    return obstacles;
}

fn new_direction(dir: char) -> char {
    assert!(VALID_DIRECTION.contains(&dir));
    match dir {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => '!',
    }
}

fn direction_coordinates(dir: char) -> (isize, isize) {
    assert!(VALID_DIRECTION.contains(&dir));
    match dir {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => (0, 0),
    }
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
