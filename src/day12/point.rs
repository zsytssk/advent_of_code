#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub move_dir: Vec<Dir>,
    pub letter: String,
}

#[derive(Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Point {
    pub fn init() -> Self {
        Point {
            x: 0,
            y: 0,
            letter: String::from(""),
            move_dir: Vec::new(),
        }
    }
    pub fn update_pos(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
    pub fn set_letter(&mut self, letter: String) {
        self.letter = letter;
    }
    pub fn set_move_dir(&mut self, dir: Vec<Dir>) {
        self.move_dir = dir;
    }
    pub fn can_move(&mut self, dir: &Point) {}
}
