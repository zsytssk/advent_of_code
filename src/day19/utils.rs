use std::{cmp, collections::HashMap, ops::Add};

use super::blueprint::{self, Blueprint, LoopItem, Robot, RobotType};

pub fn get_next(item: &LoopItem, blueprint: &Blueprint) -> Vec<LoopItem> {
  let mut new_item = item.clone();
  new_item.update_time();

  let mut index = 0;
  let mut list = vec![new_item];
  loop {
    let mut add_list = vec![];
    let mut remove_list = vec![];
    for (cur_index, item) in list.iter().enumerate() {
      let res_map = &item.resource_list;
      let cur_robot = &blueprint.robots[index];
      let num = can_create_num(res_map, cur_robot);
      if num == 0 {
        continue;
      }
      remove_list.push(cur_index);
      for i in 0..=num {
        let mut new_item = item.clone();
        if i > 0 {
          let new_res_map = minus_res(&res_map, cur_robot, i);
          new_item.set_res(new_res_map);
          new_item.add_robot_num(&cur_robot.type_name, i);
        }
        add_list.push(new_item);
      }
    }

    for index in remove_list.into_iter().rev() {
      list.remove(index);
    }
    list.extend(add_list);

    index += 1;
    if index >= blueprint.robots.len() {
      break;
    }
  }

  list
}

pub fn can_create_num(res: &HashMap<RobotType, i32>, robot: &Robot) -> i32 {
  let mut num = 0;

  let cost = &robot.cost;

  let mut num_arr = Vec::new();
  for (cost_type, cost_num) in cost.iter() {
    let has_num = match res.get(cost_type) {
      None => 0,
      Some(t) => *t,
    };
    let num_item = has_num / cost_num;
    num_arr.push(num_item)
  }
  num_arr.sort_by(|a, b| a.cmp(&b));

  num_arr.remove(0)
}

pub fn minus_res(
  res: &HashMap<RobotType, i32>,
  robot: &Robot,
  num: i32,
) -> HashMap<RobotType, i32> {
  let cost = &robot.cost;
  let mut new_res = res.clone();

  if num == 0 {
    return new_res;
  }

  for (cost_type, cost_num) in cost.iter() {
    let minus_num = num * cost_num;
    new_res
      .entry(cost_type.clone())
      .and_modify(|x| *x -= minus_num);
  }

  new_res
}

pub fn calc_top_list(
  loop_list: &mut Vec<LoopItem>,
  save_list: &mut Vec<LoopItem>,
  complete_list: &mut Vec<LoopItem>,
  blueprint: &Blueprint,
) -> Vec<LoopItem> {
  // 对比分数 + 对比时间
  // loop_list.retain(|item| {
  //     if item.is_end() &&
  // })
  todo!()
}

pub fn zip_arr<T>(arr1: Vec<T>, arr2: Vec<T>) -> Vec<T>
where
  T: Add<Output = T> + Copy,
{
  arr1.iter().zip(arr2.iter()).map(|(&x, &y)| x + y).collect()
}
