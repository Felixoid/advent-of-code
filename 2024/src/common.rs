#[cfg(debug_assertions)]
pub fn prnt_lines(lines: &Vec<Vec<char>>) {
    for line in lines {
        let line: String = line.iter().collect();
        println!("{}", line);
    }
}

pub type Lines = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Direction {
    pub dir: char,
}

impl Direction {
    const VALID_DIRECTION: [char; 4] = ['^', '>', 'v', '<'];

    pub fn new() -> Direction {
        return Direction { dir: '.' };
    }

    pub fn is_valid(&self) -> bool {
        return Direction::VALID_DIRECTION.contains(&self.dir);
    }

    pub fn next(&self) -> (isize, isize) {
        assert!(self.is_valid());
        match self.dir {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => (0, 0),
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
            '>' => '^',
            'v' => '>',
            '<' => 'v',
            _ => '!',
        }
    }

    pub fn from_dir(dir: &char) -> Option<Direction> {
        if Direction::VALID_DIRECTION.contains(dir) {
            return Some(Direction { dir: *dir });
        }
        return None;
    }
}

#[derive(PartialEq, Hash, Eq)]
pub struct Coord {
    pub i: usize,
    pub j: usize,
}

#[derive(PartialEq, Hash, Eq)]
pub struct MazePoint {
    pub i: usize,
    pub j: usize,
    pub dir: Direction,
}
