#![allow(unused)]
use std::time::Instant;

use crate::utils::read_file;

pub fn parse() {
  // parse1();

  parse2();
}

fn parse1() {
  let ori_arr = parse_input();
  let now = Instant::now();
  let mut loop_arr = ori_arr.into_iter().enumerate().collect::<Vec<_>>();

  let mut i = 0;
  let end_num = loop_arr.len();
  loop {
    if i >= end_num {
      break;
    }
    let change_pos = loop_arr
      .iter()
      .position(|(index, num)| *index == i)
      .unwrap();

    let item = loop_arr[change_pos];

    loop_arr.remove(change_pos);

    let mut new_index = change_pos as i64 + item.1;
    if new_index < 0 {
      new_index = new_index % loop_arr.len() as i64 + loop_arr.len() as i64;
    } else if new_index >= loop_arr.len() as i64 {
      new_index = new_index % loop_arr.len() as i64;
    }

    if item.1 < 0 && new_index == 0 {
      new_index = loop_arr.len() as i64;
    }

    loop_arr.insert(new_index as usize, item);
    i += 1;
  }

  let find_arr = [1000, 2000, 3000];
  let mut num = 0;
  let zero_index = loop_arr.iter().position(|&x| x.1 == 0).unwrap();
  for item in find_arr {
    let index = (item + zero_index) % loop_arr.len();
    let cur_num = loop_arr[index];
    println!("index={} | cur_num={:?}", index, cur_num);
    num += cur_num.1;
  }

  println!("time={:?}\nnum={}", now.elapsed(), num);
}

fn parse2() {
  let mut ori_arr = parse_input();
  let now = Instant::now();
  let mut loop_arr = ori_arr
    .into_iter()
    .map(|x| x * 811589153)
    .enumerate()
    .collect::<Vec<_>>();

  let mut i = 0;
  let end_num = loop_arr.len() * 10;
  loop {
    if i >= end_num {
      break;
    }
    let cur_index = i % loop_arr.len();

    let change_pos = loop_arr
      .iter()
      .position(|(index, num)| *index == cur_index)
      .unwrap();

    let item = loop_arr[change_pos];

    loop_arr.remove(change_pos);

    let mut new_index = change_pos as i64 + item.1;
    if new_index < 0 {
      new_index = new_index % loop_arr.len() as i64 + loop_arr.len() as i64;
    } else if new_index >= loop_arr.len() as i64 {
      new_index = new_index % loop_arr.len() as i64;
    }

    loop_arr.insert(new_index as usize, item);
    i += 1;
  }

  let find_arr = [1000, 2000, 3000];
  let mut num = 0;
  let zero_index = loop_arr.iter().position(|&x| x.1 == 0).unwrap();
  for item in find_arr {
    let index = (item + zero_index) % loop_arr.len();
    let cur_num = loop_arr[index];
    println!("index={} | cur_num={:?}", index, cur_num);
    num += cur_num.1;
  }

  println!("time={:?}\nnum={}", now.elapsed(), num);
}

fn parse_input() -> Vec<i64> {
  let content = read_file("day20/input.txt").unwrap();
  content
    .split("\n")
    .map(|item| item.parse::<i64>().unwrap())
    .collect::<Vec<_>>()
}
