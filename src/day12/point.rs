use std::cell::{Ref, RefCell, RefMut};

#[derive(Debug)]
pub struct Map {
    pub x: usize,
    pub y: usize,
    points: Vec<Vec<RefCell<Point>>>,
}

impl Map {
    pub fn new(x: usize, y: usize) -> Self {
        Map {
            points: Vec::new(),
            x,
            y,
        }
    }
    pub fn add_point(&mut self, point: Point) {
        let y = point.y;
        let mut line = self.points.get_mut(y);
        if (line.is_none()) {
            self.points.push(Vec::new());
            line = self.points.get_mut(y);
        }
        line.unwrap().push(RefCell::new(point));
    }
    pub fn get_point(&self, x: usize, y: usize) -> Option<&RefCell<Point>> {
        let line = match self.points.get(y) {
            None => return None,
            Some(line) => line,
        };

        match line.get(x) {
            None => None,
            Some(p) => Some(p),
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub move_dir: Vec<Dir>,
    pub letter: String,
}

#[derive(Debug, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Point {
    pub fn init(x: usize, y: usize, letter: String) -> Self {
        Point {
            x,
            y,
            letter,
            move_dir: Vec::new(),
        }
    }
    pub fn has_letter(&self, letter: &str) -> bool {
        self.letter == String::from(letter)
    }

    pub fn set_move_dir(&mut self, dir: Vec<Dir>) {
        self.move_dir = dir;
    }
    pub fn can_move(&self, dir: Ref<Point>) -> bool {
        if self.letter == String::from("S") {
            return true;
        }
        let self_char = self.letter.chars().nth(0).unwrap() as u8;
        let dir_char = dir.letter.chars().nth(0).unwrap() as u8;
        if dir_char - self_char <= 1 {
            return true;
        }
        return false;
    }
}
