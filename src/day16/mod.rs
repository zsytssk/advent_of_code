#![allow(unused)]
use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    cmp::Ordering,
    collections::HashMap,
};

use crate::utils::read_file;

mod value;

use value::*;

type PathMap = HashMap<String, (usize, i32)>;

pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let map = parse_input();
    let cur_value = map.get_value("AA");
    if cur_value.is_none() {
        panic!();
    }
    let cur_path = "AA";
    let mut path_map: PathMap = HashMap::new();
    path_map.insert(String::from(cur_path), (0, 30));
    find_path(cur_value.unwrap(), &map, cur_path, &mut path_map);

    println!("path_map:{:?}", path_map);
}

fn find_path<'a>(
    cur_value: Ref<Value>,
    map: &Map,
    cur_path: &str,
    path_map: &mut PathMap,
) {
    let cur_info = match path_map.get(cur_path) {
        Some(info) => info.clone(),
        None => return,
    };
    println!("cur_path={:?} | cur_info={:?}", cur_path, cur_info);

    let mut find_deep = false;
    for item in cur_value.to.iter().rev() {
        let value = match map.get_value(&item) {
            Some(value) => value,
            None => return,
        };
        let not_open = match cur_path.find(item) {
            Some(_) => false,
            None => true,
        };
        let (mut cur_score, mut cur_time) = cur_info.clone();
        let key = format!("{}-{}", cur_path, item);
        cur_time -= 1;
        let rate = value.rate as usize;
        if rate > 0 && not_open {
            cur_time -= 1;
            cur_score += rate * cur_time as usize;
        }
        if cur_time <= 0 {
            continue;
        }
        find_deep = true;
        path_map.insert(key.clone(), (cur_score, cur_time));
        find_path(value, map, &key, path_map);
    }

    if find_deep {
        path_map.remove(cur_path);
    }
}

fn parse_input() -> Map {
    let content = read_file("day16/demo.txt").unwrap();

    let list = content
        .split("\n")
        .map(Value::from_str)
        .map(|item| RefCell::new(item))
        .collect::<Vec<_>>();

    Map::new(list)
}
