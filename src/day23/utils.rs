use std::{
  cell::{RefCell, RefMut},
  collections::HashMap,
};

use super::map::{MoveDir, Point};

pub fn get_next_point(p: &RefCell<Point>, move_dir: &MoveDir) -> (i32, i32) {
  let p = p.borrow();
  match move_dir {
    MoveDir::North => (p.x, p.y - 1),
    MoveDir::South => (p.x, p.y + 1),
    MoveDir::West => (p.x - 1, p.y),
    MoveDir::East => (p.x + 1, p.y),
  }
}

pub fn not_need_move(
  p: &RefCell<Point>,
  points: &Vec<&RefCell<Point>>,
) -> bool {
  let p = p.borrow();
  let rel_points = [
    (p.x - 1, p.y - 1),
    (p.x, p.y - 1),
    (p.x + 1, p.y - 1),
    (p.x - 1, p.y + 1),
    (p.x, p.y + 1),
    (p.x - 1, p.y),
    (p.x + 1, p.y),
    (p.x + 1, p.y + 1),
  ];

  for p in points.iter() {
    let p = p.borrow();
    let p = (p.x, p.y);
    if rel_points.contains(&p) {
      return false;
    }
  }

  return true;
}

pub fn can_move(
  p: &RefCell<Point>,
  move_dir: &MoveDir,
  points: &Vec<&RefCell<Point>>,
) -> bool {
  let p = p.borrow();
  let rel_points = match move_dir {
    MoveDir::North => [(p.x - 1, p.y - 1), (p.x, p.y - 1), (p.x + 1, p.y - 1)],
    MoveDir::South => [(p.x - 1, p.y + 1), (p.x, p.y + 1), (p.x + 1, p.y + 1)],
    MoveDir::West => [(p.x - 1, p.y - 1), (p.x - 1, p.y), (p.x - 1, p.y + 1)],
    MoveDir::East => [(p.x + 1, p.y - 1), (p.x + 1, p.y), (p.x + 1, p.y + 1)],
  };

  for p in points.iter() {
    let p = p.borrow();
    let p = (p.x, p.y);
    if rel_points.contains(&p) {
      return false;
    }
  }

  true
}

pub fn remove_dul_p(move_map: &mut HashMap<usize, (i32, i32)>) {
  let mut remove_arr = vec![];
  for (key, p) in move_map.iter() {
    let dul_item = move_map.iter().find(|item| item.1 == p && item.0 != key);

    if dul_item.is_some() {
      remove_arr.push(key.clone());
    }
  }

  for key in remove_arr {
    move_map.remove(&key);
  }
}

pub fn get_range(points: &Vec<RefMut<Point>>) -> ((i32, i32), (i32, i32)) {
  let mut min_x = 0;
  let mut max_x = 0;
  let mut min_y = 0;
  let mut max_y = 0;
  for p in points.iter() {
    if p.x > max_x {
      max_x = p.x
    }
    if p.x < min_x {
      min_x = p.x
    }
    if p.y > max_y {
      max_y = p.y
    }
    if p.y < min_y {
      min_y = p.y
    }
  }

  let range_x = (min_x, max_x);
  let range_y = (min_y, max_y);

  (range_x, range_y)
}
