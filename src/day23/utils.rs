use std::{
  cell::{Ref, RefCell},
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
  nearby_map: &HashMap<(i32, i32), bool>,
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

  for p in rel_points {
    if nearby_map.contains_key(&p) {
      return false;
    }
  }

  return true;
}

pub fn can_move(
  p: &RefCell<Point>,
  move_dir: &MoveDir,
  nearby_map: &HashMap<(i32, i32), bool>,
) -> bool {
  let p = p.borrow();
  let rel_points = match move_dir {
    MoveDir::North => [(p.x - 1, p.y - 1), (p.x, p.y - 1), (p.x + 1, p.y - 1)],
    MoveDir::South => [(p.x - 1, p.y + 1), (p.x, p.y + 1), (p.x + 1, p.y + 1)],
    MoveDir::West => [(p.x - 1, p.y - 1), (p.x - 1, p.y), (p.x - 1, p.y + 1)],
    MoveDir::East => [(p.x + 1, p.y - 1), (p.x + 1, p.y), (p.x + 1, p.y + 1)],
  };

  for p in rel_points {
    if nearby_map.contains_key(&p) {
      return false;
    }
  }

  true
}

pub fn get_nearby_points<'a>(
  p: &'a RefCell<Point>,
  points: &Vec<&'a RefCell<Point>>,
) -> HashMap<(i32, i32), bool> {
  let range_x = (p.borrow().x - 1, p.borrow().x + 1);
  let range_y = (p.borrow().y - 1, p.borrow().y + 1);

  let mut nearby_map = HashMap::new();

  for item in points.iter() {
    let p = item.borrow();
    if p.x >= range_x.0
      && p.x <= range_x.1
      && p.y >= range_y.0
      && p.y <= range_y.1
    {
      nearby_map.insert((p.x, p.y), true);
    }
  }

  nearby_map
}

pub fn remove_dul_p(move_map: &mut HashMap<usize, (i32, i32)>) {
  let mut values: HashMap<(i32, i32), usize> = HashMap::new();

  // 统计每个值的出现次数
  for (_, value) in move_map.iter() {
    *values.entry(value.clone()).or_insert(0) += 1;
  }

  // 删除具有相同值的所有项
  move_map.retain(|_, value| values.get(value) == Some(&1));
}

pub fn get_range(points: &Vec<&RefCell<Point>>) -> ((i32, i32), (i32, i32)) {
  let mut min_x = 0;
  let mut max_x = 0;
  let mut min_y = 0;
  let mut max_y = 0;
  for p in points.iter() {
    let p = p.borrow();
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
