use std::{
    cell::{Ref, RefCell, RefMut},
    cmp::{self, Ordering},
    collections::HashMap,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MapKey {
    path: Vec<String>,
    time: i32,
    complete_path_size: usize,
}

impl MapKey {
    pub fn new(
        path: Vec<String>,
        time: i32,
        complete_path_size: usize,
    ) -> Self {
        MapKey {
            path,
            time,
            complete_path_size,
        }
    }
    pub fn get_time(&self) -> i32 {
        self.time
    }
    pub fn set_time(&self, new_time: i32) {
        self.time = new_time;
    }
    pub fn is_complete(&self) -> bool {
        self.time <= 0 && self.complete_path_size == self.path.len()
    }
    pub fn get_max_score(
        &self,
        short_path: &HashMap<(String, String), usize>,
        map: &Switches,
    ) -> usize {
        let all_keys = map.get_rate_keys();
        let time = self.time.clone();
        let path = &self.path;
        let last_key = path[path.len() - 1].clone();
        let mut rest_keys = vec![];
        for item in all_keys {
            let switch = item;
            if path.contains(&switch.name) {
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

        let mut change_time = time;
        let mut score = 0;
        for (index, item) in rest_keys.iter().enumerate() {
            let (name, rate, cost_time) = item;
            if index == 0 {
                change_time = change_time - cost_time - 1;
            } else {
                change_time = cmp::min(time - cost_time - 1, change_time - 2);
            }
            if change_time <= 0 {
                break;
            }
            score += time * rate;
        }

        score as usize
    }
    pub fn get_next_keys(
        &self,
        all_keys: &Vec<Ref<Switch>>,
        short_path: &HashMap<(String, String), usize>,
        map: &Switches,
    ) -> Vec<(MapKey, usize)> {
        let path = &self.path;
        let time = &self.time.clone();
        let last_key = path[path.len() - 1].clone();
        let mut rest_keys = vec![];
        for switch in all_keys {
            if path.contains(&switch.name) {
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
            .into_iter()
            .filter(|item| {
                let (name, rate, cost_time) = item;
                let item_time = (time - cost_time - 1);
                return item_time >= 0;
            })
            .map(|item| {
                let (name, rate, cost_time) = item;
                let mut new_keys = self.path.clone();
                new_keys.push(name);
                let time = time - cost_time - 1;
                let score = (time * rate) as usize;
                (
                    MapKey::new(new_keys, time - cost_time - 1, all_keys.len()),
                    score,
                )
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
        self.list
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
