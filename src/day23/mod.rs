#![allow(unused)]
use std::{
  cell::{RefCell, RefMut},
  collections::HashMap,
  time::Instant,
};

use crate::utils::read_file;

mod map;
mod utils;
use map::*;
use utils::*;

pub fn parse() {
  // parse1();
  parse2();
}

fn parse1() {
  let move_order =
    [MoveDir::North, MoveDir::South, MoveDir::West, MoveDir::East];

  let mut map = parse_input();
  let now = Instant::now();
  let elves = map.get_elf_points();

  let mut index = 0;
  'outer: loop {
    let mut move_map: HashMap<usize, (i32, i32)> = HashMap::new();
    for (item_index, item) in elves.iter().enumerate() {
      let nearby_points = get_nearby_points(item, &elves);

      for num in 0..4 {
        let cur_dir = match move_order.get((index + num) % 4) {
          Some(dir) => dir,
          None => unreachable!(),
        };

        let not_move = not_need_move(item, &nearby_points);
        if not_move {
          break;
        }
        let is_move = can_move(item, cur_dir, &nearby_points);
        if is_move {
          let next_p = get_next_point(item, cur_dir);
          move_map.insert(item_index, next_p);
          break;
        }
      }
    }

    if move_map.len() == 0 {
      break 'outer;
    }

    remove_dul_p(&mut move_map);
    for (i, p) in move_map.into_iter() {
      elves[i].borrow_mut().update_pos(p);
    }

    if index >= 9 {
      break;
    }

    index += 1;
  }

  let range = get_range(&elves);
  map.update_range(range);

  let round = index + 1;
  println!(
    "time={:?} | index={} get_empty_size:{:?}",
    now.elapsed(),
    round,
    map.get_empty_size()
  );
}

fn parse2() {
  let move_order =
    [MoveDir::North, MoveDir::South, MoveDir::West, MoveDir::East];

  let mut map = parse_input();
  let now = Instant::now();
  let elves = map.get_elf_points();

  let mut index = 0;
  'outer: loop {
    let mut move_map: HashMap<usize, (i32, i32)> = HashMap::new();
    for (item_index, item) in elves.iter().enumerate() {
      let nearby_points = get_nearby_points(item, &elves);

      for num in 0..4 {
        let cur_dir = match move_order.get((index + num) % 4) {
          Some(dir) => dir,
          None => unreachable!(),
        };

        let not_move = not_need_move(item, &nearby_points);
        if not_move {
          break;
        }
        let is_move = can_move(item, cur_dir, &nearby_points);
        if is_move {
          let next_p = get_next_point(item, cur_dir);
          move_map.insert(item_index, next_p);
          break;
        }
      }
    }

    if move_map.len() == 0 {
      break 'outer;
    }

    remove_dul_p(&mut move_map);
    for (i, p) in move_map.into_iter() {
      elves[i].borrow_mut().update_pos(p);
    }

    index += 1;
  }

  let range = get_range(&elves);
  map.update_range(range);

  let round = index + 1;
  println!(
    "time={:?} | index={} get_empty_size:{:?}",
    now.elapsed(),
    round,
    map.get_empty_size()
  );
  // println!("{:?}", map);
}

fn parse_input() -> Map {
  let content = read_file("day23/input.txt").unwrap();

  let points = content
    .split("\n")
    .enumerate()
    .map(|(y, line)| {
      line
        .split("")
        .filter(|&item| item != "")
        .enumerate()
        .map(|(x, p)| RefCell::new(Point::new(x as i32, y as i32, p == "#")))
        .collect::<Vec<_>>()
    })
    .flat_map(|v| v)
    .collect::<Vec<_>>();

  let range = get_range(&points.iter().collect());

  let elves_ps = points
    .into_iter()
    .filter(|item| item.borrow().has_elf)
    .collect::<Vec<_>>();

  let mut map = Map::new(elves_ps);
  map.update_range(range);

  map
}
