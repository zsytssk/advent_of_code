#![allow(unused)]
use std::{
  cell::{Ref, RefCell, RefMut},
  collections::HashMap,
  string,
  time::Instant,
};

use crate::utils::read_file;

use super::map::*;
use super::*;

// https://adventofcode.com/2022/day/12#part2
pub fn parse1() {
  let now = Instant::now();
  let map = parse_input();

  let mut start_wap = None;
  let mut end_wap = None;

  for y in 0..map.y {
    for x in 0..map.x {
      let item = map.get_point(x, y);
      if item.unwrap().borrow().has_letter("S") {
        start_wap = item;
        continue;
      }
      if item.unwrap().borrow().has_letter("E") {
        end_wap = item;
      }
    }
  }
  if (start_wap.is_none() || end_wap.is_none()) {
    panic!("start or end not found!");
  }

  let mut map_space: HashMap<String, usize> = HashMap::new();
  let (start, end) = (start_wap.unwrap(), end_wap.unwrap());
  let (find, step) = find_end_len(
    start,
    end,
    &map,
    &mut map_space,
    &vec![(start.borrow().x, start.borrow().y, String::from("S"))],
    &usize::MAX,
  );

  println!(
    "find={:?} step={:?}  cost_time={:?}",
    find,
    step - 1,
    now.elapsed()
  );
}

pub fn parse2() {
  let now = Instant::now();
  let map = parse_input();
  let mut start_arr = Vec::new();
  let mut end_wap = None;
  for y in 0..map.y {
    for x in 0..map.x {
      let item = map.get_point(x, y);
      let item_ref = item.unwrap().borrow();
      if item_ref.has_letter("a") {
        start_arr.push(item.unwrap());
        continue;
      }
      if item_ref.has_letter("E") {
        end_wap = item;
      }
    }
  }

  if (start_arr.len() == 0 || end_wap.is_none()) {
    panic!("start or end not found!");
  }

  let end_p = end_wap.unwrap();
  let end_ref = end_p.borrow();
  start_arr.sort_by(|a, b| {
    let a_ref = a.borrow();
    let b_ref = b.borrow();
    let a_space = (end_ref.x as i32 - a_ref.x as i32).abs()
      + (end_ref.y as i32 - a_ref.y as i32).abs();
    let b_space = (end_ref.x as i32 - b_ref.x as i32).abs()
      + (end_ref.y as i32 - b_ref.y as i32).abs();
    a_space.cmp(&b_space)
  });

  let mut min_path = usize::MAX;
  for (index, start) in start_arr.iter().enumerate() {
    let mut map_space: HashMap<String, usize> = HashMap::new();
    let (find, step) = find_end_len(
      start,
      end_p,
      &map,
      &mut map_space,
      &vec![(start.borrow().x, start.borrow().y, String::from("S"))],
      &min_path,
    );
    if find && min_path > step {
      min_path = step;
      let rate = format!("{}/{}", index + 1, start_arr.len());
      println!("rate={:?} start={:?} step={:?}", rate, start, min_path - 1);
    }
  }

  println!("{:?} cost_time={:?}", min_path, now.elapsed());
}

fn find_end_len(
  pos_wrap: &RefCell<Point>,
  end_wrap: &RefCell<Point>,
  map: &Map,
  map_space: &mut HashMap<String, usize>,
  path: &Vec<(usize, usize, String)>,
  min_step: &usize,
) -> (bool, usize) {
  let pos = pos_wrap.borrow();
  let key = format!("{}:{}", pos.x, pos.y);

  // 不能超过最大的次数
  if min_step <= &path.len() {
    return (false, 0);
  }

  match map_space.get(&key) {
    Some(v) => {
      if v <= &path.len() {
        return (false, 0);
      }
      map_space.insert(key, path.len());
    }
    None => {
      map_space.insert(key, path.len());
    }
  }

  if pos.is_end() {
    return (true, path.len());
  }

  let mut cur_arr = Vec::new();
  for dir in pos.get_move_dir().iter() {
    let (x, y) = get_dir_pos(&pos_wrap.borrow(), dir, map).unwrap();
    if path_has_point(&path, (x, y)) {
      continue;
    }
    let next_pos = map.get_point(x, y).unwrap();
    cur_arr.push(next_pos);
  }

  let end_ref = end_wrap.borrow();
  cur_arr.sort_by(|a, b| {
    let a_ref = a.borrow();
    let b_ref = b.borrow();
    let a_space = (end_ref.x as i32 - a_ref.x as i32).abs()
      + (end_ref.y as i32 - a_ref.y as i32).abs();
    let b_space = (end_ref.x as i32 - b_ref.x as i32).abs()
      + (end_ref.y as i32 - b_ref.y as i32).abs();

    a_space.cmp(&b_space)
  });

  let mut find_arr = Vec::new();
  for next_pos in cur_arr.iter() {
    let mut clone_path = path.clone();
    let next_ref = next_pos.borrow();
    let x = next_ref.x;
    let y = next_ref.y;

    clone_path.push((x, y, String::from(&next_ref.letter)));
    let (find, step) =
      find_end_len(next_pos, end_wrap, map, map_space, &clone_path, min_step);
    if find == true {
      find_arr.push(step)
    }
  }

  if find_arr.len() != 0 {
    find_arr.sort_by(|a, b| a.cmp(&b));
    return (true, find_arr[0]);
  }

  return (false, 0);
}
