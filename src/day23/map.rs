use core::fmt;
use std::cell::{Ref, RefCell, RefMut};

#[derive(Debug)]
pub enum MoveDir {
  North,
  South,
  West,
  East,
}

pub struct Map {
  points: Vec<RefCell<Point>>,
  range_x: (i32, i32),
  range_y: (i32, i32),
}

impl Map {
  pub fn new(points: Vec<RefCell<Point>>) -> Map {
    Map {
      points,
      range_x: (0, 0),
      range_y: (0, 0),
    }
  }
  pub fn get_p(&self, x: i32, y: i32) -> Option<Ref<Point>> {
    for p in self.points.iter() {
      let p = p.borrow();
      if p.x == x && p.y == y {
        return Some(p);
      }
    }
    return None;
  }
  pub fn get_elf_points(&self) -> Vec<&RefCell<Point>> {
    self
      .points
      .iter()
      .filter(|item| item.borrow().has_elf)
      .collect()
  }
  pub fn update_range(&mut self, range: ((i32, i32), (i32, i32))) {
    self.range_x = range.0;
    self.range_y = range.1;
  }
  pub fn get_empty_size(&self) -> i32 {
    let x = self.range_x.1 - self.range_x.0;
    let y = self.range_y.1 - self.range_y.0;

    (x + 1) * (y + 1) - self.points.len() as i32
  }
}

impl fmt::Debug for Map {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut lines = String::from("");
    for y in self.range_y.0..=self.range_y.1 {
      for x in self.range_x.0..=self.range_x.1 {
        if let Some(p) = self.get_p(x, y) {
          lines += "#";
        } else {
          lines += ".";
        }
      }

      lines += "\n";
    }
    write!(f, "{}", lines)
  }
}

pub struct Point {
  pub x: i32,
  pub y: i32,
  pub has_elf: bool,
}

impl Point {
  pub fn new(x: i32, y: i32, has_elf: bool) -> Point {
    Point { x, y, has_elf }
  }
  pub fn update_pos(&mut self, pos: (i32, i32)) {
    self.x = pos.0;
    self.y = pos.1;
  }
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mark = if self.has_elf { "#" } else { "." };
    write!(f, "({},{}){}", self.x, self.y, mark)
  }
}
