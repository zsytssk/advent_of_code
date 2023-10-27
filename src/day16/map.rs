use std::{
    cell::{Ref, RefCell, RefMut},
    cmp::Ordering,
    collections::HashMap,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MapKey {
    path: Vec<String>,
    time: i32,
}

impl MapKey {
    pub fn new(path: Vec<String>, time: i32) -> Self {
        MapKey { path, time }
    }
    pub fn get_time(&self) -> i32 {
        self.time
    }
    pub fn get_next_keys(
        &self,
        all_keys: &Vec<&RefCell<Switch>>,
        short_path: &HashMap<(String, String), usize>,
        map: &Switches,
    ) -> Vec<(MapKey, usize)> {
        let path = &self.path;
        let time = &self.time.clone();
        let last_key = path[path.len() - 1].clone();
        let mut rest_keys = vec![];
        for item in all_keys {
            let switch = item.borrow();
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
                (MapKey::new(new_keys, time - cost_time - 1), score)
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
