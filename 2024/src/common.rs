#[cfg(debug_assertions)]
pub fn prnt_lines(lines: &Vec<Vec<char>>) {
    for line in lines {
        let line: String = line.iter().collect();
        println!("{}", line);
    }
}

pub type Lines = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direction {
    pub dir: char,
}

impl Direction {
    const VALID_DIRECTION: [char; 4] = ['^', '>', 'v', '<'];

    pub fn new() -> Direction {
        return Direction { dir: '^' };
    }

    pub fn is_valid(&self) -> bool {
        return Direction::VALID_DIRECTION.contains(&self.dir);
    }

    pub fn next(&self) -> [isize; 2] {
        assert!(self.is_valid());
        match self.dir {
            '^' => [-1, 0],
            '>' => [0, 1],
            'v' => [1, 0],
            '<' => [0, -1],
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

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Coord {
    pub i: usize,
    pub j: usize,
}

impl Coord {
    pub fn get_next<'a, T>(&self, diff: [isize; 2], field: &'a Vec<Vec<T>>) -> Option<Coord> {
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
        Some(Coord::new(ni, nj))
    }

    pub fn new(i: usize, j: usize) -> Coord {
        Coord { i, j }
    }

    pub fn get<'a, T>(&self, field: &'a [Vec<T>]) -> Option<&'a T> {
        field.get(self.i)?.get(self.j)
    }
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct MazePoint {
    pub i: usize,
    pub j: usize,
    pub dir: Direction,
}
