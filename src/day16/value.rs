use std::cell::{Ref, RefCell};

#[derive(Debug)]
pub struct Map {
    list: Vec<RefCell<Value>>,
}

impl Map {
    pub fn new(list: Vec<RefCell<Value>>) -> Self {
        Map { list }
    }
    pub fn get_value(&self, name: &str) -> Option<Ref<Value>> {
        for item in self.list.iter() {
            if item.borrow().name == *name {
                return Some(item.borrow());
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct Value {
    pub name: String,
    pub rate: u8,
    pub to: Vec<String>,
}

impl Value {
    pub fn from_str(s: &str) -> Self {
        let regex = regex::Regex::new(
            r"Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (\w+[, \w+]*)",
        )
        .unwrap();
        let m = regex.captures(s).unwrap();
        let (_, match_list): (&str, [&str; 3]) = m.extract();

        Value {
            name: match_list[0].to_string(),
            rate: match_list[1].parse().unwrap(),
            to: match_list[2].split(", ").map(|s| s.to_string()).collect(),
        }
    }
}
