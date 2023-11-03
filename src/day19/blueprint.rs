use std::collections::HashMap;

use super::utils::{calc_rate, zip_arr};

#[derive(Debug, Clone)]
pub struct LoopItem {
  pub time: i32,
  pub robot_map: HashMap<RobotType, i32>,
  pub temp_robot_map: HashMap<RobotType, i32>,
  pub resource_map: HashMap<RobotType, i32>,
  pub value: i32,
  pub rate: f64,
}
impl LoopItem {
  pub fn new(time: i32, robot_map: HashMap<RobotType, i32>) -> Self {
    LoopItem {
      time,
      robot_map,
      resource_map: HashMap::new(),
      temp_robot_map: HashMap::new(),
      value: 0,
      rate: 0.0,
    }
  }
  pub fn is_end(&self) -> bool {
    self.time <= 0
  }
  pub fn update_time(&mut self) -> &HashMap<RobotType, i32> {
    self.time -= 1;
    for (type_name, num) in self.robot_map.iter_mut() {
      self
        .resource_map
        .entry(type_name.clone())
        .and_modify(|x| {
          let new_num = x.clone() + num.clone();
          if type_name == &RobotType::Geode {
            self.value = new_num;
          }
          *x = new_num
        })
        .or_insert(num.clone());
    }
    &self.resource_map
  }
  pub fn update_robot_num(&mut self) {
    for item in self.temp_robot_map.iter() {
      if let Some(v) = self.robot_map.get_mut(item.0) {
        *v += item.1
      } else {
        self.robot_map.insert(item.0.clone(), item.1.clone());
      }
    }

    self.temp_robot_map.clear();
  }
  pub fn set_res(&mut self, new_res: HashMap<RobotType, i32>) {
    self.resource_map = new_res
  }
  pub fn add_temp_robot_num(&mut self, type_name: &RobotType, num: i32) {
    if let Some(v) = self.temp_robot_map.get_mut(type_name) {
      *v += num
    } else {
      self.temp_robot_map.insert(type_name.clone(), num);
    }
  }
  pub fn calc_rate_value(&mut self, blueprint: &Blueprint) {
    self.rate = calc_rate(self, blueprint);
    match self.resource_map.get(&RobotType::Geode) {
      None => {}
      Some(t) => self.value = t.clone(),
    }
  }
}

#[derive(Debug)]
pub struct Blueprint {
  pub id: i32,
  pub robots: Vec<Robot>,
  pub rate_map: Option<HashMap<RobotType, Vec<f64>>>,
}

impl Blueprint {
  pub fn new(id: i32, robots: Vec<Robot>) -> Self {
    let mut robot = Blueprint {
      id,
      robots,
      rate_map: None,
    };
    robot.init_rate();
    robot
  }
  pub fn get_robot(&self, type_name: &RobotType) -> Option<&Robot> {
    self.robots.iter().find(|item| item.type_name == *type_name)
  }
  pub fn get_rate_len(&self) -> usize {
    match &self.rate_map {
      None => 0,
      Some(t) => t.get(&RobotType::Geode).unwrap().len(),
    }
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
    // 前面是矿的比例，后面是机器的比例（机器的比例要*剩余时间）
    let mut rate_map: HashMap<RobotType, Vec<f64>> = HashMap::new();
    rate_map.insert(
      target_robot.type_name.clone(),
      vec![1.0; target_robot.cost.len()],
    );

    for (index, (cost_type, cost_num)) in target_robot.cost.iter().enumerate() {
      let mut arr = vec![0.0; target_robot.cost.len()];
      arr[index] = 1.0 / cost_num.clone() as f64;
      rate_map.insert(cost_type.clone(), arr);
    }

    let mut cur_loop_list = target_robot
      .cost
      .clone()
      .into_iter()
      .map(|item| self.get_robot(&item.0).unwrap())
      .collect::<Vec<_>>();

    let mut loop_type_arr = vec![target_robot.type_name.clone()];
    loop {
      let mut cur_loop: Vec<&Robot> = vec![];
      for robot in cur_loop_list.iter() {
        let cost = &robot.cost;
        if loop_type_arr.contains(&robot.type_name) {
          continue;
        }
        loop_type_arr.push(robot.type_name.clone());
        let cur_rate_arr = rate_map.get(&robot.type_name).unwrap().clone();

        for (cost_type, cost_num) in cost.iter() {
          let new_rate = cur_rate_arr
            .clone()
            .into_iter()
            .map(|item| item * 0.5 / cost_num.clone() as f64)
            .collect::<Vec<_>>();

          rate_map
            .entry(cost_type.clone())
            .and_modify(|x| {
              let new_arr = zip_arr(x.clone(), new_rate.clone());
              let big_num = new_arr
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                .clone();

              *x = new_arr
                .into_iter()
                .map(|item| {
                  if item == big_num {
                    return item;
                  }
                  return 0.0;
                })
                .collect()
            })
            .or_insert(new_rate);

          match self.get_robot(cost_type) {
            None => {}
            Some(robot) => {
              cur_loop.push(robot);
            }
          }
        }
      }

      loop_type_arr.dedup();
      if loop_type_arr.len() == robots.len() {
        break;
      }
      cur_loop_list = cur_loop;
    }

    self.rate_map = Some(rate_map);
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

#[derive(Debug, Clone, PartialEq)]
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
