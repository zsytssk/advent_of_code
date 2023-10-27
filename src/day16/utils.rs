use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

use super::map::{MapKey, Switch, Switches};

pub fn get_short_path(
    path_arr: &Vec<&RefCell<Switch>>,
    map: &Switches,
) -> HashMap<(String, String), usize> {
    let mut short_path: HashMap<(String, String), usize> = HashMap::new();

    let big_num = map.list.len();

    for i in 0..path_arr.len() {
        let from = path_arr[i].borrow();
        for j in 0..path_arr.len() {
            let to = path_arr[j].borrow();
            if i == j {
                continue;
            }
            match find_short_path(&from, &to, &map) {
                None => {
                    panic!("cant find path for {}->{}", from.name, to.name);
                }
                Some(t) => {
                    short_path.insert((from.name.clone(), to.name.clone()), t);
                }
            }
        }
    }

    short_path
}

fn find_short_path(
    form_switch: &Ref<Switch>,
    to_switch: &Ref<Switch>,
    map: &Switches,
) -> Option<usize> {
    let target_name = to_switch.name.clone();
    let to = &form_switch.to;

    let mut i = 0;
    let mut arr = to.clone();
    let mut temp_map: HashMap<String, usize> = HashMap::new();
    loop {
        i += 1;
        let mut temp_arr = vec![];
        for item in &arr {
            if temp_map.contains_key(item) {
                continue;
            }
            temp_map.insert(item.clone(), i);
            if item == &target_name {
                return Some(i);
            }
            let switch = match map.get_refcell(&item) {
                None => continue,
                Some(t) => t,
            };

            temp_arr.extend(switch.borrow().to.clone());
        }

        arr = temp_arr;
    }

    return None;
}
