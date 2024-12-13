use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

#[cfg(debug_assertions)]
pub fn prnt_lines(lines: &Vec<Vec<char>>) {
    for line in lines {
        let line: String = line.iter().collect();
        println!("{}", line);
    }
}

pub type Lines = Vec<Vec<char>>;

pub fn parse_chars(file_name: &str) -> std::io::Result<Vec<Vec<char>>> {
    let mut lines: Vec<Vec<char>> = Vec::new();

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        lines.push(line?.chars().collect());
    }
    assert!(lines.iter().all(|line| line.len() == lines[0].len()));
    Ok(lines)
}

pub fn parse_file<T>(file_name: &str) -> std::io::Result<Vec<Vec<T>>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display + Send + Sync + std::error::Error,
    <T as FromStr>::Err: 'static,
{
    let mut reports: Vec<Vec<T>> = Vec::new();

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<T> = line
            .split_whitespace()
            .map(|n| {
                n.parse::<T>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<Result<_, _>>()?;
        if nums.len() == 0 {
            eprintln!("Warning: skipping '{}' line", line);
            continue;
        }
        reports.push(nums);
    }
    Ok(reports)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direction {
    pub dir: char,
}

impl Direction {
    const VALID_DIRECTION: [char; 4] = ['^', '>', 'v', '<'];
    /// Imitate diagonal direction with best symbols it could
    const VALID_DIAG: [char; 4] = ['7', 'J', 'L', 'F'];

    pub fn new() -> Direction {
        return Direction { dir: '^' };
    }

    pub fn is_valid(&self) -> bool {
        return Direction::VALID_DIRECTION.contains(&self.dir);
    }

    pub fn is_diag(&self) -> bool {
        return Direction::VALID_DIAG.contains(&self.dir);
    }

    pub fn next(&self) -> [isize; 2] {
        assert!(self.is_valid() || self.is_diag());
        match self.dir {
            '^' => [-1, 0],
            '7' => [-1, 1],
            '>' => [0, 1],
            'J' => [1, 1],
            'v' => [1, 0],
            'L' => [1, -1],
            '<' => [0, -1],
            'F' => [-1, -1],
            _ => [0, 0],
        }
    }

    pub fn turn_right(&mut self) {
        assert!(self.is_valid());
        self.dir = match self.dir {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => '!',
        }
    }

    pub fn turn_left(&mut self) {
        assert!(self.is_valid());
        self.dir = match self.dir {
            '^' => '<',
            '<' => 'v',
            'v' => '>',
            '>' => '^',
            _ => '!',
        }
    }
    pub fn diag_right(&mut self) {
        assert!(self.is_valid() || self.is_diag());
        self.dir = match self.dir {
            '^' => '7',
            '7' => '>',
            '>' => 'J',
            'J' => 'v',
            'v' => 'L',
            'L' => '<',
            '<' => 'F',
            'F' => '^',
            _ => '!',
        }
    }

    pub fn diag_left(&mut self) {
        assert!(self.is_valid() || self.is_diag());
        self.dir = match self.dir {
            '^' => 'F',
            'F' => '<',
            '<' => 'L',
            'L' => 'v',
            'v' => 'J',
            'J' => '>',
            '>' => '7',
            '7' => '^',
            _ => '!',
        }
    }

    pub fn from_dir(dir: &char) -> Option<Direction> {
        if Direction::VALID_DIRECTION.contains(dir) {
            return Some(Direction { dir: *dir });
        }
        return None;
    }

    /// Turn counterclockwise from the current position
    pub fn ccw() -> Box<dyn FnMut() -> Option<Direction>> {
        let mut dir = Direction::new();
        let start = dir.dir;
        let first = RefCell::new(Vec::new());
        Box::new(move || {
            if first.borrow().len() == 0 {
                first.borrow_mut().push(());
                return Some(dir);
            }
            dir.turn_left(); // turn left always except the first step

            if start != dir.dir {
                return Some(dir);
            }

            return None; // If no other condition was met, return false
        })
    }

    /// Turn clockwise with diagonal step, start from the Up direction
    pub fn dcw() -> Box<dyn FnMut() -> Option<Direction>> {
        let mut dir = Direction::new();
        let start = dir.dir;
        let first = RefCell::new(Vec::new());
        Box::new(move || {
            if first.borrow().len() == 0 {
                first.borrow_mut().push(());
                return Some(dir);
            }
            dir.diag_right(); // turn left always except the first step

            if start != dir.dir {
                return Some(dir);
            }

            return None; // If no other condition was met, return false
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Coord {
    pub i: usize,
    pub j: usize,
}

pub trait CoordMethods {
    fn get_next<'a, T>(&self, diff: [isize; 2], field: &'a Vec<Vec<T>>) -> Option<Self>
    where
        Self: Sized;
    fn get<'a, T>(&self, field: &'a [Vec<T>]) -> Option<&'a T>;
}

impl Coord {
    pub fn new(i: usize, j: usize) -> Self {
        Coord { i, j }
    }
}

impl CoordMethods for Coord {
    fn get_next<'a, T>(&self, diff: [isize; 2], field: &'a Vec<Vec<T>>) -> Option<Self> {
        let ni = self.i.checked_add_signed(diff[0]);
        let nj = self.j.checked_add_signed(diff[1]);
        if ni.is_none() || nj.is_none() {
            return None;
        }
        let ni = ni.expect("checked ni");
        let nj = nj.expect("checked nj");
        if (field.len()) <= ni || (field[self.i].len()) <= nj {
            return None;
        }
        Some(Self::new(ni, nj))
    }

    fn get<'a, T>(&self, field: &'a [Vec<T>]) -> Option<&'a T> {
        field.get(self.i)?.get(self.j)
    }
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct MazePoint {
    pub i: usize,
    pub j: usize,
    pub dir: Direction,
}
