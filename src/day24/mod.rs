#![allow(unused)]
use std::vec;

use crate::utils::read_file;

mod map;
mod utils;

use map::*;
use utils::*;

pub fn parse() {
  parse1();
  // parse2();
}

fn parse1() {
  let mut map = parse_input();
  let start = map.get_start();
  // let end = map.get_p(&(1,));

  // println!("start{:?}|end={:?}", start,);

  let mut move_key = MoveKey::new(vec![start]);
  let mut completed_keys = vec![];
  let mut temp_keys = vec![];
  let mut loop_keys = vec![move_key];

  let mut loop_index = 0;
  loop {
    update_map(&map);

    let mut remove_list = vec![];
    let mut add_list = vec![];
    for (index, cur_key) in loop_keys.iter().enumerate() {
      let next_keys = get_move_next(cur_key, &map);
      if next_keys.len() == 0 {
        remove_list.push(index);
        continue;
      }
      remove_list.push(index);
      add_list.extend(next_keys);
    }
    for index in remove_list.into_iter().rev() {
      loop_keys.remove(index);
    }
    loop_keys.extend(add_list);

    if loop_index % 10000 == 0 {
      println!(
        "num={:?}| temp_keys={:?} | loop_keys={}",
        get_complete_num(&completed_keys),
        temp_keys.len(),
        loop_keys.len()
      );
    }
    calc_top_keys(&mut loop_keys, &mut temp_keys, &mut completed_keys, &map);

    if loop_keys.len() == 0 && temp_keys.len() == 0 {
      break;
    }

    loop_index += 1;
    // println!("{:?}", map);
  }

  let num = completed_keys[0].get_num() - 1;
  println!("num={:?}", num);
  println!("{:?}", map);
}

fn parse_input() -> Map {
  let content = read_file("day24/input.txt").unwrap();

  Map::from_str(&content)
}

fn get_num() {}
