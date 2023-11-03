use std::{
  cmp::{self, Ordering},
  collections::HashMap,
  ops::Add,
};

use super::blueprint::{self, Blueprint, LoopItem, Robot, RobotType};

pub fn get_next(item: &LoopItem, blueprint: &Blueprint) -> Vec<LoopItem> {
  let new_item = item.clone();

  let mut index = 0;
  let mut list = vec![new_item];
  loop {
    let mut add_list = vec![];
    let mut remove_list = vec![];
    for (cur_index, item) in list.iter().enumerate() {
      let res_map = &item.resource_map;
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
          new_item.add_temp_robot_num(&cur_robot.type_name, i);
        }
        new_item.calc_rate_value(blueprint);
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

  for item in list.iter_mut() {
    item.update_time();
    item.update_robot_num();
    item.calc_rate_value(blueprint);
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
) {
  let big_num = match complete_list.get(0) {
    None => 0,
    Some(t) => t.value,
  };

  println!("big_num:{:?}", big_num);
  let max_num = 100;
  if loop_list.len() == 0 {
    let change_num = cmp::min(max_num, save_list.len());
    let add_list = save_list.split_off(save_list.len() - change_num);

    loop_list.extend(add_list);
  }

  loop_list.retain(|item| {
    if item.is_end() {
      if item.value > big_num {
        complete_list.insert(0, item.clone());
      }
      return false;
    }
    return true;
  });

  if loop_list.len() == 0 {
    return;
  }

  loop_list.sort_by(|a, b| {
    let ord = b.value.cmp(&a.value);
    if ord != Ordering::Equal {
      return ord;
    }

    b.rate.partial_cmp(&a.rate).unwrap()
  });

  let big_value = loop_list[0].value;
  let big_rate = loop_list[0].rate;
  let first_time = loop_list[0].time;

  // println!(
  //   "time={} | obot_map:{:?} | resource_map={:?}",
  //   first_time, loop_list[0].robot_map, loop_list[0].resource_map
  // );

  if loop_list.len() > max_num {
    let temp_save_ele = loop_list.split_off(max_num);
    save_list.extend(temp_save_ele);
  }

  // loop_list.retain(|item| {
  //   if item.value != big_value || item.rate != big_rate {
  //     save_list.push(item.clone());
  //     return false;
  //   }
  //   return true;
  // });

  //   println!(
  //     "big_value:{:?} | big_rate:{:?}| first_time:{:?}| loop_list.len():{:?}|",
  //     big_value,
  //     big_rate,
  //     first_time,
  //     loop_list.len()
  //   );
}

pub fn calc_rate(item: &LoopItem, blueprint: &Blueprint) -> f64 {
  let rate_map = match &blueprint.rate_map {
    None => return 0.0,
    Some(t) => t,
  };
  let time = &item.time;
  let resource_map = &item.resource_map;
  let robot_map = &item.robot_map;

  let mut rate = 0.0;

  let mut rate_arr: Vec<f64> = vec![0.0; blueprint.get_rate_len()];
  for (type_name, item_num) in robot_map.iter() {
    let mut rate_unit = match rate_map.get(type_name) {
      None => {
        continue;
      }
      Some(t) => t.clone(),
    };

    rate_unit = rate_unit
      .into_iter()
      .map(|item| item * item_num.clone() as f64 * time.clone() as f64)
      .collect();

    rate_arr = zip_arr(rate_arr, rate_unit);
  }

  for (type_name, item_num) in resource_map.iter() {
    let mut rate_unit = match rate_map.get(type_name) {
      None => {
        continue;
      }
      Some(t) => t.clone(),
    };

    rate_unit = rate_unit
      .into_iter()
      .map(|item| item * item_num.clone() as f64)
      .collect();

    rate_arr = zip_arr(rate_arr, rate_unit);
  }

  let min = match rate_arr.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
    None => 0.0,
    Some(t) => t.clone(),
  };

  rate + min
}

pub fn zip_arr<T>(arr1: Vec<T>, arr2: Vec<T>) -> Vec<T>
where
  T: Add<Output = T> + Copy,
{
  arr1.iter().zip(arr2.iter()).map(|(&x, &y)| x + y).collect()
}
