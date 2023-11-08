use std::{
  borrow::BorrowMut,
  cell::{Ref, RefCell},
  cmp,
  collections::HashMap,
};

use super::map::{Map, MoveKey, Point, PointStatus, WindDir};

pub fn get_wind_next<'a>(
  p: Ref<Point>,
  move_dir: &WindDir,
  map: &'a Map,
) -> &'a RefCell<Point> {
  let mut pos = match move_dir {
    WindDir::North => (p.x, p.y - 1),
    WindDir::South => (p.x, p.y + 1),
    WindDir::West => (p.x - 1, p.y),
    WindDir::East => (p.x + 1, p.y),
  };
  let point = map.get_p(&pos);
  let mut p = point;
  if p.borrow().is_wall() {
    if move_dir == &WindDir::North && pos.1 == 0 {
      pos.1 = map.points.len() as i32 - 2;
    }
    if move_dir == &WindDir::South && pos.1 == map.points.len() as i32 - 1 {
      pos.1 = 1;
    }
    if move_dir == &WindDir::West && pos.0 == 0 {
      pos.0 = map.points[0].len() as i32 - 2;
    }
    if move_dir == &WindDir::East && pos.0 == map.points[0].len() as i32 - 1 {
      pos.0 = 1;
    }
    p = map.get_p(&pos);
  }

  return p;
}

pub fn update_map(map: &Map) {
  let mut update_list = vec![];
  for line in map.points.iter() {
    for p in line.iter() {
      let wind_num = p.borrow().wind_num;
      if wind_num == 0 {
        continue;
      }
      // if loop_index == 2 && p.borrow().x == 4 && p.borrow().y == 1 {
      //   println!("p{}:{:?}|wind_num={:?}", loop_index, p.borrow(), wind_num);
      // }
      for i in (0..wind_num).rev() {
        let wind = p.borrow_mut().winds.remove(i);
        let nex_p = get_wind_next(p.borrow(), &wind, &map);
        nex_p.borrow_mut().add_wind(wind);
        update_list.push(nex_p);
      }
      update_list.push(p);
    }
  }

  for item in update_list.into_iter() {
    item.borrow_mut().update_wind_num();
  }
}

pub fn get_move_next(p: &MoveKey, map: &Map) -> Vec<MoveKey> {
  let cur_pos = p.get_last_pos();
  let next_list = get_nearby_pos(cur_pos, map);

  let mut arr = vec![];
  for next_pos in next_list.into_iter() {
    let mut cur_key = p.clone();
    cur_key.add_pos(next_pos);
    arr.push(cur_key);
  }

  arr
}

pub fn get_nearby_pos(p: &(i32, i32), map: &Map) -> Vec<(i32, i32)> {
  let rel_points = [
    (p.0, p.1),
    (p.0, p.1 - 1),
    (p.0, p.1 + 1),
    (p.0 - 1, p.1),
    (p.0 + 1, p.1),
  ];

  let mut arr = vec![];
  for pos in rel_points.into_iter() {
    if map.is_outer_range(&pos) {
      continue;
    }
    let p = map.get_p(&pos).borrow();
    if p.can_move_to() {
      arr.push(pos)
    }
  }

  arr
}

pub fn calc_top_keys(
  loop_keys: &mut Vec<MoveKey>,
  temp_keys: &mut Vec<MoveKey>,
  completed_keys: &mut Vec<MoveKey>,
  map: &Map,
) {
  let few_step = get_complete_num(completed_keys);

  let max_num = 1000;
  if loop_keys.len() == 0 {
    let change_num = cmp::min(max_num, temp_keys.len());
    let add_list = temp_keys.split_off(temp_keys.len() - change_num);

    loop_keys.extend(add_list);
  }

  loop_keys.retain(|item| {
    if item.is_end(map) {
      if item.get_num() < few_step {
        completed_keys.insert(0, item.clone());
      }
      return false;
    }
    let quick_step = item.get_quick_steps(map);
    if quick_step >= few_step {
      return false;
    }
    return true;
  });

  if loop_keys.len() == 0 {
    return;
  }

  loop_keys.sort_by(|a, b| {
    let rate_a = a.get_quick_steps(map);
    let rate_b = b.get_quick_steps(map);
    rate_a.cmp(&rate_b)
  });

  if loop_keys.len() > max_num {
    let temp_save_ele = loop_keys.split_off(max_num);
    temp_keys.extend(temp_save_ele);
  }
}

pub fn get_complete_num(completed_keys: &Vec<MoveKey>) -> usize {
  return match completed_keys.get(0) {
    Some(item) => item.get_num(),
    None => usize::MAX,
  };
}
