use std::collections::HashMap;

use super::utils::zip_arr;

#[derive(Debug, Clone)]
pub struct LoopItem {
  pub time: i32,
  pub robot_map: HashMap<RobotType, i32>,
  pub resource_list: HashMap<RobotType, i32>,
}
impl LoopItem {
  pub fn new(time: i32, robot_map: HashMap<RobotType, i32>) -> Self {
    LoopItem {
      time,
      robot_map,
      resource_list: HashMap::new(),
    }
  }
  pub fn is_end(&self) -> bool {
    self.time <= 0
  }
  pub fn update_time(&mut self) -> &HashMap<RobotType, i32> {
    self.time -= 1;
    for (type_name, num) in self.robot_map.iter_mut() {
      self
        .resource_list
        .entry(type_name.clone())
        .and_modify(|x| *x += num.clone())
        .or_insert(1);
    }
    &self.resource_list
  }
  pub fn set_res(&mut self, new_res: HashMap<RobotType, i32>) {
    self.resource_list = new_res
  }
  pub fn add_robot_num(&mut self, type_name: &RobotType, num: i32) {
    if let Some(v) = self.robot_map.get_mut(type_name) {
      *v += num
    } else {
      self.robot_map.insert(type_name.clone(), num);
    }
  }
}

#[derive(Debug)]
pub struct Blueprint {
  pub id: i32,
  pub robots: Vec<Robot>,
}

impl Blueprint {
  pub fn new(id: i32, robots: Vec<Robot>) -> Self {
    let mut robot = Blueprint { id, robots };
    // robot.calc_rate();
    robot
  }
  pub fn get_robot(&self, type_name: &RobotType) -> Option<&Robot> {
    self.robots.iter().find(|item| item.type_name == *type_name)
  }
  pub fn init_rate(&mut self) {
    let robots = &self.robots;
    let target_robot_wrap = robots
      .iter()
      .find(|item| item.type_name == RobotType::Geode);

    let target_robot = match target_robot_wrap {
      None => return,
      Some(t) => t,
    };
    let rate_arr = target_robot.cost.iter().map(|item| 1.0).collect::<Vec<_>>();
    // 前面是矿的比例，后面是机器的比例（机器的比例要*剩余时间）
    let mut rate_map: HashMap<RobotType, Vec<f64>> = HashMap::new();

    let mut cur_loop_list = vec![(target_robot, rate_arr)];
    loop {
      let mut cur_loop: Vec<(&Robot, Vec<f64>)> = vec![];
      for (robot, robot_rate_arr) in cur_loop_list.iter() {
        let cost = &robot.cost;

        for (index, (cost_type, cost_num)) in cost.iter().enumerate() {
          let cur_rate_arr = robot_rate_arr
            .clone()
            .iter_mut()
            .enumerate()
            .map(|(_index, v)| {
              if index == _index {
                return 1.0 / *cost_num as f64;
              }
              return 0.0;
            })
            .collect::<Vec<_>>();

          rate_map
            .entry(cost_type.clone())
            .and_modify(|x| *x = zip_arr(x.clone(), cur_rate_arr.clone()))
            .or_insert(cur_rate_arr.clone());

          match self.get_robot(cost_type) {
            None => {}
            Some(robot) => {
              cur_loop.push((robot, cur_rate_arr.clone()));
            }
          }
        }
      }
      cur_loop_list = cur_loop;
    }

    println!(
      "cur_loop_list:>{:?}\nrate_map={:?}",
      cur_loop_list, rate_map
    );
  }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RobotType {
  Ore,
  Clay,
  Obsidian,
  Geode,
}
impl RobotType {
  pub fn form_str(s: &str) -> Self {
    match s {
      "ore" => RobotType::Ore,
      "clay" => RobotType::Clay,
      "obsidian" => RobotType::Obsidian,
      "geode" => RobotType::Geode,
      _ => panic!("invalid cost type"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Robot {
  pub type_name: RobotType,
  pub cost: Vec<(RobotType, i32)>,
}

impl Robot {
  pub fn new(name: &str, cost: Vec<(RobotType, i32)>) -> Self {
    let type_name = RobotType::form_str(name);
    Robot { type_name, cost }
  }
  pub fn calc_rate(&mut self) {}
}

// #[derive(Debug, Clone)]
// pub enum CostType {
//     Ore,
//     Clay,
//     Obsidian,
// }

// impl CostType {
//     pub fn form_str(s: &str) -> Self {
//         match s {
//             "ore" => CostType::Ore,
//             "clay" => CostType::Clay,
//             "obsidian" => CostType::Obsidian,
//             _ => panic!("invalid cost type"),
//         }
//     }
// }
