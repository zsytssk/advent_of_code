#![allow(unused)]
use std::{
  borrow::BorrowMut,
  collections::{HashMap, HashSet},
  hash::Hash,
};

use crate::utils::read_file;
mod map;

use map::*;

pub fn parse() {
  // parse1();
  parse2();
}

fn parse1() {
  let mut map = parse_input();

  let line_index: i32 = 10;
  let mut range_list = Vec::new();
  let has_num = 0;
  for item in map.sensor_and_bean.iter() {
    let sensor = Point::new(item.0 .0, item.0 .1);
    let bean = Point::new(item.1 .0, item.1 .1);
    let sensor_distance = sensor.distance_from_point(&bean);
    let line_dis = sensor.distance_from_line(line_index);
    let distance_extra = sensor_distance - line_dis;

    if distance_extra < 0 {
      continue;
    }

    let empty_range_x = (sensor.x - distance_extra, sensor.x + distance_extra);
    range_list.push((sensor.x - distance_extra, sensor.x + distance_extra))
  }

  let mut uniques = HashSet::new();
  let mut remove_list = map
    .sensor_and_bean
    .iter()
    .map(|item| vec![item.0, item.1])
    .collect::<Vec<_>>()
    .concat()
    .iter()
    .map(|item| (item.0, item.1))
    .filter(|item| uniques.insert(format!("{},{}", item.0, item.1)))
    .filter(|item| item.1 == line_index)
    .collect::<Vec<_>>();

  println!("remove_num={:?}\nrange_list:{:?}", remove_list, range_list);
  let new_list = merge_ranges(range_list);
  let num = get_ranges_num(&new_list) - remove_list.len() as i32;
  println!("num={:?}\nnew_list:{:?}", num, new_list);
}

fn parse2() {
  let mut map = parse_input();
  let mix_x = 0;
  let max_x = 4000000;
  for line_index in 0..=4000000 {
    let mut range_list = Vec::new();
    let has_num = 0;
    for item in map.sensor_and_bean.iter() {
      let sensor = Point::new(item.0 .0, item.0 .1);
      let bean = Point::new(item.1 .0, item.1 .1);
      let sensor_distance = sensor.distance_from_point(&bean);
      let line_dis = sensor.distance_from_line(line_index);
      let distance_extra = sensor_distance - line_dis;

      if distance_extra < 0 {
        continue;
      }

      let empty_range_x =
        (sensor.x - distance_extra, sensor.x + distance_extra);
      range_list.push((sensor.x - distance_extra, sensor.x + distance_extra))
    }

    let new_list = merge_ranges(range_list);
    if new_list.len() == 0
      || (new_list.len() == 1
        && new_list[0].0 <= mix_x
        && new_list[0].1 >= max_x)
    {
      continue;
    }
    println!("line_index={:?} new_list:{:?}", line_index, new_list);
  }
}

fn parse_input() -> Map {
  let content = read_file("day15/input.txt").unwrap();
  let regex =
    regex::Regex::new(r"Sensor at ([^\:]+): closest beacon is at ([^\:]+)")
      .unwrap();

  let info = content
    .split("\n")
    .map(|line| {
      let m = regex.captures(line).unwrap();
      let (_, match_list): (&str, [&str; 2]) = m.extract();
      (
        Point::from_str(match_list[0]),
        Point::from_str(match_list[1]),
      )
    })
    .collect::<Vec<_>>();

  let mut map = Map::new(info);
  map
}

fn get_map_range(info: &Vec<(Point, Point)>) -> ((i32, i32), (i32, i32)) {
  let mut min_x = 0;
  let mut max_x = 0;
  let mut min_y = 0;
  let mut max_y = 0;

  let points = info
    .iter()
    .map(|item| vec![&item.0, &item.1])
    .collect::<Vec<_>>()
    .concat();

  for item in points.iter() {
    if item.x < min_x {
      min_x = item.x;
    }
    if item.x > max_x {
      max_x = item.x;
    }
    if item.y < min_y {
      min_y = item.y;
    }
    if item.y > max_y {
      max_y = item.y;
    }
  }

  return ((min_x, max_x), (min_y, max_y));
}

fn merge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
  ranges.sort_by(|a, b| a.0.cmp(&b.0));

  let mut res: Vec<(i32, i32)> = Vec::new();
  for item in ranges.iter() {
    let len = res.len();
    if len == 0 {
      res.push((item.0, item.1));
      continue;
    }
    let res_last_item = res.get_mut(len - 1).unwrap();
    if item.0 > res_last_item.1 + 1 {
      res.push((item.0, item.1));
    } else {
      if (item.1 > res_last_item.1) {
        res_last_item.1 = item.1;
      }
    }
  }

  res
}

fn get_ranges_num(ranges: &Vec<(i32, i32)>) -> i32 {
  let mut num = 0;
  for item in ranges.iter() {
    num += item.1 - item.0 + 1;
  }

  num
}
