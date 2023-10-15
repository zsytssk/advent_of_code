use std::cell::{Ref, RefCell, RefMut};

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
