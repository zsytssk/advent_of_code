use std::{
  cell::{Ref, RefCell, RefMut},
  cmp::{self, Ordering},
  collections::HashMap,
};

#[derive(Debug, Clone)]
pub enum TypePath {
  Type1 = 1,
  Type2 = 2,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MapKey {
  path1: Vec<String>,
  path2: Vec<String>,
  time1: i32,
  time2: i32,
  complete_path_size: usize,
}

impl MapKey {
  pub fn new(
    path: Vec<String>,
    time1: i32,
    time2: i32,
    complete_path_size: usize,
  ) -> Self {
    MapKey {
      path1: path.clone(),
      path2: path,
      time1: time1,
      time2: time2,
      complete_path_size,
    }
  }
  pub fn rest_time(&self) -> i32 {
    self.time1 + self.time2
  }
  pub fn get_path_len(&self, type_path: &TypePath) -> usize {
    match type_path {
      TypePath::Type1 => self.path1.len(),
      TypePath::Type2 => self.path2.len(),
    }
  }
  pub fn update_value(
    &mut self,
    type_path: &TypePath,
    new_key: String,
    time: i32,
  ) {
    match type_path {
      TypePath::Type1 => {
        self.time1 = time;
        self.path1.push(new_key);
      }
      TypePath::Type2 => {
        self.time2 = time;
        self.path2.push(new_key);
      }
    };
  }
  pub fn get_rest_key(
    &self,
    type_path: &TypePath,
    short_path: &HashMap<(String, String), usize>,
    map: &Switches,
  ) -> Vec<(String, i32, i32)> {
    let all_keys = map.get_rate_keys();
    let path = match type_path {
      TypePath::Type1 => &self.path1,
      TypePath::Type2 => &self.path2,
    };
    let last_key = path[path.len() - 1].clone();

    let time = match type_path {
      TypePath::Type1 => self.time1,
      TypePath::Type2 => self.time2,
    };

    let mut rest_keys = vec![];
    for item in all_keys {
      let switch = item;
      if self.path1.contains(&switch.name) || self.path2.contains(&switch.name)
      {
        continue;
      }
      let name = switch.name.clone();
      let rate = switch.rate.clone() as i32;
      let cost_time = short_path
        .get(&(last_key.clone(), name.clone()))
        .unwrap()
        .clone() as i32;

      rest_keys.push((name, rate, cost_time));
    }

    rest_keys.sort_by(|a, b| {
      let (a_name, a_rate, a_cost_time) = a;
      let (b_name, b_rate, b_cost_time) = b;

      let b_value = (time - b_cost_time - 1) * b_rate;
      let a_value = (time - a_cost_time - 1) * a_rate;

      b_value.cmp(&a_value)
    });

    rest_keys
  }
  pub fn set_time(&mut self, type_path: &TypePath, time: i32) {
    match type_path {
      TypePath::Type1 => self.time1 = time,
      TypePath::Type2 => self.time2 = time,
    };
  }
  pub fn is_complete(&self) -> bool {
    (self.time1 <= 0 && self.time2 <= 0)
      || self.complete_path_size + 1 == (self.path1.len() + self.path2.len())
  }
  pub fn get_max_score(
    &self,
    type_path: &TypePath,
    short_path: &HashMap<(String, String), usize>,
    map: &Switches,
  ) -> usize {
    // let time = match type_path {
    //     TypePath::Type1 => self.time1,
    //     TypePath::Type2 => self.time2,
    // };
    // let mut rest_keys = self.get_rest_key(type_path, short_path, map);

    // let mut change_time = time;
    // let mut score = 0;
    // for (index, item) in rest_keys.iter().enumerate() {
    //     let (name, rate, cost_time) = item;
    //     if index == 0 {
    //         change_time = change_time - cost_time - 1;
    //     } else {
    //         change_time = cmp::min(time - cost_time - 1, change_time - 2);
    //     }
    //     if change_time <= 0 {
    //         break;
    //     }
    //     score += time * rate;
    // }

    // score as usize
    let mut rest_keys = self.get_rest_key(type_path, short_path, map);
    let mut all = 0 as usize;
    let time1 = self.time1;
    let time2 = self.time2;

    let mut max_val = cmp::max(time1, time2);
    let mut min_val = cmp::min(time1, time2);

    for item in rest_keys.iter() {
      if max_val <= 0 {
        break;
      }
      max_val -= 2;
      if max_val >= 0 {
        all += max_val as usize * item.1 as usize;
      }
      max_val = cmp::max(max_val, min_val)
    }
    all
  }
  pub fn get_next_keys(
    &self,
    type_path: &TypePath,
    all_keys: &Vec<Ref<Switch>>,
    short_path: &HashMap<(String, String), usize>,
    map: &Switches,
  ) -> Vec<(MapKey, usize, usize)> {
    let time = match type_path {
      TypePath::Type1 => self.time1,
      TypePath::Type2 => self.time2,
    };
    let mut rest_keys = self.get_rest_key(type_path, short_path, map);

    rest_keys
      .into_iter()
      .filter(|item| {
        let (name, rate, cost_time) = item;
        let item_time = (time - cost_time - 1);
        return item_time >= 0;
      })
      .map(|item| {
        let (name, rate, cost_time) = item;
        let mut key = self.clone();

        let item_time = time - cost_time - 1;
        key.update_value(type_path, name, item_time);
        let score = (item_time * rate) as usize;
        let max_score = key.get_max_score(type_path, short_path, map);
        (key, score, max_score)
      })
      .collect::<Vec<_>>()
  }
}

#[derive(Debug)]
pub struct Switches {
  pub list: Vec<RefCell<Switch>>,
}

impl Switches {
  pub fn new(list: Vec<RefCell<Switch>>) -> Self {
    Switches { list }
  }
  pub fn get_rate_keys(&self) -> Vec<Ref<Switch>> {
    self
      .list
      .iter()
      .map(|item| item.borrow())
      .filter(|item| item.rate != 0 || item.name == "AA")
      .collect::<Vec<_>>()
  }
  pub fn get_value(&self, name: &str) -> Option<Ref<Switch>> {
    for item in self.list.iter() {
      if item.borrow().name == *name {
        return Some(item.borrow());
      }
    }
    None
  }
  pub fn get_refcell(&self, name: &str) -> Option<&RefCell<Switch>> {
    for item in self.list.iter() {
      if item.borrow().name == *name {
        return Some(item);
      }
    }

    None
  }
}

#[derive(Debug)]
pub struct Switch {
  pub name: String,
  pub rate: u8,
  pub to: Vec<String>,
}

impl Switch {
  pub fn from_str(s: &str) -> Self {
    let regex = regex::Regex::new(
            r"Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (\w+[, \w+]*)",
        )
        .unwrap();
    let m = regex.captures(s).unwrap();
    let (_, match_list): (&str, [&str; 3]) = m.extract();

    Switch {
      name: match_list[0].to_string(),
      rate: match_list[1].parse().unwrap(),
      to: match_list[2].split(", ").map(|s| s.to_string()).collect(),
    }
  }
}
