#![allow(unused)]
use std::collections::HashMap;

use regex::Regex;

use crate::utils::read_file;

use self::{
  blueprint::{Blueprint, RobotType},
  utils::{calc_top_list, get_next},
};

mod blueprint;
mod utils;
use blueprint::*;

pub fn parse() {
  parse1();
  // parse2();
}

fn parse1() {
  let blueprint_list = parse_input1();

  let mut robot_map = HashMap::new();
  robot_map.insert(RobotType::Ore, 1);
  let begin = LoopItem::new(18, robot_map);

  let mut res = vec![];
  for blueprint in blueprint_list.iter() {
    let mut complete_list: Vec<LoopItem> = vec![];
    let mut save_list: Vec<LoopItem> = vec![];
    let mut loop_list: Vec<LoopItem> = vec![begin.clone()];

    loop {
      let mut remove_list = vec![];
      let mut add_list = vec![];
      for (index, item) in loop_list.iter().enumerate() {
        let next = get_next(item, blueprint);

        // println!("next:{:?} | loop_list:{:?} ", next.len(), loop_list.len());

        if next.len() == 0 {
          continue;
        }
        remove_list.push(index);
        add_list.extend(next);
      }
      //   println!(
      //     "complete_list:{:?} | add_list:{:?} | loop_list:{:?} | save_list:{:?}|",
      //     complete_list.len(),
      //     add_list.len(),
      //     loop_list.len(),
      //     save_list.len()
      //   );

      for index in remove_list.into_iter().rev() {
        loop_list.remove(index);
      }
      loop_list.extend(add_list);

      if loop_list.len() == 0 && save_list.len() == 0 {
        break;
      }

      calc_top_list(
        &mut loop_list,
        &mut save_list,
        &mut complete_list,
        blueprint,
      );

      //   println!("loop_list:{:?}", loop_list);
      //   println!(
      //     "complete_list:{:?} | loop_list:{:?} | save_list:{:?}|",
      //     complete_list,
      //     loop_list.len(),
      //     save_list.len()
      //   );
    }

    let num = complete_list[0].value;
    res.push((blueprint.id.clone(), num));
  }

  println!("res:{:?}", res);
}

fn parse2() {
  let content = parse_input2();
  println!("{:?}", content.len());
}

fn parse_input1() -> Vec<Blueprint> {
  let title_reg = Regex::new(r"Blueprint (\d+):").unwrap();
  let robot_reg = Regex::new(
    r"Each (\w+) robot costs (\d+) ore( and (\d+) (clay|obsidian)+)?\.",
  )
  .unwrap();
  let content = read_file("day19/demo.txt").unwrap();

  content
    .split("\n\n")
    .map(|block| {
      let lines = block.split("\n").collect::<Vec<_>>();
      let title_find = title_reg.captures(lines[0]);
      let name = title_find
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();

      let mut robots = Vec::new();
      for i in 1..lines.len() {
        let line_find = robot_reg.captures(lines[i]).unwrap();
        let name: &str = line_find.get(1).unwrap().as_str();
        let cost_ore: i32 = line_find.get(2).unwrap().as_str().parse().unwrap();
        let mut cost_arr = vec![(RobotType::Ore, cost_ore)];

        match line_find.get(5) {
          None => {}
          Some(t) => {
            let extra_name = RobotType::form_str(t.as_str());
            let extra_num: i32 =
              line_find.get(4).unwrap().as_str().parse().unwrap();

            cost_arr.push((extra_name, extra_num));
          }
        };

        let robot = Robot::new(name, cost_arr);
        robots.push(robot);
      }

      Blueprint::new(name, robots)
    })
    .collect::<Vec<_>>()
}

fn parse_input2() -> Vec<Blueprint> {
  let title_reg = Regex::new(r"Blueprint (\d+)").unwrap();
  let robot_reg = Regex::new(
    r"Each (\w+) robot costs (\d+) ore( and (\d+) (clay|obsidian)+)?",
  )
  .unwrap();
  let content = read_file("day19/input.txt").unwrap();

  content
    .split("\n")
    .map(|block| {
      let lines = block.split(":").collect::<Vec<_>>();
      let title_find = title_reg.captures(lines[0]);
      let name = title_find
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();

      let line_list = lines[1]
        .split(".")
        .filter(|item| item.len() > 0)
        .collect::<Vec<_>>();

      let mut robots = Vec::new();
      for line in line_list {
        let line_find = robot_reg.captures(line).unwrap();
        let name: &str = line_find.get(1).unwrap().as_str();
        let cost_ore: i32 = line_find.get(2).unwrap().as_str().parse().unwrap();
        let mut cost_arr = vec![(RobotType::Ore, cost_ore)];

        match line_find.get(5) {
          None => {}
          Some(t) => {
            let extra_name = RobotType::form_str(t.as_str());
            let extra_num: i32 =
              line_find.get(4).unwrap().as_str().parse().unwrap();

            cost_arr.push((extra_name, extra_num));
          }
        };

        let robot = Robot::new(name, cost_arr);
        robots.push(robot);
      }

      Blueprint::new(name, robots)
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::day19::{blueprint::RobotType, utils::calc_rate};

  use super::{
    blueprint::{LoopItem, Robot},
    utils::get_next,
  };

  #[test]
  fn test_get_next() {
    let blueprints = super::parse_input1();
    let mut robot_map = HashMap::new();
    robot_map.insert(RobotType::Ore, 1);
    let mut begin = LoopItem::new(24, robot_map);

    let mut resource_list = begin.resource_map.clone();
    resource_list.insert(RobotType::Ore, 3);

    begin.set_res(resource_list);

    let next_list = get_next(&mut begin, blueprints.get(0).unwrap());
    println!("test:>{:?}\n{:?}", next_list.len(), next_list);
  }
  #[test]
  fn test_init_rate() {
    let mut blueprints = super::parse_input1();
    // blueprints[0].init_rate();
    println!("test:>{:?}", blueprints[0].rate_map);
  }
  #[test]
  fn test_calc_rate() {
    let mut blueprints = super::parse_input1();
    let blueprint = blueprints.remove(0);

    let mut robot_map = HashMap::new();
    robot_map.insert(RobotType::Ore, 1);
    // robot_map.insert(RobotType::Geode, 1);
    let begin = LoopItem::new(24, robot_map);

    // blueprints[0].init_rate();
    println!("test:> {:?}", calc_rate(&begin, &blueprint));
  }
}
